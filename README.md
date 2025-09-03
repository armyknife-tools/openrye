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

## Overview

OpenRye is a comprehensive AI-powered project and package management solution for Python and Rust. Building on the solid foundation of Rye, OpenRye adds cutting-edge AI assistance, seamless Python-Rust interoperability, and advanced development features that rival and exceed tools like uv and poetry.

## Features

### Core Capabilities
- **Python Version Management** - Automatic Python installation and version switching
- **Dependency Management** - Fast, reliable dependency resolution with uv backend
- **Virtual Environments** - Automatic venv creation and management
- **Project Scaffolding** - Quick project initialization with best practices
- **Workspace Support** - Monorepo and multi-project management

### AI Integration (Coming Soon)
- **Intelligent Code Analysis** - AI-powered error analysis and solutions
- **Smart Dependencies** - AI suggests optimal packages for your project
- **Code Generation** - Generate boilerplate and complex implementations
- **Performance Optimization** - AI identifies and optimizes bottlenecks

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

## How to Use OpenRye - Comprehensive Guide

### 🚀 Quick Start

#### First Time Setup

```bash
# Install OpenRye
curl -sSf https://openrye.dev/get | bash

# Verify installation
openrye --version

# Configure OpenRye (optional)
openrye config
```

#### Creating Your First Project

```bash
# Create a new project
openrye init my-awesome-project
cd my-awesome-project

# Or use a template from the marketplace
openrye template install fastapi-template
cd fastapi-template

# Install dependencies
openrye sync

# Run development server
openrye run dev
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

## Roadmap

- [x] Core Python project management
- [x] Fast dependency resolution with uv
- [x] Virtual environment management
- [x] Basic rebranding to OpenRye
- [ ] AI-powered code analysis
- [ ] Rust extension support
- [ ] Async/await optimizations
- [ ] Package vulnerability scanning
- [ ] Cloud workspace sync
- [ ] IDE integrations

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