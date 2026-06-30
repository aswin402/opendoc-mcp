pub mod csv;
pub mod docx;
pub mod html;
pub mod md;
pub mod pdf;
pub mod pdf_forms;
pub mod pptx;
pub mod xlsx;

use crate::ir::Document;
use std::path::Path;

/// Load any supported document into the Internal Representation (IR).
///
/// This is the universal entry point. Format is detected from file extension.
pub fn load_to_ir(file_path: &str) -> Result<Document, LoadError> {
    let path = Path::new(file_path);
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "docx" | "doc" => {
            let doc = docx::to_ir(file_path)?;
            Ok(doc)
        }
        "pptx" | "ppt" => {
            let doc = pptx::to_ir(file_path)?;
            Ok(doc)
        }
        "pdf" => {
            let mut doc = pdf::to_ir(file_path)?;
            // Try to attach form field info if available
            if let Ok(fields) = pdf_forms::list_form_fields(file_path) {
                doc.metadata.form_fields = Some(fields.len());
            }
            Ok(doc)
        }
        "xlsx" | "xls" => {
            let doc = xlsx::to_ir(file_path)
                .map_err(LoadError::ParseError)?;
            Ok(doc)
        }
        "md" | "markdown" => {
            let doc = md::to_ir(file_path)
                .map_err(LoadError::ParseError)?;
            Ok(doc)
        }
        "html" | "htm" => {
            let doc = html::to_ir(file_path)
                .map_err(LoadError::ParseError)?;
            Ok(doc)
        }
        "csv" => {
            let doc = csv::to_ir(file_path)
                .map_err(LoadError::ParseError)?;
            Ok(doc)
        }
        "txt" | "text" => {
            let content = std::fs::read_to_string(file_path)
                .map_err(|e| LoadError::IoError(e.to_string()))?;
            let mut doc = Document::new("txt");
            doc.text = Some(content);
            Ok(doc)
        }
        _ => Err(LoadError::UnsupportedFormat(ext)),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}
