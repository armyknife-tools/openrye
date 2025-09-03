// AI-Powered Pull Request Generator CLI Command  
// Creates professional PRs that beat GitHub Copilot

use anyhow::{Result, Context, bail};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::ai::pr_generator::{PRGenerator, PRContext, generate_pr_template};

/// Generate AI-powered pull requests and commit messages
#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Generate a pull request from current changes
    Generate(GenerateArgs),
    
    /// Create a commit message for staged changes
    Commit(CommitArgs),
    
    /// Generate release notes
    Release(ReleaseArgs),
    
    /// Create PR template for project
    Template(TemplateArgs),
}

#[derive(Parser, Debug)]
struct GenerateArgs {
    /// Target branch (default: main/master)
    #[arg(short, long)]
    target: Option<String>,
    
    /// Output format (github, gitlab, text)
    #[arg(short, long, default_value = "github")]
    format: String,
    
    /// Create PR via GitHub CLI
    #[arg(long)]
    create: bool,
    
    /// Add reviewers
    #[arg(long)]
    reviewers: Vec<String>,
    
    /// Add labels
    #[arg(long)]
    labels: Vec<String>,
    
    /// Mark as draft
    #[arg(long)]
    draft: bool,
}

#[derive(Parser, Debug)]
struct CommitArgs {
    /// Commit type (feat, fix, docs, style, refactor, test, chore)
    #[arg(short = 't', long)]
    commit_type: Option<String>,
    
    /// Scope of changes
    #[arg(short, long)]
    scope: Option<String>,
    
    /// Create commit automatically
    #[arg(long)]
    execute: bool,
    
    /// Mark as breaking change
    #[arg(long)]
    breaking: bool,
}

#[derive(Parser, Debug)]
struct ReleaseArgs {
    /// Version number
    version: String,
    
    /// Output file
    #[arg(short, long)]
    output: Option<PathBuf>,
    
    /// Include all PRs since last release
    #[arg(long)]
    all: bool,
}

#[derive(Parser, Debug)]
struct TemplateArgs {
    /// Project type (web, api, library, cli)
    #[arg(short = 't', long, default_value = "general")]
    project_type: String,
    
    /// Output file
    #[arg(short, long, default_value = ".github/pull_request_template.md")]
    output: PathBuf,
}

pub fn execute(args: Args) -> Result<()> {
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        match args.command {
            SubCommand::Generate(args) => generate_pr(args).await,
            SubCommand::Commit(args) => generate_commit(args).await,
            SubCommand::Release(args) => generate_release(args).await,
            SubCommand::Template(args) => create_template(args),
        }
    })
}

async fn generate_pr(args: GenerateArgs) -> Result<()> {
    println!("🚀 OpenRye AI Pull Request Generator");
    println!("=====================================\n");
    
    // Get git diff
    let diff = get_git_diff(&args.target)?;
    
    if diff.is_empty() {
        bail!("No changes detected. Make sure you have uncommitted changes or commits to push.");
    }
    
    // Get repository context
    let context = get_pr_context(&args.target)?;
    
    // Generate PR
    let generator = PRGenerator::new()?;
    let mut pr = generator.generate_from_diff(&diff, context).await?;
    
    // Add custom reviewers and labels
    if !args.reviewers.is_empty() {
        pr.reviewers = args.reviewers;
    }
    if !args.labels.is_empty() {
        pr.labels.extend(args.labels);
    }
    
    // Output PR
    match args.format.as_str() {
        "github" => output_github_pr(&pr, args.draft),
        "gitlab" => output_gitlab_pr(&pr),
        _ => output_text_pr(&pr),
    }
    
    // Create PR if requested
    if args.create {
        create_github_pr(&pr, &args.target, args.draft)?;
    }
    
    Ok(())
}

async fn generate_commit(args: CommitArgs) -> Result<()> {
    println!("✍️  Generating AI Commit Message");
    println!("================================\n");
    
    // Get staged changes
    let staged = get_staged_changes()?;
    
    if staged.is_empty() {
        bail!("No staged changes. Use 'git add' to stage changes first.");
    }
    
    // Generate commit message
    let generator = PRGenerator::new()?;
    let mut commit = generator.generate_commit_message(&staged).await?;
    
    // Override with provided values
    if let Some(t) = args.commit_type {
        commit.conventional_type = t;
    }
    if let Some(s) = args.scope {
        commit.scope = Some(s);
    }
    if args.breaking {
        commit.breaking = true;
    }
    
    // Format commit message
    let message = format_commit_message(&commit);
    
    println!("📝 Generated Commit Message:");
    println!("----------------------------");
    println!("{}", message);
    println!("----------------------------");
    
    // Execute commit if requested
    if args.execute {
        execute_git_commit(&message)?;
        println!("\n✅ Commit created successfully!");
    } else {
        println!("\n💡 To commit, run:");
        println!("git commit -m \"{}\"", message.replace("\n", "\\n"));
    }
    
    Ok(())
}

async fn generate_release(args: ReleaseArgs) -> Result<()> {
    println!("📦 Generating Release Notes");
    println!("===========================\n");
    
    // Get PRs merged since last release
    let prs = if args.all {
        get_merged_prs_since_last_release()?
    } else {
        vec![]  // Current PR only
    };
    
    // Generate release notes
    let generator = PRGenerator::new()?;
    
    // For demo, use current diff as a PR
    let diff = get_git_diff(&Some("main".to_string()))?;
    let context = get_pr_context(&Some("main".to_string()))?;
    let pr = generator.generate_from_diff(&diff, context).await?;
    
    let release_notes = generator.generate_release_notes(&pr, &args.version).await?;
    
    // Output release notes
    if let Some(output) = args.output {
        fs::write(output, &release_notes)?;
        println!("✅ Release notes saved!");
    } else {
        println!("{}", release_notes);
    }
    
    Ok(())
}

fn create_template(args: TemplateArgs) -> Result<()> {
    println!("📋 Creating PR Template");
    println!("======================\n");
    
    let template = generate_pr_template(&args.project_type);
    
    // Ensure directory exists
    if let Some(parent) = args.output.parent() {
        fs::create_dir_all(parent)?;
    }
    
    fs::write(&args.output, template)?;
    
    println!("✅ Template created at: {}", args.output.display());
    println!("\nThis template will be automatically used for all new PRs in this repository.");
    
    Ok(())
}

fn get_git_diff(target: &Option<String>) -> Result<String> {
    let target_branch = target.clone().unwrap_or_else(|| {
        // Try to detect main branch
        if branch_exists("main") {
            "main".to_string()
        } else if branch_exists("master") {
            "master".to_string()
        } else {
            "HEAD~1".to_string()
        }
    });
    
    let output = Command::new("git")
        .args(&["diff", &target_branch, "--"])
        .output()
        .context("Failed to get git diff")?;
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn get_staged_changes() -> Result<String> {
    let output = Command::new("git")
        .args(&["diff", "--staged"])
        .output()
        .context("Failed to get staged changes")?;
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn get_pr_context(target: &Option<String>) -> Result<PRContext> {
    // Get repository name
    let repo_output = Command::new("git")
        .args(&["remote", "get-url", "origin"])
        .output()
        .context("Failed to get repository URL")?;
    
    let repository = String::from_utf8_lossy(&repo_output.stdout)
        .trim()
        .rsplit('/')
        .next()
        .unwrap_or("unknown")
        .replace(".git", "");
    
    // Get current branch
    let branch_output = Command::new("git")
        .args(&["branch", "--show-current"])
        .output()
        .context("Failed to get current branch")?;
    
    let source_branch = String::from_utf8_lossy(&branch_output.stdout)
        .trim()
        .to_string();
    
    let target_branch = target.clone().unwrap_or_else(|| "main".to_string());
    
    // Get author
    let author_output = Command::new("git")
        .args(&["config", "user.name"])
        .output()
        .context("Failed to get git user")?;
    
    let author = String::from_utf8_lossy(&author_output.stdout)
        .trim()
        .to_string();
    
    // Detect project type
    let project_type = detect_project_type();
    let primary_language = detect_primary_language();
    
    Ok(PRContext {
        repository,
        source_branch,
        target_branch,
        author,
        project_type,
        primary_language,
        team_members: vec![],  // Would be loaded from config
    })
}

fn branch_exists(branch: &str) -> bool {
    Command::new("git")
        .args(&["rev-parse", "--verify", branch])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn detect_project_type() -> String {
    if PathBuf::from("package.json").exists() {
        "web".to_string()
    } else if PathBuf::from("Cargo.toml").exists() {
        "rust".to_string()
    } else if PathBuf::from("pyproject.toml").exists() || PathBuf::from("setup.py").exists() {
        "python".to_string()
    } else {
        "general".to_string()
    }
}

fn detect_primary_language() -> String {
    if PathBuf::from("Cargo.toml").exists() {
        "rust".to_string()
    } else if PathBuf::from("pyproject.toml").exists() {
        "python".to_string()
    } else if PathBuf::from("package.json").exists() {
        "javascript".to_string()
    } else {
        "unknown".to_string()
    }
}

fn output_github_pr(pr: &crate::ai::pr_generator::PullRequest, draft: bool) {
    println!("# {}", pr.title);
    println!();
    println!("{}", pr.description);
    
    if !pr.breaking_changes.is_empty() {
        println!("\n## ⚠️ Breaking Changes");
        for change in &pr.breaking_changes {
            println!("\n### {}", change.component);
            println!("{}", change.description);
            println!("\n**Migration Guide:**");
            println!("{}", change.migration_guide);
        }
    }
    
    if !pr.testing_checklist.is_empty() {
        println!("\n## ✅ Testing Checklist");
        for item in &pr.testing_checklist {
            println!("- [{}] {}", if item.checked { "x" } else { " " }, item.description);
        }
    }
    
    if let Some(notes) = &pr.deployment_notes {
        println!("\n## 🚀 Deployment Notes");
        println!("{}", notes);
    }
    
    println!("\n---");
    println!("**Reviewers:** {}", pr.reviewers.join(", "));
    println!("**Labels:** {}", pr.labels.join(", "));
    println!("**Estimated Review Time:** {}", pr.estimated_review_time);
    
    if draft {
        println!("**Status:** Draft");
    }
}

fn output_gitlab_pr(pr: &crate::ai::pr_generator::PullRequest) {
    // GitLab MR format
    println!("## {}", pr.title);
    println!();
    println!("{}", pr.description);
    // Similar to GitHub but with GitLab-specific formatting
}

fn output_text_pr(pr: &crate::ai::pr_generator::PullRequest) {
    println!("TITLE: {}", pr.title);
    println!("\nDESCRIPTION:\n{}", pr.description);
    
    if !pr.commit_messages.is_empty() {
        println!("\nCOMMIT MESSAGES:");
        for commit in &pr.commit_messages {
            println!("- {}", format_commit_message(commit));
        }
    }
}

fn format_commit_message(commit: &crate::ai::pr_generator::CommitMessage) -> String {
    let mut message = String::new();
    
    // Type and scope
    message.push_str(&commit.conventional_type);
    if let Some(scope) = &commit.scope {
        message.push_str(&format!("({})", scope));
    }
    
    // Breaking change indicator
    if commit.breaking {
        message.push_str("!");
    }
    
    // Description
    message.push_str(&format!(": {}", commit.description));
    
    // Body
    if let Some(body) = &commit.body {
        message.push_str(&format!("\n\n{}", body));
    }
    
    // Footer
    if commit.breaking {
        message.push_str("\n\nBREAKING CHANGE: This commit contains breaking changes");
    }
    
    if !commit.closes.is_empty() {
        message.push_str(&format!("\n\nCloses: {}", commit.closes.join(", ")));
    }
    
    message
}

fn execute_git_commit(message: &str) -> Result<()> {
    Command::new("git")
        .args(&["commit", "-m", message])
        .status()
        .context("Failed to create git commit")?;
    
    Ok(())
}

fn create_github_pr(pr: &crate::ai::pr_generator::PullRequest, target: &Option<String>, draft: bool) -> Result<()> {
    // Check if gh CLI is available
    if !Command::new("gh").arg("--version").output().is_ok() {
        println!("\n⚠️  GitHub CLI (gh) not found. Install it to create PRs automatically.");
        println!("   Visit: https://cli.github.com");
        return Ok(());
    }
    
    let target_branch = target.clone().unwrap_or_else(|| "main".to_string());
    
    let mut args = vec![
        "pr".to_string(),
        "create".to_string(),
        "--title".to_string(),
        pr.title.clone(),
        "--body".to_string(),
        pr.description.clone(),
        "--base".to_string(),
        target_branch,
    ];
    
    if draft {
        args.push("--draft".to_string());
    }
    
    for label in &pr.labels {
        args.push("--label".to_string());
        args.push(label.clone());
    }
    
    for reviewer in &pr.reviewers {
        args.push("--reviewer".to_string());
        args.push(reviewer.clone());
    }
    
    let status = Command::new("gh")
        .args(&args)
        .status()
        .context("Failed to create GitHub PR")?;
    
    if status.success() {
        println!("\n✅ Pull request created successfully!");
    }
    
    Ok(())
}

fn get_merged_prs_since_last_release() -> Result<Vec<String>> {
    // This would query GitHub/GitLab API for merged PRs
    // For now, return empty
    Ok(vec![])
}