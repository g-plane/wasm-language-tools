use super::*;
use insta::assert_json_snapshot;
use lsp_types::Uri;
use wat_service::LanguageService;

#[test]
fn blocks() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block $b
      (block $a
        (block $b
          (block $c
            (block $b)))))
    (block
      (block $b))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    allow_unused(&mut service, uri.clone());
    let response = service.pull_diagnostics(create_params(uri));
    assert_json_snapshot!(response);
}
