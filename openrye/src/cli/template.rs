use anyhow::Result;
use clap::Parser;

/// Manage and use project templates
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    /// List available templates
    List,
    /// Show template details
    Show(ShowArgs),
}

#[derive(Parser, Debug)]
struct ShowArgs {
    /// Template name
    name: String,
}

pub fn execute(args: Args) -> Result<()> {
    match args.command {
        Command::List => {
            println!("📋 Available Project Templates");
            println!();
            println!("Web API:");
            println!("  fastapi - Production-ready FastAPI with async, auth, and testing");
            println!();
            println!("Coming Soon:");
            println!("  django  - Django web framework with admin and ORM");
            println!("  flask   - Lightweight Flask application");
            println!("  cli     - Command-line tool with Click");
            println!();
            println!("Use 'openrye init --template <name>' to create a project");
        }
        Command::Show(show) => {
            if show.name == "fastapi" {
                println!("FastAPI Template");
                println!("================");
                println!("A production-ready FastAPI application with:");
                println!("- Async/await support");
                println!("- JWT authentication");
                println!("- SQLAlchemy ORM");
                println!("- Docker configuration");
                println!("- Testing with pytest");
                println!("- Pre-commit hooks");
            } else {
                println!("Template '{}' not found", show.name);
            }
        }
    }
    Ok(())
}