//! Git diff integration for gutter decorations.
//!
//! Runs `git diff HEAD` for a tracked file and parses unified diff hunks into
//! per-line statuses keyed by **1-based** line numbers in the working tree file.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Status of a line in the working tree relative to `HEAD`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiffStatus {
    Added,
    Modified,
    Deleted,
}

/// Find the git repository root containing `path`, if any.
pub fn git_repo_root(path: &Path) -> Option<PathBuf> {
    let dir = path.parent()?;
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if root.is_empty() {
        return None;
    }
    Some(PathBuf::from(root))
}

/// Run `git diff HEAD` with `-U0` and parse the result into line → status (1-based lines).
pub fn line_map_for_path(path: &Path) -> HashMap<usize, DiffStatus> {
    let Some(repo_root) = git_repo_root(path) else {
        return HashMap::new();
    };
    let output = match Command::new("git")
        .current_dir(&repo_root)
        .args(["diff", "--no-color", "-U0", "HEAD", "--"])
        .arg(path)
        .output()
    {
        Ok(o) => o,
        Err(_) => return HashMap::new(),
    };
    if !output.status.success() {
        return HashMap::new();
    }
    let text = String::from_utf8_lossy(&output.stdout);
    parse_unified_diff(&text)
}

/// Parse unified diff text into a map of **1-based** new-file line numbers to [`DiffStatus`].
pub fn parse_unified_diff(diff_output: &str) -> HashMap<usize, DiffStatus> {
    let mut map = HashMap::new();
    let mut lines = diff_output.lines().peekable();

    while lines.peek().is_some() {
        let line = lines.next().unwrap();
        if let Some((new_start, _new_len)) = parse_hunk_header(line) {
            apply_hunk(&mut lines, new_start, &mut map);
        }
    }

    map
}

fn parse_hunk_header(line: &str) -> Option<(usize, usize)> {
    let rest = line.strip_prefix("@@")?.trim_start();
    let (old_chunk, new_chunk) = rest.split_once(" +")?;
    let new_chunk = new_chunk.trim_start_matches('+');
    let (new_chunk, _) = new_chunk.split_once(" @@")?;

    let old_chunk = old_chunk.trim_start_matches('-').trim();
    let _old_range = parse_range(old_chunk);
    let new_range = parse_range(new_chunk);
    Some(new_range)
}

fn parse_range(spec: &str) -> (usize, usize) {
    let spec = spec.trim();
    let mut parts = spec.split(',');
    let start = parts
        .next()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1);
    let len = parts
        .next()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1);
    (start, len)
}

fn apply_hunk(
    lines: &mut std::iter::Peekable<std::str::Lines<'_>>,
    new_start: usize,
    map: &mut HashMap<usize, DiffStatus>,
) {
    let mut new_line = new_start;
    let mut pending_minus = 0usize;

    while let Some(&peek) = lines.peek() {
        if peek.starts_with("@@")
            || peek.starts_with("diff --git")
            || peek.starts_with("--- ")
            || peek.starts_with("+++ ")
        {
            break;
        }

        let line = lines.next().unwrap();

        if line.starts_with('\\') {
            continue;
        }

        let Some(first) = line.chars().next() else {
            continue;
        };

        match first {
            ' ' => {
                flush_deletions(new_line, pending_minus, map);
                pending_minus = 0;
                new_line = new_line.saturating_add(1);
            }
            '-' => {
                pending_minus = pending_minus.saturating_add(1);
            }
            '+' => {
                if pending_minus > 0 {
                    map.insert(new_line, DiffStatus::Modified);
                    pending_minus = 0;
                } else {
                    map.insert(new_line, DiffStatus::Added);
                }
                new_line = new_line.saturating_add(1);
            }
            _ => {}
        }
    }

    flush_deletions(new_line, pending_minus, map);
}

fn flush_deletions(new_line: usize, pending_minus: usize, map: &mut HashMap<usize, DiffStatus>) {
    if pending_minus == 0 {
        return;
    }
    let key = new_line.max(1);
    map.insert(key, DiffStatus::Deleted);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pure_addition() {
        let diff = "\
diff --git a/x b/x
--- a/x
+++ b/x
@@ -0,0 +1,2 @@
+alpha
+beta
";
        let m = parse_unified_diff(diff);
        assert_eq!(m.get(&1), Some(&DiffStatus::Added));
        assert_eq!(m.get(&2), Some(&DiffStatus::Added));
    }

    #[test]
    fn parse_single_line_replacement() {
        let diff = "\
@@ -3 +3 @@
-old
+new
";
        let m = parse_unified_diff(diff);
        assert_eq!(m.get(&3), Some(&DiffStatus::Modified));
    }

    #[test]
    fn parse_two_removed_one_added() {
        let diff = "\
@@ -5,3 +5,2 @@
 keep
-a
-b
+ab
";
        let m = parse_unified_diff(diff);
        assert_eq!(m.get(&6), Some(&DiffStatus::Modified));
    }

    #[test]
    fn parse_pure_deletion_hunk() {
        let diff = "\
@@ -4,2 +3,0 @@
-removed1
-removed2
";
        let m = parse_unified_diff(diff);
        assert_eq!(m.get(&3), Some(&DiffStatus::Deleted));
    }

    #[test]
    fn parse_add_after_context() {
        let diff = "\
@@ -10 +10,2 @@
 context
+inserted
";
        let m = parse_unified_diff(diff);
        assert_eq!(m.get(&11), Some(&DiffStatus::Added));
    }

    #[test]
    fn parse_no_trailing_newline_marker_skipped() {
        let diff = "\
@@ -1 +1 @@
-a
+b
\\ No newline at end of file
";
        let m = parse_unified_diff(diff);
        assert_eq!(m.get(&1), Some(&DiffStatus::Modified));
    }
}
