use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn after_instr_name() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (i32.load ))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 20)));
    assert_json_snapshot!(response);
}

#[test]
fn incomplete() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (i32.load off))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 23)));
    assert_json_snapshot!(response);
}

#[test]
fn after_mem_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func (i32.load 0 )))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 20)));
    assert_json_snapshot!(response);
}
