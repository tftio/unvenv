//! unvenv - Python venv detector CLI
//!
//! Scans the current Git working tree for non-ignored `pyvenv.cfg` files
//! and exits with error status if any are found, preventing accidental
//! commits of Python virtual environments.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use git2::Repository;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process,
};
use walkdir::WalkDir;

/// Application version from Cargo.toml
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Information extracted from a pyvenv.cfg file
#[derive(Debug)]
struct VenvInfo {
    path: PathBuf,
    home: Option<String>,
    version: Option<String>,
    include_system_site_packages: Option<String>,
}

/// Python virtual environment detector CLI
#[derive(Parser)]
#[command(name = "unvenv")]
#[command(about = "Python virtual environment detector CLI")]
#[command(version = VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show version information
    Version,
    /// Scan for unignored Python virtual environments (default)
    Scan,
}

fn main() {
    let exit_code = match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            1
        }
    };
    process::exit(exit_code);
}

fn run() -> Result<i32> {
    let cli = Cli::parse();

    // Check if stdout is a TTY for decoration
    let is_tty = atty::is(atty::Stream::Stdout);

    match cli.command {
        Some(Commands::Version) => {
            if is_tty {
                println!("{} {}", "unvenv".green().bold(), VERSION);
            } else {
                println!("unvenv {VERSION}");
            }
            Ok(0)
        }
        Some(Commands::Scan) | None => {
            // Default behavior: scan for venv files
            scan_for_venvs(is_tty)
        }
    }
}

fn scan_for_venvs(is_tty: bool) -> Result<i32> {
    // Discover Git repository from current directory
    let Ok(repo) = Repository::discover(".") else {
        // Not in a Git repository - exit successfully doing nothing
        return Ok(0);
    };

    // Check if repository is bare
    if repo.is_bare() {
        // Bare repository - exit successfully doing nothing
        return Ok(0);
    }

    // Get working directory root
    let workdir = repo
        .workdir()
        .context("Failed to get repository working directory")?;

    // Find all pyvenv.cfg files in the working tree
    let mut unignored_venvs = Vec::new();

    for entry in WalkDir::new(workdir)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            // Skip .git directory
            e.file_name().to_str() != Some(".git")
        })
    {
        let entry = entry.context("Failed to read directory entry")?;

        // Check if this is a pyvenv.cfg file
        if entry.file_name() == "pyvenv.cfg" && entry.file_type().is_file() {
            let full_path = entry.path();

            // Get path relative to repository workdir
            let rel_path = full_path
                .strip_prefix(workdir)
                .context("Failed to create relative path")?;

            // Check if file is ignored by Git
            let is_ignored = repo
                .status_should_ignore(rel_path)
                .context("Failed to check Git ignore status")?;

            if !is_ignored {
                // Parse the pyvenv.cfg file
                let venv_info = parse_pyvenv_cfg(full_path, rel_path)?;
                unignored_venvs.push(venv_info);
            }
        }
    }

    // Handle results
    if unignored_venvs.is_empty() {
        // No unignored venv files found
        Ok(0)
    } else {
        // Found unignored venv files - print helpful output and exit with error
        print_violation_report(&unignored_venvs, is_tty);
        Ok(2)
    }
}

/// Parse a pyvenv.cfg file to extract useful metadata
fn parse_pyvenv_cfg(full_path: &Path, rel_path: &Path) -> Result<VenvInfo> {
    let content = fs::read_to_string(full_path)
        .with_context(|| format!("Failed to read {}", rel_path.display()))?;

    let mut fields = HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            fields.insert(key.to_string(), value.to_string());
        }
    }

    Ok(VenvInfo {
        path: rel_path.to_path_buf(),
        home: fields.get("home").cloned(),
        version: fields.get("version").cloned(),
        include_system_site_packages: fields.get("include-system-site-packages").cloned(),
    })
}

/// Print a helpful report about policy violations
#[allow(clippy::too_many_lines)]
fn print_violation_report(venvs: &[VenvInfo], is_tty: bool) {
    if is_tty {
        println!(
            "{} Found Python virtual environment files that are not ignored by Git!",
            "WARNING:".yellow().bold()
        );
        println!();
        println!("Python virtual environments should not be committed to version control.");
        println!("They contain system-specific paths and can be large and unnecessary.");
        println!();

        println!(
            "{}",
            "Found the following unignored pyvenv.cfg files:".bold()
        );
        println!();

        for venv in venvs {
            println!("  📁 {}", venv.path.display().to_string().cyan());

            if let Some(home) = &venv.home {
                println!("     Python home: {home}");
            }
            if let Some(version) = &venv.version {
                println!("     Python version: {version}");
            }
            if let Some(include_sys) = &venv.include_system_site_packages {
                println!("     Include system packages: {include_sys}");
            }
            println!();
        }

        // Suggest gitignore entries
        let mut suggested_ignores = std::collections::HashSet::new();
        for venv in venvs {
            if let Some(parent) = venv.path.parent() {
                if let Some(dir_name) = parent.file_name() {
                    if let Some(dir_str) = dir_name.to_str() {
                        suggested_ignores.insert(format!("{dir_str}/"));
                    }
                }
            }
        }

        if !suggested_ignores.is_empty() {
            println!("{}", "Suggested .gitignore entries:".bold());
            println!();
            for ignore_entry in suggested_ignores {
                println!("  {}", ignore_entry.green());
            }
            println!();
        }

        println!("To fix this issue:");
        println!("1. Add the virtual environment directories to your .gitignore file");
        println!("2. If already committed, remove them from the index:");
        for venv in venvs {
            if let Some(parent) = venv.path.parent() {
                println!(
                    "   {}",
                    format!("git rm -r --cached {}", parent.display()).yellow()
                );
            }
        }
        println!("2. If already committed, remove them from the index:");
        for venv in venvs {
            if let Some(parent) = venv.path.parent() {
                println!(
                    "   {}",
                    format!("git rm -r --cached {}", parent.display()).yellow()
                );
            }
        }
    } else {
        // Non-TTY output: plain text without colors or decorations
        println!("WARNING: Found Python virtual environment files that are not ignored by Git!");
        println!();
        println!("Python virtual environments should not be committed to version control.");
        println!();

        println!("Found the following unignored pyvenv.cfg files:");
        for venv in venvs {
            println!("  {}", venv.path.display());
            if let Some(home) = &venv.home {
                println!("    Python home: {home}");
            }
            if let Some(version) = &venv.version {
                println!("    Python version: {version}");
            }
            if let Some(include_sys) = &venv.include_system_site_packages {
                println!("    Include system packages: {include_sys}");
            }
        }
        println!();

        // Suggest gitignore entries
        let mut suggested_ignores = std::collections::HashSet::new();
        for venv in venvs {
            if let Some(parent) = venv.path.parent() {
                if let Some(dir_name) = parent.file_name() {
                    if let Some(dir_str) = dir_name.to_str() {
                        suggested_ignores.insert(format!("{dir_str}/"));
                    }
                }
            }
        }

        if !suggested_ignores.is_empty() {
            println!("Suggested .gitignore entries:");
            for ignore_entry in suggested_ignores {
                println!("  {ignore_entry}");
            }
            println!();
        }

        println!("To fix this issue:");
        println!("1. Add the virtual environment directories to your .gitignore file");
        println!("2. If already committed, remove them from the index:");
        for venv in venvs {
            if let Some(parent) = venv.path.parent() {
                println!("   git rm -r --cached {}", parent.display());
            }
        }
    }
    println!("3. Commit the .gitignore changes");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_version_constant() {
        #[allow(clippy::len_zero)]
        {
            assert!(VERSION.len() > 0); // Check VERSION has content
        }
        assert!(VERSION.chars().next().unwrap().is_ascii_digit());
    }

    #[test]
    fn test_parse_pyvenv_cfg() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let pyvenv_path = temp_dir.path().join("pyvenv.cfg");

        let content = r"home = /usr/bin
include-system-site-packages = false
version = 3.9.7
";
        fs::write(&pyvenv_path, content)?;

        let info = parse_pyvenv_cfg(&pyvenv_path, Path::new("test/pyvenv.cfg"))?;

        assert_eq!(info.home, Some("/usr/bin".to_string()));
        assert_eq!(info.version, Some("3.9.7".to_string()));
        assert_eq!(info.include_system_site_packages, Some("false".to_string()));

        Ok(())
    }

    #[test]
    fn test_parse_empty_pyvenv_cfg() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let pyvenv_path = temp_dir.path().join("pyvenv.cfg");

        fs::write(&pyvenv_path, "")?;

        let info = parse_pyvenv_cfg(&pyvenv_path, Path::new("test/pyvenv.cfg"))?;

        assert_eq!(info.home, None);
        assert_eq!(info.version, None);
        assert_eq!(info.include_system_site_packages, None);

        Ok(())
    }
}
