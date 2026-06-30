# Implementation Plan — opendoc-mcp

**Version:** 0.0.2-dev
**Status:** Active
**Last Updated:** 2026-06-30

---

## 1. Overview

This document outlines the phased implementation plan for `opendoc-mcp`, tracking progress from the current v0.0.1 through v1.0.0.

### Current Status (v0.0.2-dev)

| Component | Status | Notes |
|-----------|--------|-------|
| MCP Server framework | ✅ Complete | rmcp SDK, stdio transport, tool registration, doc:// resources |
| DOCX handler | ✅ Complete | Create, open, edit, convert |
| PPTX handler | ✅ Partial | Create, open, edit, convert — image embedding is placeholder |
| PDF handler | ✅ Complete | Create, open, merge, extract, replace, AcroForm fill |
| XLSX handler | ✅ Complete | read via calamine, write via rust_xlsxwriter |
| HTML handler | ✅ Complete | read/write via scraper + html5ever |
| Markdown handler | ✅ Complete | read/write via pulldown-cmark |
| CSV handler | ✅ Complete | read/write |
| IR (Internal Representation) | ✅ Complete | Document/Paragraph/Table/Section/Image/Metadata pipeline |
| Engine (search/replace/template/diff/complexity) | ✅ Complete | LCS diff, regex search, template filling, complexity heuristics |
| Batch processor | ✅ Complete | Rayon-parallel directory conversion |
| Validators | ✅ Complete | Structure validation |
| Security | ✅ Complete | Path validation, OPENDOC_ALLOWED_DIRS sandbox |
| CLI | ✅ Complete | clap subcommands (convert, extract, batch, merge, validate, info, diff, formats, serve) |
| Error handling | ✅ Complete | Structured JSON with error_code, category, suggestion |
| Logging | ✅ Complete | tracing subscriber with env filter |
| Tests (unit + integration) | ✅ 20 tests | IR pipeline tests |
| CI (GitHub Actions) | ✅ Complete | Linux build + test + clippy |
| Documentation | ✅ Complete | README, AGENTS.md, PRD, Architecture, Spec, Changelog |
| PPTX image embedding | ❌ Placeholder | Returns guidance message |
| PPTX→PDF conversion | ❌ Placeholder | Returns guidance message |
| Doc comments | ❌ Missing | Need docs on public functions |
| Benchmark suite | ❌ Missing | Need criterion benchmarks |
| rust-toolchain.toml | ❌ Missing | Need MSRV pinning |
| OCR | ❌ Feature-gated | Placeholder module behind `ocr` feature flag |
| WASM | ❌ Not started | Future target |

---

## 2. Phase 1: Core Polish (v0.0.2)

**Focus:** Replace placeholders, add benchmarks, docs, MSRV.

### Tasks

| ID | Task | Priority | Est. Effort | Status |
|----|------|----------|-------------|--------|
| 1.1 | Real PPTX image embedding (binary image insertion) | Medium | 1 day | ❌ |
| 1.2 | PPTX→PDF conversion via `office2pdf` crate | Medium | 1 day | ❌ |
| 1.3 | Doc comments on all public functions | Medium | 0.5 day | ❌ |
| 1.4 | Criterion benchmark suite | Low | 1 day | ❌ |
| 1.5 | rust-toolchain.toml for MSRV pinning | Low | 0.5 day | ❌ |
| 1.6 | Update changelog and bump version | Low | 0.5 day | ❌ |

### Deliverables

- [ ] PPTX image embedding works end-to-end
- [ ] PPTX→PDF conversion produces valid output
- [ ] `cargo doc --no-deps` passes with no warnings
- [ ] Criterion benchmarks for key operations
- [ ] MSRV pinned in rust-toolchain.toml
- [ ] Published on crates.io as v0.0.2

---

## 3. Phase 2: Format Deepening (v0.1.0)

**Focus:** Real PPTX→PDF, template engine enhancements, remaining format polish.

| ID | Task | Priority | Est. Effort | Status |
|----|------|----------|-------------|--------|
| 2.1 | Real PPTX→PDF conversion (not placeholder) | High | 1 day | ❌ |
| 2.2 | Multi-page PDF creation with layout | Medium | 2 days | ❌ |
| 2.3 | Enhanced template engine (nested objects, loops) | Medium | 2 days | ❌ |
| 2.4 | DOCX image insertion | Medium | 1 day | ❌ |
| 2.5 | Expanded test coverage (80%+) | High | 2 days | ❌ |

### Deliverables

- [ ] All format handlers fully implemented (no placeholders)
- [ ] Template engine supports complex templates
- [ ] >80% code coverage
- [ ] Published as v0.1.0

---

## 4. Phase 3: AI Agent Optimization (v0.2.0)

**Focus:** RAG pipeline support, document intelligence, batch processing features.

| ID | Task | Priority | Est. Effort | Status |
|----|------|----------|-------------|--------|
| 3.1 | Text chunking strategies (heading, token, size) | High | 2 days | ❌ |
| 3.2 | Document diff between versions | Medium | 2 days | ✅ (engine/diff.rs) |
| 3.3 | Image extraction from DOCX/PPTX | Medium | 2 days | ❌ |
| 3.4 | PDF split by page range | Low | 1 day | ❌ |
| 3.5 | Batch operations (convert all X to Y) | High | 1 day | ✅ (src/batch/) |
| 3.6 | Password/encryption support | Low | 2 days | ❌ |

### Deliverables

- [ ] Text chunking with configurable strategies
- [ ] Image extraction from office documents
- [ ] PDF split and merge enhancements
- [ ] Published as v0.2.0

---

## 5. Phase 4: Enterprise & Ecosystem (v1.0.0)

**Focus:** Production readiness, WASM, signatures, streaming.

| ID | Task | Priority | Est. Effort | Status |
|----|------|----------|-------------|--------|
| 4.1 | Security audit | High | 1 week | ❌ |
| 4.2 | WASM compilation target | High | 1 week | ❌ |
| 4.3 | Streamable HTTP transport | Medium | 3 days | ❌ |
| 4.4 | Digital signatures (PDF) | Medium | 3 days | ❌ |
| 4.5 | PDF/A validation | Medium | 2 days | ❌ |
| 4.6 | Streamable output for large docs | Low | 2 days | ❌ |

### Deliverables

- [ ] Security audit completed
- [ ] WASM package publishable via npm
- [ ] HTTP transport support
- [ ] Published as v1.0.0

---

## 6. Architecture

```
DOCX ──┐
PPTX ──┤
PDF  ──┤──▶  IR  ──▶  engine (search/replace/template/diff) ──▶  export
XLSX ──┤
HTML ──┤
MD/CSV─┘
```

Every format handler implements `load_to_ir()` (import) and `save_from_ir()` (export). The engine operates exclusively on IR, making all operations format-agnostic.

---

## 7. Testing Strategy

| Level | Tool | Coverage Target |
|-------|------|-----------------|
| Unit tests | `#[test]` | 80%+ of handler functions |
| Integration tests | `#[test]` | 100% of MCP tools |
| Benchmarks | `criterion` | Key operations (create, open, convert) |
| Doc tests | `#[doc]` | All public API examples |

---

## 8. Risk Matrix

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| `rdocx` / `pptx` crate unmaintained | Low | High | Keep fork-ready; fallback crates |
| MCP spec changes | Medium | Medium | `rmcp` follows spec; bump on change |
| Performance regressions | Medium | Medium | Benchmark suite in CI |
| Rust compatibility | Low | Medium | MSRV pinning + `cargo deny` |
| Path injection / huge files | Medium | Medium | Security module + size limits |

---

## 9. Release Cadence

| Phase | Cadence | Target |
|-------|---------|--------|
| v0.0.x (Pre-alpha) | As needed | Bug fixes, polish |
| v0.1.x (Alpha) | Monthly | Major features |
| v1.0.0 (Stable) | — | Production readiness |
