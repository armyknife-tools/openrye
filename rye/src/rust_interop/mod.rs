use anyhow::{anyhow, bail, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use toml_edit::{Document, Item, Table};

pub mod templates;

/// Manages Rust extensions for Python projects
pub struct RustExtensionManager {
    project_root: PathBuf,
}

impl RustExtensionManager {
    pub fn new(project_root: PathBuf) -> Self {
        Self { project_root }
    }
    
    /// Initialize a new Rust extension in the project
    pub fn init_extension(&self, name: &str) -> Result<()> {
        let extension_path = self.project_root.join("rust_ext").join(name);
        
        // Create extension directory structure
        fs::create_dir_all(&extension_path)?;
        fs::create_dir_all(extension_path.join("src"))?;
        
        // Create Cargo.toml for the extension
        let cargo_toml = templates::cargo_toml_template(name);
        fs::write(extension_path.join("Cargo.toml"), cargo_toml)?;
        
        // Create lib.rs with PyO3 boilerplate
        let lib_rs = templates::lib_rs_template(name);
        fs::write(extension_path.join("src").join("lib.rs"), lib_rs)?;
        
        // Create build script
        let build_py = templates::build_py_template(name);
        fs::write(extension_path.join("build.py"), build_py)?;
        
        // Update pyproject.toml to include the extension
        self.update_pyproject_toml(name)?;
        
        echo!("✨ Created Rust extension '{}'", name);
        echo!("📁 Location: {}", extension_path.display());
        echo!();
        echo!("Next steps:");
        echo!("  1. Implement your Rust functions in rust_ext/{}/src/lib.rs", name);
        echo!("  2. Run 'openrye rust build' to compile the extension");
        echo!("  3. Import in Python: 'import {}'", name);
        
        Ok(())
    }
    
    /// Build all Rust extensions in the project
    pub fn build_extensions(&self) -> Result<()> {
        let rust_ext_dir = self.project_root.join("rust_ext");
        
        if !rust_ext_dir.exists() {
            bail!("No Rust extensions found. Run 'openrye rust init <name>' to create one.");
        }
        
        // Find all Cargo.toml files in rust_ext directory
        for entry in fs::read_dir(&rust_ext_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                let cargo_toml = path.join("Cargo.toml");
                if cargo_toml.exists() {
                    self.build_extension(&path)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Build a specific Rust extension
    fn build_extension(&self, extension_path: &Path) -> Result<()> {
        let extension_name = extension_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow!("Invalid extension path"))?;
        
        echo!("🔨 Building Rust extension '{}'...", extension_name);
        
        // Run maturin build
        let output = Command::new("maturin")
            .args(["build", "--release", "--out", "dist"])
            .current_dir(extension_path)
            .output();
        
        match output {
            Ok(output) if output.status.success() => {
                echo!("✅ Successfully built '{}'", extension_name);
                
                // Install the wheel
                let wheel_files: Vec<_> = fs::read_dir(extension_path.join("dist"))?
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.path().extension()
                            .and_then(|ext| ext.to_str())
                            .map(|ext| ext == "whl")
                            .unwrap_or(false)
                    })
                    .collect();
                
                if let Some(wheel) = wheel_files.first() {
                    self.install_wheel(&wheel.path())?;
                }
                
                Ok(())
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                bail!("Failed to build '{}': {}", extension_name, stderr);
            }
            Err(_) => {
                echo!("⚠️  maturin not found. Installing...");
                self.install_maturin()?;
                // Retry build
                self.build_extension(extension_path)
            }
        }
    }
    
    /// Install maturin for building Python extensions
    fn install_maturin(&self) -> Result<()> {
        Command::new("pip")
            .args(["install", "maturin"])
            .status()?;
        Ok(())
    }
    
    /// Install a built wheel file
    fn install_wheel(&self, wheel_path: &Path) -> Result<()> {
        Command::new("pip")
            .args(["install", "--force-reinstall"])
            .arg(wheel_path)
            .status()?;
        Ok(())
    }
    
    /// Update pyproject.toml to include Rust extension build
    fn update_pyproject_toml(&self, extension_name: &str) -> Result<()> {
        let pyproject_path = self.project_root.join("pyproject.toml");
        
        if !pyproject_path.exists() {
            // Create a basic pyproject.toml if it doesn't exist
            let content = templates::pyproject_rust_section(extension_name);
            fs::write(&pyproject_path, content)?;
        } else {
            // Update existing pyproject.toml
            let content = fs::read_to_string(&pyproject_path)?;
            let mut doc = content.parse::<Document>()?;
            
            // Add build-system section if not present
            if !doc.contains_key("build-system") {
                let mut build_system = Table::new();
                build_system["requires"] = Item::Value(
                    vec!["maturin>=1.0,<2.0"]
                        .into_iter()
                        .collect::<toml_edit::Array>()
                        .into()
                );
                build_system["build-backend"] = Item::Value("maturin".into());
                doc["build-system"] = Item::Table(build_system);
            }
            
            // Add tool.maturin section if not present
            if !doc.contains_key("tool") {
                doc["tool"] = Item::Table(Table::new());
            }
            
            if let Some(tool) = doc["tool"].as_table_mut() {
                if !tool.contains_key("maturin") {
                    let mut maturin = Table::new();
                    maturin["features"] = Item::Value(
                        vec!["pyo3/extension-module"]
                            .into_iter()
                            .collect::<toml_edit::Array>()
                            .into()
                    );
                    tool["maturin"] = Item::Table(maturin);
                }
            }
            
            fs::write(&pyproject_path, doc.to_string())?;
        }
        
        Ok(())
    }
    
    /// Generate Python type stubs for Rust extensions
    pub fn generate_stubs(&self) -> Result<()> {
        let rust_ext_dir = self.project_root.join("rust_ext");
        
        for entry in fs::read_dir(&rust_ext_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                let lib_rs = path.join("src").join("lib.rs");
                if lib_rs.exists() {
                    self.generate_stub_for_extension(&path)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Generate Python type stub for a specific extension
    fn generate_stub_for_extension(&self, extension_path: &Path) -> Result<()> {
        let extension_name = extension_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow!("Invalid extension path"))?;
        
        // Parse Rust code and generate .pyi file
        let lib_rs = fs::read_to_string(extension_path.join("src").join("lib.rs"))?;
        let stub_content = self.generate_stub_from_rust(&lib_rs, extension_name)?;
        
        // Write stub file
        let stub_path = self.project_root.join(format!("{}.pyi", extension_name));
        fs::write(&stub_path, stub_content)?;
        
        echo!("📝 Generated type stubs for '{}'", extension_name);
        
        Ok(())
    }
    
    /// Parse Rust code and generate Python type hints
    fn generate_stub_from_rust(&self, rust_code: &str, module_name: &str) -> Result<String> {
        // This is a simplified version - a real implementation would use syn to parse Rust
        let mut stub = String::new();
        stub.push_str(&format!("\"\"\"Type stubs for {} Rust extension\"\"\"\n\n", module_name));
        stub.push_str("from typing import Any, List, Optional\n\n");
        
        // Extract Python functions from the Rust code
        for line in rust_code.lines() {
            if line.contains("#[pyfunction]") {
                // Try to extract the next function signature
                // This is simplified - real implementation would be more robust
                stub.push_str("def function_name(*args: Any, **kwargs: Any) -> Any: ...\n");
            }
        }
        
        Ok(stub)
    }
}

/// Optimize Python code by converting hot paths to Rust
pub struct RustOptimizer {
    project_root: PathBuf,
}

impl RustOptimizer {
    pub fn new(project_root: PathBuf) -> Self {
        Self { project_root }
    }
    
    /// Analyze Python code and suggest Rust optimizations
    pub fn analyze_for_optimization(&self, py_file: &Path) -> Result<Vec<OptimizationSuggestion>> {
        let content = fs::read_to_string(py_file)?;
        let mut suggestions = Vec::new();
        
        // Look for common optimization opportunities
        if content.contains("for") && content.contains("range") {
            suggestions.push(OptimizationSuggestion {
                line: 0,
                description: "Numerical loop detected - could be optimized with Rust".to_string(),
                estimated_speedup: "10-100x".to_string(),
                rust_template: Some(templates::numerical_loop_template()),
            });
        }
        
        if content.contains("numpy") || content.contains("pandas") {
            suggestions.push(OptimizationSuggestion {
                line: 0,
                description: "Data processing detected - consider Rust for performance".to_string(),
                estimated_speedup: "5-50x".to_string(),
                rust_template: Some(templates::data_processing_template()),
            });
        }
        
        Ok(suggestions)
    }
    
    /// Convert a Python function to Rust
    pub fn convert_to_rust(&self, py_code: &str, function_name: &str) -> Result<String> {
        // This would use AI to convert Python to Rust
        // For now, return a template
        Ok(templates::python_to_rust_template(function_name, py_code))
    }
}

#[derive(Debug)]
pub struct OptimizationSuggestion {
    pub line: usize,
    pub description: String,
    pub estimated_speedup: String,
    pub rust_template: Option<String>,
}

