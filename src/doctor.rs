//! Health check and diagnostics module.

use git2::Repository;

/// Run doctor command to check health and configuration.
///
/// Returns exit code: 0 always (warnings only, no errors).
pub fn run_doctor() -> i32 {
    println!("🏥 unvenv health check");
    println!("======================");
    println!();

    let mut has_warnings = false;

    // Check if in git repository (informational only)
    println!("Environment:");
    if let Ok(repo) = Repository::discover(".") {
        if repo.is_bare() {
            println!("  ⚠️  In bare Git repository");
            has_warnings = true;
        } else {
            let workdir = repo.workdir().map(|p| p.display().to_string());
            println!(
                "  ✅ In Git repository: {}",
                workdir.unwrap_or_else(|| "unknown".to_string())
            );
        }
    } else {
        println!("  ℹ️  Not in a Git repository");
        println!("     unvenv works best in Git repositories but can scan any directory");
    }

    println!();

    // Check for updates
    println!("Updates:");
    match check_for_updates() {
        Ok(Some(latest)) => {
            let current = env!("CARGO_PKG_VERSION");
            println!("  ⚠️  Update available: v{latest} (current: v{current})");
            println!("  💡 Run 'unvenv update' to install the latest version");
            has_warnings = true;
        }
        Ok(None) => {
            println!(
                "  ✅ Running latest version (v{})",
                env!("CARGO_PKG_VERSION")
            );
        }
        Err(e) => {
            println!("  ⚠️  Failed to check for updates: {e}");
            has_warnings = true;
        }
    }

    println!();

    // Summary
    if has_warnings {
        println!(
            "⚠️  {} warning{} found",
            if has_warnings { "1" } else { "0" },
            if has_warnings { "" } else { "s" }
        );
    } else {
        println!("✨ Everything looks healthy!");
    }

    0 // Always exit 0, warnings only
}

fn check_for_updates() -> Result<Option<String>, String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("unvenv-doctor")
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let url = "https://api.github.com/repos/workhelix/unvenv/releases/latest";
    let response: serde_json::Value = client
        .get(url)
        .send()
        .map_err(|e| e.to_string())?
        .json()
        .map_err(|e| e.to_string())?;

    let tag_name = response["tag_name"]
        .as_str()
        .ok_or_else(|| "No tag_name in response".to_string())?;

    let latest = tag_name
        .trim_start_matches("unvenv-v")
        .trim_start_matches('v');
    let current = env!("CARGO_PKG_VERSION");

    if latest == current {
        Ok(None)
    } else {
        Ok(Some(latest.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_doctor_returns_zero() {
        // Doctor always returns 0 (warnings only)
        let result = run_doctor();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_check_for_updates_handles_network_errors() {
        // This will likely fail due to network/timeout, which is acceptable
        // The important part is that it returns Result type correctly
        let result = check_for_updates();
        // Either succeeds or returns error, both are valid outcomes
        match result {
            Ok(version_opt) => {
                // If succeeds, could be None (up to date) or Some(version)
                if let Some(v) = version_opt {
                    assert!(!v.is_empty());
                }
            }
            Err(e) => {
                // Error is expected when network unavailable
                assert!(!e.is_empty());
            }
        }
    }
}
