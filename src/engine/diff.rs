use crate::ir::Document;

/// Compare two documents and return a structured diff (simple line-based)
pub fn diff_documents(a: &Document, b: &Document) -> DiffResult {
    let text_a: Vec<&str> = a.paragraphs.iter().map(|p| p.text.as_str()).collect();
    let text_b: Vec<&str> = b.paragraphs.iter().map(|p| p.text.as_str()).collect();

    let n = text_a.len();
    let m = text_b.len();

    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            if text_a[i - 1] == text_b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    let mut changes = Vec::new();
    let mut i = n;
    let mut j = m;

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && text_a[i - 1] == text_b[j - 1] {
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || dp[i][j - 1] >= dp[i - 1][j]) {
            changes.push(DiffChange {
                tag: "added".to_string(),
                line: j - 1,
                old_value: String::new(),
                new_value: text_b[j - 1].to_string(),
            });
            j -= 1;
        } else if i > 0 && (j == 0 || dp[i - 1][j] >= dp[i][j - 1]) {
            changes.push(DiffChange {
                tag: "removed".to_string(),
                line: i - 1,
                old_value: text_a[i - 1].to_string(),
                new_value: String::new(),
            });
            i -= 1;
        }
    }

    changes.reverse();
    changes.sort_by_key(|c| (c.line, c.tag == "added"));

    // Post-process to merge adjacent removal and addition at the same index into a modified change
    let mut merged_changes = Vec::new();
    let mut iter = changes.into_iter().peekable();
    while let Some(change) = iter.next() {
        if change.tag == "removed" {
            if let Some(next_change) = iter.peek() {
                if next_change.tag == "added" && next_change.line == change.line {
                    let next_change = iter.next().unwrap();
                    merged_changes.push(DiffChange {
                        tag: "modified".to_string(),
                        line: change.line,
                        old_value: change.old_value,
                        new_value: next_change.new_value,
                    });
                    continue;
                }
            }
        }
        merged_changes.push(change);
    }

    DiffResult {
        paragraphs_a: a.paragraphs.len(),
        paragraphs_b: b.paragraphs.len(),
        changes: merged_changes,
    }
}

/// Result of a diff comparison between two documents.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DiffResult {
    pub paragraphs_a: usize,
    pub paragraphs_b: usize,
    pub changes: Vec<DiffChange>,
}

/// A single change entry in a diff (added, removed, or modified).
#[derive(Debug, Clone, serde::Serialize)]
pub struct DiffChange {
    pub tag: String,
    pub line: usize,
    pub old_value: String,
    pub new_value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::Paragraph;

    fn make_doc(texts: &[&str]) -> Document {
        let mut doc = Document::new("txt");
        for t in texts {
            doc.paragraphs.push(Paragraph::new(*t));
        }
        doc
    }

    #[test]
    fn test_diff_identical() {
        let a = make_doc(&["Hello", "World"]);
        let b = make_doc(&["Hello", "World"]);
        let result = diff_documents(&a, &b);
        assert_eq!(result.paragraphs_a, 2);
        assert_eq!(result.paragraphs_b, 2);
        assert!(result.changes.is_empty());
    }

    #[test]
    fn test_diff_modified() {
        let a = make_doc(&["Hello", "World"]);
        let b = make_doc(&["Hello", "Rust"]);
        let result = diff_documents(&a, &b);
        assert_eq!(result.changes.len(), 1);
        assert_eq!(result.changes[0].tag, "modified");
        assert_eq!(result.changes[0].line, 1);
        assert_eq!(result.changes[0].old_value, "World");
        assert_eq!(result.changes[0].new_value, "Rust");
    }

    #[test]
    fn test_diff_added() {
        let a = make_doc(&["Hello"]);
        let b = make_doc(&["Hello", "World"]);
        let result = diff_documents(&a, &b);
        assert_eq!(result.changes.len(), 1);
        assert_eq!(result.changes[0].tag, "added");
        assert_eq!(result.changes[0].line, 1);
        assert_eq!(result.changes[0].new_value, "World");
    }

    #[test]
    fn test_diff_removed() {
        let a = make_doc(&["Hello", "World"]);
        let b = make_doc(&["Hello"]);
        let result = diff_documents(&a, &b);
        assert_eq!(result.changes.len(), 1);
        assert_eq!(result.changes[0].tag, "removed");
        assert_eq!(result.changes[0].line, 1);
        assert_eq!(result.changes[0].old_value, "World");
    }

    #[test]
    fn test_diff_empty_docs() {
        let a = make_doc(&[]);
        let b = make_doc(&[]);
        let result = diff_documents(&a, &b);
        assert!(result.changes.is_empty());
    }

    #[test]
    fn test_diff_multiple_changes() {
        let a = make_doc(&["A", "B", "C"]);
        let b = make_doc(&["A", "X", "Y"]);
        let result = diff_documents(&a, &b);
        assert_eq!(result.changes.len(), 2);
        assert_eq!(result.changes[0].old_value, "B");
        assert_eq!(result.changes[0].new_value, "X");
        assert_eq!(result.changes[1].old_value, "C");
        assert_eq!(result.changes[1].new_value, "Y");
    }
}
