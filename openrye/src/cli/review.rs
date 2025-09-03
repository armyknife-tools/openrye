// Advanced AI-Powered Code Review CLI Command
// Professional code review that beats GitHub Copilot

use anyhow::{Result, Context};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use serde_json;

use crate::ai::code_reviewer::{CodeReview, IssueSeverity};

/// AI-powered comprehensive code review
#[derive(Parser, Debug)]
pub struct Args {
    /// Files or directories to review
    #[arg(required = true)]
    paths: Vec<PathBuf>,
    
    /// Output format (text, json, markdown, html)
    #[arg(short, long, default_value = "text")]
    format: String,
    
    /// Language (auto-detect if not specified)
    #[arg(short, long)]
    language: Option<String>,
    
    /// Only show issues of this severity or higher
    #[arg(long)]
    min_severity: Option<String>,
    
    /// Auto-fix issues where possible
    #[arg(long)]
    auto_fix: bool,
    
    /// Generate pull request description
    #[arg(long)]
    pr: bool,
    
    /// Output file (stdout if not specified)
    #[arg(short, long)]
    output: Option<PathBuf>,
    
    /// Check against specific compliance standards
    #[arg(long)]
    compliance: Vec<String>,
    
    /// Include performance profiling
    #[arg(long)]
    performance: bool,
    
    /// Include security audit
    #[arg(long)]
    security: bool,
    
    /// CI/CD mode (fail on critical issues)
    #[arg(long)]
    ci: bool,
}

pub fn execute(args: Args) -> Result<()> {
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        review_files(args).await
    })
}

async fn review_files(args: Args) -> Result<()> {
    println!("🔍 OpenRye AI Code Review (Beyond GitHub Copilot)");
    println!("================================================\n");
    
    let mut all_reviews = Vec::new();
    let mut total_issues = 0;
    let mut blocking_issues = 0;
    
    for path in &args.paths {
        if path.is_file() {
            let review = review_single_file(&path, &args).await?;
            
            total_issues += review.summary.total_issues;
            blocking_issues += review.summary.blocking_issues;
            
            all_reviews.push(review);
        } else if path.is_dir() {
            let reviews = review_directory(&path, &args).await?;
            
            for review in reviews {
                total_issues += review.summary.total_issues;
                blocking_issues += review.summary.blocking_issues;
                all_reviews.push(review);
            }
        }
    }
    
    // Generate output
    let output = match args.format.as_str() {
        "json" => generate_json_output(&all_reviews),
        "markdown" => generate_markdown_output(&all_reviews),
        "html" => generate_html_output(&all_reviews),
        _ => generate_text_output(&all_reviews),
    };
    
    // Write output
    if let Some(output_path) = args.output {
        fs::write(output_path, output)?;
        println!("✅ Review saved to file");
    } else {
        println!("{}", output);
    }
    
    // Generate PR description if requested
    if args.pr {
        println!("\n📝 Pull Request Description:");
        println!("============================");
        for review in &all_reviews {
            println!("{}", review.generate_fix_pr_description());
        }
    }
    
    // CI mode - fail on critical issues
    if args.ci && blocking_issues > 0 {
        eprintln!("\n❌ CI Check Failed: {} blocking issues found", blocking_issues);
        std::process::exit(1);
    }
    
    // Summary
    println!("\n📊 Review Summary:");
    println!("==================");
    println!("Files reviewed: {}", all_reviews.len());
    println!("Total issues: {}", total_issues);
    println!("Blocking issues: {}", blocking_issues);
    
    if blocking_issues == 0 {
        println!("\n✅ Code is ready to merge!");
    } else {
        println!("\n⚠️  {} blocking issues must be resolved before merge", blocking_issues);
    }
    
    Ok(())
}

async fn review_single_file(path: &PathBuf, args: &Args) -> Result<CodeReview> {
    println!("Reviewing: {}", path.display());
    
    let language = args.language.clone().unwrap_or_else(|| {
        detect_language(path)
    });
    
    let mut review = CodeReview::review_file(path, &language).await?;
    
    // Filter by severity if requested
    if let Some(min_severity) = &args.min_severity {
        let min_sev = parse_severity(min_severity);
        review.issues.retain(|issue| {
            severity_to_number(&issue.severity) >= severity_to_number(&min_sev)
        });
    }
    
    // Auto-fix if requested
    if args.auto_fix {
        apply_auto_fixes(&review, path)?;
    }
    
    Ok(review)
}

async fn review_directory(dir: &PathBuf, args: &Args) -> Result<Vec<CodeReview>> {
    let mut reviews = Vec::new();
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && is_code_file(&path) {
            let review = review_single_file(&path, args).await?;
            reviews.push(review);
        }
    }
    
    Ok(reviews)
}

fn detect_language(path: &PathBuf) -> String {
    match path.extension().and_then(|s| s.to_str()) {
        Some("py") => "python".to_string(),
        Some("rs") => "rust".to_string(),
        Some("js") | Some("jsx") => "javascript".to_string(),
        Some("ts") | Some("tsx") => "typescript".to_string(),
        Some("go") => "go".to_string(),
        Some("java") => "java".to_string(),
        Some("cpp") | Some("cc") | Some("cxx") => "cpp".to_string(),
        Some("c") => "c".to_string(),
        Some("rb") => "ruby".to_string(),
        Some("php") => "php".to_string(),
        _ => "unknown".to_string(),
    }
}

fn is_code_file(path: &PathBuf) -> bool {
    matches!(
        path.extension().and_then(|s| s.to_str()),
        Some("py") | Some("rs") | Some("js") | Some("ts") | 
        Some("go") | Some("java") | Some("cpp") | Some("c") |
        Some("rb") | Some("php") | Some("jsx") | Some("tsx")
    )
}

fn parse_severity(s: &str) -> IssueSeverity {
    match s.to_lowercase().as_str() {
        "critical" => IssueSeverity::Critical,
        "high" => IssueSeverity::High,
        "medium" => IssueSeverity::Medium,
        "low" => IssueSeverity::Low,
        _ => IssueSeverity::Info,
    }
}

fn severity_to_number(sev: &IssueSeverity) -> u8 {
    match sev {
        IssueSeverity::Critical => 5,
        IssueSeverity::High => 4,
        IssueSeverity::Medium => 3,
        IssueSeverity::Low => 2,
        IssueSeverity::Info => 1,
    }
}

fn apply_auto_fixes(review: &CodeReview, path: &PathBuf) -> Result<()> {
    let mut content = fs::read_to_string(path)?;
    let mut fixes_applied = 0;
    
    for issue in &review.issues {
        if issue.auto_fixable {
            if let Some(fixed_code) = &issue.fixed_code {
                // Apply fix (simplified - in production would need proper AST manipulation)
                content = content.replace(&issue.code_snippet, fixed_code);
                fixes_applied += 1;
            }
        }
    }
    
    if fixes_applied > 0 {
        fs::write(path, content)?;
        println!("✅ Applied {} auto-fixes to {}", fixes_applied, path.display());
    }
    
    Ok(())
}

fn generate_text_output(reviews: &[CodeReview]) -> String {
    let mut output = String::new();
    
    for review in reviews {
        output.push_str(&format!("\n📄 File Review\n"));
        output.push_str(&format!("Quality: {:?}\n", review.summary.overall_quality));
        output.push_str(&format!("Risk: {:?}\n", review.summary.risk_level));
        output.push_str(&format!("Ready to merge: {}\n", 
            if review.summary.ready_to_merge { "✅" } else { "❌" }));
        
        if !review.issues.is_empty() {
            output.push_str("\n🔴 Issues Found:\n");
            for issue in &review.issues {
                output.push_str(&format!(
                    "  [{:?}] {} - {} (Line {})\n",
                    issue.severity,
                    issue.category.to_string(),
                    issue.message,
                    issue.line_start
                ));
                
                if !issue.suggestion.is_empty() {
                    output.push_str(&format!("    💡 {}\n", issue.suggestion));
                }
            }
        }
        
        if !review.security.vulnerabilities.is_empty() {
            output.push_str("\n🔒 Security Issues:\n");
            for vuln in &review.security.vulnerabilities {
                output.push_str(&format!(
                    "  {} - {} ({})\n",
                    vuln.severity, vuln.description, vuln.cwe_id
                ));
            }
        }
        
        if !review.performance.bottlenecks.is_empty() {
            output.push_str("\n⚡ Performance Issues:\n");
            for bottleneck in &review.performance.bottlenecks {
                output.push_str(&format!(
                    "  {} - Impact: {}\n",
                    bottleneck.location, bottleneck.impact
                ));
            }
        }
    }
    
    output
}

fn generate_json_output(reviews: &[CodeReview]) -> String {
    serde_json::to_string_pretty(reviews).unwrap_or_default()
}

fn generate_markdown_output(reviews: &[CodeReview]) -> String {
    let mut output = String::new();
    
    output.push_str("# Code Review Report\n\n");
    
    for (i, review) in reviews.iter().enumerate() {
        output.push_str(&format!("## File {}\n\n", i + 1));
        output.push_str(&format!("- **Quality**: {:?}\n", review.summary.overall_quality));
        output.push_str(&format!("- **Risk**: {:?}\n", review.summary.risk_level));
        output.push_str(&format!("- **Issues**: {}\n", review.summary.total_issues));
        output.push_str(&format!("- **Blocking**: {}\n\n", review.summary.blocking_issues));
        
        if !review.issues.is_empty() {
            output.push_str("### Issues\n\n");
            output.push_str("| Severity | Category | Description | Line |\n");
            output.push_str("|----------|----------|-------------|------|\n");
            
            for issue in &review.issues {
                output.push_str(&format!(
                    "| {:?} | {} | {} | {} |\n",
                    issue.severity,
                    issue.category.to_string(),
                    issue.message,
                    issue.line_start
                ));
            }
            output.push_str("\n");
        }
    }
    
    output
}

fn generate_html_output(reviews: &[CodeReview]) -> String {
    let mut html = String::new();
    
    html.push_str(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Code Review Report</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; margin: 40px; }
        .review { margin-bottom: 40px; padding: 20px; border: 1px solid #ddd; border-radius: 8px; }
        .critical { color: #d32f2f; }
        .high { color: #f57c00; }
        .medium { color: #fbc02d; }
        .low { color: #388e3c; }
        .info { color: #1976d2; }
        table { width: 100%; border-collapse: collapse; }
        th, td { padding: 8px; text-align: left; border-bottom: 1px solid #ddd; }
        .ready { color: #4caf50; }
        .not-ready { color: #f44336; }
    </style>
</head>
<body>
    <h1>🔍 OpenRye AI Code Review Report</h1>
"#);
    
    for review in reviews {
        html.push_str(&format!(r#"
    <div class="review">
        <h2>File Review</h2>
        <p><strong>Quality:</strong> {:?}</p>
        <p><strong>Risk Level:</strong> {:?}</p>
        <p class="{}"><strong>Ready to Merge:</strong> {}</p>
        "#,
            review.summary.overall_quality,
            review.summary.risk_level,
            if review.summary.ready_to_merge { "ready" } else { "not-ready" },
            if review.summary.ready_to_merge { "✅ Yes" } else { "❌ No" }
        ));
        
        if !review.issues.is_empty() {
            html.push_str("<h3>Issues</h3><table>");
            html.push_str("<tr><th>Severity</th><th>Category</th><th>Description</th><th>Line</th></tr>");
            
            for issue in &review.issues {
                html.push_str(&format!(
                    r#"<tr class="{}"><td>{:?}</td><td>{}</td><td>{}</td><td>{}</td></tr>"#,
                    format!("{:?}", issue.severity).to_lowercase(),
                    issue.severity,
                    issue.category.to_string(),
                    issue.message,
                    issue.line_start
                ));
            }
            html.push_str("</table>");
        }
        
        html.push_str("</div>");
    }
    
    html.push_str("</body></html>");
    html
}

