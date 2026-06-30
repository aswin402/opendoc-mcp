//! Batch processing — process entire directories of documents.
//!
//! Uses rayon for parallel execution.

use rayon::prelude::*;
use std::path::Path;

/// Batch conversion: convert all files matching a pattern
pub fn batch_convert(
    input_dir: &str,
    pattern: &str,
    target_format: &str,
    output_dir: &str,
) -> Vec<BatchResult> {
    let entries: Vec<_> = match std::fs::read_dir(input_dir) {
        Ok(dir) => dir
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                let target_ext = pattern.trim_start_matches('*').trim_start_matches('.');
                if target_ext.is_empty() {
                    p.extension().is_some()
                } else {
                    p.extension()
                        .and_then(|e| e.to_str())
                        .map(|ext| ext == target_ext)
                        .unwrap_or(false)
                }
            })
            .collect(),
        Err(_) => return vec![],
    };

    entries
        .par_iter()
        .map(|path| {
            let filename = path.file_name().unwrap_or_default().to_string_lossy();
            let stem = path.file_stem().unwrap_or_default().to_string_lossy();
            let output_path = Path::new(output_dir).join(format!("{}.{}", stem, target_format));

            let result = crate::converters::convert(
                path.to_str().unwrap_or(""),
                target_format,
                output_path.to_str().unwrap_or(""),
            );

            match result {
                Ok(conv) => BatchResult {
                    file: filename.to_string(),
                    success: true,
                    output: conv.output,
                    error: None,
                },
                Err(e) => BatchResult {
                    file: filename.to_string(),
                    success: false,
                    output: String::new(),
                    error: Some(e.to_string()),
                },
            }
        })
        .collect()
}

/// Result of a single batch conversion operation.
#[derive(Debug, Clone, serde::Serialize)]
pub struct BatchResult {
    pub file: String,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}
