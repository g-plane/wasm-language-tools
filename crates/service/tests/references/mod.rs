use insta::assert_json_snapshot;
use lspt::{Position, ReferenceContext, ReferenceParams, TextDocumentIdentifier};
use wat_service::LanguageService;

mod block;
mod data;
mod elem;
mod field;
mod func;
mod global;
mod local;
mod mem;
mod table;
mod tag;
mod ty;

fn create_params(uri: String, line: u32, character: u32, include_declaration: bool) -> ReferenceParams {
    ReferenceParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line, character },
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
        context: ReferenceContext { include_declaration },
    }
}

#[test]
fn ignored_tokens() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $func (export \"func\")
        (unreachable $func)
        (cal 0) ;; typo
    )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    assert!(
        service
            .find_references(create_params(uri.clone(), 1, 4, true))
            .is_none()
    );
    assert!(
        service
            .find_references(create_params(uri.clone(), 2, 29, true))
            .is_none()
    );
    assert!(
        service
            .find_references(create_params(uri.clone(), 3, 7, true))
            .is_none()
    );
    assert!(
        service
            .find_references(create_params(uri.clone(), 3, 25, true))
            .is_none()
    );
    assert!(
        service
            .find_references(create_params(uri.clone(), 4, 14, true))
            .is_none()
    );
    assert!(
        service
            .find_references(create_params(uri.clone(), 4, 23, true))
            .is_none()
    );
}

#[test]
fn hex_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func)
  (func)
  (func)
  (func)
  (func)
  (func)
  (func)
  (func)
  (func)
  (func)
  (func)
  (func)
  (func
    call 0xa
    call 0xA))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.find_references(create_params(uri, 15, 11, true));
    assert_json_snapshot!(response);
}
