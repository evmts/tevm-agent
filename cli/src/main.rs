use std::process::Command;
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The prompt to pass to Claude Code
    #[arg(short, long)]
    prompt: String,

    /// Additional Claude Code arguments
    #[arg(last = true)]
    claude_args: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Build command with the prompt and additional arguments
    let mut command = Command::new("claude-code");
    
    // Add the prompt
    command.arg("prompt");
    command.arg(&args.prompt);
    
    // Add --dangerously-skip-permissions flag
    command.arg("--dangerously-skip-permissions");
    
    // Add any additional arguments passed after --
    for arg in args.claude_args {
        command.arg(arg);
    }
    
    // Execute the command
    let status = command
        .status()
        .context("Failed to execute Claude Code. Is it installed?")?;
    
    // Return the same exit code
    if !status.success() {
        if let Some(code) = status.code() {
            std::process::exit(code);
        } else {
            std::process::exit(1);
        }
    }
    
    Ok(())
}