// Advanced AI Code Review Module - Beyond GitHub Copilot
// Comprehensive Python and Rust code analysis with enterprise-grade features

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use super::{AIProvider, create_ai_provider};

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeReview {
    pub summary: ReviewSummary,
    pub issues: Vec<ReviewIssue>,
    pub security: SecurityAnalysis,
    pub performance: PerformanceAnalysis,
    pub best_practices: Vec<BestPractice>,
    pub refactoring_suggestions: Vec<RefactoringSuggestion>,
    pub test_coverage: TestCoverageAnalysis,
    pub documentation: DocumentationAnalysis,
    pub complexity_metrics: ComplexityMetrics,
    pub ai_confidence: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewSummary {
    pub overall_quality: QualityRating,
    pub risk_level: RiskLevel,
    pub ready_to_merge: bool,
    pub blocking_issues: usize,
    pub total_issues: usize,
    pub estimated_fix_time: String,
    pub highlights: Vec<String>,
    pub concerns: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QualityRating {
    Excellent,
    Good,
    Acceptable,
    NeedsImprovement,
    Poor,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewIssue {
    pub severity: IssueSeverity,
    pub category: IssueCategory,
    pub file: String,
    pub line_start: usize,
    pub line_end: usize,
    pub message: String,
    pub suggestion: String,
    pub code_snippet: String,
    pub fixed_code: Option<String>,
    pub references: Vec<String>,
    pub auto_fixable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IssueSeverity {
    Critical,  // Must fix - blocks merge
    High,      // Should fix - serious issue
    Medium,    // Consider fixing - quality issue
    Low,       // Nice to fix - minor issue
    Info,      // Informational only
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IssueCategory {
    Security,
    Performance,
    Bug,
    CodeSmell,
    Style,
    Complexity,
    Duplication,
    Documentation,
    Testing,
    Accessibility,
    TypeSafety,
    MemorySafety,  // Rust-specific
    Concurrency,   // Rust-specific
    Lifetime,      // Rust-specific
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityAnalysis {
    pub vulnerabilities: Vec<Vulnerability>,
    pub owasp_compliance: HashMap<String, bool>,
    pub cve_matches: Vec<CVEMatch>,
    pub supply_chain_risks: Vec<SupplyChainRisk>,
    pub secrets_detected: Vec<SecretDetection>,
    pub security_score: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vulnerability {
    pub severity: String,
    pub cwe_id: String,
    pub description: String,
    pub location: String,
    pub remediation: String,
    pub exploit_scenario: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CVEMatch {
    pub cve_id: String,
    pub package: String,
    pub current_version: String,
    pub fixed_version: String,
    pub severity: String,
    pub published_date: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplyChainRisk {
    pub package: String,
    pub risk_type: String,
    pub description: String,
    pub alternatives: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecretDetection {
    pub secret_type: String,
    pub file: String,
    pub line: usize,
    pub confidence: f32,
    pub recommendation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceAnalysis {
    pub bottlenecks: Vec<Bottleneck>,
    pub optimization_opportunities: Vec<Optimization>,
    pub algorithm_complexity: Vec<ComplexityAnalysis>,
    pub memory_issues: Vec<MemoryIssue>,
    pub async_antipatterns: Vec<AsyncAntipattern>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bottleneck {
    pub location: String,
    pub impact: String,
    pub suggestion: String,
    pub estimated_improvement: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Optimization {
    pub optimization_type: String,
    pub location: String,
    pub current_approach: String,
    pub optimized_approach: String,
    pub performance_gain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexityAnalysis {
    pub function: String,
    pub time_complexity: String,
    pub space_complexity: String,
    pub suggestion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryIssue {
    pub issue_type: String,
    pub location: String,
    pub description: String,
    pub fix: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsyncAntipattern {
    pub pattern: String,
    pub location: String,
    pub issue: String,
    pub correct_pattern: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BestPractice {
    pub category: String,
    pub current_practice: String,
    pub recommended_practice: String,
    pub benefits: Vec<String>,
    pub example: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefactoringSuggestion {
    pub pattern: String,
    pub description: String,
    pub before: String,
    pub after: String,
    pub benefits: Vec<String>,
    pub effort: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCoverageAnalysis {
    pub current_coverage: f32,
    pub recommended_coverage: f32,
    pub uncovered_critical_paths: Vec<String>,
    pub missing_test_types: Vec<String>,
    pub test_quality_issues: Vec<TestQualityIssue>,
    pub suggested_tests: Vec<SuggestedTest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestQualityIssue {
    pub test_name: String,
    pub issue: String,
    pub suggestion: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuggestedTest {
    pub test_type: String,
    pub target_function: String,
    pub test_code: String,
    pub rationale: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentationAnalysis {
    pub coverage: f32,
    pub missing_docs: Vec<MissingDoc>,
    pub quality_issues: Vec<DocQualityIssue>,
    pub suggested_improvements: Vec<DocImprovement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MissingDoc {
    pub item_type: String,
    pub name: String,
    pub location: String,
    pub suggested_doc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocQualityIssue {
    pub location: String,
    pub issue: String,
    pub suggestion: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocImprovement {
    pub current: String,
    pub improved: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexityMetrics {
    pub cyclomatic_complexity: HashMap<String, usize>,
    pub cognitive_complexity: HashMap<String, usize>,
    pub lines_of_code: usize,
    pub technical_debt: String,
    pub maintainability_index: f32,
    pub code_duplication: f32,
}

// Comprehensive Python-specific review prompt
pub const PYTHON_REVIEW_PROMPT: &str = r#"
Perform a comprehensive professional code review of this Python code.

CRITICAL SECURITY CHECKS:
1. SQL injection vulnerabilities
2. Command injection risks
3. Path traversal vulnerabilities
4. XSS and CSRF vulnerabilities
5. Insecure deserialization
6. Hardcoded secrets and API keys
7. Weak cryptography usage
8. OWASP Top 10 compliance
9. Dependency vulnerabilities (check for known CVEs)
10. Input validation and sanitization

PERFORMANCE ANALYSIS:
1. Algorithm complexity (time and space)
2. Database query optimization (N+1 queries, missing indexes)
3. Memory leaks and excessive allocations
4. Inefficient data structures
5. Blocking I/O in async contexts
6. CPU-intensive operations in hot paths
7. Caching opportunities
8. Parallel processing opportunities
9. Generator vs list comprehension usage
10. String concatenation in loops

PYTHON BEST PRACTICES:
1. PEP 8 style compliance
2. PEP 484 type hints usage
3. Proper exception handling (no bare except)
4. Context managers for resource management
5. Proper use of decorators and metaclasses
6. Avoiding mutable default arguments
7. Using enumerate() instead of range(len())
8. List/dict/set comprehensions where appropriate
9. F-strings for formatting (Python 3.6+)
10. Dataclasses for data containers (Python 3.7+)
11. Pattern matching usage (Python 3.10+)
12. Walrus operator appropriate usage (Python 3.8+)

CODE QUALITY METRICS:
1. Cyclomatic complexity per function (should be < 10)
2. Lines per function (should be < 50)
3. Function parameter count (should be < 5)
4. Class cohesion and coupling
5. Code duplication detection
6. Dead code identification
7. Magic number usage
8. Global variable usage
9. Nested depth (should be < 4)
10. File length (should be < 500 lines)

TESTING REQUIREMENTS:
1. Test coverage (aim for > 80%)
2. Unit test presence and quality
3. Integration test coverage
4. Edge case testing
5. Error condition testing
6. Mock usage appropriateness
7. Test isolation and independence
8. Test naming conventions
9. Assertion meaningfulness
10. Performance test presence

DOCUMENTATION STANDARDS:
1. Module-level docstrings
2. Class and method docstrings (Google/NumPy style)
3. Complex algorithm explanations
4. Type hint completeness
5. Example usage in docstrings
6. Parameter and return value documentation
7. Exception documentation
8. README completeness
9. API documentation
10. Inline comment quality

MODERN PYTHON FEATURES:
1. Async/await proper usage
2. Type hints and generics
3. Protocol usage for duck typing
4. Structural pattern matching
5. Union types and Optional usage
6. Literal types for constants
7. TypedDict for structured dicts
8. Final and ClassVar annotations
9. Overload for multiple signatures
10. ParamSpec and Concatenate for decorators

FRAMEWORK-SPECIFIC CHECKS:
- Django: ORM optimization, signal usage, middleware security
- FastAPI: Pydantic model validation, dependency injection, async routes
- Flask: Blueprint organization, request validation, CORS configuration
- SQLAlchemy: Query optimization, session management, relationship loading
- Pandas: Vectorization usage, memory optimization, method chaining
- NumPy: Broadcasting usage, memory layout, dtype optimization

OUTPUT FORMAT:
Provide a comprehensive JSON analysis with:
- Summary with quality rating and risk assessment
- Categorized issues with severity, location, and fixes
- Security vulnerabilities with CVE matches
- Performance bottlenecks with optimization suggestions
- Test coverage gaps with suggested tests
- Documentation improvements
- Refactoring opportunities with before/after examples
- Complexity metrics and technical debt assessment
"#;

// Comprehensive Rust-specific review prompt
pub const RUST_REVIEW_PROMPT: &str = r#"
Perform a comprehensive professional code review of this Rust code.

MEMORY SAFETY CHECKS:
1. Unsafe block justification and safety
2. Proper lifetime annotations
3. Borrowing rules compliance
4. Move semantics correctness
5. Reference counting cycles (Rc/Arc)
6. Interior mutability proper usage
7. Raw pointer dereferencing safety
8. Memory leak possibilities
9. Stack overflow risks
10. Uninitialized memory access

CONCURRENCY SAFETY:
1. Data race prevention
2. Deadlock possibilities
3. Proper Mutex/RwLock usage
4. Send and Sync trait compliance
5. Thread safety of shared state
6. Atomic ordering correctness
7. Channel usage patterns
8. Actor model implementation
9. Lock-free data structure usage
10. Async runtime compatibility

RUST BEST PRACTICES:
1. Idiomatic Rust patterns
2. Error handling with Result/Option
3. Trait design and implementation
4. Generic constraints appropriateness
5. Module organization
6. Visibility modifiers usage
7. Const correctness
8. Type inference vs explicit types
9. Pattern matching exhaustiveness
10. Iterator usage over loops

PERFORMANCE OPTIMIZATION:
1. Zero-cost abstractions usage
2. Allocation minimization
3. Copy vs Clone vs Move
4. Inline hints appropriateness
5. SIMD opportunities
6. Const generics usage
7. Compile-time computation
8. Lazy evaluation patterns
9. Cache-friendly data layouts
10. Vectorization opportunities

ERROR HANDLING:
1. Result vs panic appropriateness
2. Error type design
3. Error propagation with ?
4. Custom error types with thiserror
5. Error context with anyhow
6. Recoverable vs unrecoverable errors
7. Error messages clarity
8. Backtrace inclusion
9. Error conversion implementations
10. Fallback strategies

ASYNC RUST:
1. Async trait usage
2. Future pinning correctness
3. Executor compatibility
4. Async cancellation safety
5. Select! usage patterns
6. Stream processing patterns
7. Async recursion handling
8. Blocking operations in async
9. Spawn vs spawn_blocking
10. Async drop considerations

MACRO HYGIENE:
1. Macro safety and hygiene
2. Procedural macro correctness
3. Declarative macro patterns
4. Token stream manipulation
5. Compile-time validation
6. Error message quality
7. IDE compatibility
8. Documentation generation
9. Test coverage for macros
10. Macro composition

TESTING STANDARDS:
1. Unit test coverage
2. Integration test design
3. Property-based testing
4. Benchmark presence
5. Doctest examples
6. Test organization
7. Mock usage (mockall, etc.)
8. Fuzz testing setup
9. Miri testing for unsafe
10. Cross-platform testing

DEPENDENCY ANALYSIS:
1. Minimal dependency usage
2. Security audit of deps
3. License compatibility
4. Version pinning strategy
5. Feature flag usage
6. Optional dependencies
7. Build time impact
8. Binary size impact
9. Compile time impact
10. Supply chain risks

BUILD AND DISTRIBUTION:
1. Cargo.toml optimization
2. Feature flag design
3. Build script safety
4. Cross-compilation support
5. WASM compatibility
6. No-std compatibility
7. Workspace organization
8. Documentation building
9. CI/CD configuration
10. Release profile optimization

OUTPUT FORMAT:
Provide a comprehensive JSON analysis with:
- Summary with safety rating and performance assessment
- Memory safety issues with specific locations
- Concurrency problems with race condition risks
- Performance bottlenecks with optimization paths
- Lifetime and borrowing issues with fixes
- Unsafe block analysis with justification review
- Test coverage gaps with suggested property tests
- Documentation completeness for public APIs
- Dependency security audit results
- Build configuration improvements
"#;

impl CodeReview {
    pub async fn review_file(path: &Path, language: &str) -> Result<Self> {
        let code = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;
        
        let provider = create_ai_provider()?;
        
        let prompt = match language {
            "python" | "py" => PYTHON_REVIEW_PROMPT,
            "rust" | "rs" => RUST_REVIEW_PROMPT,
            _ => return Self::generic_review(&code, language, provider).await,
        };
        
        let review_prompt = format!(
            "{}\n\nFile: {}\nLanguage: {}\n\nCode to review:\n```{}\n{}\n```",
            prompt,
            path.display(),
            language,
            language,
            code
        );
        
        let response = provider.generate(&review_prompt, None).await?;
        
        // Parse JSON response into CodeReview struct
        serde_json::from_str(&response)
            .context("Failed to parse code review response")
    }
    
    async fn generic_review(code: &str, language: &str, provider: Box<dyn AIProvider>) -> Result<Self> {
        let prompt = format!(
            "Perform a comprehensive code review of this {} code. \
            Check for bugs, security issues, performance problems, and best practices. \
            Return a JSON response with the review results.\n\n\
            Code:\n```{}\n{}\n```",
            language, language, code
        );
        
        let response = provider.generate(&prompt, None).await?;
        serde_json::from_str(&response)
            .context("Failed to parse generic code review response")
    }
    
    pub fn get_blocking_issues(&self) -> Vec<&ReviewIssue> {
        self.issues
            .iter()
            .filter(|issue| matches!(issue.severity, IssueSeverity::Critical))
            .collect()
    }
    
    pub fn generate_fix_pr_description(&self) -> String {
        let mut description = String::new();
        
        description.push_str("## 🔍 Code Review Results\n\n");
        
        // Summary
        description.push_str(&format!(
            "**Quality**: {:?} | **Risk**: {:?} | **Ready to Merge**: {}\n\n",
            self.summary.overall_quality,
            self.summary.risk_level,
            if self.summary.ready_to_merge { "✅" } else { "❌" }
        ));
        
        // Critical issues
        if self.summary.blocking_issues > 0 {
            description.push_str("### 🚨 Blocking Issues\n");
            for issue in self.get_blocking_issues() {
                description.push_str(&format!(
                    "- [ ] **{}**: {} (Line {})\n",
                    issue.category.to_string(),
                    issue.message,
                    issue.line_start
                ));
            }
            description.push_str("\n");
        }
        
        // Security findings
        if !self.security.vulnerabilities.is_empty() {
            description.push_str("### 🔒 Security Findings\n");
            for vuln in &self.security.vulnerabilities {
                description.push_str(&format!(
                    "- **{}**: {} ({})\n",
                    vuln.severity, vuln.description, vuln.cwe_id
                ));
            }
            description.push_str("\n");
        }
        
        // Performance improvements
        if !self.performance.optimization_opportunities.is_empty() {
            description.push_str("### ⚡ Performance Improvements\n");
            for opt in &self.performance.optimization_opportunities {
                description.push_str(&format!(
                    "- {} (Estimated gain: {})\n",
                    opt.optimization_type, opt.performance_gain
                ));
            }
            description.push_str("\n");
        }
        
        description
    }
}

impl IssueCategory {
    pub fn to_string(&self) -> &str {
        match self {
            IssueCategory::Security => "Security",
            IssueCategory::Performance => "Performance",
            IssueCategory::Bug => "Bug",
            IssueCategory::CodeSmell => "Code Smell",
            IssueCategory::Style => "Style",
            IssueCategory::Complexity => "Complexity",
            IssueCategory::Duplication => "Duplication",
            IssueCategory::Documentation => "Documentation",
            IssueCategory::Testing => "Testing",
            IssueCategory::Accessibility => "Accessibility",
            IssueCategory::TypeSafety => "Type Safety",
            IssueCategory::MemorySafety => "Memory Safety",
            IssueCategory::Concurrency => "Concurrency",
            IssueCategory::Lifetime => "Lifetime",
        }
    }
}