# Changelog — opendoc-mcp

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.0.2] — 2026-06-30

### Added

#### IR Architecture (Internal Representation)
- Universal `Document` model with `Paragraph`, `Table`, `Section`, `Image`, `Metadata`
- All handlers implement `to_ir()` for unified format-agnostic processing
- Engine module: `search` (regex/text), `replace`, `template`, `diff` (LCS), `complexity` (heuristics)
- Batch processor with rayon for parallel directory conversion
- Validators module for document structure checks
- `doc://` MCP resources for read-only document access

#### New Format Support
- **XLSX** — Read via `calamine`, write via `rust_xlsxwriter`, convert to/from IR
- **HTML** — Read/write via `scraper` + `html5ever`
- **Markdown** — Read/write via `pulldown-cmark`, export from IR
- **CSV** — Read/write via native Rust (included in IR pipeline)
- **TXT** — Read/write (plain text fallback)
- **PDF Forms** — List fields and fill AcroForm values

#### Expanded Conversion Pipeline
- Cross-format conversion via `converters::convert()` (DOCX→PDF, DOCX→MD, DOCX→HTML, PPTX→MD, PPTX→PDF, PDF→TXT, PDF→MD, XLSX→CSV)
- Generic IR→target export (JSON, TXT, MD, HTML, CSV, XLSX, DOCX)
- Real PPTX→PDF conversion (text extracted via IR, rendered via lopdf)
- Real PPTX image embedding (OPC/ZIP package manipulation with PNG, JPEG, GIF, BMP, TIFF, SVG)

#### Tool Consolidation
- Consolidated 39 tools into ~20 unified MCP tools
- `open_document` — Single tool for all formats (detail_level: full/summary/metadata_only)
- `replace_text` — Unified find/replace across DOCX, PDF, and IR
- `convert` — Unified conversion with `target_format` parameter
- Structured JSON params (`serde_json::Value`) for `fill_template`, `fill_pdf_form`, `create_xlsx`
- Structured error responses with `error_code`, `category`, and `suggestion`
- `list_capabilities` updated with all formats and tools

#### Infrastructure
- CLI subcommands via clap (`convert`, `extract`, `batch`, `merge`, `validate`, `info`, `diff`, `formats`, `serve`)
- Security module with `validate_path!()` macro and `OPENDOC_ALLOWED_DIRS` sandbox
- GitHub Actions CI (Linux: build + test + clippy)
- `rust-toolchain.toml` for MSRV pinning (1.75.0)
- AGENTS.md with full architecture documentation
- Criterion benchmark suite (3 benchmarks: DOCX→IR, TXT→IR, search)

### Documentation
- Doc comments on all public functions across all modules
- Updated README.md with current tool list and format support
- Updated architecture.md with IR-centric design
- Updated spec.md reflecting consolidated tools

### Fixed
- PPTX `to_pdf()` placeholder replaced with real converter delegation
- PPTX `add_slide_image()` placeholder replaced with real OPC image embedding

## [0.0.1] — 2026-06-28

### Added

#### DOCX Support
- `create_document` — Create new Word documents with optional title
- `open_document` — Read document metadata (paragraphs, tables, content count, author)
- `add_paragraph` — Append formatted text with bold, italic, and font size options
- `add_table` — Insert tables with headers and data rows
- `find_replace_text` — Regex-powered find and replace in document content
- `document_to_pdf` — Convert DOCX to PDF with selectable text (via `rdocx` layout engine)
- `document_to_markdown` — Convert DOCX to Markdown format

#### PPTX Support
- `create_presentation` — Create new PowerPoint files
- `open_presentation` — Read presentation metadata (slide count)
- `add_slide` — Add slides with title and optional body bullet points
- `add_slide_image` — Reference images on slides (basic, full embedding pending)
- `presentation_to_markdown` — Export slide content as Markdown
- `presentation_to_pdf` — Placeholder (will use `office2pdf` in v0.1.0)

#### PDF Support
- `create_pdf` — Generate single-page PDFs with Helvetica font
- `open_pdf` — Read PDF metadata (page count, encryption status, version)
- `merge_pdfs` — Combine multiple PDFs into one with automatic object renumbering
- `extract_pdf_text` — Extract text from full document or specific page
- `pdf_replace_text` — Find and replace text in PDF content streams

#### Utility
- `list_capabilities` — List all available tools, formats, and server version
- Server information with instructions for AI agents

#### Infrastructure
- MCP protocol compliance via `rmcp` SDK (spec 2025-06-18)
- stdio transport for zero-configuration setup
- Structured logging with `tracing` + `tracing-subscriber` (level controlled via `RUST_LOG`)
- JSON-RPC 2.0 compliant tool invocation
- All responses returned as structured JSON for easy AI agent consumption
- Modular handler architecture — each format is an independent module

### Technical Details

- **Runtime:** Tokio async (single-threaded for stdio transport)
- **DOCX engine:** `rdocx` 0.1 (pure Rust, no C dependencies)
- **PPTX engine:** `pptx` 0.1 (pure Rust OPC XML manipulation)
- **PDF engine:** `lopdf` 0.31 (pure Rust PDF read/write/merge)
- **Binary size:** ~4.2 MB (stripped release build)
- **Startup time:** ~3 ms to first tool invocation
- **Memory (idle):** ~3.5 MB RSS

### Known Limitations

- PPTX image embedding returns a guidance message (not yet implemented)
- PPTX→PDF conversion returns a guidance message (requires `office2pdf` crate)
- PDF creation is single-page only with fixed positioning
- No XLSX support yet (planned for v0.1.0)
- No HTML support yet (planned for v0.1.0)
- No native Markdown read/write yet (planned for v0.1.0)
- No unit tests yet (planned for v0.0.2)
- No CI pipeline yet (planned for v0.0.2)

---

## [Unreleased]

### Planned for v0.0.2

- [ ] Unit tests for all handler functions
- [ ] Integration tests for server tool dispatch
- [ ] PPTX image embedding (full binary image insertion)
- [ ] PPTX→PDF conversion via `office2pdf`
- [ ] GitHub Actions CI (Linux, macOS, Windows)
- [ ] Doc comments on all public functions
- [ ] Benchmark suite with criterion
- [ ] rust-toolchain.toml for MSRV pinning

### Planned for v0.1.0

- [ ] XLSX support (create, read, edit via `rust_xlsxwriter` + `calamine`)
- [ ] HTML read/write support
- [ ] Markdown read/write support (as native format)
- [ ] Template-based document generation (JSON + template → document)
- [ ] `anytomd-rs` integration for unified document→Markdown
- [ ] `office2pdf` integration for real office→PDF conversion

### Planned for v0.2.0

- [ ] Text chunking for RAG (by heading, token count, byte size)
- [ ] Batch directory processing (convert, extract, transform)
- [ ] Document diff / comparison
- [ ] Image extraction from DOCX/PPTX
- [ ] PDF split by page range
- [ ] Password/encryption support
- [ ] Streaming output for large documents

### Planned for v1.0.0

- [ ] Security audit
- [ ] Path traversal protection / sandboxed directories
- [ ] WASM compilation target
- [ ] Streamable HTTP transport
- [ ] Digital signatures (PDF)
- [ ] PDF/A validation
- [ ] Official documentation site

---

## Archive

*No prior versions. This is the initial release.*
