use rdocx::Document;

fn docx_result_to_string<T: serde::Serialize>(result: Result<T, rdocx::Error>) -> String {
    match result {
        Ok(val) => serde_json::to_string_pretty(&val).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}")),
        Err(e) => format!("{{\"error\":\"{e}\"}}"),
    }
}

pub fn create_document(file_path: &str, title: Option<&str>) -> String {
    let mut doc = Document::new();
    doc.set_author("Opendoc MCP");

    if let Some(t) = title {
        let mut p = doc.add_paragraph(t);
        p.add_run(t).bold(true).size(24.0);
    }

    match doc.save(file_path) {
        Ok(_) => serde_json::json!({"success": true, "path": file_path, "format": "docx"}).to_string(),
        Err(e) => format!("{{\"error\":\"{e}\"}}"),
    }
}

pub fn open_document(file_path: &str) -> String {
    match Document::open(file_path) {
        Ok(doc) => {
            let info = serde_json::json!({
                "path": file_path,
                "paragraphs": doc.paragraph_count(),
                "tables": doc.table_count(),
                "content_items": doc.content_count(),
                "title": doc.title(),
                "author": doc.author(),
            });
            serde_json::to_string_pretty(&info).unwrap_or_default()
        }
        Err(e) => format!("{{\"error\":\"{e}\"}}"),
    }
}

pub fn add_paragraph(
    file_path: &str,
    text: &str,
    bold: Option<bool>,
    italic: Option<bool>,
    font_size: Option<f32>,
) -> String {
    let mut doc = match Document::open(file_path) {
        Ok(d) => d,
        Err(e) => return format!("{{\"error\":\"{e}\"}}"),
    };

    let mut p = doc.add_paragraph(text);
    let mut run = p.add_run(text);
    if bold.unwrap_or(false) {
        run = run.bold(true);
    }
    if italic.unwrap_or(false) {
        run = run.italic(true);
    }
    if let Some(sz) = font_size {
        let _ = run.size(sz as f64);
    }

    match doc.save(file_path) {
        Ok(_) => serde_json::json!({"success": true, "path": file_path, "text_length": text.len()}).to_string(),
        Err(e) => format!("{{\"error\":\"{e}\"}}"),
    }
}

pub fn add_table(file_path: &str, headers: &[String], data: &[Vec<String>]) -> String {
    let mut doc = match Document::open(file_path) {
        Ok(d) => d,
        Err(e) => return format!("{{\"error\":\"{e}\"}}"),
    };

    let rows = data.len() + 1; // +1 for header
    let cols = headers.len().max(if data.is_empty() { 0 } else { data[0].len() });

    let mut table = doc.add_table(rows, cols);

    // Set headers
    for (col, header) in headers.iter().enumerate() {
        if let Some(mut cell) = table.cell(0, col) {
            let _ = cell.set_text(header);
        }
    }

    // Set data
    for (row_idx, row_data) in data.iter().enumerate() {
        for (col_idx, cell_text) in row_data.iter().enumerate() {
            if col_idx < cols {
                    if let Some(mut cell) = table.cell(row_idx + 1, col_idx) {
                    let _ = cell.set_text(cell_text);
                }
            }
        }
    }

    match doc.save(file_path) {
        Ok(_) => serde_json::json!({"success": true, "rows": rows, "cols": cols}).to_string(),
        Err(e) => format!("{{\"error\":\"{e}\"}}"),
    }
}

pub fn find_replace_text(file_path: &str, find: &str, replace: &str) -> String {
    let mut doc = match Document::open(file_path) {
        Ok(d) => d,
        Err(e) => return format!("{{\"error\":\"{e}\"}}"),
    };

    // Use regex-based find/replace on paragraph text
    let re = match regex::Regex::new(find) {
        Ok(r) => r,
        Err(e) => return format!("{{\"error\":\"invalid regex: {e}\"}}"),
    };

    let mut count = 0u32;
    // Work with paragraphs
    let para_indices: Vec<usize> = (0..doc.paragraph_count()).collect();
    for idx in para_indices {
        if let Some(mut p) = doc.paragraph_mut(idx) {
            let text = p.text();
            if re.is_match(&text) {
                let new_text = re.replace_all(&text, replace).to_string();
                // Clear existing runs and add new text
                let _ = p.add_run(&new_text);
                count += 1;
            }
        }
    }

    match doc.save(file_path) {
        Ok(_) => serde_json::json!({"success": true, "replacements": count}).to_string(),
        Err(e) => format!("{{\"error\":\"{e}\"}}"),
    }
}

pub fn to_pdf(source: &str, output: &str) -> String {
    let doc = match Document::open(source) {
        Ok(d) => d,
        Err(e) => return format!("{{\"error\":\"{e}\"}}"),
    };

    match doc.to_pdf() {
        Ok(pdf_bytes) => {
            match std::fs::write(output, &pdf_bytes) {
                Ok(_) => serde_json::json!({
                    "success": true,
                    "source": source,
                    "output": output,
                    "size_bytes": pdf_bytes.len()
                }).to_string(),
                Err(e) => format!("{{\"error\":\"{e}\"}}"),
            }
        }
        Err(e) => format!("{{\"error\":\"{e}\"}}"),
    }
}

pub fn to_markdown(source: &str, output: &str) -> String {
    let doc = match Document::open(source) {
        Ok(d) => d,
        Err(e) => return format!("{{\"error\":\"{e}\"}}"),
    };

    let md = doc.to_markdown();
    match std::fs::write(output, &md) {
        Ok(_) => serde_json::json!({
            "success": true,
            "source": source,
            "output": output,
            "size_bytes": md.len()
        }).to_string(),
        Err(e) => format!("{{\"error\":\"{e}\"}}"),
    }
}
