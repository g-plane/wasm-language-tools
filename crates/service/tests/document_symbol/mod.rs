use insta::assert_json_snapshot;
use lspt::{DocumentSymbolParams, TextDocumentIdentifier};
use wat_service::LanguageService;

#[test]
fn symbols() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
    (func $func (param $p i32) (local $l1 i32) (local $l2 f32) (local (ref 0)))
    (func (type 0))
    (type (func))
    (type $ty (func))
    (type $struct (struct))
    (type $array (array))
    (global $global i32)
    (global (mut i32))
    (memory $memory 1)
    (memory 1)
    (table $table 1 funcref)
    (type (struct (field i32 (ref 0)) (field $field (mut i32))))
    (tag $tag)
)

(module
    (@deprecated)
    (func)
    (@deprecated)
    (type (func))
    (@deprecated "Use another global")
    (global i32)
    (@deprecated)
    (memory 1)
    (@deprecated "This is deprecated")
    (table 1 funcref)
    (@deprecated)
    (tag)
)
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_symbol(DocumentSymbolParams {
        text_document: TextDocumentIdentifier { uri },
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    });
    assert_json_snapshot!(response);
}
