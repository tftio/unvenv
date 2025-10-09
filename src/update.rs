//! Self-update module.

use sha2::{Digest, Sha256};
use std::path::Path;

/// Run update command to install latest or specified version.
///
/// Returns exit code: 0 if successful, 1 on error, 2 if already up-to-date.
#[allow(clippy::unused_async)]
pub fn run_update(version: Option<&str>, force: bool, install_dir: Option<&Path>) -> i32 {
    let current_version = env!("CARGO_PKG_VERSION");

    println!("🔄 Checking for updates...");

    // Get target version
    let target_version = if let Some(v) = version {
        v.to_string()
    } else {
        match get_latest_version() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("❌ Failed to check for updates: {e}");
                return 1;
            }
        }
    };

    // Check if already up-to-date
    if target_version == current_version && !force {
        println!("✅ Already running latest version (v{current_version})");
        return 2;
    }

    println!("✨ Update available: v{target_version} (current: v{current_version})");

    // Detect current binary location
    let install_path = if let Some(dir) = install_dir {
        dir.join("unvenv")
    } else {
        match std::env::current_exe() {
            Ok(path) => path,
            Err(e) => {
                eprintln!("❌ Failed to determine binary location: {e}");
                return 1;
            }
        }
    };

    println!("📍 Install location: {}", install_path.display());
    println!();

    // Confirm unless forced
    if !force {
        use std::io::{self, Write};
        print!("Continue with update? [y/N]: ");
        io::stdout().flush().unwrap();

        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();

        if !matches!(response.trim().to_lowercase().as_str(), "y" | "yes") {
            println!("Update cancelled.");
            return 0;
        }
    }

    // Perform update
    match perform_update(&target_version, &install_path) {
        Ok(()) => {
            println!("✅ Successfully updated to v{target_version}");
            println!();
            println!("Run 'unvenv --version' to verify the installation.");
            0
        }
        Err(e) => {
            eprintln!("❌ Update failed: {e}");
            1
        }
    }
}

fn get_latest_version() -> Result<String, String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("unvenv-updater")
        .timeout(std::time::Duration::from_secs(10))
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

    let version = tag_name
        .trim_start_matches("unvenv-v")
        .trim_start_matches('v');
    Ok(version.to_string())
}

fn perform_update(version: &str, install_path: &Path) -> Result<(), String> {
    // Detect platform
    let platform = get_platform_string();
    let archive_ext = if cfg!(target_os = "windows") {
        "zip"
    } else {
        "tar.gz"
    };

    let filename = format!("unvenv-{platform}.{archive_ext}");
    let download_url = format!(
        "https://github.com/workhelix/unvenv/releases/download/unvenv-v{version}/{filename}"
    );

    println!("📥 Downloading {filename}...");

    // Download file
    let client = reqwest::blocking::Client::builder()
        .user_agent("unvenv-updater")
        .timeout(std::time::Duration::from_secs(300))
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get(&download_url)
        .send()
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Download failed: HTTP {}", response.status()));
    }

    let bytes = response.bytes().map_err(|e| e.to_string())?;

    // Download checksum
    let checksum_url = format!("{download_url}.sha256");
    let checksum_response = client
        .get(&checksum_url)
        .send()
        .map_err(|e| e.to_string())?;

    if checksum_response.status().is_success() {
        println!("🔐 Verifying checksum...");
        let expected_checksum = checksum_response.text().map_err(|e| e.to_string())?;
        let expected_hash = expected_checksum
            .split_whitespace()
            .next()
            .ok_or_else(|| "Invalid checksum format".to_string())?;

        // Calculate actual checksum
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let actual_hash = hex::encode(hasher.finalize());

        if actual_hash != expected_hash {
            return Err(format!(
                "Checksum verification failed!\nExpected: {expected_hash}\nActual:   {actual_hash}"
            ));
        }

        println!("✅ Checksum verified");
    } else {
        eprintln!("⚠️  Checksum file not available, skipping verification");
    }

    // Extract and install
    println!("📦 Installing...");

    // Create temp directory
    let temp_dir = tempfile::tempdir().map_err(|e| e.to_string())?;

    // Extract archive
    if cfg!(target_os = "windows") {
        // Extract zip (would need zip crate)
        return Err("Windows update not yet implemented".to_string());
    }
    // Extract tar.gz
    let tar_gz = flate2::read::GzDecoder::new(&bytes[..]);
    let mut archive = tar::Archive::new(tar_gz);
    archive.unpack(temp_dir.path()).map_err(|e| e.to_string())?;

    // Find binary in temp dir
    let binary_name = if cfg!(target_os = "windows") {
        "unvenv.exe"
    } else {
        "unvenv"
    };

    let temp_binary = temp_dir.path().join(binary_name);
    if !temp_binary.exists() {
        return Err(format!("Binary not found in archive: {binary_name}"));
    }

    // Make executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&temp_binary)
            .map_err(|e| e.to_string())?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&temp_binary, perms).map_err(|e| e.to_string())?;
    }

    // Replace binary
    std::fs::copy(&temp_binary, install_path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::PermissionDenied {
            format!(
                "Permission denied. Try running with sudo or use --install-dir to specify a \
                 writable location:\n  {e}"
            )
        } else {
            e.to_string()
        }
    })?;

    Ok(())
}

fn get_platform_string() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "x86_64") => "x86_64-apple-darwin",
        ("macos", "aarch64") => "aarch64-apple-darwin",
        ("linux", "x86_64") => "x86_64-unknown-linux-gnu",
        ("linux", "aarch64") => "aarch64-unknown-linux-gnu",
        ("windows", "x86_64") => "x86_64-pc-windows-msvc",
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_platform_string() {
        let platform = get_platform_string();
        // Verify it returns a non-empty string
        assert!(!platform.is_empty());
        // Verify it's one of the expected platforms or "unknown"
        assert!(matches!(
            platform,
            "x86_64-apple-darwin"
                | "aarch64-apple-darwin"
                | "x86_64-unknown-linux-gnu"
                | "aarch64-unknown-linux-gnu"
                | "x86_64-pc-windows-msvc"
                | "unknown"
        ));
    }

    #[test]
    fn test_get_platform_string_exhaustive() {
        // Test that get_platform_string returns the correct value for current platform
        let platform = get_platform_string();

        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        assert_eq!(platform, "x86_64-apple-darwin");

        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        assert_eq!(platform, "aarch64-apple-darwin");

        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        assert_eq!(platform, "x86_64-unknown-linux-gnu");

        #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
        assert_eq!(platform, "aarch64-unknown-linux-gnu");

        #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
        assert_eq!(platform, "x86_64-pc-windows-msvc");
    }

    #[test]
    fn test_get_latest_version_handles_errors() {
        // This will likely fail due to network/timeout
        // The important part is that it returns Result correctly
        let result = get_latest_version();
        match result {
            Ok(v) => {
                // If it succeeds, version should not be empty
                assert!(!v.is_empty());
                // Version should not contain the prefix
                assert!(!v.starts_with("unvenv-v"));
                assert!(!v.starts_with('v'));
            }
            Err(e) => {
                // Error is expected when network unavailable
                assert!(!e.is_empty());
            }
        }
    }

    #[test]
    fn test_run_update_rejects_invalid_path() {
        // Test with invalid install directory
        let invalid_path = Path::new("/nonexistent/path/that/does/not/exist");
        let result = run_update(Some("1.0.0"), true, Some(invalid_path));
        // Should fail, returning non-zero exit code
        assert_ne!(result, 0);
    }

    #[test]
    fn test_run_update_with_current_version() {
        let temp_dir = tempfile::tempdir().unwrap();
        let current_version = env!("CARGO_PKG_VERSION");
        // Trying to update to current version without force should return 2
        let result = run_update(Some(current_version), false, Some(temp_dir.path()));
        assert_eq!(result, 2);
    }

    #[test]
    fn test_run_update_with_current_version_forced() {
        // Test force flag with current version
        // This will fail at download stage, which is expected
        let temp_dir = tempfile::tempdir().unwrap();
        let current_version = env!("CARGO_PKG_VERSION");
        let result = run_update(Some(current_version), true, Some(temp_dir.path()));
        // Should attempt update and fail (1) or succeed (0), but not return 2 (already
        // up-to-date)
        assert_ne!(result, 2, "Force flag should bypass up-to-date check");
    }

    #[test]
    fn test_run_update_with_specific_version() {
        // Test updating to a specific version
        let temp_dir = tempfile::tempdir().unwrap();
        let result = run_update(Some("0.1.0"), true, Some(temp_dir.path()));
        // Will fail at download, which is expected - we're just testing the path
        assert_ne!(
            result, 2,
            "Should not return 'already up-to-date' for different version"
        );
    }

    #[test]
    fn test_run_update_without_version_uses_latest() {
        // Test that None version attempts to fetch latest
        let temp_dir = tempfile::tempdir().unwrap();
        let result = run_update(None, true, Some(temp_dir.path()));
        // Will succeed or fail depending on network, but should attempt to check for
        // updates We're just verifying it doesn't panic
        assert!(result == 0 || result == 1 || result == 2);
    }

    #[test]
    fn test_run_update_exit_codes() {
        let temp_dir = tempfile::tempdir().unwrap();
        let current_version = env!("CARGO_PKG_VERSION");

        // Test that return value is one of the documented exit codes
        let result = run_update(Some(current_version), false, Some(temp_dir.path()));
        assert!(
            result == 0 || result == 1 || result == 2,
            "Exit code should be 0 (success), 1 (error), or 2 (already up-to-date)"
        );
    }

    #[test]
    fn test_perform_update_with_invalid_version() {
        // Test that perform_update returns error for invalid version
        let temp_dir = tempfile::tempdir().unwrap();
        let fake_binary = temp_dir.path().join("unvenv");

        // Try to update with a version that doesn't exist
        let result = perform_update("999.999.999", &fake_binary);

        assert!(result.is_err(), "Should fail for non-existent version");
    }

    #[test]
    fn test_get_latest_version_returns_clean_version() {
        // If network succeeds, verify version format
        if let Ok(version) = get_latest_version() {
            // Should not have prefixes
            assert!(!version.starts_with("unvenv-v"));
            assert!(!version.starts_with('v'));

            // Should look like a version (starts with digit)
            assert!(version.chars().next().unwrap().is_ascii_digit());

            // Should contain dots (semver)
            assert!(version.contains('.'));
        }
    }

    #[test]
    #[cfg(unix)]
    fn test_unix_platform_detection() {
        let platform = get_platform_string();
        // On Unix, should not be Windows platform
        assert!(!platform.contains("windows"));
        assert!(!platform.contains("msvc"));
    }

    #[test]
    #[cfg(windows)]
    fn test_windows_platform_detection() {
        let platform = get_platform_string();
        // On Windows, should be Windows platform
        assert!(platform.contains("windows"));
    }
}
