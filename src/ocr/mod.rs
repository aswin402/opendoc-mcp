//! OCR pipeline for scanned documents.
//!
//! Provides OCR using pdfium-render (PDF→image) + tesseract (image→text).
//! This is an optional feature — when pdfium/tesseract aren't available,
//! OCR returns an error with guidance on how to enable it.

#[cfg(feature = "ocr")]
mod ocr_impl {
    use crate::ir::Document;

    pub fn ocr_document(_file_path: &str, _language: Option<&str>) -> Result<Document, String> {
        // TODO: v0.2.0 — implement using pdfium-render for page rendering
        // and tesseract-sys/tesseract crate for text extraction.
        //
        // Pipeline:
        //   1. pdfium_render::Pdfium::new() -> document
        //   2. For each page: render to image (300 DPI)
        //   3. tesseract::ocr(&image) -> text
        //   4. Assemble into Document IR
        //
        // For now, return a clear error with setup instructions.
        Err("OCR is planned for v0.2.0. Build with --features ocr and ensure pdfium/tesseract are installed on your system.\n\
             \n\
             Required system packages:\n\
             - Ubuntu/Debian: apt install libtesseract-dev libleptonica-dev\n\
             - macOS: brew install tesseract leptonica\n\
             - Windows: vcpkg install tesseract leptonica\n\
             \n\
             Required Rust deps: pdfium-render, pdfium-auto, tesseract-sys, leptess\n\
             \n\
             Alternatively, use external OCR and pass via --text flag.".to_string())
    }

    pub fn is_ocr_available() -> bool {
        // Will check for pdfium/tesseract at runtime
        false // placeholder
    }

    pub fn available_languages() -> Vec<String> {
        // Query tesseract for installed language packs
        vec!["eng".to_string()]
    }
}

#[cfg(not(feature = "ocr"))]
mod ocr_impl {
    use crate::ir::Document;

    pub fn ocr_document(_file_path: &str, _language: Option<&str>) -> Result<Document, String> {
        Err("OCR feature not enabled. Build with --features ocr to enable.\n\
             See https://github.com/aswin402/opendoc-mcp#ocr for setup instructions.".to_string())
    }

    pub fn is_ocr_available() -> bool {
        false
    }

    pub fn available_languages() -> Vec<String> {
        vec![]
    }
}

pub use ocr_impl::*;

/// OCR configuration
#[derive(Debug, Clone)]
pub struct OcrConfig {
    pub language: String,
    pub dpi: u32,
    pub psm: i32,          // Tesseract page segmentation mode
    pub preprocess: bool,  // Apply image preprocessing (deskew, denoise)
}

impl Default for OcrConfig {
    fn default() -> Self {
        Self {
            language: "eng".to_string(),
            dpi: 300,
            psm: 3,  // Fully automatic page segmentation
            preprocess: true,
        }
    }
}
