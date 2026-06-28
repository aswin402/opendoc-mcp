use pptx::Presentation;

fn pptx_result_to_string<T: serde::Serialize>(result: pptx::PptxResult<T>) -> String {
    match result {
        Ok(val) => serde_json::to_string_pretty(&val).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}")),
        Err(e) => format!("{{\"error\":\"{e}\"}}"),
    }
}

/// Create a simple slide with a title text box
fn create_title_slide_xml(title: &str) -> Vec<u8> {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr/>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="Title"/>
          <p:cNvSpPr txBox="1"/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm>
            <a:off x="914400" y="685800"/>
            <a:ext cx="8229600" cy="1143000"/>
          </a:xfrm>
          <a:prstGeom prst="rect">
            <a:avLst/>
          </a:prstGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p>
            <a:r>
              <a:rPr sz="4400" b="1"/>
              <a:t>{title}</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sld>"#,
        title = escape_xml(title)
    ).into_bytes()
}

/// Create a content slide with title and body text
fn create_content_slide_xml(title: &str, body_items: &[String]) -> Vec<u8> {
    let body_xml: String = body_items.iter().enumerate().map(|(_i, item)| {
        format!(
            r#"<a:p><a:r><a:rPr sz="2800"/><a:t>{}</a:t></a:r></a:p>"#,
            escape_xml(item)
        )
    }).collect();

    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr/>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="Title"/>
          <p:cNvSpPr txBox="1"/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm>
            <a:off x="914400" y="685800"/>
            <a:ext cx="8229600" cy="1143000"/>
          </a:xfrm>
          <a:prstGeom prst="rect">
            <a:avLst/>
          </a:prstGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p>
            <a:r>
              <a:rPr sz="4400" b="1"/>
              <a:t>{title}</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="3" name="Content"/>
          <p:cNvSpPr txBox="1"/>
          <p:nvPr/>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm>
            <a:off x="914400" y="2286000"/>
            <a:ext cx="8229600" cy="4953000"/>
          </a:xfrm>
          <a:prstGeom prst="rect">
            <a:avLst/>
          </a:prstGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          {body_xml}
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sld>"#,
        title = escape_xml(title),
        body_xml = body_xml
    ).into_bytes()
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub fn create_presentation(file_path: &str, _title: Option<&str>) -> String {
    match Presentation::new() {
        Ok(mut prs) => {
            // Add a title slide
            if let Ok(layouts) = prs.slide_layouts() {
                if let Some(layout) = layouts.first() {
                    let _ = prs.add_slide(layout);
                }
            }
            match prs.save(file_path) {
                Ok(_) => serde_json::json!({
                    "success": true,
                    "path": file_path,
                    "format": "pptx"
                }).to_string(),
                Err(e) => format!("{{\"error\":\"{e}\"}}"),
            }
        }
        Err(e) => format!("{{\"error\":\"{e}\"}}"),
    }
}

pub fn open_presentation(file_path: &str) -> String {
    match Presentation::open(file_path) {
        Ok(prs) => {
            let slide_count = prs.slide_count().unwrap_or(0);
            let info = serde_json::json!({
                "path": file_path,
                "slides": slide_count,
                "format": "pptx"
            });
            serde_json::to_string_pretty(&info).unwrap_or_default()
        }
        Err(e) => format!("{{\"error\":\"{e}\"}}"),
    }
}

pub fn add_slide(file_path: &str, title: &str, body: Option<&[String]>) -> String {
    let mut prs = match Presentation::open(file_path) {
        Ok(p) => p,
        Err(e) => return format!("{{\"error\":\"{e}\"}}"),
    };

    let layouts = match prs.slide_layouts() {
        Ok(l) => l,
        Err(e) => return format!("{{\"error\":\"{e}\"}}"),
    };

    // Pick the first layout from available layouts
    let layout = match layouts.first() {
        Some(l) => l.clone(),
        None => return r#"{"error":"no slide layouts available"}"#.to_string(),
    };

    match prs.add_slide(&layout) {
        Ok(slide_ref) => {
            // Get the slide index
            let slide_idx = prs.slide_index(&slide_ref).unwrap_or(0);

            // Determine XML content based on whether we have body
            let xml = if let Some(body_items) = body {
                if body_items.is_empty() {
                    create_title_slide_xml(title)
                } else {
                    create_content_slide_xml(title, body_items)
                }
            } else {
                create_title_slide_xml(title)
            };

            // Set the slide content
            if let Ok(xml_mut) = prs.slide_xml_mut(&slide_ref) {
                *xml_mut = xml;
            }

            match prs.save(file_path) {
                Ok(_) => serde_json::json!({
                    "success": true,
                    "slide_number": slide_idx + 1,
                    "title": title
                }).to_string(),
                Err(e) => format!("{{\"error\":\"{e}\"}}"),
            }
        }
        Err(e) => format!("{{\"error\":\"{e}\"}}"),
    }
}

pub fn add_slide_image(_file_path: &str, slide_number: u32, image_path: &str) -> String {
    // For now, add a slide with image reference noted
    // Full image embedding support requires extending the OPC package
    serde_json::json!({
        "success": false,
        "note": "Image embedding on slides is available in the extended API. Use slide XML manipulation for custom content.",
        "slide": slide_number,
        "image": image_path
    }).to_string()
}

pub fn to_pdf(source: &str, _output: &str) -> String {
    // PPTX to PDF requires the office2pdf crate or external tool
    // For now, return guidance
    serde_json::json!({
        "success": false,
        "note": "PPTX to PDF conversion requires the office2pdf crate. Install with: cargo add office2pdf",
        "source": source,
        "alternative": "Use the document_to_pdf tool after converting PPTX to DOCX, or export slides as images"
    }).to_string()
}

pub fn to_markdown(source: &str) -> String {
    match Presentation::open(source) {
        Ok(prs) => {
            let html = prs.export_html().unwrap_or_default();
            // Simple HTML to Markdown-like conversion
            let md = html
                .replace("<div>", "\n")
                .replace("</div>", "\n")
                .replace("<h1>", "# ")
                .replace("</h1>", "\n")
                .replace("<h2>", "## ")
                .replace("</h2>", "\n")
                .replace("<p>", "")
                .replace("</p>", "\n\n")
                .replace("<br>", "\n")
                .replace("<li>", "- ")
                .replace("</li>", "\n");

            let slide_count = prs.slide_count().unwrap_or(0);
            let result = serde_json::json!({
                "success": true,
                "slides": slide_count,
                "markdown": md
            });
            serde_json::to_string_pretty(&result).unwrap_or_default()
        }
        Err(e) => format!("{{\"error\":\"{e}\"}}"),
    }
}
