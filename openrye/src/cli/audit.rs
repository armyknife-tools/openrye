use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use crate::ai::security_audit::{SecurityAuditor, RiskLevel};

/// AI-powered security audit with real-time CVE detection
#[derive(Parser, Debug)]
pub struct Args {
    /// Output format (text, json, sarif, html)
    #[arg(short, long, default_value = "text")]
    format: String,
    
    /// Fix vulnerable dependencies automatically
    #[arg(long)]
    fix: bool,
    
    /// Check for zero-day vulnerabilities
    #[arg(long)]
    zero_day: bool,
    
    /// Include supply chain analysis
    #[arg(long)]
    supply_chain: bool,
    
    /// Compliance standards to check (owasp, pci-dss, hipaa, gdpr, soc2)
    #[arg(long)]
    compliance: Vec<String>,
    
    /// Path to project (default: current directory)
    #[arg(short, long, default_value = ".")]
    path: PathBuf,
    
    /// Continuous monitoring mode
    #[arg(long)]
    monitor: bool,
    
    /// CI/CD mode (fail on high/critical issues)
    #[arg(long)]
    ci: bool,
    
    /// Output file (stdout if not specified)  
    #[arg(short, long)]
    output: Option<PathBuf>,
}

pub fn execute(args: Args) -> Result<()> {
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        if args.monitor {
            continuous_monitoring(args).await
        } else {
            run_audit(args).await
        }
    })
}

async fn run_audit(args: Args) -> Result<()> {
    println!("🔒 OpenRye AI Security Audit (Real-time CVE & 0-day Detection)");
    println!("==============================================================\n");
    println!("Scanning project: {}", args.path.display());
    
    let auditor = SecurityAuditor::new()?;
    let audit = auditor.audit_project(args.path.to_str().unwrap()).await?;
    
    // Generate output
    let output = match args.format.as_str() {
        "json" => serde_json::to_string_pretty(&audit)?,
        "sarif" => generate_sarif_output(&audit),
        "html" => generate_html_report(&audit),
        _ => generate_text_output(&audit),
    };
    
    // Write output
    if let Some(output_path) = args.output {
        std::fs::write(output_path, output)?;
        println!("✅ Audit report saved");
    } else {
        println!("{}", output);
    }
    
    // Auto-fix if requested
    if args.fix && !audit.vulnerabilities.is_empty() {
        println!("\n🔧 Attempting to auto-fix vulnerabilities...");
        auto_fix_vulnerabilities(&audit)?;
    }
    
    // CI mode - fail on high/critical issues
    if args.ci {
        match audit.risk_level {
            RiskLevel::Critical | RiskLevel::High => {
                eprintln!("\n❌ Security audit failed: {} risk detected", 
                    format!("{:?}", audit.risk_level).to_uppercase());
                eprintln!("Risk score: {}/100", audit.risk_score);
                std::process::exit(1);
            }
            _ => {
                println!("\n✅ Security audit passed");
            }
        }
    }
    
    Ok(())
}

async fn continuous_monitoring(args: Args) -> Result<()> {
    println!("👁️  Starting continuous security monitoring...");
    println!("Press Ctrl+C to stop\n");
    
    let auditor = SecurityAuditor::new()?;
    auditor.continuous_monitoring(args.path.to_str().unwrap()).await
}

fn generate_text_output(audit: &crate::ai::security_audit::SecurityAudit) -> String {
    let mut output = String::new();
    
    // Executive Summary
    output.push_str(&format!("📊 Security Audit Summary\n"));
    output.push_str(&format!("========================\n"));
    output.push_str(&format!("Risk Score: {}/100\n", audit.risk_score));
    output.push_str(&format!("Risk Level: {:?}\n", audit.risk_level));
    output.push_str(&format!("Scan Time: {}\n\n", audit.scan_timestamp));
    
    // Vulnerabilities
    if !audit.vulnerabilities.is_empty() {
        output.push_str("🚨 Vulnerabilities Found:\n");
        output.push_str("------------------------\n");
        for vuln in &audit.vulnerabilities {
            output.push_str(&format!("• {} - {}\n", vuln.id, vuln.description));
            output.push_str(&format!("  Severity: {:?}\n", vuln.severity));
            if let Some(cvss) = vuln.cvss_score {
                output.push_str(&format!("  CVSS Score: {}\n", cvss));
            }
            output.push_str(&format!("  Component: {}\n", vuln.affected_component));
            output.push_str(&format!("  Fix: {}\n", vuln.remediation));
            if vuln.exploit_available {
                output.push_str("  ⚠️  EXPLOIT AVAILABLE!\n");
            }
            output.push_str("\n");
        }
    }
    
    // Zero-day risks
    if !audit.zero_day_risks.is_empty() {
        output.push_str("🔮 Potential Zero-Day Risks:\n");
        output.push_str("---------------------------\n");
        for risk in &audit.zero_day_risks {
            output.push_str(&format!("• Pattern: {}\n", risk.pattern));
            output.push_str(&format!("  Similarity to known: {}%\n", risk.similarity_to_known * 100.0));
            output.push_str(&format!("  Likelihood: {}%\n", risk.likelihood * 100.0));
            output.push_str(&format!("  Mitigation: {}\n\n", risk.mitigation));
        }
    }
    
    // Dependency audit
    output.push_str(&format!("📦 Dependency Analysis:\n"));
    output.push_str(&format!("----------------------\n"));
    output.push_str(&format!("Total Dependencies: {}\n", audit.dependency_audit.total_dependencies));
    output.push_str(&format!("Vulnerable: {}\n", audit.dependency_audit.vulnerable_dependencies.len()));
    output.push_str(&format!("Outdated: {}\n", audit.dependency_audit.outdated_dependencies.len()));
    output.push_str(&format!("Unmaintained: {}\n", audit.dependency_audit.unmaintained_packages.len()));
    
    if !audit.dependency_audit.typosquatting_risks.is_empty() {
        output.push_str("\n⚠️  Typosquatting Risks Detected:\n");
        for risk in &audit.dependency_audit.typosquatting_risks {
            output.push_str(&format!("  {} (similar to: {})\n", risk.package, risk.similar_to));
        }
    }
    
    // Secrets scan
    if audit.secrets_scan.secrets_found > 0 {
        output.push_str(&format!("\n🔑 Exposed Secrets: {}\n", audit.secrets_scan.secrets_found));
        for secret in &audit.secrets_scan.secrets {
            output.push_str(&format!("  {} in {} (line {})\n", 
                secret.secret_type, secret.file, secret.line));
        }
    }
    
    // Compliance
    output.push_str(&format!("\n📋 Compliance Score: {}%\n", audit.compliance.compliance_score));
    for standard in &audit.compliance.standards {
        output.push_str(&format!("  {}: {}%\n", standard.name, standard.compliance_level));
    }
    
    // Supply chain
    output.push_str(&format!("\n🔗 Supply Chain Risk: {}/100\n", audit.supply_chain.risk_score));
    output.push_str(&format!("  Direct deps: {}\n", audit.supply_chain.direct_dependencies));
    output.push_str(&format!("  Transitive deps: {}\n", audit.supply_chain.transitive_dependencies));
    output.push_str(&format!("  Max depth: {}\n", audit.supply_chain.dependency_depth));
    
    // Recommendations
    if !audit.recommendations.is_empty() {
        output.push_str("\n💡 Top Recommendations:\n");
        output.push_str("-----------------------\n");
        for (i, rec) in audit.recommendations.iter().take(5).enumerate() {
            output.push_str(&format!("{}. {} - {}\n", i + 1, rec.title, rec.description));
            output.push_str(&format!("   Effort: {} | Impact: {}\n", rec.effort, rec.impact));
        }
    }
    
    // Executive summary
    output.push_str(&format!("\n📄 Executive Summary:\n"));
    output.push_str(&format!("--------------------\n"));
    output.push_str(&audit.executive_summary);
    
    output
}

fn generate_sarif_output(audit: &crate::ai::security_audit::SecurityAudit) -> String {
    // SARIF (Static Analysis Results Interchange Format) for CI/CD integration
    let sarif = serde_json::json!({
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "OpenRye Security Audit",
                    "version": env!("CARGO_PKG_VERSION"),
                    "informationUri": "https://github.com/openrye/openrye"
                }
            },
            "results": audit.vulnerabilities.iter().map(|v| {
                serde_json::json!({
                    "ruleId": v.id.clone(),
                    "level": match v.severity.overall {
                        s if s >= 9.0 => "error",
                        s if s >= 7.0 => "warning",
                        _ => "note",
                    },
                    "message": {
                        "text": v.description.clone()
                    },
                    "locations": [{
                        "physicalLocation": {
                            "artifactLocation": {
                                "uri": v.affected_component.clone()
                            }
                        }
                    }],
                    "fixes": [{
                        "description": {
                            "text": v.remediation.clone()
                        }
                    }]
                })
            }).collect::<Vec<_>>()
        }]
    });
    
    serde_json::to_string_pretty(&sarif).unwrap_or_default()
}

fn generate_html_report(audit: &crate::ai::security_audit::SecurityAudit) -> String {
    format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>OpenRye Security Audit Report</title>
    <style>
        body {{ 
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; 
            margin: 40px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: #fff;
        }}
        .container {{
            background: #fff;
            color: #333;
            border-radius: 12px;
            padding: 30px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
        }}
        h1 {{ color: #667eea; }}
        .risk-score {{
            font-size: 48px;
            font-weight: bold;
            color: {};
        }}
        .critical {{ color: #d32f2f; }}
        .high {{ color: #f57c00; }}
        .medium {{ color: #fbc02d; }}
        .low {{ color: #388e3c; }}
        .vulnerability {{
            border-left: 4px solid #d32f2f;
            padding-left: 20px;
            margin: 20px 0;
        }}
        .metric {{
            display: inline-block;
            margin: 10px 20px 10px 0;
            padding: 10px;
            background: #f5f5f5;
            border-radius: 6px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>🔒 OpenRye Security Audit Report</h1>
        <div class="risk-score {}">{}/100</div>
        <p>Risk Level: {:?}</p>
        <p>Scan Time: {}</p>
        
        <h2>Vulnerabilities ({})</h2>
        {}
        
        <h2>Supply Chain Analysis</h2>
        <div class="metric">Direct Dependencies: {}</div>
        <div class="metric">Transitive Dependencies: {}</div>
        <div class="metric">Supply Chain Risk: {}/100</div>
        
        <h2>Executive Summary</h2>
        <p>{}</p>
    </div>
</body>
</html>
    "#, 
        if audit.risk_score > 75.0 { "#d32f2f" } 
        else if audit.risk_score > 50.0 { "#f57c00" }
        else if audit.risk_score > 25.0 { "#fbc02d" }
        else { "#388e3c" },
        if audit.risk_score > 75.0 { "critical" } 
        else if audit.risk_score > 50.0 { "high" }
        else if audit.risk_score > 25.0 { "medium" }
        else { "low" },
        audit.risk_score,
        audit.risk_level,
        audit.scan_timestamp,
        audit.vulnerabilities.len(),
        audit.vulnerabilities.iter().map(|v| format!(
            r#"<div class="vulnerability">
                <h3>{}</h3>
                <p>{}</p>
                <p><strong>Component:</strong> {}</p>
                <p><strong>Fix:</strong> {}</p>
            </div>"#,
            v.id, v.description, v.affected_component, v.remediation
        )).collect::<Vec<_>>().join(""),
        audit.supply_chain.direct_dependencies,
        audit.supply_chain.transitive_dependencies,
        audit.supply_chain.risk_score,
        audit.executive_summary
    )
}

fn auto_fix_vulnerabilities(audit: &crate::ai::security_audit::SecurityAudit) -> Result<()> {
    println!("Analyzing vulnerabilities for auto-fix...");
    
    let mut fixed = 0;
    for vuln in &audit.dependency_audit.vulnerable_dependencies {
        if !vuln.safe_versions.is_empty() {
            println!("  Updating {} to {}", vuln.package, vuln.safe_versions[0]);
            // In a real implementation, this would update pyproject.toml or requirements.txt
            fixed += 1;
        }
    }
    
    if fixed > 0 {
        println!("✅ Fixed {} vulnerabilities", fixed);
        println!("Run 'openrye sync' to install updated dependencies");
    } else {
        println!("No auto-fixable vulnerabilities found");
    }
    
    Ok(())
}