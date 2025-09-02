use anyhow::{bail, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

use crate::ai::{get_ai_assistant, AIAssistant};
use crate::pyproject::PyProject;

/// AI-powered development assistance
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    /// Analyze errors and suggest fixes
    Analyze(AnalyzeArgs),
    /// Get intelligent dependency suggestions
    Suggest(SuggestArgs),
    /// Generate code from prompts
    Generate(GenerateArgs),
    /// Optimize existing code for performance
    Optimize(OptimizeArgs),
}

#[derive(Parser, Debug)]
struct AnalyzeArgs {
    /// Error message or file containing error output
    #[arg(help = "Error message or path to error file")]
    error: String,
    /// Additional context about the error
    #[arg(short, long)]
    context: Option<String>,
}

#[derive(Parser, Debug)]
struct SuggestArgs {
    /// Project type (e.g., web, data-science, cli, api)
    #[arg(short = 't', long, default_value = "general")]
    project_type: String,
    /// Project description
    #[arg(short = 'd', long)]
    description: Option<String>,
    /// Automatically add suggested dependencies
    #[arg(short = 'a', long)]
    auto_add: bool,
}

#[derive(Parser, Debug)]
struct GenerateArgs {
    /// Code generation prompt
    prompt: String,
    /// Target programming language
    #[arg(short = 'l', long, default_value = "python")]
    language: String,
    /// Output file path
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,
}

#[derive(Parser, Debug)]
struct OptimizeArgs {
    /// File containing code to optimize
    file: PathBuf,
    /// Programming language (auto-detected if not specified)
    #[arg(short = 'l', long)]
    language: Option<String>,
    /// Overwrite the original file
    #[arg(short = 'i', long)]
    in_place: bool,
}

pub fn execute(cmd: Args) -> Result<()> {
    let assistant = match get_ai_assistant() {
        Ok(a) => a,
        Err(e) => {
            bail!(
                "Failed to initialize AI assistant: {}\n\n\
                To use AI features, set one of these environment variables:\n\
                - ANTHROPIC_API_KEY for Claude\n\
                - OPENAI_API_KEY for GPT\n\n\
                Optional: Set ANTHROPIC_MODEL or OPENAI_MODEL to use a specific model.",
                e
            );
        }
    };
    
    match cmd.command {
        Command::Analyze(args) => analyze_error(assistant, args),
        Command::Suggest(args) => suggest_dependencies(assistant, args),
        Command::Generate(args) => generate_code(assistant, args),
        Command::Optimize(args) => optimize_code(assistant, args),
    }
}

fn analyze_error(assistant: Box<dyn AIAssistant>, args: AnalyzeArgs) -> Result<()> {
    let error = if PathBuf::from(&args.error).exists() {
        fs::read_to_string(&args.error)?
    } else {
        args.error.clone()
    };
    
    let context = args.context.unwrap_or_else(|| {
        // Try to get context from current project
        if let Ok(project) = PyProject::load_or_discover(None) {
            format!("Project: {}", project.name())
        } else {
            String::new()
        }
    });
    
    echo!("🔍 Analyzing error...");
    let solution = assistant.analyze_error(&error, &context)?;
    
    echo!();
    echo!("{}", style("AI Analysis:").bold().cyan());
    echo!("{}", solution);
    
    Ok(())
}

fn suggest_dependencies(assistant: Box<dyn AIAssistant>, args: SuggestArgs) -> Result<()> {
    let description = if let Some(desc) = args.description {
        desc
    } else {
        // Try to get description from pyproject.toml
        if let Ok(project) = PyProject::load_or_discover(None) {
            project.project_description().unwrap_or_default().to_string()
        } else {
            String::new()
        }
    };
    
    echo!("🤔 Analyzing project requirements...");
    let suggestions = assistant.suggest_dependencies(&args.project_type, &description)?;
    
    if suggestions.is_empty() {
        echo!("No additional dependencies suggested.");
        return Ok(());
    }
    
    echo!();
    echo!("{}", style("Suggested dependencies:").bold().cyan());
    for dep in &suggestions {
        echo!("  • {}", dep);
    }
    
    if args.auto_add {
        echo!();
        echo!("📦 Adding suggested dependencies...");
        
        // Run the add command for each dependency
        for dep in suggestions {
            std::process::Command::new("openrye")
                .args(["add", &dep])
                .status()?;
        }
        
        echo!("{}", style("✓ Dependencies added successfully!").green());
    } else {
        echo!();
        echo!("To add these dependencies, run:");
        echo!("  openrye add {}", suggestions.join(" "));
    }
    
    Ok(())
}

fn generate_code(assistant: Box<dyn AIAssistant>, args: GenerateArgs) -> Result<()> {
    echo!("🎨 Generating {} code...", args.language);
    
    let code = assistant.generate_code(&args.prompt, &args.language)?;
    
    if let Some(output) = args.output {
        fs::write(&output, &code)?;
        echo!("{}", style(format!("✓ Code written to {}", output.display())).green());
    } else {
        echo!();
        echo!("{}", style("Generated code:").bold().cyan());
        echo!("{}", style("─".repeat(50)).dim());
        echo!("{}", code);
        echo!("{}", style("─".repeat(50)).dim());
    }
    
    Ok(())
}

fn optimize_code(assistant: Box<dyn AIAssistant>, args: OptimizeArgs) -> Result<()> {
    let code = fs::read_to_string(&args.file)?;
    
    let language = args.language.unwrap_or_else(|| {
        // Detect language from file extension
        args.file
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| match ext {
                "py" => "python",
                "rs" => "rust",
                "js" | "mjs" => "javascript",
                "ts" => "typescript",
                _ => "text",
            })
            .unwrap_or("text")
            .to_string()
    });
    
    echo!("⚡ Optimizing {} code...", language);
    
    let optimized = assistant.optimize_code(&code, &language)?;
    
    if args.in_place {
        fs::write(&args.file, &optimized)?;
        echo!("{}", style(format!("✓ File {} optimized in place", args.file.display())).green());
    } else {
        let output_path = args.file.with_extension(format!("optimized.{}", 
            args.file.extension().and_then(|e| e.to_str()).unwrap_or("txt")
        ));
        fs::write(&output_path, &optimized)?;
        echo!("{}", style(format!("✓ Optimized code written to {}", output_path.display())).green());
    }
    
    Ok(())
}

// Helper macros for consistent output
macro_rules! echo {
    () => {
        println!()
    };
    ($($arg:tt)*) => {
        println!($($arg)*)
    };
}

macro_rules! style {
    ($text:expr) => {
        console::style($text)
    };
}

use {echo, style};