# 🤖 OpenRye AI Features Usage Guide

## Setup

### Using OpenAI (GPT-4/GPT-5)

1. **Get your OpenAI API key**
   - Sign up at https://platform.openai.com
   - Create an API key in your account settings
   - You have $49 in credits to use!

2. **Set environment variable**
   ```bash
   export OPENAI_API_KEY="sk-..."
   ```

3. **Optional: Set model** (defaults to GPT-4 Turbo)
   ```bash
   export OPENAI_MODEL="gpt-4-turbo-preview"  # or "gpt-5" when available
   ```

### Using Anthropic Claude (When you have credits)

1. **Set environment variable**
   ```bash
   export ANTHROPIC_API_KEY="your-key-here"
   ```

## Available Commands

### 1. Code Analysis
Analyze Python code for issues, improvements, and security concerns:

```bash
# Analyze a Python file
openrye ai analyze main.py

# Get JSON output for CI/CD integration
openrye ai analyze main.py --format json
```

### 2. Dependency Suggestions
Get AI-powered dependency recommendations:

```bash
# Suggest deps based on description
openrye ai deps --description "FastAPI web app with PostgreSQL and Redis"

# Analyze and optimize existing dependencies
openrye ai deps --optimize

# Read description from file
openrye ai deps --file project-requirements.txt
```

### 3. Error Fixing
Get AI help to fix errors:

```bash
# Fix a specific error
openrye ai fix "ImportError: No module named 'pandas'"

# Provide context file
openrye ai fix "TypeError in line 42" --context main.py
```

### 4. Project Scaffolding
Generate project structure from description:

```bash
# Generate project structure
openrye ai scaffold "REST API with authentication and database"

# Specify output directory
openrye ai scaffold "Data science project with Jupyter" --output ./my-project
```

### 5. Test Generation
Generate pytest tests for your code:

```bash
# Generate tests for a file
openrye ai test calculator.py

# Save tests to file
openrye ai test calculator.py --output test_calculator.py
```

### 6. Python to Rust Conversion
Convert Python hot paths to Rust for performance:

```bash
# Convert Python function to Rust with PyO3
openrye ai to-rust compute.py

# Save Rust code to file
openrye ai to-rust compute.py --output compute.rs
```

### 7. AI Configuration
Manage AI provider settings:

```bash
# Show current configuration
openrye ai config --show

# Set provider
openrye ai config --provider openai
openrye ai config --provider anthropic

# Set model
openrye ai config --model gpt-4-turbo-preview
openrye ai config --model gpt-5  # When available
```

## Examples

### Complete Workflow Example

```bash
# 1. Set up API key
export OPENAI_API_KEY="sk-..."

# 2. Create new project with AI assistance
openrye ai scaffold "FastAPI microservice with JWT auth" --output my-service
cd my-service

# 3. Initialize project
openrye init

# 4. Get dependency suggestions
openrye ai deps --description "FastAPI with JWT, PostgreSQL, Redis caching"

# 5. Analyze code quality
openrye ai analyze src/main.py

# 6. Generate tests
openrye ai test src/auth.py --output tests/test_auth.py

# 7. Fix any errors
openrye ai fix "ValidationError in Pydantic model" --context src/models.py

# 8. Convert performance-critical code to Rust
openrye ai to-rust src/crypto.py --output src/crypto_rust.rs
```

### CI/CD Integration Example

```yaml
# .github/workflows/ai-analysis.yml
name: AI Code Analysis

on: [push, pull_request]

jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install OpenRye
        run: |
          curl -sSf https://openrye.dev/get | bash
          
      - name: Run AI Analysis
        env:
          OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}
        run: |
          openrye ai analyze src/ --format json > analysis.json
          
      - name: Post results
        run: |
          # Process analysis.json and post as PR comment
```

## 💰 Pricing & Your $49 Credits

### GPT-5 vs GPT-4 Pricing
| Model | Input Cost | Output Cost | Your $49 Gets You |
|-------|------------|-------------|-------------------|
| **GPT-5** (Default) | $1.25/1M | $10/1M | **39.2M input tokens** |
| GPT-4 Turbo | $10/1M | $30/1M | 4.9M input tokens |
| GPT-3.5 Turbo | $0.50/1M | $1.50/1M | 98M input tokens |

**GPT-5 is 8x cheaper than GPT-4 and performs better!**

## Cost Optimization Tips

1. **Use GPT-5 by default**
   - 8x cheaper input than GPT-4
   - 3x cheaper output
   - Better performance (75% vs 31% on coding)
   - 272K context window (2x larger)

2. **Leverage semantic caching**
   - 90% discount on repeated inputs
   - Only $0.125/1M for cached tokens
   - Perfect for analyzing similar code

3. **Batch operations**
   - Analyze multiple files in one session
   - Group dependency suggestions

4. **Monitor usage**
   - Check OpenAI dashboard for usage stats
   - $49 = ~39.2 million GPT-5 tokens!
   - Enough for ~10,000+ code analyses

## Troubleshooting

### API Key Issues
```bash
# Verify key is set
echo $OPENAI_API_KEY

# Test connection
openrye ai config --show
```

### Rate Limiting
- OpenAI has rate limits per minute
- Add delays between requests in scripts
- Use exponential backoff for retries

### Model Selection
```bash
# Best performance + value (DEFAULT - 8x cheaper than GPT-4!)
export OPENAI_MODEL="gpt-5"

# For maximum cost savings
export OPENAI_MODEL="gpt-5-mini"  # Even cheaper
export OPENAI_MODEL="gpt-5-nano"  # Most economical

# Legacy models (more expensive)
export OPENAI_MODEL="gpt-4-turbo-preview"  # 8x more expensive
export OPENAI_MODEL="gpt-3.5-turbo"  # Less capable
```

## Security Notes

1. **Never commit API keys**
   - Add to .env file
   - Use environment variables
   - Add .env to .gitignore

2. **Use secrets in CI/CD**
   - GitHub Secrets
   - Environment variables
   - Never hardcode keys

3. **Rotate keys regularly**
   - Generate new keys periodically
   - Revoke old keys

## Placeholder for Claude

When you get Anthropic API credits, the same commands will work:

```bash
# Set Claude API key
export ANTHROPIC_API_KEY="your-key"

# OpenRye automatically detects and uses available provider
openrye ai analyze main.py  # Uses Claude if ANTHROPIC_API_KEY is set
```

## Future Features (Coming Soon)

- **Real-time code suggestions** while typing
- **Project-wide refactoring** with AI
- **Automatic PR reviews** with AI feedback
- **Vulnerability scanning** with AI analysis
- **Performance profiling** with optimization suggestions
- **Multi-file context** understanding
- **Custom training** on your codebase

## Support

- Report issues: https://github.com/openrye/openrye/issues
- API Status: https://status.openai.com
- OpenAI Pricing: https://openai.com/pricing