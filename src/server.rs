use crate::handlers::{docx, pdf, pptx};
use rmcp::{
    model::{ServerCapabilities, ServerInfo},
    serve_server, tool,
    transport::stdio,
    ServerHandler,
};

#[derive(Debug, Clone, Default)]
pub struct OpendocServer;

#[tool(tool_box)]
impl OpendocServer {
    // ── DOCX Tools ──

    #[tool(description = "Create a new DOCX document and save to a file path. Returns the path to the created file.")]
    fn create_document(
        &self,
        #[tool(param)]
        #[schemars(description = "File path to save the document (e.g., /path/to/output.docx)")]
        file_path: String,
        #[tool(param)]
        #[schemars(description = "Optional title for the document")]
        title: Option<String>,
    ) -> String {
        docx::create_document(&file_path, title.as_deref())
    }

    #[tool(description = "Open an existing DOCX document and return its metadata (pages, paragraphs, etc.)")]
    fn open_document(
        &self,
        #[tool(param)]
        #[schemars(description = "File path to the existing document")]
        file_path: String,
    ) -> String {
        docx::open_document(&file_path)
    }

    #[tool(description = "Add a paragraph with text to a DOCX document at a specific path")]
    fn add_paragraph(
        &self,
        #[tool(param)]
        #[schemars(description = "File path to the document")]
        file_path: String,
        #[tool(param)]
        #[schemars(description = "Text content of the paragraph")]
        text: String,
        #[tool(param)]
        #[schemars(description = "Optional bold formatting")]
        bold: Option<bool>,
        #[tool(param)]
        #[schemars(description = "Optional italic formatting")]
        italic: Option<bool>,
        #[tool(param)]
        #[schemars(description = "Optional font size in points (e.g., 12)")]
        font_size: Option<f32>,
    ) -> String {
        docx::add_paragraph(&file_path, &text, bold, italic, font_size)
    }

    #[tool(description = "Add a table to a DOCX document")]
    fn add_table(
        &self,
        #[tool(param)]
        #[schemars(description = "File path to the document")]
        file_path: String,
        #[tool(param)]
        #[schemars(description = "Headers for the table as a JSON array of strings")]
        headers: Vec<String>,
        #[tool(param)]
        #[schemars(description = "Table data as a JSON array of arrays of strings")]
        data: Vec<Vec<String>>,
    ) -> String {
        docx::add_table(&file_path, &headers, &data)
    }

    #[tool(description = "Find and replace text in a DOCX document")]
    fn find_replace_text(
        &self,
        #[tool(param)]
        #[schemars(description = "File path to the document")]
        file_path: String,
        #[tool(param)]
        #[schemars(description = "Text to find")]
        find: String,
        #[tool(param)]
        #[schemars(description = "Replacement text")]
        replace: String,
    ) -> String {
        docx::find_replace_text(&file_path, &find, &replace)
    }

    #[tool(description = "Convert a DOCX document to PDF")]
    fn document_to_pdf(
        &self,
        #[tool(param)]
        #[schemars(description = "Source DOCX file path")]
        source: String,
        #[tool(param)]
        #[schemars(description = "Output PDF file path")]
        output: String,
    ) -> String {
        docx::to_pdf(&source, &output)
    }

    #[tool(description = "Convert a DOCX document to Markdown")]
    fn document_to_markdown(
        &self,
        #[tool(param)]
        #[schemars(description = "Source DOCX file path")]
        source: String,
        #[tool(param)]
        #[schemars(description = "Output Markdown file path")]
        output: String,
    ) -> String {
        docx::to_markdown(&source, &output)
    }

    // ── PPTX Tools ──

    #[tool(description = "Create a new PowerPoint presentation and save to a file path")]
    fn create_presentation(
        &self,
        #[tool(param)]
        #[schemars(description = "File path to save the presentation (e.g., /path/to/output.pptx)")]
        file_path: String,
        #[tool(param)]
        #[schemars(description = "Optional title for the presentation")]
        title: Option<String>,
    ) -> String {
        pptx::create_presentation(&file_path, title.as_deref())
    }

    #[tool(description = "Open an existing presentation and return its metadata")]
    fn open_presentation(
        &self,
        #[tool(param)]
        #[schemars(description = "File path to the existing presentation")]
        file_path: String,
    ) -> String {
        pptx::open_presentation(&file_path)
    }

    #[tool(description = "Add a slide to a presentation with a title and optional body text")]
    fn add_slide(
        &self,
        #[tool(param)]
        #[schemars(description = "File path to the presentation")]
        file_path: String,
        #[tool(param)]
        #[schemars(description = "Slide title")]
        title: String,
        #[tool(param)]
        #[schemars(description = "Optional body content (bullet points as JSON array)")]
        body: Option<Vec<String>>,
    ) -> String {
        pptx::add_slide(&file_path, &title, body.as_deref())
    }

    #[tool(description = "Add an image to a slide in a presentation")]
    fn add_slide_image(
        &self,
        #[tool(param)]
        #[schemars(description = "File path to the presentation")]
        file_path: String,
        #[tool(param)]
        #[schemars(description = "Slide number (1-based)")]
        slide_number: u32,
        #[tool(param)]
        #[schemars(description = "Path to the image file")]
        image_path: String,
    ) -> String {
        pptx::add_slide_image(&file_path, slide_number, &image_path)
    }

    #[tool(description = "Convert a presentation to PDF")]
    fn presentation_to_pdf(
        &self,
        #[tool(param)]
        #[schemars(description = "Source PPTX file path")]
        source: String,
        #[tool(param)]
        #[schemars(description = "Output PDF file path")]
        output: String,
    ) -> String {
        pptx::to_pdf(&source, &output)
    }

    #[tool(description = "Export presentation to Markdown text")]
    fn presentation_to_markdown(
        &self,
        #[tool(param)]
        #[schemars(description = "Source PPTX file path")]
        source: String,
    ) -> String {
        pptx::to_markdown(&source)
    }

    // ── PDF Tools ──

    #[tool(description = "Create a simple PDF with text content")]
    fn create_pdf(
        &self,
        #[tool(param)]
        #[schemars(description = "File path to save the PDF")]
        file_path: String,
        #[tool(param)]
        #[schemars(description = "Text content for the PDF")]
        text: String,
        #[tool(param)]
        #[schemars(description = "Optional author metadata")]
        author: Option<String>,
    ) -> String {
        pdf::create_pdf(&file_path, &text, author.as_deref())
    }

    #[tool(description = "Open a PDF and extract its metadata and text content")]
    fn open_pdf(
        &self,
        #[tool(param)]
        #[schemars(description = "File path to the PDF")]
        file_path: String,
    ) -> String {
        pdf::open_pdf(&file_path)
    }

    #[tool(description = "Merge multiple PDF files into one")]
    fn merge_pdfs(
        &self,
        #[tool(param)]
        #[schemars(description = "List of source PDF file paths to merge")]
        sources: Vec<String>,
        #[tool(param)]
        #[schemars(description = "Output PDF file path")]
        output: String,
    ) -> String {
        pdf::merge_pdfs(&sources, &output)
    }

    #[tool(description = "Extract text from a PDF file")]
    fn extract_pdf_text(
        &self,
        #[tool(param)]
        #[schemars(description = "File path to the PDF")]
        file_path: String,
        #[tool(param)]
        #[schemars(description = "Optional page number (0-based) to extract from specific page")]
        page: Option<u32>,
    ) -> String {
        pdf::extract_text(&file_path, page)
    }

    #[tool(description = "Replace text in a PDF document")]
    fn pdf_replace_text(
        &self,
        #[tool(param)]
        #[schemars(description = "File path to the PDF")]
        file_path: String,
        #[tool(param)]
        #[schemars(description = "Text to find")]
        find: String,
        #[tool(param)]
        #[schemars(description = "Replacement text")]
        replace: String,
    ) -> String {
        pdf::replace_text(&file_path, &find, &replace)
    }

    // ── Utility Tools ──

    #[tool(description = "List all available tools and their descriptions")]
    fn list_capabilities(&self) -> String {
        serde_json::to_string_pretty(&serde_json::json!({
            "server": "opendoc-mcp",
            "version": env!("CARGO_PKG_VERSION"),
            "formats": ["DOCX", "PPTX", "PDF"],
            "tools": [
                "create_document", "open_document", "add_paragraph", "add_table",
                "find_replace_text", "document_to_pdf", "document_to_markdown",
                "create_presentation", "open_presentation", "add_slide",
                "add_slide_image", "presentation_to_pdf", "presentation_to_markdown",
                "create_pdf", "open_pdf", "merge_pdfs", "extract_pdf_text", "pdf_replace_text"
            ]
        }))
        .unwrap_or_default()
    }
}

#[tool(tool_box)]
impl ServerHandler for OpendocServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Opendoc MCP Server — High-performance document CRUD for AI agents. \
                Supports DOCX, PPTX, and PDF formats. \
                Use the tools below to create, read, edit, and convert documents."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

impl OpendocServer {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn run(self) -> anyhow::Result<()> {
        tracing::info!("Starting Opendoc MCP Server via stdio...");
        serve_server(self, stdio()).await?;
        Ok(())
    }
}
