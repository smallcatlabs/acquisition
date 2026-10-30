#!/usr/bin/env bash
# windows-cc-stub.sh — the C compiler, C++ compiler and archiver the gate's
# `winclippy` stage names for the Windows target, standing in for MSVC's,
# which a Mac or Linux host lacks (#106). It compiles nothing: for each output it is
# named, it writes an empty file, so a build script that runs `cc` finishes
# and clippy reads the Rust above it. Nothing it writes is linked, because
# clippy links nothing; a build that links through it is broken by design.
#
# The output forms it reads: `-o <file>` and `-o<file>` (a compiler driver),
# `-Fo<file>`, `/Fo<file>` and their `:` forms (cl), `-out:<file>` and
# `/OUT:<file>` (lib), and an `ar`-style archive, the first argument after
# a mode word (`crs`). Every other argument is ignored, and a run that names
# no output (a version or family probe) writes nothing and succeeds.
set -euo pipefail
out=()
# `ar crs <archive> <objects>`: the archive follows the mode word.
if (($# >= 2)) && [[ $1 =~ ^-?[crsuqtvD]+$ ]]; then out+=("$2"); fi
prev=''
for a in "$@"; do
  if [[ $prev == -o ]]; then out+=("$a"); prev=''; continue; fi
  case $a in
    -o) prev=-o ;;
    [-/][Oo][Uu][Tt]:?*) out+=("${a:5}") ;;
    -o?*) out+=("${a#-o}") ;;
    [-/][Ff]o:?*) out+=("${a:4}") ;;
    [-/][Ff]o?*) out+=("${a:3}") ;;
  esac
done
for f in ${out[@]+"${out[@]}"}; do
  mkdir -p "$(dirname "$f")"
  : >"$f"
done
