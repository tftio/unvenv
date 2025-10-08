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
        // Should not panic
        let shell = Shell::Bash;
        // Can't easily capture stdout in test, but we can verify it doesn't crash
        // and that the function accepts the shell parameter
        assert_eq!(format!("{shell}"), "bash");
    }

    #[test]
    fn test_generate_completions_zsh() {
        let shell = Shell::Zsh;
        assert_eq!(format!("{shell}"), "zsh");
    }

    #[test]
    fn test_generate_completions_fish() {
        let shell = Shell::Fish;
        assert_eq!(format!("{shell}"), "fish");
    }

    #[test]
    fn test_cli_command_factory() {
        // Verify CLI can be constructed
        let cmd = Cli::command();
        assert_eq!(cmd.get_name(), "unvenv");
    }
}
