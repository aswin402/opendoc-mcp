use rdocx::Document;


/// Create a new DOCX document and save to a file path.
/// Returns the path to the created file on success.
pub fn create_document(file_path: &str, title: Option<&str>) -> String {
    let mut doc = Document::new();
    doc.set_author("Opendoc MCP");

    if let Some(t) = title {
        let mut p = doc.add_paragraph("");
        p.add_run(t).bold(true).size(24.0);
    }

    match doc.save(file_path) {
        Ok(_) => serde_json::json!({"success": true, "path": file_path, "format": "docx"}).to_string(),
        Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
    }
}

/// Open a DOCX file and return its metadata (paragraphs, tables, content count, author).
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
        Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
    }
}

/// Append a formatted paragraph to an existing DOCX document.
/// Supports bold, italic, and font size options.
pub fn add_paragraph(
    file_path: &str,
    text: &str,
    bold: Option<bool>,
    italic: Option<bool>,
    font_size: Option<f32>,
) -> String {
    let mut doc = match Document::open(file_path) {
        Ok(d) => d,
        Err(e) => return serde_json::json!({"error": e.to_string()}).to_string(),
    };

    let mut p = doc.add_paragraph("");
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
        Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
    }
}

/// Insert a table with headers and data rows into a DOCX document.
pub fn add_table(file_path: &str, headers: &[String], data: &[Vec<String>]) -> String {
    let mut doc = match Document::open(file_path) {
        Ok(d) => d,
        Err(e) => return serde_json::json!({"error": e.to_string()}).to_string(),
    };

    let rows = data.len() + 1; // +1 for header
    let cols = headers.len().max(if data.is_empty() { 0 } else { data[0].len() });

    let mut table = doc.add_table(rows, cols);

    // Set headers
    for (col, header) in headers.iter().enumerate() {
        if let Some(mut cell) = table.cell(0, col) {
            cell.set_text(header);
        }
    }

    // Set data
    for (row_idx, row_data) in data.iter().enumerate() {
        for (col_idx, cell_text) in row_data.iter().enumerate() {
            if col_idx < cols {
                    if let Some(mut cell) = table.cell(row_idx + 1, col_idx) {
                    cell.set_text(cell_text);
                }
            }
        }
    }

    match doc.save(file_path) {
        Ok(_) => serde_json::json!({"success": true, "rows": rows, "cols": cols}).to_string(),
        Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
    }
}

/// Find and replace text in a DOCX document using regex pattern matching.
pub fn find_replace_text(file_path: &str, find: &str, replace: &str) -> String {
    let mut doc = match Document::open(file_path) {
        Ok(d) => d,
        Err(e) => return serde_json::json!({"error": e.to_string()}).to_string(),
    };

    match doc.replace_regex(find, replace) {
        Ok(count) => match doc.save(file_path) {
            Ok(_) => serde_json::json!({"success": true, "replacements": count}).to_string(),
            Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
        },
        Err(e) => serde_json::json!({"error": format!("Replace error: {e}")}).to_string(),
    }
}

/// Convert a DOCX document to PDF using the rdocx layout engine.
pub fn to_pdf(source: &str, output: &str) -> String {
    let doc = match Document::open(source) {
        Ok(d) => d,
        Err(e) => return serde_json::json!({"error": e.to_string()}).to_string(),
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
                Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
            }
        }
        Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
    }
}

/// Export a DOCX document to Markdown format.
pub fn to_markdown(source: &str, output: &str) -> String {
    let doc = match Document::open(source) {
        Ok(d) => d,
        Err(e) => return serde_json::json!({"error": e.to_string()}).to_string(),
    };

    let md = doc.to_markdown();
    match std::fs::write(output, &md) {
        Ok(_) => serde_json::json!({
            "success": true,
            "source": source,
            "output": output,
            "size_bytes": md.len()
        }).to_string(),
        Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
    }
}

/// Embed an image into a DOCX document.
pub fn add_image(
    file_path: &str,
    image_path: &str,
    width_inches: Option<f64>,
    height_inches: Option<f64>,
) -> String {
    let mut doc = match Document::open(file_path) {
        Ok(d) => d,
        Err(e) => return serde_json::json!({"error": e.to_string()}).to_string(),
    };

    let img_bytes = match std::fs::read(image_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            return serde_json::json!({
                "error": format!("Cannot read image file '{}': {}", image_path, e),
                "suggestion": "Check that the image path exists and is readable."
            })
            .to_string();
        }
    };

    let path = std::path::Path::new(image_path);
    let filename = path.file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("image.png");

    let w = rdocx::Length::inches(width_inches.unwrap_or(2.0));
    let h = rdocx::Length::inches(height_inches.unwrap_or(1.5));

    doc.add_picture(&img_bytes, filename, w, h);

    match doc.save(file_path) {
        Ok(_) => serde_json::json!({
            "success": true,
            "path": file_path,
            "image": image_path,
            "width_inches": width_inches.unwrap_or(2.0),
            "height_inches": height_inches.unwrap_or(1.5),
        }).to_string(),
        Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
    }
}

/// Load a DOCX file into the Internal Representation (IR)
pub fn to_ir(file_path: &str) -> Result<crate::ir::Document, crate::handlers::LoadError> {
    let doc = Document::open(file_path)
        .map_err(|e| crate::handlers::LoadError::ParseError(e.to_string()))?;

    let mut ir = crate::ir::Document::new("docx");
    ir.path = Some(file_path.to_string());
    ir.metadata.title = doc.title().map(|s| s.to_string());
    ir.metadata.author = doc.author().map(|s| s.to_string());

    for p in doc.paragraphs() {
        let text = p.text().to_string();
        if !text.is_empty() {
            ir.paragraphs.push(crate::ir::elements::Paragraph::new(text));
        }
    }

    for img in doc.images() {
        let width_pixels = (img.width_emu as f64 / 9525.0) as u32;
        let height_pixels = (img.height_emu as f64 / 9525.0) as u32;

        ir.images.push(crate::ir::elements::Image {
            name: img.name.clone().unwrap_or_else(|| img.embed_id.clone()),
            width: width_pixels,
            height: height_pixels,
            mime_type: "image/png".to_string(),
            data_base64: None,
            path: None,
        });
    }

    for table in doc.tables() {
            let rows = table.row_count();
            let cols = table.column_count();
            let mut headers = Vec::new();
            let mut data = Vec::new();

            for row in 0..rows {
                let mut row_data = Vec::new();
                for col in 0..cols {
                    if let Some(cell) = table.cell(row, col) {
                        row_data.push(cell.text().to_string());
                    } else {
                        row_data.push(String::new());
                    }
                }
                if row == 0 {
                    headers = row_data;
                } else {
                    data.push(row_data);
                }
            }

            ir.tables.push(crate::ir::elements::Table::new(headers, data));
    }

    Ok(ir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docx_lifecycle() {
        let dir = std::env::temp_dir();
        let path = dir.join("test_lifecycle.docx");
        let p = path.to_str().unwrap();

        // 1. Create document
        let res = create_document(p, Some("My Title"));
        assert!(res.contains("\"success\":true"));

        // 2. Open document info
        let info = open_document(p);
        assert!(info.contains("\"paragraphs\": 1"));

        // 3. Add paragraph
        let res_p = add_paragraph(p, "New Paragraph", Some(true), Some(false), Some(14.0));
        assert!(res_p.contains("\"success\":true"));

        // 4. Add table
        let headers = vec!["ColA".to_string(), "ColB".to_string()];
        let data = vec![vec!["A1".to_string(), "B1".to_string()]];
        let res_t = add_table(p, &headers, &data);
        assert!(res_t.contains("\"success\":true"));

        // 5. Find and replace
        let res_r = find_replace_text(p, "Paragraph", "DocPara");
        assert!(res_r.contains("\"success\":true"));

        // 6. Add image
        let img_path = dir.join("test_lifecycle_img.png");
        let png_data: Vec<u8> = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
            0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
            0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53, 0xDE,
            0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54,
            0x08, 0xD7, 0x63, 0xF8, 0xCF, 0xC0, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0xE2, 0x21, 0xBC,
            0x33, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44,
            0xAE, 0x42, 0x60, 0x82,
        ];
        std::fs::write(&img_path, &png_data).unwrap();
        let res_img = add_image(p, img_path.to_str().unwrap(), Some(2.0), Some(1.5));
        assert!(res_img.contains("\"success\":true"));

        // 7. Convert to IR
        let ir = to_ir(p).unwrap();
        assert_eq!(ir.paragraphs.len(), 2); // Title and paragraph
        assert_eq!(ir.tables.len(), 1);
        assert_eq!(ir.tables[0].headers, vec!["ColA", "ColB"]);
        assert_eq!(ir.images.len(), 1);

        // 8. Extract images
        let out_img_dir = dir.join("extracted_docx_imgs");
        let imgs = crate::handlers::extract_images_from_zip(p, out_img_dir.to_str().unwrap()).unwrap();
        assert_eq!(imgs.len(), 1);
        assert!(std::path::Path::new(&imgs[0]).exists());

        // Clean up
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_file(img_path);
        let _ = std::fs::remove_dir_all(out_img_dir);
    }
}
