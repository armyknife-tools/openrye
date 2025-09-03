<div align="center">
  <img src="docs/static/favicon.svg" width="100">
  <p><strong>OpenRye:</strong> AI-Powered Python & Rust Development Platform</p>
</div>

----
<div align="center">

[![OpenRye](https://img.shields.io/badge/OpenRye-v0.46.0-blue)](https://github.com/openrye/openrye)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange)](https://www.rust-lang.org/)

</div>

## 📖 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [🚀 Complete Quick Start Guide](#-complete-quick-start-guide)
  - [Essential Commands Cheat Sheet](#-essential-commands-cheat-sheet)
  - [Complete Workflow Examples](#-complete-workflow-example)
  - [Command Line Arguments Reference](#-command-line-arguments-reference)
  - [Pro Tips & Best Practices](#-pro-tips--best-practices)
- [Template System](#-template-system)
- [2025-2026 Roadmap](#-2025-2026-roadmap-next-generation-python-development)
- [Contributing](#contributing)
- [License](#license)

## Overview

OpenRye is a comprehensive AI-powered project and package management solution for Python and Rust. Building on the solid foundation of Rye, OpenRye adds cutting-edge AI assistance, seamless Python-Rust interoperability, and advanced development features that rival and exceed tools like uv and poetry.

## Features

### Core Capabilities
- **Python Version Management** - Automatic Python installation and version switching
- **Dependency Management** - Fast, reliable dependency resolution with uv backend
- **Virtual Environments** - Automatic venv creation and management
- **Project Scaffolding** - Quick project initialization with best practices
- **Workspace Support** - Monorepo and multi-project management

### 🤖 AI Integration (Now Available!)
- **Intelligent Code Analysis** - AI-powered error analysis and solutions
- **Smart Dependencies** - AI suggests optimal packages for your project
- **Code Generation** - Generate boilerplate and complex implementations
- **Performance Optimization** - AI identifies and optimizes bottlenecks
- **Test Generation** - Automatically generate pytest test cases
- **Python to Rust Conversion** - Convert hot paths to Rust for speed
- **Error Fixing** - Get AI assistance to fix errors
- **Supports OpenAI (GPT-4/GPT-5) and Anthropic Claude**

See [AI Usage Guide](AI_USAGE_GUIDE.md) for detailed instructions.

### Rust Extensions (Coming Soon)
- **Native Performance** - Write Python extensions in Rust
- **Seamless Integration** - PyO3-based Python-Rust interop
- **Automatic Bindings** - Generate Python type stubs from Rust code
- **Hot Path Optimization** - Convert Python bottlenecks to Rust

## Installation

### Quick Install

**macOS and Linux:**
```bash
curl -sSf https://openrye.dev/get | bash
```

**Windows:**
```powershell
irm https://openrye.dev/get.ps1 | iex
```

### Build from Source

```bash
git clone https://github.com/openrye/openrye.git
cd openrye
cargo build --release
./target/release/openrye self install
```

### Requirements
- Rust 1.75+ (for building from source)
- Python 3.8+ (managed automatically by OpenRye)

## 🚀 Complete Quick Start Guide

### Getting Maximum Value from OpenRye

OpenRye combines the speed of Rust with AI intelligence to give you the ultimate Python development experience. Here's how to leverage every feature:

### ⚡ Essential Commands Cheat Sheet

```bash
# Project Management
openrye init <name>           # Create new project
openrye add <package>         # Add dependency
openrye remove <package>      # Remove dependency
openrye sync                  # Install all dependencies
openrye run <script>          # Run project scripts
openrye build                 # Build distributions
openrye publish               # Publish to PyPI

# AI-Powered Development (GPT-5 - 8x cheaper than GPT-4!)
openrye ai analyze <file>     # Analyze code quality
openrye ai deps               # Suggest dependencies
openrye ai fix <error>        # Fix errors with AI
openrye ai test <file>        # Generate tests
openrye ai to-rust <file>     # Convert to Rust

# Python Management
openrye fetch <version>       # Install Python version
openrye pin <version>         # Pin Python version
openrye toolchain list        # List Python versions

# Development Tools
openrye fmt                   # Format code
openrye lint                  # Run linters
openrye test                  # Run tests
openrye shell                 # Activate virtualenv
```

### 🎯 Complete Workflow Example

#### 1. Starting a New Project
```bash
# Method 1: From scratch
openrye init my-api --python 3.11
cd my-api

# Method 2: From template (recommended!)
openrye template list                     # See available templates
openrye init my-api --template fastapi    # Use FastAPI template
cd my-api

# Method 3: Existing project
cd existing-project
openrye init                              # Initialize in current directory
```

#### 2. Managing Dependencies
```bash
# Add production dependencies
openrye add fastapi "uvicorn[standard]" sqlalchemy
openrye add "pydantic>=2.0"              # Version constraints

# Add development dependencies
openrye add --dev pytest black mypy ruff
openrye add --dev "pytest-cov>=4.0"

# Add optional dependency groups
openrye add --optional-group docs mkdocs mkdocs-material

# Remove unwanted packages
openrye remove old-package

# Sync all dependencies (like npm install)
openrye sync

# Update dependencies
openrye lock --update-all                # Update all
openrye lock --update requests           # Update specific
```

#### 3. AI-Powered Development
```bash
# Setup (one-time)
export OPENAI_API_KEY="sk-..."          # Your API key

# Code Analysis
openrye ai analyze src/main.py          # Single file
openrye ai analyze src/ --recursive     # Entire directory
openrye ai analyze . --format json      # JSON output for CI/CD

# Dependency Management
openrye ai deps --description "REST API with PostgreSQL and Redis"
openrye ai deps --optimize              # Optimize existing deps
openrye ai deps --file requirements.txt # From requirements file

# Error Fixing
openrye ai fix "ImportError: No module named 'pandas'"
openrye ai fix "TypeError in line 42" --context src/main.py

# Test Generation
openrye ai test src/calculator.py --output tests/test_calculator.py
openrye ai test src/ --coverage         # Generate with coverage

# Performance Optimization
openrye ai to-rust src/compute.py       # Convert hot paths to Rust
openrye ai optimize src/slow_function.py # Get optimization tips
```

#### 4. Running Your Project
```bash
# Define scripts in pyproject.toml
[tool.openrye.scripts]
dev = "uvicorn main:app --reload"
test = "pytest tests/ -v"
lint = "ruff check . && mypy ."
format = "black . && ruff format ."

# Run scripts
openrye run dev                         # Start dev server
openrye run test                        # Run tests
openrye run lint                        # Run linters
openrye run format                      # Format code

# Direct Python execution
openrye run python script.py            # Run Python script
openrye run python -m module            # Run module
```

#### 5. Python Version Management
```bash
# Install Python versions
openrye fetch 3.11.7                    # Specific version
openrye fetch 3.12                      # Latest 3.12.x
openrye fetch --list                    # See available versions

# Pin project to Python version
openrye pin 3.11                        # Pin to 3.11.x
openrye pin 3.11.7                      # Pin exact version

# List installed Pythons
openrye toolchain list
openrye toolchain list --all            # Include downloadable
```

#### 6. Building and Publishing
```bash
# Build your package
openrye build                           # Build wheel and sdist
openrye build --wheel                   # Wheel only
openrye build --sdist                   # Source dist only

# Publish to PyPI
openrye publish                         # Publish to PyPI
openrye publish --repository testpypi   # Test PyPI
openrye publish --skip-build            # If already built
```

### 📋 Command Line Arguments Reference

#### Global Options
```bash
openrye --version                       # Show version
openrye --help                          # Show help
openrye -v                             # Verbose output
openrye -q                             # Quiet mode
```

#### Init Command
```bash
openrye init [OPTIONS] [PATH]
  --python <VERSION>                    # Python version (e.g., 3.11)
  --template <NAME>                     # Use template
  --no-pin                             # Don't pin Python version
  --virtual                            # Create virtual package
  --lib                                # Create library project
  --script                             # Create single-file script
```

#### Add Command
```bash
openrye add [OPTIONS] <PACKAGES>...
  --dev                                # Add as dev dependency
  --optional-group <GROUP>             # Add to optional group
  --pin <STRATEGY>                     # Pin strategy (exact, >=)
  --features <FEATURES>                # Package features/extras
  --git <URL>                         # Install from git
  --path <PATH>                       # Install from local path
  --sync                               # Sync after adding
  --pre                                # Allow pre-releases
```

#### Sync Command
```bash
openrye sync [OPTIONS]
  --no-dev                            # Skip dev dependencies
  --with-group <GROUP>                # Include optional group
  --all-groups                        # Include all groups
  --no-lock                           # Don't update lock file
  --force                             # Force reinstall
```

#### Run Command
```bash
openrye run [OPTIONS] <COMMAND> [ARGS]...
  --list                              # List available scripts
  --python <VERSION>                  # Use specific Python
  --pyproject <FILE>                  # Custom pyproject.toml
```

#### AI Commands
```bash
# Analyze
openrye ai analyze [OPTIONS] <FILE>
  --format <FORMAT>                   # Output format (text/json)
  --recursive                         # Analyze directory
  --fix                              # Auto-fix issues

# Dependencies
openrye ai deps [OPTIONS]
  --description <DESC>                # Project description
  --file <FILE>                      # Read from file
  --optimize                         # Optimize existing
  --save                             # Save to pyproject.toml

# Fix
openrye ai fix [OPTIONS] <ERROR>
  --context <FILE>                   # Context file
  --apply                            # Apply fix automatically

# Test Generation
openrye ai test [OPTIONS] <FILE>
  --output <FILE>                    # Output file
  --framework <NAME>                 # Test framework (pytest/unittest)
  --coverage                         # Include coverage

# Rust Conversion
openrye ai to-rust [OPTIONS] <FILE>
  --output <FILE>                    # Output Rust file
  --benchmark                        # Include benchmarks
  --bindings                         # Generate Python bindings
```

### 💡 Pro Tips & Best Practices

#### Performance Optimization
```bash
# Use UV backend (default, 100x faster)
openrye config set use-uv true

# Enable global cache
openrye config set global-python true

# Use hardlinks for packages
openrye config set use-hardlinks true

# Parallel operations
openrye sync --jobs 8
```

#### Workspace Management
```bash
# Create workspace
mkdir my-workspace && cd my-workspace
openrye init --virtual

# Add projects
openrye workspace add packages/core
openrye workspace add packages/api
openrye workspace add packages/cli

# Sync all workspace packages
openrye sync --workspace
```

#### Environment Variables
```bash
# Load .env files
openrye run --env-file .env python script.py
openrye run --env-file .env.local --env-file .env dev

# Set Python path
export OPENRYE_PYTHON="/usr/bin/python3.11"

# Set home directory
export OPENRYE_HOME="$HOME/.openrye"

# Debug mode
export OPENRYE_LOG=debug openrye sync
```

#### CI/CD Integration
```yaml
# GitHub Actions
- name: Install OpenRye
  run: |
    curl -sSf https://openrye.dev/get | bash
    echo "$HOME/.rye/shims" >> $GITHUB_PATH

- name: Setup project
  run: |
    openrye fetch 3.11
    openrye sync

- name: AI Analysis
  env:
    OPENAI_API_KEY: ${{ secrets.OPENAI_API_KEY }}
  run: |
    openrye ai analyze src/ --format json
    openrye test
```

### 🎓 Advanced Usage

#### Custom Scripts
```toml
# pyproject.toml
[tool.openrye.scripts]
dev = { cmd = "uvicorn main:app --reload", env = { DEBUG = "1" } }
test = { chain = ["test:unit", "test:integration"] }
"test:unit" = "pytest tests/unit -v"
"test:integration" = "pytest tests/integration -v"
migrate = "alembic upgrade head"
seed = "python scripts/seed_db.py"
```

#### Lock File Management
```bash
# Generate/update lock file
openrye lock

# Update all dependencies
openrye lock --update-all

# Update specific package
openrye lock --update fastapi

# Verify lock file
openrye lock --verify
```

#### Virtual Environments
```bash
# Activate shell
openrye shell

# Or manually
source .venv/bin/activate           # Unix/macOS
.venv\Scripts\activate               # Windows

# Run in venv without activation
openrye run python script.py

# Show venv info
openrye show
```

### 🚫 Common Gotchas & Solutions

```bash
# Problem: SSL certificate errors
openrye config set trusted-host pypi.org

# Problem: Permission denied
chmod +x ~/.rye/shims/python

# Problem: Slow downloads
openrye config set index-url https://pypi.org/simple

# Problem: Package conflicts
openrye sync --force

# Problem: AI not working
openrye ai config --show             # Check configuration
export OPENAI_API_KEY="sk-..."      # Set API key
```


### 📦 Template System

#### Using Templates

1. **Browse Available Templates**
   ```bash
   # List all templates
   openrye template list
   
   # Search for specific templates
   openrye template search fastapi
   
   # Get template details
   openrye template info fastapi-template
   ```

2. **Install a Template**
   ```bash
   # Install from marketplace
   openrye template install fastapi-template
   
   # Install from URL
   openrye template install https://github.com/user/template.git
   
   # Install from local .ryet file
   openrye template install ./my-template.ryet
   ```

3. **Popular Templates**
   - `fastapi-template` - Production-ready FastAPI with async support
   - `django-rest-template` - Django REST Framework with authentication
   - `data-science-toolkit` - Jupyter, pandas, scikit-learn setup
   - `cli-tool-template` - Click-based CLI application
   - `ml-pipeline` - Machine learning project structure

#### Creating Custom Templates

1. **Package Your Project as a Template**
   ```bash
   # Navigate to your project
   cd my-project
   
   # Create template manifest
   openrye template init
   
   # Package as .ryet file
   openrye template package
   
   # Output: my-project.ryet
   ```

2. **Template Configuration (template.toml)**
   ```toml
   [template]
   name = "my-template"
   version = "1.0.0"
   description = "My awesome template"
   author = "Your Name"
   
   [template.variables]
   project_name = { prompt = "Project name?", default = "my-project" }
   python_version = { prompt = "Python version?", default = "3.11" }
   
   [template.hooks]
   post_install = "openrye sync && openrye run test"
   ```

3. **Share Your Template**
   ```bash
   # Submit to marketplace
   openrye template submit my-template.ryet
   
   # Or share directly
   openrye template share my-template.ryet --public
   ```

### 🔧 Core Commands

#### Project Management

```bash
# Initialize new project
openrye init [project-name] [--python 3.11]

# Add dependencies
openrye add fastapi uvicorn[standard]
openrye add --dev pytest black mypy

# Remove dependencies
openrye remove package-name

# Update dependencies
openrye sync                 # Install all dependencies
openrye sync --update        # Update to latest versions
openrye sync --update-all    # Update including transitive deps

# Lock dependencies
openrye lock                 # Generate lock file
openrye lock --update        # Update and lock
```

#### Python Version Management

```bash
# List installed Python versions
openrye toolchain list

# Install specific Python version
openrye toolchain fetch 3.11.7
openrye toolchain fetch cpython@3.12.0

# Set project Python version
openrye pin 3.11             # Pin to 3.11.x
openrye pin 3.11.7           # Pin to exact version

# Set global default Python
openrye config --set-default-toolchain cpython@3.11.7
```

#### Virtual Environment Management

```bash
# Activate virtual environment
source $(openrye venv activate)  # Linux/Mac
openrye venv activate            # Windows

# Deactivate
deactivate

# Recreate virtual environment
openrye sync --force

# Access venv Python directly
openrye run python script.py
openrye run pip list
```

#### Running Scripts and Commands

```bash
# Run Python scripts
openrye run python main.py
openrye run python -m mymodule

# Run project scripts (from pyproject.toml)
openrye run dev              # runs: uvicorn app:main --reload
openrye run test             # runs: pytest tests/
openrye run format           # runs: black . && isort .

# Pass arguments
openrye run pytest -- -v --cov=src
openrye run python script.py -- --arg1 value1
```

### 🛠️ Advanced Features

#### Workspace Management (Monorepos)

```bash
# Create workspace
openrye init --workspace my-workspace
cd my-workspace

# Add projects to workspace
openrye init backend --path packages/backend
openrye init frontend --path packages/frontend
openrye init shared --path packages/shared

# Manage workspace dependencies
openrye add --workspace numpy    # Add to all projects
openrye sync --workspace         # Sync all projects

# Run commands across workspace
openrye run --workspace test     # Run tests in all projects
```

#### Building and Publishing

```bash
# Build project
openrye build                    # Build wheel and sdist
openrye build --wheel            # Build only wheel
openrye build --sdist            # Build only source distribution

# Publish to PyPI
openrye publish                  # Publish to PyPI
openrye publish --test           # Publish to TestPyPI
openrye publish --repository url  # Publish to custom repository

# Generate requirements.txt
openrye export -o requirements.txt
openrye export --dev -o requirements-dev.txt
```

#### Security and Auditing

```bash
# Security audit (coming soon)
openrye audit                    # Check for vulnerabilities
openrye audit --fix             # Auto-fix vulnerabilities
openrye audit --ignore VULN-ID  # Ignore specific vulnerability

# License checking
openrye licenses                # List all dependency licenses
openrye licenses --check        # Verify license compatibility
```

#### AI Features (Coming Soon)

```bash
# AI-assisted debugging
openrye ai debug error.log      # Analyze error and suggest fixes
openrye ai explain function.py  # Explain code functionality

# AI code generation
openrye ai generate "FastAPI endpoint for user authentication"
openrye ai optimize slow_function.py  # Suggest optimizations

# AI dependency recommendations
openrye ai suggest              # Suggest packages for your project
openrye ai compare package1 package2  # Compare similar packages
```

### 📝 Configuration

#### Project Configuration (pyproject.toml)

```toml
[project]
name = "my-project"
version = "0.1.0"
description = "My awesome project"
authors = [{name = "Your Name", email = "you@example.com"}]
dependencies = [
    "fastapi>=0.104.0",
    "pydantic>=2.0.0",
]

[project.scripts]
dev = "uvicorn src.main:app --reload"
test = "pytest tests/ -v"
format = "black . && isort ."
lint = "mypy src/ && ruff check ."

[tool.openrye]
managed = true
dev-dependencies = [
    "pytest>=7.4.0",
    "black>=23.0.0",
    "mypy>=1.7.0",
]

[tool.openrye.scripts]
# Custom scripts with descriptions
dev = { cmd = "uvicorn src.main:app --reload", help = "Run development server" }
prod = { cmd = "gunicorn src.main:app", help = "Run production server" }
migrate = { chain = ["alembic upgrade head", "python scripts/seed.py"] }
```

#### Global Configuration

```bash
# View current configuration
openrye config --show-path
openrye config --get

# Set configurations
openrye config --set-default-toolchain cpython@3.11
openrye config --set-pypi-index https://pypi.org/simple/
openrye config --set-auto-install true

# Configure behavior
openrye config behavior.global-python true     # Use OpenRye Python globally
openrye config behavior.use-uv true           # Use uv for installations
openrye config behavior.autosync true         # Auto-sync on project changes
```

### 🔍 Troubleshooting

#### Common Issues and Solutions

1. **Python Version Issues**
   ```bash
   # Reset Python version
   openrye pin --remove
   openrye toolchain remove cpython@3.x.x
   openrye toolchain fetch cpython@3.11.7
   openrye pin 3.11.7
   ```

2. **Dependency Conflicts**
   ```bash
   # Clear cache and reinstall
   openrye cache clear
   rm -rf .venv
   openrye sync --force
   
   # Use different resolver
   openrye config behavior.use-uv false
   openrye sync
   ```

3. **Virtual Environment Issues**
   ```bash
   # Recreate virtual environment
   openrye sync --force
   
   # Verify venv activation
   which python  # Should show .venv/bin/python
   ```

4. **Build Failures**
   ```bash
   # Clean build artifacts
   rm -rf dist/ build/
   openrye build --clean
   
   # Verbose build for debugging
   openrye build --verbose
   ```

### 🎯 Common Workflows

#### Web Development with FastAPI

```bash
# Start new FastAPI project
openrye template install fastapi-template
cd fastapi-template
openrye sync

# Development
openrye run dev                  # Start dev server
openrye run test                 # Run tests
openrye run migrate              # Run migrations

# Production
openrye build
docker build -t myapp .
docker run -p 8000:8000 myapp
```

#### Data Science Project

```bash
# Setup data science environment
openrye template install data-science-toolkit
cd data-science-toolkit
openrye sync

# Work with Jupyter
openrye run jupyter lab
openrye run notebook analysis.ipynb

# Run experiments
openrye run python experiments/train_model.py
openrye run python scripts/evaluate.py
```

#### CLI Tool Development

```bash
# Create CLI tool
openrye template install cli-tool-template
cd cli-tool-template
openrye sync

# Development
openrye run python -m src.cli --help
openrye run test

# Build and distribute
openrye build
openrye publish
```

### 📚 Additional Resources

- **Documentation**: https://openrye.dev/docs
- **Template Marketplace**: https://openrye.dev/templates
- **GitHub**: https://github.com/openrye/openrye
- **Discord Community**: https://discord.gg/openrye
- **Blog**: https://openrye.dev/blog

### 💡 Tips and Best Practices

1. **Always use virtual environments** - OpenRye manages them automatically
2. **Pin Python versions** for reproducibility across team members
3. **Use templates** for new projects to follow best practices
4. **Commit pyproject.toml and requirements.lock** to version control
5. **Use workspaces** for monorepo projects
6. **Run `openrye fmt` and `openrye lint`** before committing
7. **Use `openrye audit`** regularly to check for vulnerabilities
8. **Leverage AI features** for complex debugging and optimization

## Basic Commands Reference

```bash
# Quick command reference
openrye --help                   # Show all commands
openrye [command] --help        # Show command help

# Essential commands
openrye init                    # Create new project
openrye add                     # Add dependencies
openrye sync                    # Install dependencies
openrye run                     # Run scripts
openrye build                   # Build project
openrye publish                 # Publish to PyPI
```

### Python Version Management

```bash
# List available Python versions
openrye toolchain list

# Install a specific Python version
openrye toolchain fetch 3.12

# Pin project to specific Python
openrye pin 3.12

# Use PyPy instead of CPython
openrye pin pypy@3.10
```

### Workspace Management

```bash
# Initialize a workspace
openrye init --virtual

# Add workspace members in pyproject.toml
[tool.rye]
workspace = ["packages/*"]

# Sync all workspace packages
openrye sync
```

### Development Workflow

```bash
# Install development dependencies
openrye add --dev pytest black mypy

# Run tests
openrye test

# Run with specific Python version
openrye run --python 3.11 python script.py

# Update dependencies
openrye lock --update-all
openrye sync
```

## Configuration

### Global Configuration

Global settings are stored in `~/.rye/config.toml`:

```toml
[default]
# Default Python version for new projects
python = "3.12"

# Package index URL
index-url = "https://pypi.org/simple"

# Use uv for operations (recommended)
use-uv = true

# Global Python installation
global-python = true

[proxy]
# HTTP proxy settings
http = "http://proxy.example.com:8080"
https = "http://proxy.example.com:8080"
```

### Project Configuration

Project settings in `pyproject.toml`:

```toml
[project]
name = "my-project"
version = "0.1.0"
description = "My OpenRye project"
dependencies = [
    "requests>=2.28",
    "pandas~=2.0",
]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.rye]
managed = true
python-version = "3.12"
dev-dependencies = [
    "pytest>=7.0",
    "black>=23.0",
]

[tool.rye.scripts]
dev = "python -m flask run --debug"
test = "pytest tests/"
lint = "black . && mypy ."
```

## Environment Variables

- `RYE_HOME` - OpenRye installation directory (default: `~/.rye`)
- `RYE_NO_AUTO_INSTALL` - Disable automatic Python installation
- `RYE_TOOLCHAIN` - Override Python toolchain selection
- `RYE_UV_BACKEND` - Force uv backend usage
- `OPENRYE_LOG` - Set log level (error, warn, info, debug)

## AI Features Configuration (Future)

To enable AI features when available:

```bash
# Set API keys
export ANTHROPIC_API_KEY="your-key"
export OPENAI_API_KEY="your-key"

# Use AI assistance
openrye ai analyze error.log
openrye ai suggest --type web
openrye ai generate "FastAPI endpoint for user auth"
openrye ai optimize slow_function.py
```

## Rust Extension Development (Future)

```bash
# Create a Rust extension
openrye rust init my_extension

# Build Rust extensions
openrye rust build

# Generate Python type stubs
openrye rust stubs

# Analyze Python code for optimization
openrye rust optimize script.py
```

## Migration Guide

### From pip/venv

```bash
# In your existing project
openrye init --from-requirements requirements.txt
openrye sync
```

### From Poetry

```bash
# Convert Poetry project
openrye init --from-poetry
openrye sync
```

### From Pipenv

```bash
# Import from Pipfile
openrye init --from-pipfile
openrye sync
```

## Troubleshooting

### Common Issues

**Python not found:**
```bash
openrye toolchain fetch 3.12
openrye pin 3.12
```

**Dependency conflicts:**
```bash
openrye lock --reset
openrye sync --force
```

**SSL certificate errors:**
```bash
# Add trusted host
openrye config --set index-url=https://pypi.org/simple
openrye config --set trusted-host=pypi.org
```

**Permission errors on Unix:**
```bash
# Fix permissions
chmod +x ~/.rye/shims/python
chmod +x ~/.rye/shims/python3
```

### Debug Mode

```bash
# Enable verbose output
OPENRYE_LOG=debug openrye sync

# Check configuration
openrye config --show-path
openrye config --show
```

## Performance Tips

1. **Use uv backend** (enabled by default) for 10-100x faster operations
2. **Enable global Python** to share Python installations across projects
3. **Use workspaces** for monorepo projects to share dependencies
4. **Pin exact versions** in production for reproducibility
5. **Use lock files** to ensure consistent environments

## Architecture

OpenRye is built with:
- **Rust** - Core application for speed and reliability
- **UV** - Ultra-fast Python package installer
- **PyO3** - Rust-Python interoperability (future)
- **Indygreg Python** - Standalone Python distributions

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md).

```bash
# Setup development environment
git clone https://github.com/openrye/openrye.git
cd openrye
cargo build
cargo test

# Run development version
cargo run -- --version
```

## 🚀 2025-2026 Roadmap: Next-Generation Python Development

### Q1 2025: Performance & Core Features
**Goal: Match and exceed uv's performance benchmarks**

#### ⚡ Ultra-Fast Operations (Targeting 100x speedup)
- [ ] **Parallel dependency resolution** - Multi-threaded resolver in Rust
- [ ] **Global module cache with hardlinks** - Copy-on-write optimization like uv
- [ ] **Incremental compilation cache** - Cache Python bytecode across projects
- [ ] **Native ARM64/M1 optimizations** - Platform-specific performance tuning
- [ ] **Zero-copy installations** - Hardlink packages from global cache

#### 🔧 Core Tool Unification
- [ ] **Replace pip, pipx, pyenv completely** - Single binary for all operations
- [ ] **Built-in Python installation** - Download and manage Python versions natively
- [ ] **Hermetic script execution** - Run scripts with inline dependencies (like uv)
- [ ] **Cross-platform lockfiles** - Platform-independent dependency resolution
- [ ] **Workspace-aware operations** - Efficient monorepo support

### Q2 2025: AI-Powered Development
**Goal: First AI-native Python package manager**

#### 🤖 Intelligent Assistance
- [ ] **AI dependency recommendations** - Suggest best packages for your use case
- [ ] **Smart conflict resolution** - AI resolves version conflicts automatically
- [ ] **Code-aware dependency analysis** - Scan code to identify missing/unused deps
- [ ] **Vulnerability prediction** - ML model predicts security issues before they're reported
- [ ] **Performance bottleneck detection** - AI identifies slow dependencies

#### 🧠 Code Generation & Analysis
- [ ] **Project scaffolding with AI** - Generate entire project structures from prompts
- [ ] **Automatic type stub generation** - Create .pyi files for untyped packages
- [ ] **Smart error recovery** - AI suggests fixes for installation/runtime errors
- [ ] **Dependency optimization** - AI suggests lighter alternatives
- [ ] **Custom template generation** - AI creates templates from descriptions

### Q3 2025: Rust Integration & Performance
**Goal: Seamless Python-Rust development**

#### 🦀 Native Extensions
- [ ] **PyO3 integration** - Build Python extensions in Rust seamlessly
- [ ] **Automatic FFI generation** - Generate Python bindings from Rust code
- [ ] **Hot path optimization** - Convert Python bottlenecks to Rust automatically
- [ ] **Maturin integration** - Built-in support for mixed Python/Rust projects
- [ ] **WASM compilation** - Compile Python/Rust to WebAssembly

#### ⚙️ Advanced Performance
- [ ] **JIT compilation support** - Integrate with Python JIT compilers
- [ ] **Distributed builds** - Build packages across multiple machines
- [ ] **Incremental recompilation** - Only rebuild changed components
- [ ] **Profile-guided optimization** - Use runtime profiles to optimize packages
- [ ] **Memory-mapped package loading** - Faster package imports

### Q4 2025: Ecosystem Integration
**Goal: Best-in-class developer experience**

#### 🔌 IDE & Tool Integration
- [ ] **VS Code extension** - Full IDE integration with IntelliSense
- [ ] **JetBrains plugin** - PyCharm/IntelliJ integration
- [ ] **LSP server** - Language Server Protocol for any editor
- [ ] **GitHub Actions** - Native CI/CD integration
- [ ] **Pre-commit hooks** - Automatic code quality checks

#### 🌐 Cloud & Collaboration
- [ ] **Cloud workspace sync** - Sync environments across machines
- [ ] **Remote development** - Develop on remote machines seamlessly
- [ ] **Shared dependency cache** - Team-wide package cache
- [ ] **Private package registry** - Host internal packages
- [ ] **Reproducible builds** - Bit-for-bit identical builds

### Q1 2026: Advanced Package Management
**Goal: Redefine package management paradigms**

#### 📦 Next-Gen Packaging
- [ ] **Quantum dependency resolution** - Use quantum algorithms for NP-hard problems
- [ ] **Blockchain package verification** - Immutable package integrity
- [ ] **P2P package distribution** - Decentralized package hosting
- [ ] **Smart contracts for dependencies** - Automated licensing compliance
- [ ] **Neural package search** - Natural language package discovery

#### 🔒 Security & Compliance
- [ ] **Supply chain analysis** - Full dependency tree security scanning
- [ ] **License compatibility checker** - Automatic license conflict detection
- [ ] **SBOM generation** - Software Bill of Materials for compliance
- [ ] **Runtime security monitoring** - Detect malicious package behavior
- [ ] **Automated CVE patching** - Apply security patches automatically

### Q2 2026: Platform Innovation
**Goal: Beyond traditional package management**

#### 🌟 Revolutionary Features
- [ ] **Time-travel debugging** - Revert to any previous environment state
- [ ] **Predictive dependency updates** - AI predicts breaking changes
- [ ] **Cross-language interop** - Seamless Python/Rust/Go/JS integration
- [ ] **Quantum-resistant cryptography** - Future-proof security
- [ ] **Neural code optimization** - AI optimizes entire codebases

#### 🏗️ Infrastructure
- [ ] **Edge computing support** - Deploy to edge devices seamlessly
- [ ] **Kubernetes native** - Built-in K8s deployment
- [ ] **Serverless packaging** - Optimize for Lambda/Functions
- [ ] **IoT device support** - Package for embedded systems
- [ ] **Mobile deployment** - Python apps on iOS/Android

## 📊 Performance Targets

| Operation | Current Tools | OpenRye Target | Improvement |
|-----------|--------------|----------------|-------------|
| Install 100 packages | 60s (pip) | 0.5s | 120x |
| Resolve complex deps | 30s (poetry) | 0.3s | 100x |
| Create virtualenv | 3s (venv) | 0.1s | 30x |
| Install Python | 45s (pyenv) | 2s | 22x |
| Package search | 2s (pip) | 0.05s | 40x |

## 🎯 Key Differentiators from Competitors

### vs. uv
- **AI-powered features** - Intelligent assistance uv doesn't have
- **Rust extensions** - Native Python-Rust interop
- **Template marketplace** - Community-driven templates
- **Advanced caching** - Neural predictive caching

### vs. Poetry
- **100x faster** - Rust performance vs Python
- **Python management** - Built-in Python installation
- **AI assistance** - Smart dependency resolution
- **Global cache** - Shared package storage

### vs. PDM
- **Better UX** - Simpler, more intuitive interface
- **Performance** - Rust-based for maximum speed
- **AI features** - Intelligent development assistance
- **Template system** - Rich project templates

### vs. Pixi/Conda
- **Pure Python focus** - Not trying to manage system packages
- **Faster resolver** - Custom Rust implementation
- **AI integration** - Smart package recommendations
- **Modern architecture** - Built for cloud-native development

## 🎪 Unique Innovations Coming to OpenRye

1. **Template Marketplace** - .ryet package format for sharing templates
2. **AI Code Analysis** - Real-time code quality suggestions
3. **Rust Hot Paths** - Automatic Python→Rust optimization
4. **Distributed Builds** - Build on multiple machines in parallel
5. **Quantum Algorithms** - Next-gen dependency resolution
6. **Neural Caching** - Predictive package pre-fetching
7. **Blockchain Verification** - Immutable package integrity
8. **Time-travel Environments** - Snapshot and restore any state
9. **Cross-language Interop** - Seamless multi-language projects
10. **Edge Deployment** - One-click IoT/edge packaging

## Old Roadmap (Completed)

- [x] Core Python project management
- [x] Fast dependency resolution with uv
- [x] Virtual environment management
- [x] Basic rebranding to OpenRye

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

OpenRye builds upon the excellent foundation of Rye, originally created by Armin Ronacher. We're grateful for the open-source community's contributions that make this project possible.

## Support

- **Issues:** [GitHub Issues](https://github.com/openrye/openrye/issues)
- **Discussions:** [GitHub Discussions](https://github.com/openrye/openrye/discussions)
- **Security:** Report vulnerabilities to security@openrye.dev

---

<div align="center">
  <sub>Built with ❤️ by the OpenRye community</sub>
</div>