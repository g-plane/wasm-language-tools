use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn global_type_mut_type() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global (mut ))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 17)));
    assert_json_snapshot!(response);
}
