use serde::{Deserialize, Serialize};
use crate::ir::elements::*;
use crate::ir::metadata::Metadata;

/// The universal document representation.
///
/// Every format handler converts to/from this struct.
/// All editing tools operate on this struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// File format (docx, pptx, pdf, xlsx, html, md, csv)
    pub format: String,

    /// File path if loaded from disk
    pub path: Option<String>,

    /// Document metadata
    pub metadata: Metadata,

    /// Content sections (headings, slides, sheets)
    pub sections: Vec<Section>,

    /// Paragraph content
    pub paragraphs: Vec<Paragraph>,

    /// Tables
    pub tables: Vec<Table>,

    /// Embedded images
    pub images: Vec<Image>,

    /// Raw text content (for formats without rich structure)
    pub text: Option<String>,
}

impl Document {
    /// Create a new Document with the given format identifier.
    /// Format is a lowercase string like "docx", "pdf", "pptx", "xlsx", "html", "md", "csv", "txt".
    pub fn new(format: impl Into<String>) -> Self {
        Self {
            format: format.into(),
            path: None,
            metadata: Metadata::default(),
            sections: Vec::new(),
            paragraphs: Vec::new(),
            tables: Vec::new(),
            images: Vec::new(),
            text: None,
        }
    }

    /// Get the document outline (headings / slide titles / sheet names)
    pub fn outline(&self) -> Vec<&str> {
        let mut out = Vec::new();
        for section in &self.sections {
            out.push(section.title.as_str());
        }
        for p in &self.paragraphs {
            if p.is_heading {
                out.push(p.text.as_str());
            }
        }
        out
    }

    /// Estimate token count for LLM context window planning
    pub fn estimate_tokens(&self) -> usize {
        let mut count = 0usize;
        for p in &self.paragraphs {
            count += p.text.len() / 4; // ~4 chars per token
        }
        for t in &self.tables {
            for row in &t.rows {
                for cell in row {
                    count += cell.len() / 4;
                }
            }
        }
        if let Some(text) = &self.text {
            count += text.len() / 4;
        }
        count
    }

    /// Chunk document for RAG pipelines
    pub fn chunk_for_embedding(&self, max_tokens: usize) -> Vec<Chunk> {
        let mut chunks = Vec::new();
        let mut current = String::new();
        let mut current_tokens = 0usize;
        let mut current_heading = String::new();

        for p in &self.paragraphs {
            if p.is_heading {
                current_heading = p.text.clone();
            }
            let tokens = p.text.len() / 4 + 1;
            if current_tokens + tokens > max_tokens && !current.is_empty() {
                chunks.push(Chunk {
                    text: current.clone(),
                    heading: current_heading.clone(),
                    index: chunks.len(),
                });
                current.clear();
                current_tokens = 0;
            }
            current.push_str(&p.text);
            current.push('\n');
            current_tokens += tokens;
        }

        if !current.is_empty() {
            chunks.push(Chunk {
                text: current,
                heading: current_heading,
                index: chunks.len(),
            });
        }

        chunks
    }
}

/// A section of a document (chapter, slide, worksheet)
/// A content section within a document (heading group, slide, or worksheet).
/// Sections help organize hierarchical document structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub title: String,
    pub level: u32,       // 0 = root, 1 = heading 1, 2 = heading 2, etc.
    pub index: usize,
    pub content: Vec<Paragraph>,
}

/// A chunk of text for RAG embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub text: String,
    pub heading: String,
    pub index: usize,
}
