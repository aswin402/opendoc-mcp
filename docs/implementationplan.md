# Implementation Plan — opendoc-mcp

**Version:** 0.0.1
**Status:** Active (Pre-Alpha)
**Last Updated:** 2026-06-28

---

## 1. Overview

This document outlines the phased implementation plan for `opendoc-mcp`, from the current v0.0.1 pre-alpha through v1.0.0 production release.

### Current Status (v0.0.1)

| Component | Status | Notes |
|-----------|--------|-------|
| MCP Server framework | ✅ Complete | rmcp SDK, stdio transport, tool registration |
| DOCX handler | ✅ Basic | Create, open, edit, convert |
| PPTX handler | ✅ Basic | Create, open, edit, convert (partial) |
| PDF handler | ✅ Basic | Create, open, merge, extract, replace |
| Error handling | ✅ Complete | All errors return structured JSON |
| Logging | ✅ Complete | tracing subscriber with env filter |
| Tests | ❌ Missing | Need unit and integration tests |
| Documentation | ✅ Complete | README, PRD, Architecture, Spec, Changelog |

---

## 2. Phase 1: Core Stabilization (v0.0.2)

**Focus:** Fix known gaps, add tests, polish error handling.

### Tasks

| ID | Task | Priority | Est. Effort | Dependencies |
|----|------|----------|-------------|--------------|
| 1.1 | Add unit tests for all handler functions | High | 2 days | None |
| 1.2 | Add integration tests for server tool dispatch | High | 1 day | None |
| 1.3 | Implement PPTX image embedding (real, not placeholder) | Medium | 1 day | None |
| 1.4 | Implement PPTX→PDF conversion via `office2pdf` | Medium | 1 day | Add `office2pdf` dep |
| 1.5 | Cross-platform CI (Linux, macOS, Windows) | Medium | 1 day | GitHub Actions |
| 1.6 | Add doc comments to all public functions | Medium | 0.5 day | None |
| 1.7 | Benchmark suite with criterion | Low | 1 day | None |
| 1.8 | MSRV policy and rust-toolchain.toml | Low | 0.5 day | None |

### Deliverables

- [ ] `cargo test` passes with >80% code coverage
- [ ] CI pipeline runs on push/PR
- [ ] PPTX image embedding works end-to-end
- [ ] PPTX→PDF conversion produces valid output
- [ ] Published on crates.io as v0.0.2

---

## 3. Phase 2: Format Expansion (v0.1.0)

**Focus:** Add XLSX, HTML, Markdown support. Implement template engine.

### Tasks

| ID | Task | Priority | Est. Effort | Dependencies |
|----|------|----------|-------------|--------------|
| 2.1 | **XLSX handler** — Create spreadsheets via `rust_xlsxwriter` | High | 3 days | None |
| 2.2 | **XLSX handler** — Read spreadsheets via `calamine` | High | 2 days | None |
| 2.3 | **HTML handler** — Read/write HTML documents | High | 2 days | None |
| 2.4 | **Markdown handler** — Read/write Markdown as native format | High | 1 day | None |
| 2.5 | **Template engine** — JSON data + DOCX/PPTX template → filled document | High | 3 days | 2.1 |
| 2.6 | Integrate `anytomd-rs` for unified document→Markdown | Medium | 1 day | None |
| 2.7 | Integrate `office2pdf` for real office→PDF conversion | Medium | 1 day | None |
| 2.8 | Update server.rs with new tool registrations | High | 0.5 day | 2.1-2.4 |

### Deliverables

- [ ] XLSX create, open, edit, convert tools available
- [ ] HTML read/write tools available
- [ ] Markdown read/write tools available
- [ ] Template-based document generation works
- [ ] DOCX/PPTX→PDF conversion via `office2pdf` is functional
- [ ] Published on crates.io as v0.1.0

---

## 4. Phase 3: AI Agent Optimization (v0.2.0)

**Focus:** RAG pipeline support, batch processing, document intelligence.

### Tasks

| ID | Task | Priority | Est. Effort | Dependencies |
|----|------|----------|-------------|--------------|
| 3.1 | **Text chunking** — Split documents into chunks for RAG (by heading, token count, byte size) | High | 2 days | None |
| 3.2 | **Batch processor** — Process entire directories (convert all DOCX to PDF, extract all text, etc.) | High | 3 days | None |
| 3.3 | **Document metadata extraction** — Extract headings, links, images, tables as structured data | Medium | 2 days | None |
| 3.4 | **Image extraction** — Extract embedded images from DOCX/PPTX | Medium | 2 days | None |
| 3.5 | **Document diff** — Compare two documents and report changes | Medium | 3 days | None |
| 3.6 | **PDF split** — Split PDF by page range | Medium | 1 day | None |
| 3.7 | **Password protection** — Add/remove document encryption | Low | 2 days | None |
| 3.8 | **Streaming output** — Return large document content in chunks | Low | 2 days | None |

### Deliverables

- [ ] Text chunking with multiple strategies (heading, token, size)
- [ ] Batch directory processing (recursive, filtered by format)
- [ ] Document diff between two versions
- [ ] PDF split by page range
- [ ] Password/encryption support
- [ ] Published on crates.io as v0.2.0

---

## 5. Phase 4: Enterprise & Ecosystem (v1.0.0)

**Focus:** Production readiness, security audit, WASM support, integrations.

### Tasks

| ID | Task | Priority | Est. Effort | Dependencies |
|----|------|----------|-------------|--------------|
| 4.1 | **Security audit** — Third-party security review | High | 1 week | None |
| 4.2 | **Path traversal protection** — Sandbox directories with configurable allowlist | High | 1 day | None |
| 4.3 | **WASM target** — Compile core handlers to WebAssembly | High | 1 week | None |
| 4.4 | **Streamable HTTP transport** — Support MCP HTTP transport | Medium | 3 days | None |
| 4.5 | **Digital signatures** — PDF signing via `lopdf` or native crypto | Medium | 3 days | None |
| 4.6 | **PDF/A validation** — Check PDF/A compliance | Medium | 2 days | None |
| 4.7 | **Document comparison** — Visual diff for DOCX/PDF | Low | 1 week | 3.5 |
| 4.8 | **Plugin system** — Allow custom format handlers as dynamic libs | Low | 2 weeks | None |
| 4.9 | **Official website / docs site** — mkdocs or similar | Low | 1 week | None |

### Deliverables

- [ ] Security audit report completed and findings addressed
- [ ] WASM package published to npm
- [ ] Streamable HTTP transport implemented
- [ ] Digital signatures working for PDF
- [ ] PDF/A validation tool
- [ ] Published on crates.io as v1.0.0

---

## 6. Detailed Task Breakdown

### 6.1 Adding a New Format Handler (Template)

When adding support for a new document format, follow this pattern:

```
1. Create src/handlers/<format>.rs
2. Add pub mod <format> to src/handlers/mod.rs
3. Implement these functions:
   - create_<format>() -> JSON string
   - open_<format>() -> JSON string
   - Optional: edit/conversion functions
4. Add #[tool(...)] methods in server.rs
5. Add format to list_capabilities() tool
6. Add tests in tests/ directory
7. Update README.md format table
```

### 6.2 Converting Placeholders to Real Implementations

Current placeholders needing conversion:

| Location | Placeholder | Target Version | Replacement |
|----------|-------------|----------------|-------------|
| `pptx.rs:add_slide_image` | "Image embedding..." | v0.0.2 | Real image insertion via `pptx` crate |
| `pptx.rs:to_pdf` | "PPTX to PDF requires office2pdf crate" | v0.1.0 | `office2pdf::convert()` |
| `pdf.rs:create_pdf` | Single-page only, fixed position | v0.1.0 | Multi-page with proper layout |

---

## 7. Dependency Roadmap

| Dependency | v0.0.1 | v0.0.2 | v0.1.0 | v0.2.0 | v1.0.0 |
|------------|--------|--------|--------|--------|--------|
| `rmcp` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `rdocx` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `pptx` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `lopdf` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `regex` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `tokio` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `anyhow`/`thiserror` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `serde`/`serde_json` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `tracing` | ✅ | ✅ | ✅ | ✅ | ✅ |
| `office2pdf` | — | — | ✅ | ✅ | ✅ |
| `anytomd-rs` | — | — | ✅ | ✅ | ✅ |
| `rust_xlsxwriter` | — | — | ✅ | ✅ | ✅ |
| `calamine` | — | — | ✅ | ✅ | ✅ |
| `criterion` | — | ✅ | ✅ | ✅ | ✅ |

---

## 8. Risk Matrix

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| `rdocx` or `pptx` crate becomes unmaintained | Low | High | Keep fork-ready; have fallback crates (`docx-rs`, custom OPC) |
| MCP specification changes | Medium | Medium | `rmcp` SDK follows spec; update when spec bumps |
| Performance regressions from added features | Medium | Medium | Benchmark suite in CI; track perf targets |
| Rust edition/crate compatibility issues | Low | Medium | Pin MSRV; use `cargo deny` for dependency checks |
| AI agent unpredictable input (path injection, huge files) | Medium | Medium | Path validation; size limits; timeout on operations |

---

## 9. Testing Strategy

### 9.1 Test Levels

| Level | Tool | Coverage Target |
|-------|------|-----------------|
| Unit tests | built-in `#[test]` | 90% of handler functions |
| Integration tests | built-in `#[test]` | 100% of MCP tools |
| Benchmark tests | `criterion` | Key operations (create, open, convert) |
| Fuzz tests | `cargo-fuzz` | PDF/DOCX/PPTX parsing (future) |

### 9.2 Test Files

```
tests/
├── common/           # Shared test utilities
│   └── mod.rs
├── handlers/
│   ├── docx_tests.rs
│   ├── pdf_tests.rs
│   └── pptx_tests.rs
├── server_tests.rs   # MCP protocol-level tests
└── integration/
    ├── create_edit_read_workflow.rs
    ├── format_conversion_workflow.rs
    └── error_handling.rs
```

### 9.3 Test Scenarios (Per Handler)

```
✅ Success: Valid file, valid parameters
✅ Error:   Non-existent file path
✅ Error:   Invalid format (e.g., PDF passed to DOCX handler)
✅ Error:   Permission denied
✅ Edge:    Empty document
✅ Edge:    Very large text content
✅ Edge:    Unicode/special characters in content
✅ Edge:    Concurrent access to the same file
```

---

## 10. Release Process

### 10.1 Version Bump Checklist

- [ ] All tests pass (`cargo test`)
- [ ] No compiler warnings (`cargo check`)
- [ ] Benchmark targets met (if applicable)
- [ ] CHANGELOG.md updated with new version entry
- [ ] Cargo.toml version updated
- [ ] README.md updated if features changed
- [ ] `cargo publish --dry-run` succeeds
- [ ] Git tag created (`v0.0.1`, `v0.1.0`, etc.)

### 10.2 Release Cadence

| Phase | Cadence | Target |
|-------|---------|--------|
| v0.0.x (Pre-alpha) | As needed | Fixes, small features |
| v0.1.x - v0.2.x (Alpha) | Monthly | Major features |
| v1.0.0 (Stable) | — | Production readiness |

---

## 11. Effort Summary

| Phase | Features | Est. Effort | Timeline |
|-------|----------|-------------|----------|
| v0.0.2 | Tests, PPTX images, PPTX→PDF, CI | 1 week | Next |
| v0.1.0 | XLSX, HTML, MD, template engine | 2-3 weeks | Month 2 |
| v0.2.0 | RAG, batch, diff, split, encrypt | 3-4 weeks | Month 3 |
| v1.0.0 | Security, WASM, signatures, HTTP | 4-6 weeks | Month 4-5 |

**Total estimated time to v1.0.0:** 4-5 months (part-time contributor pace)
**Total estimated time to v1.0.0:** 2-3 months (dedicated development)
