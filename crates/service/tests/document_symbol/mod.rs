use insta::assert_json_snapshot;
use lspt::{DocumentSymbolParams, TextDocumentIdentifier};
use wat_service::LanguageService;

#[test]
fn symbols() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $func (param $p i32) (local $l1 i32) (local $l2 i32) (local i32))
    (func (type 0))
    (type (func))
    (type $ty (func))
    (type $struct (struct))
    (type $array (array))
    (global $global i32)
    (global i32)
    (memory $memory 1)
    (memory 1)
    (table $table 1 funcref)
)

(module
    (func)
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_symbol(DocumentSymbolParams {
        text_document: TextDocumentIdentifier { uri },
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    });
    assert_json_snapshot!(response);
}
