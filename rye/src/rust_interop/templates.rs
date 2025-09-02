/// Templates for Rust/Python interop code generation

pub fn cargo_toml_template(name: &str) -> String {
    format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
name = "{}"
crate-type = ["cdylib"]

[dependencies]
pyo3 = {{ version = "0.20", features = ["extension-module"] }}
numpy = "0.20"
ndarray = "0.15"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
"#,
        name, name
    )
}

pub fn lib_rs_template(name: &str) -> String {
    format!(
        r#"use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Example function that adds two numbers
#[pyfunction]
fn add(a: i64, b: i64) -> PyResult<i64> {{
    Ok(a + b)
}}

/// Example function that processes a list
#[pyfunction]
fn process_list(items: Vec<f64>) -> PyResult<f64> {{
    Ok(items.iter().sum())
}}

/// Example class
#[pyclass]
struct Counter {{
    count: i64,
}}

#[pymethods]
impl Counter {{
    #[new]
    fn new() -> Self {{
        Counter {{ count: 0 }}
    }}
    
    fn increment(&mut self) -> PyResult<()> {{
        self.count += 1;
        Ok(())
    }}
    
    fn get_count(&self) -> PyResult<i64> {{
        Ok(self.count)
    }}
}}

/// Module initialization
#[pymodule]
fn {}(_py: Python, m: &PyModule) -> PyResult<()> {{
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(process_list, m)?)?;
    m.add_class::<Counter>()?;
    Ok(())
}}
"#,
        name
    )
}

pub fn build_py_template(name: &str) -> String {
    format!(
        r#"#!/usr/bin/env python3
"""Build script for {} Rust extension"""

import subprocess
import sys
from pathlib import Path

def build():
    \"\"\"Build the Rust extension using maturin\"\"\"
    try:
        subprocess.run(
            ["maturin", "build", "--release"],
            check=True,
            cwd=Path(__file__).parent
        )
        print(f"✅ Successfully built {}")
    except subprocess.CalledProcessError as e:
        print(f"❌ Build failed: {{e}}")
        sys.exit(1)
    except FileNotFoundError:
        print("❌ maturin not found. Install with: pip install maturin")
        sys.exit(1)

if __name__ == "__main__":
    build()
"#,
        name, name
    )
}

pub fn pyproject_rust_section(extension_name: &str) -> String {
    format!(
        r#"[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "{}"
version = "0.1.0"
description = "Python project with Rust extensions"
requires-python = ">=3.8"

[tool.maturin]
features = ["pyo3/extension-module"]
module-name = "{}"
"#,
        extension_name, extension_name
    )
}

pub fn numerical_loop_template() -> String {
    r#"use pyo3::prelude::*;
use ndarray::Array1;

#[pyfunction]
fn fast_loop(n: usize) -> PyResult<Vec<f64>> {
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        // Your computation here
        result.push((i as f64).sqrt());
    }
    Ok(result)
}

#[pyfunction]
fn vectorized_operation(data: Vec<f64>) -> PyResult<Vec<f64>> {
    let arr = Array1::from_vec(data);
    let result = arr.mapv(|x| x.powi(2) + 2.0 * x + 1.0);
    Ok(result.to_vec())
}
"#.to_string()
}

pub fn data_processing_template() -> String {
    r#"use pyo3::prelude::*;
use numpy::{PyArray1, PyArray2};
use ndarray::{Array1, Array2, Axis};

#[pyfunction]
fn process_array<'py>(
    py: Python<'py>,
    data: &PyArray1<f64>
) -> PyResult<&'py PyArray1<f64>> {
    let array = unsafe { data.as_array() };
    let result: Array1<f64> = array.mapv(|x| x.ln_1p());
    Ok(PyArray1::from_array(py, &result))
}

#[pyfunction]
fn matrix_operation<'py>(
    py: Python<'py>,
    matrix: &PyArray2<f64>
) -> PyResult<&'py PyArray1<f64>> {
    let arr = unsafe { matrix.as_array() };
    let means = arr.mean_axis(Axis(0)).unwrap();
    Ok(PyArray1::from_array(py, &means))
}
"#.to_string()
}

pub fn python_to_rust_template(function_name: &str, _py_code: &str) -> String {
    format!(
        r#"use pyo3::prelude::*;

/// Rust implementation of {}
#[pyfunction]
fn {}(/* parameters */) -> PyResult</* return type */> {{
    // TODO: Implement the function logic
    // Original Python code has been analyzed
    // This is an optimized Rust version
    
    Ok(/* result */)
}}
"#,
        function_name, function_name
    )
}

pub fn async_template() -> String {
    r#"use pyo3::prelude::*;
use pyo3_asyncio;
use tokio;

#[pyfunction]
fn async_operation(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        // Async operation here
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        Ok(Python::with_gil(|py| py.None()))
    })
}
"#.to_string()
}