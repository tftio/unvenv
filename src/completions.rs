//! Shell completion generation module.

use clap::CommandFactory;
use clap_complete::Shell;
use std::io;

use crate::Cli;

/// Generate shell completion scripts.
///
/// Outputs both instructions and the completion script to stdout.
pub fn generate_completions(shell: Shell) {
    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();

    // Print instructions
    println!("# Shell completion for {bin_name}");
    println!("#");
    println!("# To enable completions, add this to your shell config:");
    println!("#");

    match shell {
        Shell::Bash => {
            println!("# For bash (~/.bashrc):");
            println!("#   source <({bin_name} completions bash)");
        }
        Shell::Zsh => {
            println!("# For zsh (~/.zshrc):");
            println!("#   {bin_name} completions zsh > ~/.zsh/completions/_{bin_name}");
            println!("#   # Ensure fpath includes ~/.zsh/completions");
        }
        Shell::Fish => {
            println!("# For fish (~/.config/fish/config.fish):");
            println!("#   {bin_name} completions fish | source");
        }
        _ => {
            println!("# For {shell}:");
            println!("#   {bin_name} completions {shell} > /path/to/completions/_{bin_name}");
        }
    }

    println!();

    // Generate completions
    clap_complete::generate(shell, &mut cmd, bin_name, &mut io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_completions_bash() {
        // Generate completions to a buffer and verify output contains expected strings
        let mut cmd = Cli::command();
        let mut buf = Vec::new();
        clap_complete::generate(Shell::Bash, &mut cmd, "unvenv", &mut buf);

        let output = String::from_utf8(buf).expect("Invalid UTF-8");
        // Verify output contains bash-specific completion code
        assert!(
            output.contains("_unvenv"),
            "Should contain completion function name"
        );
        assert!(output.contains("unvenv"), "Should contain binary name");
    }

    #[test]
    fn test_generate_completions_zsh() {
        let mut cmd = Cli::command();
        let mut buf = Vec::new();
        clap_complete::generate(Shell::Zsh, &mut cmd, "unvenv", &mut buf);

        let output = String::from_utf8(buf).expect("Invalid UTF-8");
        // Verify output contains zsh-specific completion code
        assert!(
            output.contains("#compdef"),
            "Should contain zsh compdef directive"
        );
        assert!(output.contains("unvenv"), "Should contain binary name");
    }

    #[test]
    fn test_generate_completions_fish() {
        let mut cmd = Cli::command();
        let mut buf = Vec::new();
        clap_complete::generate(Shell::Fish, &mut cmd, "unvenv", &mut buf);

        let output = String::from_utf8(buf).expect("Invalid UTF-8");
        // Verify output contains fish-specific completion code
        assert!(
            output.contains("complete"),
            "Should contain fish complete command"
        );
        assert!(output.contains("unvenv"), "Should contain binary name");
    }

    #[test]
    fn test_cli_command_factory() {
        // Verify CLI can be constructed and has expected properties
        let cmd = Cli::command();
        assert_eq!(cmd.get_name(), "unvenv");

        // Verify subcommands exist
        let subcommands: Vec<_> = cmd.get_subcommands().map(clap::Command::get_name).collect();
        assert!(
            subcommands.contains(&"version"),
            "Should have version subcommand"
        );
        assert!(subcommands.contains(&"scan"), "Should have scan subcommand");
        assert!(
            subcommands.contains(&"completions"),
            "Should have completions subcommand"
        );
        assert!(
            subcommands.contains(&"doctor"),
            "Should have doctor subcommand"
        );
        assert!(
            subcommands.contains(&"update"),
            "Should have update subcommand"
        );
    }

    #[test]
    fn test_completions_for_all_shells() {
        // Test that completions can be generated for all shell types without panicking
        let shells = [
            Shell::Bash,
            Shell::Zsh,
            Shell::Fish,
            Shell::Elvish,
            Shell::PowerShell,
        ];

        for shell in shells {
            let mut cmd = Cli::command();
            let mut buf = Vec::new();
            clap_complete::generate(shell, &mut cmd, "unvenv", &mut buf);

            // Verify we got some output
            assert!(
                !buf.is_empty(),
                "Completions for {shell} should not be empty"
            );

            // Verify it's valid UTF-8
            let output = String::from_utf8(buf).expect("Completions should be valid UTF-8");
            assert!(
                !output.is_empty(),
                "Completion output for {shell} should not be empty"
            );
        }
    }
}
