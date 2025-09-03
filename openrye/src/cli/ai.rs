use anyhow::{Result, Context};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

use crate::ai;

/// AI-powered development assistance
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    /// Analyze code for issues and improvements
    Analyze(AnalyzeArgs),
    
    /// Suggest dependencies for your project
    Deps(DepsArgs),
    
    /// Fix an error with AI assistance
    Fix(FixArgs),
    
    /// Generate project structure from description
    Scaffold(ScaffoldArgs),
    
    /// Generate tests for code
    Test(TestArgs),
    
    /// Convert Python code to Rust
    #[command(name = "to-rust")]
    ToRust(ToRustArgs),
    
    /// Configure AI provider settings
    Config(ConfigArgs),
}

#[derive(Parser, Debug)]
struct AnalyzeArgs {
    /// Path to Python file to analyze
    file: PathBuf,
    
    /// Output format (text, json)
    #[arg(short, long, default_value = "text")]
    format: String,
}

#[derive(Parser, Debug)]
struct DepsArgs {
    /// Project description or requirements file
    #[arg(short, long)]
    description: Option<String>,
    
    /// Read description from file
    #[arg(short, long)]
    file: Option<PathBuf>,
    
    /// Optimize existing dependencies
    #[arg(short, long)]
    optimize: bool,
}

#[derive(Parser, Debug)]
struct FixArgs {
    /// Error message to fix
    error: String,
    
    /// Provide context file
    #[arg(short, long)]
    context: Option<PathBuf>,
}

#[derive(Parser, Debug)]
struct ScaffoldArgs {
    /// Project description
    description: String,
    
    /// Output directory
    #[arg(short, long, default_value = ".")]
    output: PathBuf,
}

#[derive(Parser, Debug)]
struct TestArgs {
    /// Python file to generate tests for
    file: PathBuf,
    
    /// Output test file
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[derive(Parser, Debug)]
struct ToRustArgs {
    /// Python file to convert
    file: PathBuf,
    
    /// Output Rust file
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[derive(Parser, Debug)]
struct ConfigArgs {
    /// Set AI provider (openai, anthropic)
    #[arg(long)]
    provider: Option<String>,
    
    /// Set OpenAI model (gpt-4, gpt-4-turbo-preview, gpt-5)
    #[arg(long)]
    model: Option<String>,
    
    /// Show current configuration
    #[arg(long)]
    show: bool,
}

pub fn execute(args: Args) -> Result<()> {
    // Create async runtime
    let runtime = tokio::runtime::Runtime::new()?;
    
    runtime.block_on(async {
        match args.command {
            Command::Analyze(args) => analyze_code(args).await,
            Command::Deps(args) => suggest_dependencies(args).await,
            Command::Fix(args) => fix_error(args).await,
            Command::Scaffold(args) => scaffold_project(args).await,
            Command::Test(args) => generate_tests(args).await,
            Command::ToRust(args) => convert_to_rust(args).await,
            Command::Config(args) => configure_ai(args).await,
        }
    })
}

async fn analyze_code(args: AnalyzeArgs) -> Result<()> {
    println!("🔍 Analyzing {}...", args.file.display());
    
    let code = fs::read_to_string(&args.file)
        .with_context(|| format!("Failed to read file: {}", args.file.display()))?;
    
    let provider = ai::create_ai_provider()?;
    let analysis = provider.analyze_code(&code).await?;
    
    match args.format.as_str() {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&analysis)?);
        }
        _ => {
            println!("\n📊 Code Analysis Results:\n");
            
            if !analysis.issues.is_empty() {
                println!("⚠️  Issues Found:");
                for issue in &analysis.issues {
                    println!("  - [{:?}] {}", issue.severity, issue.message);
                    if let Some(line) = issue.line {
                        println!("    Line: {}", line);
                    }
                    if let Some(suggestion) = &issue.suggestion {
                        println!("    💡 Suggestion: {}", suggestion);
                    }
                }
            }
            
            if !analysis.suggestions.is_empty() {
                println!("\n💡 Improvement Suggestions:");
                for suggestion in &analysis.suggestions {
                    println!("  - {}", suggestion);
                }
            }
            
            if !analysis.performance_tips.is_empty() {
                println!("\n⚡ Performance Tips:");
                for tip in &analysis.performance_tips {
                    println!("  - {}", tip);
                }
            }
            
            if !analysis.security_warnings.is_empty() {
                println!("\n🔒 Security Warnings:");
                for warning in &analysis.security_warnings {
                    println!("  - {}", warning);
                }
            }
        }
    }
    
    Ok(())
}

async fn suggest_dependencies(args: DepsArgs) -> Result<()> {
    println!("🎯 Analyzing project requirements...");
    
    let description = if let Some(desc) = args.description {
        desc
    } else if let Some(file) = args.file {
        fs::read_to_string(file)?
    } else if args.optimize {
        // Read from pyproject.toml or requirements.txt
        if PathBuf::from("pyproject.toml").exists() {
            fs::read_to_string("pyproject.toml")?
        } else if PathBuf::from("requirements.txt").exists() {
            fs::read_to_string("requirements.txt")?
        } else {
            anyhow::bail!("No project description provided and no requirements file found");
        }
    } else {
        anyhow::bail!("Please provide a description, file, or use --optimize");
    };
    
    let provider = ai::create_ai_provider()?;
    
    if args.optimize {
        let current_deps: Vec<String> = description
            .lines()
            .filter(|line| !line.starts_with('#') && !line.is_empty())
            .map(|s| s.to_string())
            .collect();
        
        let optimized = ai::helpers::optimize_dependencies(current_deps).await?;
        
        println!("\n📦 Optimized Dependencies:\n");
        for dep in optimized {
            println!("{}", dep);
        }
    } else {
        let dependencies = provider.suggest_dependencies(&description).await?;
        
        println!("\n📦 Suggested Dependencies:\n");
        for dep in dependencies {
            println!("{}", dep);
        }
    }
    
    println!("\n💡 Add these to your pyproject.toml or requirements.txt");
    
    Ok(())
}

async fn fix_error(args: FixArgs) -> Result<()> {
    println!("🔧 Analyzing error...");
    
    let context = if let Some(context_file) = args.context {
        fs::read_to_string(context_file)?
    } else {
        "No additional context provided".to_string()
    };
    
    let provider = ai::create_ai_provider()?;
    let solution = provider.fix_error(&args.error, &context).await?;
    
    println!("\n💡 Suggested Solution:\n");
    println!("{}", solution);
    
    Ok(())
}

async fn scaffold_project(args: ScaffoldArgs) -> Result<()> {
    println!("🏗️  Generating project structure...");
    
    let structure = ai::helpers::suggest_project_structure(&args.description).await?;
    
    println!("\n📁 Recommended Project Structure:\n");
    println!("{}", structure);
    
    println!("\n💡 Use 'openrye init' to create this structure");
    
    Ok(())
}

async fn generate_tests(args: TestArgs) -> Result<()> {
    println!("🧪 Generating tests for {}...", args.file.display());
    
    let code = fs::read_to_string(&args.file)?;
    let tests = ai::helpers::generate_tests(&code).await?;
    
    if let Some(output) = args.output {
        fs::write(&output, &tests)?;
        println!("✅ Tests written to {}", output.display());
    } else {
        println!("\n📝 Generated Tests:\n");
        println!("{}", tests);
    }
    
    Ok(())
}

async fn convert_to_rust(args: ToRustArgs) -> Result<()> {
    println!("🦀 Converting Python to Rust...");
    
    let python_code = fs::read_to_string(&args.file)?;
    let rust_code = ai::helpers::python_to_rust(&python_code).await?;
    
    if let Some(output) = args.output {
        fs::write(&output, &rust_code)?;
        println!("✅ Rust code written to {}", output.display());
    } else {
        println!("\n📝 Rust Code:\n");
        println!("{}", rust_code);
    }
    
    println!("\n💡 Add PyO3 to your Cargo.toml to use this code");
    
    Ok(())
}

async fn configure_ai(args: ConfigArgs) -> Result<()> {
    if args.show {
        println!("🤖 AI Configuration:\n");
        
        if std::env::var("OPENAI_API_KEY").is_ok() {
            println!("✅ OpenAI API configured");
            if let Ok(model) = std::env::var("OPENAI_MODEL") {
                println!("   Model: {}", model);
            } else {
                println!("   Model: gpt-5 (default - 8x cheaper than GPT-4!)");
            }
        } else {
            println!("❌ OpenAI API not configured (set OPENAI_API_KEY)");
        }
        
        if std::env::var("ANTHROPIC_API_KEY").is_ok() {
            println!("✅ Anthropic Claude API configured");
        } else {
            println!("❌ Anthropic Claude API not configured (set ANTHROPIC_API_KEY)");
        }
        
        return Ok(());
    }
    
    if let Some(provider) = args.provider {
        match provider.as_str() {
            "openai" => {
                println!("To use OpenAI, set the OPENAI_API_KEY environment variable:");
                println!("export OPENAI_API_KEY=\"your-api-key-here\"");
            }
            "anthropic" | "claude" => {
                println!("To use Anthropic Claude, set the ANTHROPIC_API_KEY environment variable:");
                println!("export ANTHROPIC_API_KEY=\"your-api-key-here\"");
            }
            _ => {
                anyhow::bail!("Unknown provider. Use 'openai' or 'anthropic'");
            }
        }
    }
    
    if let Some(model) = args.model {
        std::env::set_var("OPENAI_MODEL", model);
        println!("✅ Model configured");
    }
    
    Ok(())
}