//! The Windows arm of C112: who this user is, a named pipe only this
//! user can open, and whether a pipe's server runs as this user. The
//! world derives the pipe's name ([`crate::world::World::socket_path`]);
//! the daemon creates it under [`with_user_only_attributes`], and a
//! client checks [`server_runs_as_current_user`] before `hello`.
//!
//! # C112 — as built (Windows)
//!
//! A named pipe lives in one machine-wide namespace, so the privacy a
//! Unix socket takes from its 0700 runtime directory is the pipe's own:
//!
//! - **The name** is `\\.\pipe\acq-<SID>-<world id>`: the user's SID
//!   keeps two users on one world apart, the world id keeps two worlds
//!   apart. Derived, never chosen.
//! - **Serving it.** The daemon creates the first instance with a
//!   protected DACL granting this user alone (`D:P(A;;GA;;;<SID>)`): no
//!   other account can open it or create a further instance. A name
//!   another process already holds fails the first-instance create, and
//!   the daemon refuses to start naming the pipe — a squatter costs a
//!   start, never a conversation.
//! - **Reaching it.** A client asks the pipe for its server's process id
//!   and compares that process's token user with its own; anything else
//!   is refused before a byte is written. The pipe's owner is not used:
//!   an elevated server's objects are owned by the Administrators group.
//!
//! Beside them, this user's profile directory from the process token,
//! where the real-mode lock lives (C132, [`profile_dir`]), a file's id
//! for C84's artifact identity ([`file_id`]),
//! and the spawn's guard on this process's std handles
//! ([`disinherit_std_handles`]): the same FFI, kept in one module.
//!
//! Every call here is FFI over `windows-sys`; each unsafe block names the
//! handle or buffer whose lifetime it relies on.

use std::ffi::{OsString, c_void};
use std::io;
use std::os::windows::ffi::OsStringExt as _;
use std::os::windows::io::RawHandle;
use std::path::PathBuf;

use windows_sys::Win32::Foundation::{
    CloseHandle, HANDLE, HANDLE_FLAG_INHERIT, LocalFree, SetHandleInformation,
};
use windows_sys::Win32::Security::Authorization::{
    ConvertSidToStringSidW, ConvertStringSecurityDescriptorToSecurityDescriptorW, SDDL_REVISION_1,
};
use windows_sys::Win32::Security::{
    EqualSid, GetTokenInformation, PSECURITY_DESCRIPTOR, PSID, SECURITY_ATTRIBUTES, TOKEN_QUERY,
    TOKEN_USER, TokenUser,
};
use windows_sys::Win32::Storage::FileSystem::{
    BY_HANDLE_FILE_INFORMATION, GetFileInformationByHandle,
};
use windows_sys::Win32::System::Pipes::GetNamedPipeServerProcessId;
use windows_sys::Win32::System::Threading::{
    GetCurrentProcess, OpenProcess, OpenProcessToken, PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows_sys::Win32::UI::Shell::GetUserProfileDirectoryW;

/// A process's `TOKEN_USER`, owned: the SID it carries points into this
/// buffer, so the buffer outlives every use of [`TokenUserBuf::sid`].
struct TokenUserBuf(Vec<u8>);

impl TokenUserBuf {
    /// The token user of `process`, a handle the caller keeps open for
    /// the duration of the call.
    fn of(process: HANDLE) -> io::Result<TokenUserBuf> {
        let mut token: HANDLE = std::ptr::null_mut();
        // SAFETY: `process` is open for this call; `token` is written on
        // success and closed below.
        if unsafe { OpenProcessToken(process, TOKEN_QUERY, &mut token) } == 0 {
            return Err(io::Error::last_os_error());
        }
        let mut len = 0u32;
        // SAFETY: a null buffer of length 0 asks for the size only.
        unsafe { GetTokenInformation(token, TokenUser, std::ptr::null_mut(), 0, &mut len) };
        let mut buf = vec![0u8; len as usize];
        // SAFETY: `buf` is `len` bytes, as the size query asked for.
        let ok = unsafe {
            GetTokenInformation(
                token,
                TokenUser,
                buf.as_mut_ptr().cast::<c_void>(),
                len,
                &mut len,
            )
        };
        let err = io::Error::last_os_error();
        // SAFETY: `token` was opened above and is closed once.
        unsafe { CloseHandle(token) };
        if ok == 0 || buf.len() < std::mem::size_of::<TOKEN_USER>() {
            return Err(err);
        }
        Ok(TokenUserBuf(buf))
    }

    fn current() -> io::Result<TokenUserBuf> {
        // SAFETY: the pseudo-handle needs no closing and is always valid.
        TokenUserBuf::of(unsafe { GetCurrentProcess() })
    }

    /// The SID, valid while `self` lives.
    fn sid(&self) -> PSID {
        // SAFETY: `of` checked the buffer holds a `TOKEN_USER`; an unaligned
        // read copies the struct out of the byte buffer.
        unsafe { std::ptr::read_unaligned(self.0.as_ptr().cast::<TOKEN_USER>()) }
            .User
            .Sid
    }
}

/// This user's SID as a string (`S-1-5-21-…`): what the pipe's name and
/// its DACL carry.
pub fn current_user_sid() -> io::Result<String> {
    let user = TokenUserBuf::current()?;
    let mut text: *mut u16 = std::ptr::null_mut();
    // SAFETY: the SID lives in `user`; `text` is allocated by the call and
    // freed below with LocalFree.
    if unsafe { ConvertSidToStringSidW(user.sid(), &mut text) } == 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: `text` is a NUL-terminated wide string from the call above.
    let s = unsafe {
        let mut n = 0;
        while *text.add(n) != 0 {
            n += 1;
        }
        String::from_utf16_lossy(std::slice::from_raw_parts(text, n))
    };
    // SAFETY: allocated by ConvertSidToStringSidW, freed once.
    unsafe { LocalFree(text.cast()) };
    Ok(s)
}

/// C132: the profile directory of this process token's user
/// (`GetUserProfileDirectoryW`), which the system reads from the token
/// and the registry — never `%USERPROFILE%` or `%LOCALAPPDATA%`, so no
/// variable a process sets moves the real-mode lock.
pub fn profile_dir() -> io::Result<PathBuf> {
    let mut token: HANDLE = std::ptr::null_mut();
    // SAFETY: the pseudo-handle needs no closing and is always valid;
    // `token` is written on success and closed below.
    if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) } == 0 {
        return Err(io::Error::last_os_error());
    }
    let mut len = 0u32;
    // SAFETY: a null buffer asks for the size, in wide characters with
    // the terminator; `token` is open.
    unsafe { GetUserProfileDirectoryW(token, std::ptr::null_mut(), &mut len) };
    let mut buf = vec![0u16; len as usize];
    let ok = if buf.is_empty() {
        0
    } else {
        // SAFETY: `buf` is `len` wide characters, as the size query
        // asked for; `token` is open.
        unsafe { GetUserProfileDirectoryW(token, buf.as_mut_ptr(), &mut len) }
    };
    let err = io::Error::last_os_error();
    // SAFETY: `token` was opened above and is closed once.
    unsafe { CloseHandle(token) };
    if ok == 0 {
        return Err(io::Error::new(
            err.kind(),
            format!("could not read this user's profile directory from the process token: {err}"),
        ));
    }
    let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    if end == 0 {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "the process token's user has an empty profile directory",
        ));
    }
    Ok(PathBuf::from(OsString::from_wide(&buf[..end])))
}

/// Run `create` with a `SECURITY_ATTRIBUTES` whose protected DACL grants
/// this user alone, freed when `create` returns: what the daemon hands
/// tokio's `create_with_security_attributes_raw` for every instance.
pub fn with_user_only_attributes<T>(
    create: impl FnOnce(*mut c_void) -> io::Result<T>,
) -> io::Result<T> {
    let sid = current_user_sid()?;
    let sddl: Vec<u16> = format!("D:P(A;;GA;;;{sid})\0").encode_utf16().collect();
    let mut sd: PSECURITY_DESCRIPTOR = std::ptr::null_mut();
    // SAFETY: `sddl` is NUL-terminated; `sd` is allocated by the call and
    // freed below with LocalFree.
    if unsafe {
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            sddl.as_ptr(),
            SDDL_REVISION_1,
            &mut sd,
            std::ptr::null_mut(),
        )
    } == 0
    {
        return Err(io::Error::last_os_error());
    }
    let mut attrs = SECURITY_ATTRIBUTES {
        nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
        lpSecurityDescriptor: sd,
        bInheritHandle: 0,
    };
    let made = create((&mut attrs as *mut SECURITY_ATTRIBUTES).cast::<c_void>());
    // SAFETY: allocated above, freed once, after its last use in `create`.
    unsafe { LocalFree(sd.cast()) };
    made
}

/// Whether the server end of the connected pipe `pipe` runs as this
/// user. A server this user may not query is not this user's.
pub fn server_runs_as_current_user(pipe: RawHandle) -> io::Result<bool> {
    let mut pid = 0u32;
    // SAFETY: `pipe` is the caller's open client handle.
    if unsafe { GetNamedPipeServerProcessId(pipe as HANDLE, &mut pid) } == 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: a query-only open of a pid; closed below.
    let process = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) };
    if process.is_null() {
        let e = io::Error::last_os_error();
        return if e.kind() == io::ErrorKind::PermissionDenied {
            Ok(false)
        } else {
            Err(e)
        };
    }
    let server = TokenUserBuf::of(process);
    // SAFETY: opened above, closed once.
    unsafe { CloseHandle(process) };
    let server = match server {
        Ok(server) => server,
        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => return Ok(false),
        Err(e) => return Err(e),
    };
    let me = TokenUserBuf::current()?;
    // SAFETY: both SIDs live in their buffers for the call.
    Ok(unsafe { EqualSid(server.sid(), me.sid()) } != 0)
}

/// The id of an open file: its volume serial number and its file index,
/// from the handle (`crate::world::file_id`, C84).
pub fn file_id(file: &std::fs::File) -> io::Result<(u64, u64)> {
    use std::os::windows::io::AsRawHandle;
    // SAFETY: an all-zero BY_HANDLE_FILE_INFORMATION is a valid value of
    // a plain C struct, overwritten by the call.
    let mut info: BY_HANDLE_FILE_INFORMATION = unsafe { std::mem::zeroed() };
    // SAFETY: `file` is open for the call and `info` is ours to write.
    if unsafe { GetFileInformationByHandle(file.as_raw_handle() as HANDLE, &mut info) } == 0 {
        return Err(io::Error::last_os_error());
    }
    Ok((
        u64::from(info.dwVolumeSerialNumber),
        (u64::from(info.nFileIndexHigh) << 32) | u64::from(info.nFileIndexLow),
    ))
}

/// Make this process's stdin, stdout and stderr non-inheritable, before
/// it spawns a daemon that must outlive it. `CreateProcess` hands a child
/// every inheritable handle, not only its stdio: a daemon that inherited
/// the pipe a caller reads this process's output from would hold it open
/// until the daemon exits, and the caller — a script, a test, an MCP
/// host — would wait that long for end-of-file. A child spawned later
/// with inherited stdio still gets it: `std` duplicates the handle it
/// passes. A handle that is not there (no console) is skipped.
pub fn disinherit_std_handles() {
    use std::os::windows::io::AsRawHandle;
    for handle in [
        std::io::stdin().as_raw_handle(),
        std::io::stdout().as_raw_handle(),
        std::io::stderr().as_raw_handle(),
    ] {
        if !handle.is_null() {
            // SAFETY: a std handle of this process, or a stale value the
            // call refuses; only the inherit flag is changed.
            unsafe { SetHandleInformation(handle as HANDLE, HANDLE_FLAG_INHERIT, 0) };
        }
    }
}
