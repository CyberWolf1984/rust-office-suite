use writer::doc_model::WriterDoc;
use libsfx2_rs::document::{Document, DocumentState};

#[test]
fn test_blank_document_initialization() {
    let doc = WriterDoc::new_blank();
    assert_eq!(doc.state(), DocumentState::New);
    assert_eq!(doc.paragraphs.len(), 1);
    assert_eq!(doc.paragraphs[0].runs[0].text, "");
}
