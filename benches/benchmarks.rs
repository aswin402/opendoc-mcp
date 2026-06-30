use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_docx_to_ir(c: &mut Criterion) {
    let dir = std::env::temp_dir();
    let path = dir.join("bench_docx.docx");
    let p = path.to_str().unwrap();

    // Create a test DOCX
    let mut doc = rdocx::Document::new();
    doc.save(p).unwrap();

    c.bench_function("docx_to_ir", |b| {
        b.iter(|| {
            let _ = opendoc_mcp::handlers::docx::to_ir(black_box(p));
        })
    });

    let _ = std::fs::remove_file(path);
}

fn bench_load_to_ir(c: &mut Criterion) {
    let dir = std::env::temp_dir();
    let path = dir.join("bench_txt.txt");
    let p = path.to_str().unwrap();
    std::fs::write(p, "Hello, World!").unwrap();

    c.bench_function("load_txt_to_ir", |b| {
        b.iter(|| {
            let _ = opendoc_mcp::handlers::load_to_ir(black_box(p));
        })
    });

    let _ = std::fs::remove_file(path);
}

fn bench_search(c: &mut Criterion) {
    use opendoc_mcp::engine::search;
    use opendoc_mcp::ir::Document;

    let mut doc = Document::new("txt");
    for i in 0..100 {
        doc.paragraphs
            .push(opendoc_mcp::ir::Paragraph::new(format!("Paragraph {}", i)));
    }

    c.bench_function("search_100_paragraphs", |b| {
        b.iter(|| {
            let _ = search::search_document(black_box(&doc), black_box("Paragraph"), false);
        })
    });
}

criterion_group!(benches, bench_docx_to_ir, bench_load_to_ir, bench_search);
criterion_main!(benches);
