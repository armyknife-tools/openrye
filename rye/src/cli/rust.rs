use anyhow::{bail, Result};
use clap::Parser;
use std::path::PathBuf;

use crate::pyproject::PyProject;
use crate::rust_interop::{RustExtensionManager, RustOptimizer};

/// Manage Rust extensions for Python projects
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    /// Initialize a new Rust extension
    Init(InitArgs),
    /// Build all Rust extensions
    Build(BuildArgs),
    /// Generate Python type stubs for Rust extensions
    Stubs(StubsArgs),
    /// Analyze Python code for Rust optimization opportunities
    Optimize(OptimizeArgs),
    /// Convert Python function to Rust
    Convert(ConvertArgs),
}

#[derive(Parser, Debug)]
struct InitArgs {
    /// Name of the Rust extension module
    name: String,
    /// Project directory (defaults to current directory)
    #[arg(short = 'p', long)]
    project: Option<PathBuf>,
}

#[derive(Parser, Debug)]
struct BuildArgs {
    /// Project directory (defaults to current directory)
    #[arg(short = 'p', long)]
    project: Option<PathBuf>,
    /// Build in release mode (optimized)
    #[arg(short = 'r', long)]
    release: bool,
}

#[derive(Parser, Debug)]
struct StubsArgs {
    /// Project directory (defaults to current directory)
    #[arg(short = 'p', long)]
    project: Option<PathBuf>,
}

#[derive(Parser, Debug)]
struct OptimizeArgs {
    /// Python file to analyze
    file: PathBuf,
    /// Show detailed analysis
    #[arg(short = 'v', long)]
    verbose: bool,
}

#[derive(Parser, Debug)]
struct ConvertArgs {
    /// Python file containing the function
    file: PathBuf,
    /// Function name to convert
    function: String,
    /// Output Rust file
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,
}

pub fn execute(cmd: Args) -> Result<()> {
    match cmd.command {
        Command::Init(args) => init_extension(args),
        Command::Build(args) => build_extensions(args),
        Command::Stubs(args) => generate_stubs(args),
        Command::Optimize(args) => analyze_optimization(args),
        Command::Convert(args) => convert_to_rust(args),
    }
}

fn init_extension(args: InitArgs) -> Result<()> {
    let project_root = if let Some(path) = args.project {
        path
    } else {
        std::env::current_dir()?
    };
    
    // Validate extension name
    if !args.name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        bail!("Extension name must contain only alphanumeric characters and underscores");
    }
    
    let manager = RustExtensionManager::new(project_root);
    manager.init_extension(&args.name)?;
    
    Ok(())
}

fn build_extensions(args: BuildArgs) -> Result<()> {
    let project_root = if let Some(path) = args.project {
        path
    } else {
        std::env::current_dir()?
    };
    
    echo!("🦀 Building Rust extensions...");
    echo!();
    
    let manager = RustExtensionManager::new(project_root);
    manager.build_extensions()?;
    
    echo!();
    echo!("{}", style("✨ All Rust extensions built successfully!").green());
    
    Ok(())
}

fn generate_stubs(args: StubsArgs) -> Result<()> {
    let project_root = if let Some(path) = args.project {
        path
    } else {
        std::env::current_dir()?
    };
    
    echo!("📝 Generating Python type stubs for Rust extensions...");
    
    let manager = RustExtensionManager::new(project_root);
    manager.generate_stubs()?;
    
    echo!("{}", style("✓ Type stubs generated successfully!").green());
    
    Ok(())
}

fn analyze_optimization(args: OptimizeArgs) -> Result<()> {
    if !args.file.exists() {
        bail!("File not found: {}", args.file.display());
    }
    
    let project_root = args.file
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap());
    
    let optimizer = RustOptimizer::new(project_root);
    let suggestions = optimizer.analyze_for_optimization(&args.file)?;
    
    if suggestions.is_empty() {
        echo!("✨ No obvious optimization opportunities found.");
        echo!("Your Python code is already well-optimized!");
    } else {
        echo!("{}", style("🔍 Optimization Opportunities Found:").bold().cyan());
        echo!();
        
        for (i, suggestion) in suggestions.iter().enumerate() {
            echo!("{}. {}", i + 1, style(&suggestion.description).yellow());
            echo!("   Estimated speedup: {}", style(&suggestion.estimated_speedup).green());
            
            if args.verbose {
                if let Some(template) = &suggestion.rust_template {
                    echo!();
                    echo!("   Suggested Rust implementation:");
                    echo!("{}", style("   ─────────────────────────────").dim());
                    for line in template.lines() {
                        echo!("   {}", line);
                    }
                    echo!("{}", style("   ─────────────────────────────").dim());
                }
            }
            echo!();
        }
        
        echo!("💡 Run 'openrye rust convert' to automatically convert functions to Rust");
    }
    
    Ok(())
}

fn convert_to_rust(args: ConvertArgs) -> Result<()> {
    if !args.file.exists() {
        bail!("File not found: {}", args.file.display());
    }
    
    let py_code = std::fs::read_to_string(&args.file)?;
    
    // Check if function exists in the file
    if !py_code.contains(&format!("def {}", args.function)) {
        bail!("Function '{}' not found in {}", args.function, args.file.display());
    }
    
    let project_root = args.file
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::env::current_dir().unwrap());
    
    echo!("🔄 Converting Python function '{}' to Rust...", args.function);
    
    let optimizer = RustOptimizer::new(project_root);
    let rust_code = optimizer.convert_to_rust(&py_code, &args.function)?;
    
    if let Some(output) = args.output {
        std::fs::write(&output, &rust_code)?;
        echo!("{}", style(format!("✓ Rust code written to {}", output.display())).green());
    } else {
        echo!();
        echo!("{}", style("Generated Rust code:").bold().cyan());
        echo!("{}", style("─".repeat(50)).dim());
        echo!("{}", rust_code);
        echo!("{}", style("─".repeat(50)).dim());
    }
    
    echo!();
    echo!("📚 Next steps:");
    echo!("  1. Review and refine the generated Rust code");
    echo!("  2. Add it to a Rust extension with 'openrye rust init <name>'");
    echo!("  3. Build with 'openrye rust build'");
    
    Ok(())
}

