// AI-Powered Pull Request Generator - Better than GitHub Copilot
// Generates comprehensive PR descriptions, commit messages, and review responses

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use super::{AIProvider, create_ai_provider};

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequest {
    pub title: String,
    pub description: String,
    pub commit_messages: Vec<CommitMessage>,
    pub changelog: Changelog,
    pub testing_checklist: Vec<TestItem>,
    pub deployment_notes: Option<String>,
    pub breaking_changes: Vec<BreakingChange>,
    pub related_issues: Vec<String>,
    pub reviewers: Vec<String>,
    pub labels: Vec<String>,
    pub estimated_review_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitMessage {
    pub hash: Option<String>,
    pub conventional_type: String,  // feat, fix, docs, style, refactor, test, chore
    pub scope: Option<String>,
    pub description: String,
    pub body: Option<String>,
    pub breaking: bool,
    pub closes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Changelog {
    pub version: String,
    pub date: String,
    pub sections: ChangelogSections,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangelogSections {
    pub features: Vec<String>,
    pub fixes: Vec<String>,
    pub performance: Vec<String>,
    pub security: Vec<String>,
    pub documentation: Vec<String>,
    pub deprecated: Vec<String>,
    pub removed: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestItem {
    pub category: String,
    pub description: String,
    pub checked: bool,
    pub automated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BreakingChange {
    pub component: String,
    pub description: String,
    pub migration_guide: String,
    pub affected_versions: Vec<String>,
}

pub struct PRGenerator {
    provider: Box<dyn AIProvider>,
}

impl PRGenerator {
    pub fn new() -> Result<Self> {
        Ok(Self {
            provider: create_ai_provider()?,
        })
    }
    
    pub async fn generate_from_diff(&self, diff: &str, context: PRContext) -> Result<PullRequest> {
        let prompt = self.build_pr_prompt(diff, &context);
        let response = self.provider.generate(&prompt, None).await?;
        
        let mut pr: PullRequest = serde_json::from_str(&response)
            .context("Failed to parse PR generation response")?;
        
        // Enhance with additional analysis
        pr = self.enhance_pr_description(pr, diff).await?;
        pr = self.add_smart_labels(pr, diff).await?;
        pr = self.suggest_reviewers(pr, &context).await?;
        
        Ok(pr)
    }
    
    fn build_pr_prompt(&self, diff: &str, context: &PRContext) -> String {
        format!(r#"
Generate a comprehensive pull request based on the following git diff and context.

PROJECT CONTEXT:
- Repository: {}
- Branch: {} -> {}
- Author: {}
- Project Type: {}
- Primary Language: {}

ANALYSIS REQUIREMENTS:

1. TITLE GENERATION:
   - Concise, descriptive title (50 chars max)
   - Use conventional commit format
   - Include ticket number if found in diff

2. DESCRIPTION STRUCTURE:
   ## Summary
   - 2-3 sentences explaining the changes
   - Business value and impact
   
   ## Changes Made
   - Categorized list of changes
   - Technical details for complex changes
   
   ## Testing
   - How changes were tested
   - Test coverage information
   - Manual testing steps
   
   ## Screenshots/Examples
   - Before/after comparisons if UI changes
   - Code examples for API changes
   
   ## Performance Impact
   - Benchmark results if applicable
   - Resource usage changes
   
   ## Security Considerations
   - Security implications
   - Compliance requirements met

3. COMMIT MESSAGE GENERATION:
   - Follow conventional commits specification
   - Group related changes
   - Include co-authors if mentioned
   - Link to issues/tickets
   - Add breaking change footer if needed

4. CHANGELOG GENERATION:
   - Follow Keep a Changelog format
   - Categorize by: Added, Changed, Deprecated, Removed, Fixed, Security
   - User-facing descriptions
   - Version bump recommendation

5. TESTING CHECKLIST:
   - Unit tests added/updated
   - Integration tests added/updated
   - Manual testing completed
   - Performance testing (if applicable)
   - Security testing (if applicable)
   - Accessibility testing (if UI)
   - Cross-browser testing (if web)
   - Mobile testing (if applicable)
   - Documentation updated
   - Breaking changes documented

6. DEPLOYMENT NOTES:
   - Database migrations required
   - Environment variable changes
   - Configuration updates
   - Dependencies to install
   - Rollback procedure
   - Feature flags to enable
   - Monitoring additions

7. BREAKING CHANGES:
   - API changes
   - Database schema changes
   - Configuration format changes
   - Dependency updates
   - Deprecations
   - Migration guides

8. SMART SUGGESTIONS:
   - Related issues to link
   - Suggested reviewers based on code ownership
   - Appropriate labels
   - Milestone assignment
   - Estimated review time

DIFF TO ANALYZE:
```diff
{}
```

OUTPUT FORMAT:
Generate a JSON response with all the above sections populated professionally.
Focus on clarity, completeness, and helping reviewers understand the changes quickly.
"#, 
            context.repository,
            context.source_branch,
            context.target_branch,
            context.author,
            context.project_type,
            context.primary_language,
            diff
        )
    }
    
    async fn enhance_pr_description(&self, mut pr: PullRequest, diff: &str) -> Result<PullRequest> {
        // Add code complexity analysis
        let complexity_prompt = format!(
            "Analyze the complexity of these changes and suggest review focus areas:\n{}",
            diff
        );
        
        let complexity_analysis = self.provider.generate(&complexity_prompt, None).await?;
        
        // Append complexity analysis to description
        pr.description.push_str("\n\n## 🔍 Review Focus Areas\n");
        pr.description.push_str(&complexity_analysis);
        
        Ok(pr)
    }
    
    async fn add_smart_labels(&self, mut pr: PullRequest, diff: &str) -> Result<PullRequest> {
        let label_prompt = format!(
            "Suggest GitHub/GitLab labels for this PR based on the changes:\n\
            Common labels: bug, feature, enhancement, documentation, refactor, \
            performance, security, breaking-change, needs-review, ready-to-merge\n\
            Diff: {}",
            diff
        );
        
        let labels_response = self.provider.generate(&label_prompt, None).await?;
        
        // Parse and add labels
        for label in labels_response.lines() {
            if !label.trim().is_empty() {
                pr.labels.push(label.trim().to_string());
            }
        }
        
        Ok(pr)
    }
    
    async fn suggest_reviewers(&self, mut pr: PullRequest, context: &PRContext) -> Result<PullRequest> {
        // Analyze code ownership and expertise
        let reviewer_prompt = format!(
            "Based on the files changed and the type of changes, suggest appropriate reviewers:\n\
            Team members: {}\n\
            Consider expertise in: {}",
            context.team_members.join(", "),
            context.primary_language
        );
        
        let reviewers_response = self.provider.generate(&reviewer_prompt, None).await?;
        
        for reviewer in reviewers_response.lines() {
            if !reviewer.trim().is_empty() {
                pr.reviewers.push(reviewer.trim().to_string());
            }
        }
        
        Ok(pr)
    }
    
    pub async fn generate_commit_message(&self, staged_changes: &str) -> Result<CommitMessage> {
        let prompt = format!(r#"
Generate a professional git commit message following conventional commits specification.

STAGED CHANGES:
```diff
{}
```

REQUIREMENTS:
1. Type: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert
2. Optional scope in parentheses
3. Description: imperative mood, lowercase, no period
4. Optional body: explain what and why, not how
5. Optional footer: breaking changes, issue references

EXAMPLES:
- feat(auth): add OAuth2 integration
- fix(api): resolve race condition in user creation
- docs: update API documentation for v2 endpoints
- refactor!: rename User model fields (BREAKING CHANGE)

OUTPUT FORMAT:
JSON with: conventional_type, scope, description, body, breaking, closes
"#, staged_changes);
        
        let response = self.provider.generate(&prompt, None).await?;
        serde_json::from_str(&response)
            .context("Failed to parse commit message")
    }
    
    pub async fn generate_review_response(&self, review_comments: &[ReviewComment]) -> Result<String> {
        let comments_text = review_comments.iter()
            .map(|c| format!("Comment by {}: {}", c.author, c.text))
            .collect::<Vec<_>>()
            .join("\n\n");
        
        let prompt = format!(r#"
Generate professional responses to these code review comments.

REVIEW COMMENTS:
{}

RESPONSE GUIDELINES:
1. Be respectful and constructive
2. Acknowledge valid points
3. Explain reasoning for decisions
4. Propose compromises where appropriate
5. Thank reviewers for their input
6. Clarify misunderstandings politely
7. Commit to making requested changes or explain why not
8. Ask for clarification if needed

Generate a response for each comment that maintains professionalism while addressing the feedback.
"#, comments_text);
        
        self.provider.generate(&prompt, None).await
    }
    
    pub async fn generate_release_notes(&self, pr: &PullRequest, version: &str) -> Result<String> {
        let prompt = format!(r#"
Generate comprehensive release notes for version {} based on this pull request.

PULL REQUEST:
Title: {}
Description: {}
Breaking Changes: {:?}

RELEASE NOTES REQUIREMENTS:
1. Executive summary (2-3 sentences)
2. Key features and improvements
3. Bug fixes
4. Performance improvements
5. Security updates
6. Breaking changes with migration guide
7. Deprecations
8. Known issues
9. Contributors acknowledgment
10. Links to detailed documentation

Make it engaging and informative for both technical and non-technical audiences.
"#, version, pr.title, pr.description, pr.breaking_changes);
        
        self.provider.generate(&prompt, None).await
    }
}

#[derive(Debug)]
pub struct PRContext {
    pub repository: String,
    pub source_branch: String,
    pub target_branch: String,
    pub author: String,
    pub project_type: String,
    pub primary_language: String,
    pub team_members: Vec<String>,
}

#[derive(Debug)]
pub struct ReviewComment {
    pub author: String,
    pub text: String,
    pub file: String,
    pub line: usize,
}

// PR Template Generator
pub fn generate_pr_template(project_type: &str) -> String {
    match project_type {
        "web" => WEB_PR_TEMPLATE.to_string(),
        "api" => API_PR_TEMPLATE.to_string(),
        "library" => LIBRARY_PR_TEMPLATE.to_string(),
        _ => DEFAULT_PR_TEMPLATE.to_string(),
    }
}

const DEFAULT_PR_TEMPLATE: &str = r#"
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed

## Checklist
- [ ] My code follows the style guidelines
- [ ] I have performed a self-review
- [ ] I have commented my code where necessary
- [ ] I have updated the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix/feature works
- [ ] New and existing unit tests pass locally
"#;

const WEB_PR_TEMPLATE: &str = r#"
## Description
Brief description of changes

## Screenshots
[Add screenshots for UI changes]

## Browser Testing
- [ ] Chrome
- [ ] Firefox
- [ ] Safari
- [ ] Edge

## Responsive Testing
- [ ] Mobile
- [ ] Tablet
- [ ] Desktop

## Accessibility
- [ ] Screen reader tested
- [ ] Keyboard navigation works
- [ ] Color contrast meets WCAG standards
"#;

const API_PR_TEMPLATE: &str = r#"
## Description
Brief description of changes

## API Changes
- [ ] New endpoints added
- [ ] Existing endpoints modified
- [ ] Breaking changes
- [ ] Backwards compatible

## Performance Impact
- [ ] Load tested
- [ ] Database queries optimized
- [ ] Caching implemented where appropriate

## Security
- [ ] Input validation added
- [ ] Authentication/authorization checked
- [ ] SQL injection prevented
- [ ] XSS prevention in place
"#;

const LIBRARY_PR_TEMPLATE: &str = r#"
## Description
Brief description of changes

## API Changes
- [ ] New public APIs added
- [ ] Existing APIs modified
- [ ] Deprecations added
- [ ] Breaking changes

## Compatibility
- [ ] Backwards compatible
- [ ] Minimum version requirements updated
- [ ] Migration guide provided

## Documentation
- [ ] API docs updated
- [ ] Examples updated
- [ ] README updated
- [ ] CHANGELOG updated
"#;