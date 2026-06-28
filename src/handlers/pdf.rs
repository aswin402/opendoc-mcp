use lopdf::{Document, Object, ObjectId, Stream, Dictionary};

fn pdf_result_to_string<T: serde::Serialize>(result: std::result::Result<T, lopdf::Error>) -> String {
    match result {
        Ok(val) => serde_json::to_string_pretty(&val).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}")),
        Err(e) => format!("{{\"error\":\"{e}\"}}"),
    }
}

pub fn create_pdf(file_path: &str, text: &str, _author: Option<&str>) -> String {
    let mut doc = Document::new();

    // Create font object
    let font_id = doc.add_object(Object::Dictionary(Dictionary::from_iter([
        (b"Type".to_vec(), Object::Name(b"Font".to_vec())),
        (b"Subtype".to_vec(), Object::Name(b"Type1".to_vec())),
        (b"BaseFont".to_vec(), Object::Name(b"Helvetica".to_vec())),
    ])));

    // Escape special chars in PDF text
    let escaped_text = text
        .replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)");

    let content = format!(
        "BT /F1 12 Tf 100 700 Td ({}) Tj ET",
        escaped_text
    );

    let content_id = doc.add_object(Object::Stream(Stream {
        dict: Dictionary::new(),
        content: content.into_bytes(),
        allows_compression: true,
        start_position: None,
    }));

    let page_id = doc.new_object_id();
    let pages_id = doc.new_object_id();

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

    let pages = Object::Dictionary(Dictionary::from_iter([
        (b"Type".to_vec(), Object::Name(b"Pages".to_vec())),
        (
            b"Kids".to_vec(),
            Object::Array(vec![Object::Reference(page_id)]),
        ),
        (b"Count".to_vec(), Object::Integer(1)),
    ]));

    doc.objects.insert(pages_id, pages);

    // Set the catalog
    if let Ok(catalog) = doc.catalog_mut() {
        catalog.set("Pages", Object::Reference(pages_id));
    }

    match doc.save(file_path) {
        Ok(_) => serde_json::json!({
            "success": true,
            "path": file_path,
            "format": "pdf",
            "pages": 1
        }).to_string(),
        Err(e) => format!("{{\"error\":\"io: {e}\"}}"),
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
        Err(e) => format!("{{\"error\":\"{e}\"}}"),
    }
}

pub fn merge_pdfs(sources: &[String], output: &str) -> String {
    if sources.is_empty() {
        return r#"{"error":"no source files provided"}"#.to_string();
    }

    let mut result_doc = match Document::load(&sources[0]) {
        Ok(d) => d,
        Err(e) => return format!("{{\"error\":\"{e}\"}}"),
    };

    let mut max_id = result_doc.max_id + 1;

    for source in sources.iter().skip(1) {
        let doc = match Document::load(source) {
            Ok(d) => d,
            Err(e) => return format!("{{\"error\":\"{e}\"}}"),
        };

        let mut renumbered = doc;
        renumbered.renumber_objects_with(max_id);
        let source_max_id = renumbered.max_id;
        max_id = source_max_id + 1;

        // Get pages before merging
        let _page_ids: Vec<ObjectId> = renumbered.get_pages().values().copied().collect();

        // Add all objects
        for (obj_id, obj) in std::mem::take(&mut renumbered.objects) {
            result_doc.objects.insert(obj_id, obj);
        }

        // Find and update the Pages tree in result_doc
        // The pages object should have been inserted with its new IDs
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

    match result_doc.save(output) {
        Ok(_) => serde_json::json!({
            "success": true,
            "sources": sources,
            "output": output
        }).to_string(),
        Err(e) => format!("{{\"error\":\"io: {e}\"}}"),
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
                _ => return r#"{"error":"page number out of range"}"#.to_string(),
            };

            match doc.extract_text(&page_numbers) {
                Ok(text) => serde_json::json!({"success": true, "text": text, "pages": page_numbers.len()}).to_string(),
                Err(e) => format!("{{\"error\":\"{e}\"}}"),
            }
        }
        Err(e) => format!("{{\"error\":\"{e}\"}}"),
    }
}

pub fn replace_text(file_path: &str, find: &str, replace: &str) -> String {
    let mut doc = match Document::load(file_path) {
        Ok(d) => d,
        Err(e) => return format!("{{\"error\":\"{e}\"}}"),
    };

    let pages = doc.get_pages();
    let mut total_replacements = 0u32;

    for (page_num, _) in &pages {
        match doc.replace_text(*page_num, find, replace) {
            Ok(_) => total_replacements += 1,
            Err(_) => {}
        }
    }

    match doc.save(file_path) {
        Ok(_) => serde_json::json!({
            "success": true,
            "pages_modified": total_replacements,
            "find": find,
            "replace": replace
        }).to_string(),
        Err(e) => format!("{{\"error\":\"io: {e}\"}}"),
    }
}
