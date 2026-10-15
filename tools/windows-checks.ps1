# windows-checks.ps1 — the study's checks still open on Windows
# (`PLATFORMS-SLICE.md`, observations): a second user, mixed elevation, a
# host that runs its children in a job object. Mock only (C88): every
# daemon here is ACQ_PROVIDER=mock with ACQ_NO_KEYRING=1 in a scratch
# store; nothing talks to GGG or touches a keychain. Each check prints
# PASS, FAIL or MEASURED with its evidence and appends the line to
# C:\acq-check\results.txt, which the Mac session reads back over ssh.
#
#   powershell -ExecutionPolicy Bypass -File tools\windows-checks.ps1 <check> [-Pipe <name>]
#
# Checks, and who runs them:
#   stage         (user A, e.g. acqtest)  copy target\debug's acq.exe, acqd.exe and
#                                         acq-mcp.exe to C:\acq-check\bin for every user
#   serve         (user A)                start A's mock daemon; its pipe name goes to
#                                         C:\acq-check\pipe-<user>.txt
#   reach-other   (user B, e.g. acqtest2) open A's pipe: denied is a PASS (C112: only
#                                         that user can reach it)
#   squat         (user B)                take A's pipe name while A has no daemon, and
#                                         hold it until Enter
#   contend       (user A, while B squats) A's daemon must refuse to start, and A's
#                                         client must not converse with B's pipe
#   stop          (user A)                stop A's mock daemon
#   elevation     (an administrator, run in an unelevated and then an elevated shell,
#                  each way round)        starts or reaches a mock daemon and reports
#                                         what the other elevation sees: MEASURED
#   job           (anyone)                a lazily spawned acqd under a parent in a
#                                         kill-on-close job object: does it outlive the
#                                         job? Three job kinds: MEASURED
#
# B learns A's pipe name from C:\acq-check\pipe-<A>.txt (pass -Pipe to
# override). Run `stage` first, as the user whose build it is; it copies
# this script to C:\acq-check, where every account runs it from.
param(
    [Parameter(Mandatory = $true, Position = 0)]
    [ValidateSet('stage', 'serve', 'reach-other', 'squat', 'contend', 'stop', 'elevation', 'job')]
    [string]$Check,
    [string]$Pipe,
    [string]$Other = 'acqtest'
)
$ErrorActionPreference = 'Stop'
$Root = 'C:\acq-check'
$Bin = Join-Path $Root 'bin'
$Results = Join-Path $Root 'results.txt'
$Me = $env:USERNAME
$IsAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole(
    [Security.Principal.WindowsBuiltInRole]::Administrator)
$Shell = if ($IsAdmin) { 'elevated' } else { 'unelevated' }

function Record([string]$verdict, [string]$what) {
    $line = '{0:u}  {1,-8} {2,-11} {3}/{4}: {5}' -f (Get-Date).ToUniversalTime(), $verdict, $Check, $Me, $Shell, $what
    Write-Host $line
    if (Test-Path $Root) { Add-Content -Path $Results -Value $line -Encoding UTF8 }
}

# The mock, in a scratch store of this user's (its world, and so its pipe,
# is this user's alone).
function Use-Mock([string]$tag = 'store') {
    $base = Join-Path $env:TEMP "acq-check\$tag"
    New-Item -ItemType Directory -Force -Path $base | Out-Null
    foreach ($v in 'ACQ_GGG', 'ACQ_TRIPWIRE', 'ACQ_MAX_SENDS', 'ACQ_IDLE_SHUTDOWN', 'ACQ_ACCOUNT', 'ACQ_NO_SPAWN') {
        Remove-Item "Env:$v" -ErrorAction SilentlyContinue
    }
    $env:ACQ_PROVIDER = 'mock'
    $env:ACQ_NO_KEYRING = '1'
    $env:ACQ_STORE_DIR = Join-Path $base 'store'
    $env:ACQ_LOG_DIR = Join-Path $base 'logs'
    New-Item -ItemType Directory -Force -Path $env:ACQ_STORE_DIR | Out-Null
}

function Acq([string[]]$a) {
    $out = & (Join-Path $Bin 'acq.exe') @a 2>&1 | Out-String
    [pscustomobject]@{ Code = $LASTEXITCODE; Text = $out.Trim() }
}

function Short([string]$pipe) { $pipe -replace '^\\\\\.\\pipe\\', '' }

function Pipe-Of([string]$user) {
    if ($Pipe) { return $Pipe }
    $f = Join-Path $Root "pipe-$user.txt"
    if (-not (Test-Path $f)) { throw "no ${f}: run 'serve' as $user first, or pass -Pipe" }
    (Get-Content -Raw $f).Trim()
}

function Daemon-Alive([int]$daemonPid) {
    [bool](Get-Process -Id $daemonPid -ErrorAction SilentlyContinue)
}

switch ($Check) {
    'stage' {
        $src = Join-Path $PSScriptRoot '..\target\debug'
        New-Item -ItemType Directory -Force -Path $Bin | Out-Null
        foreach ($b in 'acq.exe', 'acqd.exe', 'acq-mcp.exe') { Copy-Item -Force (Join-Path $src $b) $Bin }
        # This script too: the other accounts cannot read the stager's profile.
        Copy-Item -Force $PSCommandPath $Root
        # Every local user may read and run them; only the stager writes.
        icacls $Bin /grant '*S-1-5-32-545:(OI)(CI)RX' | Out-Null
        icacls $Root /grant '*S-1-5-32-545:(OI)(CI)M' | Out-Null
        $v = Acq @('version', '--json')
        Record 'PASS' "staged to ${Bin}: $($v.Text)"
    }

    'serve' {
        Use-Mock
        $r = Acq @('submit', 'sleep', '--detach')
        $s = Acq @('daemon', 'status', '--json')
        $j = $s.Text | ConvertFrom-Json
        if (-not $j.running) { Record 'FAIL' "no daemon came up: $($r.Text) | $($s.Text)"; exit 1 }
        Set-Content -Path (Join-Path $Root "pipe-$Me.txt") -Value $j.socket -Encoding ASCII
        Record 'PASS' "daemon pid $($j.pid) on $($j.socket)"
    }

    'stop' {
        # Both of this user's worlds: serve's and elevation's.
        foreach ($tag in 'store', 'elevation') {
            Use-Mock $tag
            $r = Acq @('daemon', 'stop', '--json')
            Record 'PASS' "stop ($tag): $($r.Text)"
        }
    }

    'reach-other' {
        $name = Pipe-Of $Other
        $c = New-Object System.IO.Pipes.NamedPipeClientStream('.', (Short $name), [System.IO.Pipes.PipeDirection]::InOut)
        try {
            $c.Connect(3000)
            Record 'FAIL' "opened $Other's pipe $name as ${Me}: C112 says only its user can reach it"
            exit 1
        } catch [System.UnauthorizedAccessException] {
            Record 'PASS' "denied opening $Other's pipe $name"
        } catch [System.TimeoutException] {
            Record 'FAIL' "no answer from $name in 3 s: is $Other's daemon serving? ('serve' as $Other)"
            exit 1
        } finally { $c.Dispose() }
    }

    'squat' {
        $name = Pipe-Of $Other
        try {
            $s = New-Object System.IO.Pipes.NamedPipeServerStream((Short $name), [System.IO.Pipes.PipeDirection]::InOut, 1)
        } catch {
            Record 'MEASURED' "could not take $name (is $Other's daemon still up? 'stop' as $Other): $($_.Exception.Message)"
            exit 1
        }
        Record 'PASS' "holding $Other's pipe name $name; run 'contend' as $Other now"
        Read-Host 'Press Enter to release the pipe'
        $s.Dispose()
    }

    'contend' {
        Use-Mock
        $name = Pipe-Of $Me
        # The client first: a squatter's pipe is never conversed with.
        $s = Acq @('daemon', 'status', '--json')
        $talked = $s.Text -match '"running":\s*true'
        if ($talked) { Record 'FAIL' "the client conversed with a pipe it does not own: $($s.Text)" }
        else { Record 'PASS' "the client refused the squatted pipe: $($s.Text)" }
        # Then the daemon: it must refuse to start, naming the pipe.
        $p = Start-Process -FilePath (Join-Path $Bin 'acqd.exe') -PassThru -NoNewWindow `
            -RedirectStandardError (Join-Path $env:TEMP 'acq-check\acqd.err') `
            -RedirectStandardOutput (Join-Path $env:TEMP 'acq-check\acqd.out')
        $null = $p.Handle   # so ExitCode is kept after the exit
        if (-not $p.WaitForExit(10000)) {
            $p.Kill()
            Record 'FAIL' "the daemon did not refuse within 10 s: it serves beside a squatter"
            exit 1
        }
        $err = (Get-Content -Raw (Join-Path $env:TEMP 'acq-check\acqd.err') -ErrorAction SilentlyContinue)
        $log = Get-ChildItem -Recurse $env:ACQ_LOG_DIR -Filter daemon.log -ErrorAction SilentlyContinue |
            ForEach-Object { Get-Content $_.FullName -Tail 3 } | Out-String
        if ($p.ExitCode -ne 0) { Record 'PASS' "the daemon refused (exit $($p.ExitCode)): $($err)$($log)".Trim() }
        else { Record 'FAIL' "the daemon exited 0 beside a squatter: $err$log" ; exit 1 }
    }

    'elevation' {
        # Run in both shells: the first starts (or finds) a daemon, the
        # second reports what it sees of it. The store is this user's in
        # both, so both derive one world and one pipe.
        # One administrator in both shells: an unelevated admin's token
        # still lists Administrators, as deny-only.
        if (-not (whoami /groups | Select-String -SimpleMatch 'S-1-5-32-544')) {
            Record 'FAIL' "$Me is no administrator: run 'elevation' as one admin, unelevated and elevated"
            exit 1
        }
        Use-Mock 'elevation'
        # Held up until 'stop', however long the walk to the other shell.
        $env:ACQ_IDLE_SHUTDOWN = '900'
        $s = Acq @('daemon', 'status', '--json')
        if ($s.Text -match '"running":\s*true') {
            $j = $s.Text | ConvertFrom-Json
            Record 'MEASURED' "reached a running daemon (pid $($j.pid), compatible $($j.compatible)): $($s.Text -replace '\s+', ' ')"
            $r = Acq @('jobs', '--json')
            Record 'MEASURED' "a request over it: exit $($r.Code): $($r.Text -replace '\s+', ' ')"
        } else {
            $r = Acq @('submit', 'sleep', '--detach')
            $s2 = Acq @('daemon', 'status', '--json')
            Record 'MEASURED' "no daemon reachable ($($s.Text -replace '\s+', ' ')); started one: $($r.Text) | $($s2.Text -replace '\s+', ' ')"
            Write-Host "Now run 'elevation' in the other shell (elevated <-> unelevated), then 'stop' here."
        }
    }

    'job' {
        Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class AcqJob {
    [StructLayout(LayoutKind.Sequential)] public struct Basic {
        public long PerProcessUserTimeLimit; public long PerJobUserTimeLimit; public uint LimitFlags;
        public UIntPtr MinimumWorkingSetSize; public UIntPtr MaximumWorkingSetSize; public uint ActiveProcessLimit;
        public UIntPtr Affinity; public uint PriorityClass; public uint SchedulingClass; }
    [StructLayout(LayoutKind.Sequential)] public struct Io { public ulong A, B, C, D, E, F; }
    [StructLayout(LayoutKind.Sequential)] public struct Extended {
        public Basic BasicLimits; public Io IoInfo; public UIntPtr ProcessMemoryLimit; public UIntPtr JobMemoryLimit;
        public UIntPtr PeakProcessMemoryUsed; public UIntPtr PeakJobMemoryUsed; }
    [DllImport("kernel32.dll", SetLastError = true)] static extern IntPtr CreateJobObject(IntPtr a, string name);
    [DllImport("kernel32.dll", SetLastError = true)] static extern bool SetInformationJobObject(IntPtr job, int cls, ref Extended info, int len);
    [DllImport("kernel32.dll", SetLastError = true)] public static extern bool AssignProcessToJobObject(IntPtr job, IntPtr process);
    [DllImport("kernel32.dll", SetLastError = true)] public static extern bool CloseHandle(IntPtr h);
    [DllImport("kernel32.dll", SetLastError = true)] public static extern bool IsProcessInJob(IntPtr process, IntPtr job, out bool result);
    public static IntPtr Make(uint flags) {
        IntPtr job = CreateJobObject(IntPtr.Zero, null);
        if (job == IntPtr.Zero) throw new System.ComponentModel.Win32Exception();
        Extended e = new Extended(); e.BasicLimits.LimitFlags = flags;
        if (!SetInformationJobObject(job, 9, ref e, Marshal.SizeOf(typeof(Extended)))) throw new System.ComponentModel.Win32Exception();
        return job;
    }
}
'@
        $inJob = $false
        [AcqJob]::IsProcessInJob((Get-Process -Id $PID).Handle, [IntPtr]::Zero, [ref]$inJob) | Out-Null
        Record 'MEASURED' "this shell is in a job object already: $inJob"
        # KILL_ON_JOB_CLOSE 0x2000; BREAKAWAY_OK 0x800; SILENT_BREAKAWAY_OK 0x1000
        # (what libuv, under Node-based hosts, sets).
        $kinds = [ordered]@{
            'kill-on-close'                 = [uint32]0x2000
            'kill-on-close, breakaway ok'   = [uint32](0x2000 -bor 0x800)
            'kill-on-close, silent breakaway' = [uint32](0x2000 -bor 0x1000)
        }
        foreach ($kind in $kinds.Keys) {
            Use-Mock "job-$($kinds[$kind])"
            Acq @('daemon', 'stop') | Out-Null
            $job = [AcqJob]::Make($kinds[$kind])
            # The host's child: waits a moment (so it is in the job before
            # it spawns anything), then runs a job command that lazily
            # starts the daemon, then exits.
            $acq = Join-Path $Bin 'acq.exe'
            $psi = New-Object System.Diagnostics.ProcessStartInfo('cmd.exe', "/c ping -n 3 127.0.0.1 >nul & `"$acq`" submit sleep --detach")
            $psi.UseShellExecute = $false
            $psi.CreateNoWindow = $true
            $child = [System.Diagnostics.Process]::Start($psi)
            [AcqJob]::AssignProcessToJobObject($job, $child.Handle) | Out-Null
            $child.WaitForExit()
            $s = Acq @('daemon', 'status', '--json')
            if ($s.Text -notmatch '"running":\s*true') {
                Record 'FAIL' "${kind}: no daemon after the child's submit: $($s.Text -replace '\s+', ' ')"
                [AcqJob]::CloseHandle($job) | Out-Null
                continue
            }
            $daemonPid = ($s.Text | ConvertFrom-Json).pid
            # The host exits: its job handle closes.
            [AcqJob]::CloseHandle($job) | Out-Null
            Start-Sleep -Milliseconds 1500
            $alive = Daemon-Alive $daemonPid
            Record 'MEASURED' "${kind}: the daemon (pid $daemonPid) $(if ($alive) { 'outlived' } else { 'died with' }) the job"
            if ($alive) { Acq @('daemon', 'stop') | Out-Null }
        }
    }
}
