use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn export_desc_memory() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (export \"\" (memory ))
    (memory $memory (data))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 23)));
    assert_json_snapshot!(response);
}

#[test]
fn export_desc_memory_following_dollar() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (export \"\" (memory $))
    (memory $memory (data))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 24)));
    assert_json_snapshot!(response);
}

#[test]
fn export_desc_memory_incomplete() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (export \"\" (memory $m))
    (memory $memory (data))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 25)));
    assert_json_snapshot!(response);
}
