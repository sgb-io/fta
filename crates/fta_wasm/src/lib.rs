use fta::analyze;
use serde::Deserialize;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[derive(Deserialize)]
pub struct AnalyzeOptions {
    #[serde(default)]
    json: bool,
}

impl Default for AnalyzeOptions {
    fn default() -> Self {
        Self { json: false }
    }
}

#[wasm_bindgen]
pub fn analyze_project(project_path: &str, options: JsValue) {
    let default_options = AnalyzeOptions::default();
    let options: AnalyzeOptions = from_value(options).unwrap_or(default_options);
    analyze(&project_path.to_string(), options.json);
}
