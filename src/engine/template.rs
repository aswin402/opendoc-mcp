use crate::ir::Document;

/// Simple template engine: replace `{{key}}` placeholders with values
pub fn fill_template(doc: &mut Document, vars: &[(String, String)]) -> usize {
    let mut count = 0;

    for (key, value) in vars {
        let placeholder = format!("{{{{{}}}}}", key);
        let pattern = regex::escape(&placeholder);
        let re = match regex::RegexBuilder::new(&pattern).size_limit(1_000_000).build() {
            Ok(r) => r,
            Err(_) => continue,
        };

        for p in &mut doc.paragraphs {
            let new = re.replace_all(&p.text, value.as_str()).to_string();
            if new != p.text {
                count += 1;
                p.text = new;
            }
        }

        for section in &mut doc.sections {
            let new = re.replace_all(&section.title, value.as_str()).to_string();
            if new != section.title {
                count += 1;
                section.title = new;
            }
        }

        for table in &mut doc.tables {
            if let Some(ref mut cap) = table.caption {
                let new = re.replace_all(cap, value.as_str()).to_string();
                if new != *cap {
                    count += 1;
                    *cap = new;
                }
            }
            for header in &mut table.headers {
                let new = re.replace_all(header, value.as_str()).to_string();
                if new != *header {
                    count += 1;
                    *header = new;
                }
            }
            for row in &mut table.rows {
                for cell in row {
                    let new = re.replace_all(cell, value.as_str()).to_string();
                    if new != *cell {
                        count += 1;
                        *cell = new;
                    }
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Paragraph, Section, Table};

    #[test]
    fn test_fill_single_key() {
        let mut doc = Document::new("txt");
        doc.paragraphs.push(Paragraph::new("Hello {{name}}!"));
        let vars = vec![("name".to_string(), "World".to_string())];
        let count = fill_template(&mut doc, &vars);
        assert_eq!(count, 1);
        assert_eq!(doc.paragraphs[0].text, "Hello World!");
    }

    #[test]
    fn test_fill_multiple_vars() {
        let mut doc = Document::new("txt");
        doc.paragraphs.push(Paragraph::new("{{greeting}} {{name}}!"));
        let vars = vec![
            ("greeting".to_string(), "Hi".to_string()),
            ("name".to_string(), "Alice".to_string()),
        ];
        let count = fill_template(&mut doc, &vars);
        assert_eq!(count, 2);
        assert_eq!(doc.paragraphs[0].text, "Hi Alice!");
    }

    #[test]
    fn test_fill_no_match() {
        let mut doc = Document::new("txt");
        doc.paragraphs.push(Paragraph::new("Hello World"));
        let vars = vec![("foo".to_string(), "bar".to_string())];
        let count = fill_template(&mut doc, &vars);
        assert_eq!(count, 0);
        assert_eq!(doc.paragraphs[0].text, "Hello World");
    }

    #[test]
    fn test_fill_in_section_title() {
        let mut doc = Document::new("txt");
        doc.sections.push(Section {
            title: "Chapter {{num}}".to_string(),
            level: 1,
            index: 0,
            content: vec![],
        });
        let vars = vec![("num".to_string(), "1".to_string())];
        let count = fill_template(&mut doc, &vars);
        assert_eq!(count, 1);
        assert_eq!(doc.sections[0].title, "Chapter 1");
    }

    #[test]
    fn test_fill_in_table_cell() {
        let mut doc = Document::new("csv");
        doc.tables.push(Table {
            headers: vec!["Key".to_string()],
            rows: vec![vec!["{{value}}".to_string()]],
            caption: None,
        });
        let vars = vec![("value".to_string(), "42".to_string())];
        let count = fill_template(&mut doc, &vars);
        assert_eq!(count, 1);
        assert_eq!(doc.tables[0].rows[0][0], "42");
    }

    #[test]
    fn test_fill_repeated_key() {
        let mut doc = Document::new("txt");
        doc.paragraphs.push(Paragraph::new("{{x}} + {{x}} = {{y}}"));
        let vars = vec![
            ("x".to_string(), "1".to_string()),
            ("y".to_string(), "2".to_string()),
        ];
        let count = fill_template(&mut doc, &vars);
        assert_eq!(count, 2);
        assert_eq!(doc.paragraphs[0].text, "1 + 1 = 2");
    }

    #[test]
    fn test_fill_table_headers_and_caption() {
        let mut doc = Document::new("docx");
        let table = Table {
            headers: vec!["Header {{h}}".to_string()],
            rows: vec![vec!["Value".to_string()]],
            caption: Some("Table Caption {{c}}".to_string()),
        };
        doc.tables.push(table);
        let vars = vec![
            ("h".to_string(), "Alpha".to_string()),
            ("c".to_string(), "Omega".to_string()),
        ];
        let count = fill_template(&mut doc, &vars);
        assert_eq!(count, 2);
        assert_eq!(doc.tables[0].headers[0], "Header Alpha");
        assert_eq!(doc.tables[0].caption.as_ref().unwrap(), "Table Caption Omega");
    }
}
