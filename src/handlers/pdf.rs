use lopdf::{Document, Object, ObjectId, Stream, Dictionary};


pub fn create_pdf(file_path: &str, text: &str, _author: Option<&str>) -> String {
    let mut doc = Document::new();

    // Create font object
    let font_id = doc.add_object(Object::Dictionary(Dictionary::from_iter([
        (b"Type".to_vec(), Object::Name(b"Font".to_vec())),
        (b"Subtype".to_vec(), Object::Name(b"Type1".to_vec())),
        (b"BaseFont".to_vec(), Object::Name(b"Helvetica".to_vec())),
    ])));

    // Split text into lines
    let mut lines = Vec::new();
    for line in text.lines() {
        let escaped = line
            .replace('\\', "\\\\")
            .replace('(', "\\(")
            .replace(')', "\\)");
        lines.push(escaped);
    }

    let mut page_ids = Vec::new();
    let pages_id = doc.new_object_id();

    // Group lines into pages (40 lines per page)
    let chunks = lines.chunks(40);
    for chunk in chunks {
        let mut content_parts = Vec::new();
        content_parts.push("BT /F1 12 Tf 50 700 Td".to_string());
        for (i, line) in chunk.iter().enumerate() {
            if i > 0 {
                content_parts.push(format!("0 -15 Td ({}) Tj", line));
            } else {
                content_parts.push(format!("({}) Tj", line));
            }
        }
        content_parts.push("ET".to_string());
        let content_str = content_parts.join(" ");

        let content_id = doc.add_object(Object::Stream(Stream {
            dict: Dictionary::new(),
            content: content_str.into_bytes(),
            allows_compression: true,
            start_position: None,
        }));

        let page_id = doc.new_object_id();
        let page = Object::Dictionary(Dictionary::from_iter([
            (b"Type".to_vec(), Object::Name(b"Page".to_vec())),
            (b"Parent".to_vec(), Object::Reference(pages_id)),
            (b"Contents".to_vec(), Object::Reference(content_id)),
            (
                b"Resources".to_vec(),
                Object::Dictionary(Dictionary::from_iter([(
                    b"Font".to_vec(),
                    Object::Dictionary(Dictionary::from_iter([(
                        b"F1".to_vec(),
                        Object::Reference(font_id),
                    )])),
                )])),
            ),
            (
                b"MediaBox".to_vec(),
                Object::Array(vec![
                    Object::Integer(0),
                    Object::Integer(0),
                    Object::Integer(612),
                    Object::Integer(792),
                ]),
            ),
        ]));

        doc.objects.insert(page_id, page);
        page_ids.push(page_id);
    }

    if page_ids.is_empty() {
        let content_id = doc.add_object(Object::Stream(Stream {
            dict: Dictionary::new(),
            content: b"BT /F1 12 Tf 50 700 Td () Tj ET".to_vec(),
            allows_compression: true,
            start_position: None,
        }));
        let page_id = doc.new_object_id();
        let page = Object::Dictionary(Dictionary::from_iter([
            (b"Type".to_vec(), Object::Name(b"Page".to_vec())),
            (b"Parent".to_vec(), Object::Reference(pages_id)),
            (b"Contents".to_vec(), Object::Reference(content_id)),
            (
                b"Resources".to_vec(),
                Object::Dictionary(Dictionary::from_iter([(
                    b"Font".to_vec(),
                    Object::Dictionary(Dictionary::from_iter([(
                        b"F1".to_vec(),
                        Object::Reference(font_id),
                    )])),
                )])),
            ),
            (
                b"MediaBox".to_vec(),
                Object::Array(vec![
                    Object::Integer(0),
                    Object::Integer(0),
                    Object::Integer(612),
                    Object::Integer(792),
                ]),
            ),
        ]));
        doc.objects.insert(page_id, page);
        page_ids.push(page_id);
    }

    let pages = Object::Dictionary(Dictionary::from_iter([
        (b"Type".to_vec(), Object::Name(b"Pages".to_vec())),
        (
            b"Kids".to_vec(),
            Object::Array(page_ids.iter().map(|id| Object::Reference(*id)).collect()),
        ),
        (b"Count".to_vec(), Object::Integer(page_ids.len() as i64)),
    ]));
    doc.objects.insert(pages_id, pages);

    let catalog_id = doc.new_object_id();
    let catalog = Object::Dictionary(Dictionary::from_iter([
        (b"Type".to_vec(), Object::Name(b"Catalog".to_vec())),
        (b"Pages".to_vec(), Object::Reference(pages_id)),
    ]));
    doc.objects.insert(catalog_id, catalog);
    doc.trailer.set("Root", Object::Reference(catalog_id));

    doc.max_id = catalog_id.0;

    match doc.save(file_path) {
        Ok(_) => serde_json::json!({
            "success": true,
            "path": file_path,
            "format": "pdf",
            "pages": page_ids.len()
        }).to_string(),
        Err(e) => serde_json::json!({"error": format!("io: {e}")}).to_string(),
    }
}

pub fn open_pdf(file_path: &str) -> String {
    match Document::load(file_path) {
        Ok(doc) => {
            let pages = doc.get_pages();
            let page_count = pages.len();
            let is_encrypted = doc.is_encrypted();

            serde_json::json!({
                "path": file_path,
                "pages": page_count,
                "encrypted": is_encrypted,
                "version": doc.version,
            }).to_string()
        }
        Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
    }
}

pub fn merge_pdfs(sources: &[String], output: &str) -> String {
    if sources.is_empty() {
        return serde_json::json!({"error": "no source files provided"}).to_string();
    }

    let mut result_doc = match Document::load(&sources[0]) {
        Ok(d) => d,
        Err(e) => return serde_json::json!({"error": e.to_string()}).to_string(),
    };

    let mut max_id = result_doc.max_id + 1;

    for source in sources.iter().skip(1) {
        let doc = match Document::load(source) {
            Ok(d) => d,
            Err(e) => return serde_json::json!({"error": e.to_string()}).to_string(),
        };

        let mut renumbered = doc;
        renumbered.renumber_objects_with(max_id);
        let source_max_id = renumbered.max_id;
        max_id = source_max_id + 1;

        // Get pages before merging
        let page_ids: Vec<ObjectId> = renumbered.get_pages().values().copied().collect();

        // Add all objects
        for (obj_id, obj) in std::mem::take(&mut renumbered.objects) {
            result_doc.objects.insert(obj_id, obj);
        }

        // Get the root Pages object ID from Catalog
        let mut root_pages_id = None;
        if let Ok(catalog) = result_doc.catalog() {
            if let Ok(Object::Reference(p_ref)) = catalog.get(b"Pages") {
                root_pages_id = Some(*p_ref);
            }
        }

        if let Some(p_id) = root_pages_id {
            if let Ok(dict) = result_doc.get_dictionary_mut(p_id) {
                if let Ok(Object::Array(ref mut kids)) = dict.get_mut(b"Kids") {
                    for page_id in page_ids {
                        kids.push(Object::Reference(page_id));
                    }
                }
            }
        }

        // Find and update the Pages tree in result_doc
        let page_count = result_doc.get_pages().len() as i64;

        // Update page count in pages dict - find it by type
        let pages_to_update: Vec<ObjectId> = result_doc.objects.iter()
            .filter(|(_, obj)| {
                if let Object::Dictionary(dict) = obj {
                    matches!(dict.get(b"Type"), Ok(Object::Name(name)) if name == b"Pages")
                } else {
                    false
                }
            })
            .map(|(id, _)| *id)
            .collect();

        for pages_id in pages_to_update {
            if let Ok(dict) = result_doc.get_dictionary_mut(pages_id) {
                dict.set("Count", Object::Integer(page_count));
            }
        }
    }

    result_doc.max_id = max_id - 1;

    match result_doc.save(output) {
        Ok(_) => serde_json::json!({
            "success": true,
            "sources": sources,
            "output": output
        }).to_string(),
        Err(e) => serde_json::json!({"error": format!("io: {e}")}).to_string(),
    }
}

pub fn extract_text(file_path: &str, page: Option<u32>) -> String {
    match Document::load(file_path) {
        Ok(doc) => {
            let pages = doc.get_pages();
            let page_numbers: Vec<u32> = match page {
                Some(p) if p < pages.len() as u32 => {
                    let keys: Vec<&u32> = pages.keys().collect();
                    vec![*keys[p as usize]]
                }
                None => pages.keys().copied().collect(),
                _ => return serde_json::json!({"error": "page number out of range"}).to_string(),
            };

            match doc.extract_text(&page_numbers) {
                Ok(text) => serde_json::json!({"success": true, "text": text, "pages": page_numbers.len()}).to_string(),
                Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
            }
        }
        Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
    }
}

pub fn replace_text(file_path: &str, find: &str, replace: &str) -> String {
    let mut doc = match Document::load(file_path) {
        Ok(d) => d,
        Err(e) => return serde_json::json!({"error": e.to_string()}).to_string(),
    };

    let pages = doc.get_pages();
    let mut total_replacements = 0u32;
    for page_num in pages.keys() {
        if doc.replace_text(*page_num, find, replace).is_ok() {
            total_replacements += 1;
        }
    }
    match doc.save(file_path) {
        Ok(_) => serde_json::json!({
            "success": true,
            "pages_modified": total_replacements,
            "find": find,
            "replace": replace
        }).to_string(),
        Err(e) => serde_json::json!({"error": format!("io: {e}")}).to_string(),
    }
}

/// Load a PDF file into the Internal Representation (IR)
pub fn to_ir(file_path: &str) -> Result<crate::ir::Document, crate::handlers::LoadError> {
    let doc = lopdf::Document::load(file_path)
        .map_err(|e| crate::handlers::LoadError::ParseError(e.to_string()))?;

    let mut ir = crate::ir::Document::new("pdf");
    ir.path = Some(file_path.to_string());
    ir.metadata.page_count = Some(doc.get_pages().len() as u32);
    ir.metadata.encrypted = doc.is_encrypted();

    let pages: Vec<u32> = doc.get_pages().keys().copied().collect();
    if let Ok(text) = doc.extract_text(&pages) {
        for line in text.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                ir.paragraphs.push(crate::ir::elements::Paragraph::new(trimmed));
            }
        }
    }

    Ok(ir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_pdfs() {
        let dir = std::env::temp_dir();
        let path1 = dir.join("test_merge_1.pdf");
        let path2 = dir.join("test_merge_2.pdf");
        let path_out = dir.join("test_merge_out.pdf");

        let p1 = path1.to_str().unwrap();
        let p2 = path2.to_str().unwrap();
        let p_out = path_out.to_str().unwrap();

        // Create two single page PDFs
        let _ = create_pdf(p1, "First PDF Page Content", None);
        let _ = create_pdf(p2, "Second PDF Page Content", None);

        // Merge them
        let res = merge_pdfs(&[p1.to_string(), p2.to_string()], p_out);
        assert!(res.contains("\"success\":true"));

        // Load the merged PDF and verify page count is 2
        let doc = lopdf::Document::load(p_out).unwrap();
        assert_eq!(doc.get_pages().len(), 2);

        // Clean up
        let _ = std::fs::remove_file(path1);
        let _ = std::fs::remove_file(path2);
        let _ = std::fs::remove_file(path_out);
    }

    #[test]
    fn test_create_pdf_multi_page() {
        let dir = std::env::temp_dir();
        let path = dir.join("test_multipage.pdf");
        let p = path.to_str().unwrap();

        // Create 50 lines of text to trigger 2 pages
        let mut text = String::new();
        for i in 1..=50 {
            text.push_str(&format!("Line {}\n", i));
        }

        let res = create_pdf(p, &text, None);
        assert!(res.contains("\"success\":true"));
        assert!(res.contains("\"pages\":2"));

        let doc = lopdf::Document::load(p).unwrap();
        assert_eq!(doc.get_pages().len(), 2);

        let _ = std::fs::remove_file(path);
    }
}
