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

## Usage

### Basic Commands

```bash
# Create a new Python project
openrye init my-project
cd my-project

# Add dependencies
openrye add numpy pandas requests

# Sync dependencies and create venv
openrye sync

# Run Python scripts
openrye run python script.py

# Run project scripts
openrye run my-script

# Format and lint code
openrye fmt
openrye lint

# Build project
openrye build

# Publish to PyPI
openrye publish
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