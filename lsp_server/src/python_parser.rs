// src/python_parser.rs
use anyhow::{anyhow, Context, Result};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ParseResult {
    pub nodes: Vec<NodeInfo>,
    pub substitutions: Vec<SubstitutionInfo>,
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct NodeInfo {
    #[serde(rename = "type")]
    pub type_: String,
    pub args: HashMap<String, String>,
    pub lineno: usize,
    pub col_offset: usize,
}

#[derive(Debug, Deserialize)]
pub struct SubstitutionInfo {
    pub name: String,
    pub lineno: usize,
    pub col_offset: usize,
}

//

pub fn parse_launch_file(code: &str) -> Result<ParseResult> {
    Python::with_gil(|py| {
        let ast_mod = py.import("ast").context("Failed to import ast module")?;
        let json_mod = py.import("json").context("Failed to import json module")?;

        let parsed = ast_mod.call_method1("parse", (code,))?;
        let parser_code = include_str!("../scripts/parser_launch.py");  // Fixed path

        let locals = PyDict::new(py);
        locals.set_item("ast", ast_mod)?;
        locals.set_item("parsed", parsed)?;
        locals.set_item("json", json_mod)?;

        py.run(parser_code, None, Some(&locals))  // Added reference &
            .context("Failed to execute parser code")?;

        let result_str = locals.get_item("result")
            .context("Failed to access Python locals")?  // Handle Result
            .ok_or_else(|| anyhow!("Missing result in Python locals"))?  // Handle Option
            .extract::<String>()
            .context("Failed to extract result string")?;

        serde_json::from_str(&result_str)
            .context("Failed to deserialize parse result")
    })
        
  
}
