use insta::assert_json_snapshot;
use lspt::{FoldingRangeParams, TextDocumentIdentifier};
use wat_service::LanguageService;

fn create_params(uri: String) -> FoldingRangeParams {
    FoldingRangeParams {
        text_document: TextDocumentIdentifier { uri },
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    }
}

#[test]
fn with_sequence() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    local.get 0
    if (result f64)
      f64.const 1
    else
      local.get 0
    end))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.folding_range(create_params(uri));
    assert_json_snapshot!(response);
}
