<div align="center">
  <img src="docs/static/favicon.svg" width="100">
  <p><strong>OpenRye:</strong> AI-Powered Python & Rust Development Platform</p>
</div>

----
<div align="center">

[![OpenRye](https://img.shields.io/badge/OpenRye-v0.45.0-blue)](https://github.com/openrye/openrye)
[![Discord](https://dcbadge.vercel.app/api/server/drbkcdtSbg?style=flat)](https://discord.gg/drbkcdtSbg)

</div>

> [!NOTE]
> OpenRye is the next evolution of Rye, enhanced with AI capabilities and seamless Rust integration.
> This project combines the best of Python package management with AI-assisted development
> and native Rust performance extensions.

OpenRye is a comprehensive AI-powered project and package management solution for Python and Rust.
Building on the solid foundation of Rye, OpenRye adds cutting-edge AI assistance from Anthropic and OpenAI,
seamless Python-Rust interoperability, and advanced development features that rival and exceed tools like
uv and poetry. It provides a unified experience to install and manage Python installations, 
`pyproject.toml` based projects, dependencies, virtualenvs, and Rust extensions seamlessly.

## Key Features

### 🤖 AI-Powered Development
* **Intelligent Code Suggestions:** Get AI-powered code completions and suggestions
* **Automated Dependency Resolution:** AI helps resolve complex dependency conflicts
* **Smart Error Handling:** AI analyzes errors and provides actionable solutions
* **Code Generation:** Generate boilerplate code and project structures with AI

### 🦀 Rust Integration
* **Native Extensions:** Write Python extensions in Rust with zero friction
* **Performance Optimization:** Automatically identify and optimize hot paths with Rust
* **Seamless Interop:** Call Rust code from Python as easily as importing a module
* **Cargo Integration:** Full cargo compatibility for Rust dependencies

### 📦 Advanced Package Management
* **Smart Dependency Resolution:** AI-enhanced conflict resolution
* **Multi-Language Support:** Manage Python and Rust dependencies together
* **Workspace Management:** Handle complex monorepo structures effortlessly
* **Universal Tool Installation:** Install and manage development tools globally

## In The Box

OpenRye picks and ships the right tools so you can get started in minutes:

* **Bootstraps Python:** Automated access to [Indygreg Python Builds](https://github.com/indygreg/python-build-standalone/) and PyPy
* **AI Assistant:** Built-in AI helper using Anthropic Claude and OpenAI GPT models
* **Rust Toolchain:** Automatic Rust setup and management for extensions
* **Linting and Formatting:** Bundled [ruff](https://github.com/astral-sh/ruff) with `openrye lint` and `openrye fmt`
* **Virtual Environments:** Industry-standard virtualenv management
* **Building & Publishing:** Streamlined wheel building and package publishing
* **Dependency Locking:** Advanced locking with [uv](https://github.com/astral-sh/uv) backend

## Installation

Get started with OpenRye in under a minute:

* **Linux and macOS:**

    ```bash
    curl -sSf https://openrye.dev/get | bash
    ```

* **Windows:**

    Download and run the installer ([64-bit](https://github.com/openrye/openrye/releases/latest/download/openrye-x86_64-windows.exe) or [32-bit](https://github.com/openrye/openrye/releases/latest/download/openrye-x86-windows.exe)).

For more details and other options, refer to the [installation instructions](https://openrye.dev/guide/installation/).

## Quick Start

```bash
# Create a new Python project with AI assistance
openrye init my-project --ai

# Add dependencies with smart suggestions
openrye add numpy pandas --suggest

# Create a Rust extension
openrye rust init my_extension

# Get AI help with your code
openrye ai assist "How do I optimize this function?"

# Build and test with AI-powered error analysis
openrye build --ai-analyze
openrye test --smart
```

## Learn More

* [Visit the Website](https://openrye.dev/)
* [Read the Documentation](https://openrye.dev/guide/)
* [AI Integration Guide](https://openrye.dev/guide/ai/)
* [Rust Extensions Tutorial](https://openrye.dev/guide/rust/)
* [Report Issues](https://github.com/openrye/openrye/issues)

## Community

* [Discussion Forum](https://github.com/openrye/openrye/discussions) - Project discussions
* [Discord](https://discord.gg/openrye) - Real-time developer chat
* [Issue Tracker](https://github.com/openrye/openrye/issues) - Bug reports and feature requests

## License

MIT - See [LICENSE](LICENSE) for details

## Acknowledgments

OpenRye builds upon the excellent foundation of Rye, originally created by Armin Ronacher.
We're grateful for the open-source community's contributions that make this project possible.