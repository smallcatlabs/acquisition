//! A unified line diff for the golden files (C122), the standard library
//! alone: the protocol's tests link serde and nothing else
//! (`tools/arch-check.sh`), and they include this file by `#[path]`
//! without the harness beside it. A longest-common-subsequence table over
//! the lines; the files it compares are a few hundred lines at most.
#![allow(dead_code)]

/// The lines of `expected` and `actual` as a unified diff, every change
/// with three lines of context, `-` for the file on disk and `+` for this
/// build. Empty when the two are equal.
pub fn unified(expected: &str, actual: &str) -> String {
    let a: Vec<&str> = expected.lines().collect();
    let b: Vec<&str> = actual.lines().collect();
    // lcs[i][j]: the common subsequence of a[i..] and b[j..].
    let mut lcs = vec![vec![0usize; b.len() + 1]; a.len() + 1];
    for i in (0..a.len()).rev() {
        for j in (0..b.len()).rev() {
            lcs[i][j] = if a[i] == b[j] {
                lcs[i + 1][j + 1] + 1
            } else {
                lcs[i + 1][j].max(lcs[i][j + 1])
            };
        }
    }
    // The edit script: ' ', '-' or '+' with its line.
    let mut script = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < b.len() {
        if i < a.len() && j < b.len() && a[i] == b[j] {
            script.push((' ', a[i]));
            i += 1;
            j += 1;
        } else if i < a.len() && (j == b.len() || lcs[i + 1][j] >= lcs[i][j + 1]) {
            script.push(('-', a[i]));
            i += 1;
        } else {
            script.push(('+', b[j]));
            j += 1;
        }
    }
    // Keep the changes and three lines either side; elide the rest.
    const CONTEXT: usize = 3;
    let changed: Vec<usize> = (0..script.len()).filter(|&k| script[k].0 != ' ').collect();
    let near = |k: usize| changed.iter().any(|&c| c.abs_diff(k) <= CONTEXT);
    let mut out = String::new();
    let mut elided = false;
    for (k, (mark, line)) in script.iter().enumerate() {
        if near(k) {
            if elided {
                out.push_str("@@\n");
                elided = false;
            }
            out.push_str(&format!("{mark}{line}\n"));
        } else {
            elided = true;
        }
    }
    out
}

/// Fail naming `path` and printing the diff when `actual` is not
/// `expected`, with `regenerate`, the command that rewrites the file once
/// the change is intended. A trailing newline counts.
pub fn same_text(expected: &str, actual: &str, path: &str, regenerate: &str) {
    if expected == actual {
        return;
    }
    let mut diff = unified(expected, actual);
    if diff.is_empty() {
        diff = "(the lines are equal; the line endings or the final newline differ)\n".into();
    }
    panic!(
        "{path} no longer matches this build (- the file, + this build):\n{diff}\n\
         review the diff, then regenerate with `{regenerate}` if it is intended"
    );
}
