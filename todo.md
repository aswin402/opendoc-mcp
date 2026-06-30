# opendoc-mcp Improvement To-Do List

Detailed steps can be found in the [Implementation Plan](file:///home/aswin/programming/vscode/myProjects/ai_agent_tools/opendoc/docs/superpowers/plans/2026-06-30-mcp-refactor-consolidation.md).

- [x] **Task 1: Consolidate Open/Read/Metadata Tools**
  - [x] Consolidate `open_pptx`, `open_pdf`, `summarize_structure`, `document_statistics`, `extract_metadata` into a unified `open_document` tool.
- [x] **Task 2: Consolidate Replace/Find Tools**
  - [x] Consolidate `docx_find_replace`, `pdf_replace_text` into a unified `replace_text` tool.
- [x] **Task 3: Consolidate Conversion Tools**
  - [x] Consolidate `docx_to_pdf`, `docx_to_markdown`, `pptx_to_markdown`, `export_to_xlsx` into the unified `convert` tool.
- [x] **Task 4: Standardize Input Parameters (Structured JSON)**
  - [x] Update `fill_template` (`variables`), `fill_pdf_form` (`values`), `create_xlsx` (`sheets`) to accept raw JSON objects (`serde_json::Value`).
- [x] **Task 5: Structured Error Responses**
  - [x] Implement `"error_code"`, `"category"`, and `"suggestion"` fields for error reporting.
- [x] **Task 6: Expose Document Resources**
  - [x] Implement read-only document primitives like `doc://{path}` and `doc://{path}/outline`.
- [x] **Task 7: Clean up & Documentation Update**
  - [x] Remove unused handler methods.
  - [x] Update `README.md` to match actual tool names.
