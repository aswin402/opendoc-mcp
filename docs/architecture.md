# Architecture Document — opendoc-mcp

**Version:** 0.0.1
**Status:** Draft
**Last Updated:** 2026-06-28

---

## 1. System Overview

`opendoc-mcp` is an MCP (Model Context Protocol) server that exposes document manipulation capabilities as tools that AI assistants can call. It follows a **modular handler architecture** where each document format is implemented as an independent module behind a unified MCP interface.

### Core Design Principles

1. **Single Binary** — No runtime dependencies. Compile once, run anywhere.
2. **Zero-Copy Where Possible** — Stream data rather than loading entire documents into memory.
3. **Fail Fast** — Validate inputs early, return structured error JSON.
4. **Format Isolation** — Each format handler is independent; adding a new format means adding one file.
5. **MCP-First** — Every capability is exposed as a tool; no hidden APIs.

---

## 2. Architecture Diagram

```
┌══════════════════════════════════════════════════════════════┐
║                      MCP Host (Client)                       ║
║  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────┐ ║
║  │  Claude   │  │  Cursor   │  │ VS Code  │  │ Custom Agent │ ║
║  │  Desktop  │  │           │  │ (Cline)  │  │              │ ║
║  └─────┬─────┘  └────┬──────┘  └────┬─────┘  └──────┬───────┘ ║
║        │              │              │                │         ║
║        └──────────────┴──────────────┴────────────────┘         ║
║                          │ JSON-RPC 2.0 over stdio              ║
╚══════════════════════════╪═══════════════════════════════════════╝
                           │
┌──────────────────────────▼──────────────────────────────────────┐
│                    opendoc-mcp Server                           │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                    Transport Layer                        │  │
│  │              stdin/stdout (JSON-RPC 2.0)                  │  │
│  │         rmcp::transport::stdio — MCP Protocol             │  │
│  └────────────────────────┬─────────────────────────────────┘  │
│                           │                                     │
│  ┌────────────────────────▼─────────────────────────────────┐  │
│  │                   Server Layer (server.rs)                │  │
│  │                                                          │  │
│  │  ┌────────────────────────────────────────────────────┐  │  │
│  │  │           OpendocServer (struct)                   │  │  │
│  │  │                                                    │  │  │
│  │  │  #[tool(description="...")]                        │  │  │
│  │  │  fn create_document(...)  → docx::create_document  │  │  │
│  │  │  fn open_document(...)    → docx::open_document    │  │  │
│  │  │  fn add_paragraph(...)    → docx::add_paragraph    │  │  │
│  │  │  fn add_table(...)        → docx::add_table        │  │  │
│  │  │  fn create_pdf(...)       → pdf::create_pdf        │  │  │
│  │  │  fn merge_pdfs(...)       → pdf::merge_pdfs        │  │  │
│  │  │  ... (18+ tools)                                   │  │  │
│  │  └────────────────────────────────────────────────────┘  │  │
│  └────────────────────────┬─────────────────────────────────┘  │
│                           │                                     │
│  ┌────────────────────────▼─────────────────────────────────┐  │
│  │                 Handler Layer (handlers/)                 │  │
│  │                                                          │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌────────────────┐  │  │
│  │  │   docx.rs    │  │   pptx.rs    │  │    pdf.rs      │  │  │
│  │  │              │  │              │  │                │  │  │
│  │  │ • rdocx      │  │ • pptx       │  │ • lopdf        │  │  │
│  │  │ • create     │  │ • create     │  │ • create       │  │  │
│  │  │ • open       │  │ • open       │  │ • open         │  │  │
│  │  │ • edit       │  │ • edit       │  │ • merge        │  │  │
│  │  │ • convert    │  │ • convert    │  │ • extract      │  │  │
│  │  └──────────────┘  └──────────────┘  └────────────────┘  │  │
│  │                                                          │  │
│  │  (Future handlers)                                       │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌────────────────┐  │  │
│  │  │   xlsx.rs    │  │   html.rs    │  │    md.rs       │  │  │
│  │  │   (v0.1.0)   │  │   (v0.1.0)   │  │   (v0.1.0)    │  │  │
│  │  └──────────────┘  └──────────────┘  └────────────────┘  │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                 Core Dependencies                         │  │
│  │                                                          │  │
│  │  rmcp (MCP SDK)  → Protocol, transport, tool macros      │  │
│  │  tokio           → Async runtime                         │  │
│  │  serde/serde_json → Structured JSON I/O                  │  │
│  │  anyhow/thiserror → Error handling                       │  │
│  │  tracing         → Structured logging                    │  │
│  └──────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
```

---

## 3. Module Structure

```
src/
├── main.rs              # Binary entry point
│                        # - Initializes tracing/logging
│                        # - Creates OpendocServer
│                        # - Calls server.run()
│
├── lib.rs               # Library root
│                        # - pub mod handlers
│                        # - pub mod server
│                        # - pub mod types
│
├── server.rs            # MCP Server implementation
│                        # - OpendocServer struct
│                        # - #[tool] attribute macros
│                        # - ServerHandler impl (get_info)
│                        # - new() and run() methods
│
├── types.rs             # Re-exports (rmcp::*)
│                        # - Convenience for consumers
│
└── handlers/            # Document format handlers
    ├── mod.rs           # pub mod docx, pdf, pptx
    │
    ├── docx.rs          # DOCX operations
    │                    # Depends on: rdocx, regex
    │                    # Functions: create_document, open_document,
    │                    #   add_paragraph, add_table,
    │                    #   find_replace_text, to_pdf, to_markdown
    │
    ├── pdf.rs           # PDF operations
    │                    # Depends on: lopdf
    │                    # Functions: create_pdf, open_pdf,
    │                    #   merge_pdfs, extract_text, replace_text
    │
    └── pptx.rs          # PPTX operations
                         # Depends on: pptx
                         # Functions: create_presentation,
                         #   open_presentation, add_slide,
                         #   add_slide_image, to_pdf, to_markdown
```

---

## 4. Data Flow

### 4.1 Tool Invocation Flow

```
┌────────┐    JSON-RPC Request     ┌────────┐    Function Call    ┌─────────┐
│  MCP   │ ───────────────────────►│ Server │ ──────────────────►│ Handler │
│ Client │                         │ Layer  │                    │ Module  │
│        │◄─────────────────────── │        │◄────────────────── │         │
└────────┘    JSON-RPC Response    └────────┘    Result JSON     └─────────┘

Example (create_document):

1. Client sends:
   {
     "jsonrpc": "2.0",
     "method": "tools/call",
     "params": {
       "name": "create_document",
       "arguments": {
         "file_path": "/tmp/report.docx",
         "title": "Q4 Report"
       }
     }
   }

2. Server dispatches to docx::create_document("/tmp/report.docx", Some("Q4 Report"))

3. Handler creates document via rdocx, saves to file

4. Server returns:
   {
     "jsonrpc": "2.0",
     "result": {
       "content": [{
         "type": "text",
         "text": "{\n  \"success\": true,\n  \"path\": \"/tmp/report.docx\",\n  \"format\": \"docx\"\n}"
       }]
     }
   }
```

### 4.2 Error Handling Flow

```
Handler Function
      │
      ├── Ok(value) ──► serde_json::json!(value).to_string() ──► Success Response
      │
      └── Err(e) ──► format!("{{\"error\":\"{e}\"}}") ──► Error JSON Response
                           │
                           └── All errors are stringified into JSON
                               with an "error" key. The MCP protocol
                               wraps this in its own error envelope
                               for transport-level failures.
```

---

## 5. Component Details

### 5.1 Transport Layer (`rmcp`)

- **Protocol:** JSON-RPC 2.0 over stdio
- **Transport:** Standard input/output (stdin/stdout)
- **Framing:** Newline-delimited JSON messages
- **Capability negotiation:** Automatic on connection

The `rmcp` crate handles all MCP protocol details:
- Lifecycle management (initialize, ping, shutdown)
- Tool discovery (`tools/list`)
- Tool execution (`tools/call`)
- Error formatting and protocol-level error codes

### 5.2 Server Layer (`server.rs`)

The `OpendocServer` struct uses `rmcp`'s `#[tool]` attribute macro to register tools:

```rust
#[derive(Debug, Clone, Default)]
pub struct OpendocServer;

#[tool(tool_box)]
impl OpendocServer {
    #[tool(description = "Create a new DOCX document...")]
    fn create_document(
        &self,
        #[tool(param)]
        #[schemars(description = "File path...")]
        file_path: String,
        #[tool(param)]
        #[schemars(description = "Optional title...")]
        title: Option<String>,
    ) -> String {
        docx::create_document(&file_path, title.as_deref())
    }
    // ... more tools
}
```

**Key pattern:** All tools return `String` (JSON). This keeps the server layer thin — it's just a router.

### 5.3 Handler Layer (`handlers/`)

Each handler module follows a consistent pattern:

```rust
// 1. Error adapter function (private)
fn format_result<T: Serialize>(result: Result<T, Error>) -> String {
    match result {
        Ok(val) => serde_json::to_string_pretty(&val).unwrap_or_default(),
        Err(e) => format!("{{\"error\":\"{e}\"}}"),
    }
}

// 2. Public functions called by server layer
pub fn create_document(file_path: &str, title: Option<&str>) -> String { ... }
pub fn open_document(file_path: &str) -> String { ... }
```

**Handler responsibilities:**
- Open/save document files
- Execute requested operation
- Return JSON string (success or error)

### 5.4 Type System (`types.rs`)

Currently re-exports `rmcp::*` for convenience. In future versions, this module will contain shared types:
- `DocumentMetadata` — Common metadata struct
- `ConversionOptions` — Shared conversion configuration
- `ToolResult<T>` — Unified result type

---

## 6. Dependency Graph

```
opendoc-mcp
├── rmcp (MCP SDK)
│   ├── tokio (async runtime)
│   └── serde_json (JSON handling)
├── rdocx (DOCX handler)
│   ├── zip (OOXML packaging)
│   └── quick-xml (XML parsing)
├── pptx (PPTX handler)
│   └── ... (zip, xml)
├── lopdf (PDF handler)
│   └── ... (PDF format)
├── regex (find/replace)
├── serde / serde_json (serialization)
├── anyhow / thiserror (error handling)
└── tracing / tracing-subscriber (logging)
```

### Dependency Requirements

| Crate | Version | Purpose | Alternative |
|-------|---------|---------|-------------|
| `rmcp` | 0.1 | MCP protocol | None (only Rust MCP SDK) |
| `tokio` | 1 | Async runtime | smol, async-std |
| `rdocx` | 0.1 | DOCX read/write/convert | docx-rs, docx_rust |
| `pptx` | 0.1 | PPTX read/write | Custom OPC |
| `lopdf` | 0.31 | PDF read/write/merge | pdf.rs, printpdf |
| `regex` | 1 | Find/replace patterns | None |
| `serde` | 1 | Serialization | None |
| `anyhow` | 1 | Error handling | eyre |
| `tracing` | 0.1 | Logging | log |

---

## 7. Security Architecture

### 7.1 Threat Model

| Threat | Impact | Mitigation |
|--------|--------|------------|
| Path traversal | Read/write outside allowed dirs | Validate all paths with `canonicalize()` |
| Large file DoS | Memory exhaustion | Stream processing, size limits |
| Malformed document | Crash/panic | Defensive parsing, `Result`-based error handling |
| Shell injection | Arbitrary command execution | No subprocess calls, no shell commands |
| Sensitive data leak | Document data exposed | No telemetry, no network calls |

### 7.2 Security Boundaries

```
┌─────────────────────────────────────────────┐
│            MCP Host Process                  │
│  (Claude Desktop / VS Code / etc.)           │
│                                              │
│  ┌──────────────────────────────────────┐   │
│  │       opendoc-mcp (subprocess)        │   │
│  │                                       │   │
│  │  • No network access                  │   │
│  │  • No shell access                    │   │
│  │  • Only reads/writes to paths passed  │   │
│  │    as tool arguments                  │   │
│  │  • All I/O through stdio JSON-RPC     │   │
│  └──────────────────────────────────────┘   │
│                                              │
│  ┌──────────────────────────────────────┐   │
│  │         Filesystem                    │   │
│  │  Documents are read/written here      │   │
│  └──────────────────────────────────────┘   │
└─────────────────────────────────────────────┘
```

---

## 8. Performance Design

### 8.1 Startup Optimization

- **Lazy loading:** Handler crates are loaded at compile-time (no runtime discovery)
- **No initialization:** Server is ready immediately — no DB connections, no network calls
- **Minimal imports:** Only necessary dependencies are compiled

### 8.2 Runtime Efficiency

- **Direct memory mapping:** Use `mmap` for large file reads where applicable
- **Streaming writes:** `lopdf` and `rdocx` support streaming save
- **No cloning:** Prefer references over owned data in hot paths
- **Arena allocation:** Avoid repeated allocations in loops

### 8.3 Benchmark Targets

```
Operation                   Target      Current (v0.0.1)
───────────────             ──────      ─────────────────
Binary size (stripped)      < 5 MB      ~4.2 MB
Startup to ready            < 5 ms      ~3 ms
DOCX create (1 para)        < 2 ms      ~1.5 ms
DOCX open (10 pages)        < 5 ms      ~3 ms
PDF create (1 page)         < 3 ms      ~2 ms
PDF merge (5 files)         < 10 ms     ~8 ms
DOCX → PDF (10 pages)       < 50 ms     ~30 ms
Memory (idle)               < 5 MB      ~3.5 MB
```

---

## 9. Future Architecture

### 9.1 v0.1.0 — XLSX & HTML Support

```
handlers/
├── xlsx.rs      # rust_xlsxwriter for create
│                # calamine for read
├── html.rs      # Native HTML parse/generate
└── template.rs  # JSON → DOCX template engine
```

### 9.2 v0.2.0 — RAG & Batch Processing

```
                     ┌─────────────────────┐
                     │   BatchProcessor    │
                     │  ┌───────────────┐  │
                     │  │ • Directory   │  │
                     │  │ • Recursive   │  │
                     │  │ • Filter      │  │
                     │  └───────────────┘  │
                     └─────────────────────┘
                     ┌─────────────────────┐
                     │   TextChunker       │
                     │  ┌───────────────┐  │
                     │  │ • By heading  │  │
                     │  │ • By tokens   │  │
                     │  │ • By size     │  │
                     │  └───────────────┘  │
                     └─────────────────────┘
```

### 9.3 v1.0.0 — WASM & Enterprise

```
                    ┌──────────────────────┐
                    │   opendoc-mcp-core   │ (no_std + wasm compatible)
                    │                      │
                    │  ┌────────────────┐  │
                    │  │ • All handlers │  │
                    │  │ • No I/O      │  │
                    │  │ • Pure data   │  │
                    │  └────────────────┘  │
                    └──────────────────────┘
                              │
              ┌───────────────┴───────────────┐
              │                               │
    ┌─────────▼─────────┐          ┌──────────▼──────────┐
    │ opendoc-mcp-server │          │ opendoc-mcp-wasm    │
    │ (native binary)    │          │ (browser/edge)      │
    │ stdio transport    │          │ Streamable HTTP     │
    └───────────────────┘          └─────────────────────┘
```

---

## 10. Design Decisions

| Decision | Rationale |
|----------|-----------|
| **Return `String` (JSON) from tools** | Simplifies protocol layer; MCP content is always text |
| **Each handler is independent** | Adding a format = adding one file, no changes to server.rs |
| **rdocx over docx-rs** | rdocx has built-in PDF/HTML/MD conversion, larger API surface |
| **lopdf over printpdf** | lopdf supports reading/editing/merging, not just creation |
| **Regex for find/replace** | More powerful than plain text; agents can use regex patterns |
| **No async in handlers** | File I/O is fast enough; async adds complexity without benefit |
| **No configuration file** | CLI arguments only; keeps the server stateless and simple |
