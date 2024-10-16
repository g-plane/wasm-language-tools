use insta::assert_json_snapshot;
use lsp_types::{Position, TextDocumentIdentifier, TextDocumentPositionParams, Uri};
use wat_service::LanguageService;

fn create_params(uri: Uri, position: Position) -> TextDocumentPositionParams {
    TextDocumentPositionParams {
        text_document: TextDocumentIdentifier { uri },
        position,
    }
}

#[test]
fn ignored_tokens() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $func (export \"func\")
        (unreachable $func)
        (call 0) ;; comment
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    assert!(service
        .prepare_rename(create_params(uri.clone(), Position::new(1, 4)))
        .is_none());
    assert!(service
        .prepare_rename(create_params(uri.clone(), Position::new(2, 29)))
        .is_none());
    assert!(service
        .prepare_rename(create_params(uri.clone(), Position::new(3, 7)))
        .is_none());
    assert!(service
        .prepare_rename(create_params(uri.clone(), Position::new(3, 18)))
        .is_none());
    assert!(service
        .prepare_rename(create_params(uri.clone(), Position::new(4, 15)))
        .is_none());
    assert!(service
        .prepare_rename(create_params(uri.clone(), Position::new(4, 23)))
        .is_none());
}

#[test]
fn ident() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $func)
    (start $func)
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.prepare_rename(create_params(uri, Position::new(2, 14)));
    assert_json_snapshot!(response);
}
