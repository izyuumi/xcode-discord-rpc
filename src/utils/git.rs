use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// Resolve the `git` binary path once and reuse it for subsequent calls.
///
/// We try well-known absolute paths first, then fall back to the bare `git`
/// name so that PATH is still consulted as a last resort. This is important
/// when the app runs as a macOS launch agent where PATH is minimal and may not
/// include the directory that contains `git`.
fn git_command() -> Command {
    static RESOLVED_GIT_PATH: OnceLock<PathBuf> = OnceLock::new();

    let git_path = RESOLVED_GIT_PATH.get_or_init(|| {
        // Prefer well-known absolute paths so the binary is found even when PATH
        // is stripped down (e.g. inside a launchd agent).
        let candidates = ["/usr/bin/git", "/usr/local/bin/git", "/opt/homebrew/bin/git", "git"];

        for candidate in candidates {
            if candidate == "git" || Path::new(candidate).exists() {
                // Ensure the candidate is runnable by validating `git --version`.
                if Command::new(candidate)
                    .arg("--version")
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false)
                {
                    return PathBuf::from(candidate);
                }
            }
        }

        // Final fallback (when no candidate passed the usability check).
        PathBuf::from("git")
    });

    Command::new(git_path)
}

/// Returns the name of the active git branch for the repository containing
/// `project_path`. The path may point to a file or directory; the function
/// walks upward to find the nearest git repository automatically (via
/// `git -C <dir>`).
///
/// Returns `None` when:
/// - `project_path` is empty
/// - the path is not inside a git repository
/// - the repository is in a detached-HEAD state (returns `"HEAD"`)
/// - any other git or I/O error occurs
pub fn get_git_branch(project_path: &str) -> Option<String> {
    if project_path.is_empty() {
        return None;
    }

    // Resolve the directory to run git in: if the path points to a file,
    // use its parent directory so `git -C` receives a directory.
    let dir = {
        let p = Path::new(project_path);
        if p.is_file() {
            p.parent()
                .map(|d| d.to_string_lossy().to_string())
                .unwrap_or_else(|| project_path.to_string())
        } else {
            project_path.to_string()
        }
    };

    let output = git_command()
        .args(["-C", &dir, "rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Detached HEAD is not a meaningful branch name to display.
    if branch.is_empty() || branch == "HEAD" {
        return None;
    }

    Some(branch)
}

/// Truncate a branch name so that the full state string
/// `"{base} • {branch}"` never exceeds Discord's 128-byte limit.
///
/// All length arithmetic is done in UTF-8 bytes (matching Rust's `str::len`)
/// so the result never overflows the limit even when multi-byte characters
/// (such as the `•` separator or `…` ellipsis) are present.
///
/// When the base alone meets or exceeds the limit it is truncated to fit and
/// returned as a `String`. This function always returns a valid `String` and
/// never fails.
pub fn format_branch_state(base: &str, branch: &str) -> String {
    const DISCORD_STATE_LIMIT: usize = 128;
    const SEPARATOR: &str = " • "; // 5 bytes in UTF-8
    const ELLIPSIS: &str = "…"; // 3 bytes in UTF-8

    let base_len = base.len();

    if base_len >= DISCORD_STATE_LIMIT {
        // Edge case: base itself is already at/over the limit.
        return truncate_to_char_boundary(base, DISCORD_STATE_LIMIT).to_string();
    }

    // Use checked_sub to avoid potential underflow when base_len is close to
    // the limit (e.g. base_len == DISCORD_STATE_LIMIT - 1 would leave no room
    // for the separator).
    let available = match DISCORD_STATE_LIMIT.checked_sub(base_len + SEPARATOR.len()) {
        None | Some(0) => {
            // Not enough room for even the separator — return truncated base.
            return truncate_to_char_boundary(base, DISCORD_STATE_LIMIT).to_string();
        }
        Some(n) => n,
    };

    // Empty branch would produce a trailing separator — return base only.
    if branch.is_empty() {
        return base.to_string();
    }

    if branch.len() <= available {
        // Branch fits without any truncation.
        format!("{base}{SEPARATOR}{branch}")
    } else if available > ELLIPSIS.len() {
        // Truncate the branch so that branch_bytes + ellipsis_bytes == available.
        let max_branch_bytes = available - ELLIPSIS.len();
        let truncated = truncate_to_char_boundary(branch, max_branch_bytes);
        format!("{base}{SEPARATOR}{truncated}{ELLIPSIS}")
    } else {
        // Not enough room even for one branch character — omit the branch.
        base.to_string()
    }
}

/// Returns a `&str` slice of `s` whose length in bytes is at most `max_bytes`,
/// truncated at a valid UTF-8 character boundary.
fn truncate_to_char_boundary(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }
    // Walk backwards from max_bytes until we land on a char boundary.
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_branch_state_short() {
        let result = format_branch_state("in MyApp", "main");
        assert_eq!(result, "in MyApp • main");
    }

    #[test]
    fn format_branch_state_truncates_long_branch() {
        let base = "in MyApp";
        // Create a branch name that would exceed the limit
        let long_branch = "a".repeat(200);
        let result = format_branch_state(base, &long_branch);
        assert!(result.len() <= 128);
        assert!(result.contains('…'));
    }

    #[test]
    fn format_branch_state_exactly_at_limit() {
        let base = "in X";
        // Fill remaining space exactly
        let branch = "b".repeat(128 - base.len() - " • ".len());
        let result = format_branch_state(base, &branch);
        assert_eq!(result.len(), 128);
        assert!(!result.contains('…'));
    }

    #[test]
    fn format_branch_state_base_at_limit() {
        // base is exactly 128 bytes — should be returned as-is (truncated to limit)
        let base = "a".repeat(128);
        let result = format_branch_state(&base, "main");
        assert!(result.len() <= 128);
        // Branch should not appear when base alone fills the limit
        assert!(!result.contains("main"));
    }

    #[test]
    fn format_branch_state_base_exceeds_limit() {
        // base exceeds 128 bytes — should be truncated
        let base = "a".repeat(200);
        let result = format_branch_state(&base, "main");
        assert!(result.len() <= 128);
    }

    #[test]
    fn format_branch_state_multibyte_branch() {
        // Japanese characters are 3 bytes each in UTF-8
        let base = "in MyApp";
        let branch = "機能/新しいブランチ"; // multibyte UTF-8 branch name
        let result = format_branch_state(base, branch);
        assert!(result.len() <= 128);
    }

    #[test]
    fn format_branch_state_empty_branch() {
        // Empty branch name — should return base without trailing separator
        let result = format_branch_state("in MyApp", "");
        assert_eq!(result, "in MyApp");
        assert!(result.len() <= 128);
    }
}
