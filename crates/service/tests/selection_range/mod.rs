use insta::assert_json_snapshot;
use lspt::{Position, SelectionRangeParams, TextDocumentIdentifier};
use wat_service::LanguageService;

fn create_params(uri: String, positions: Vec<Position>) -> SelectionRangeParams {
    SelectionRangeParams {
        text_document: TextDocumentIdentifier { uri },
        positions,
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    }
}

#[test]
fn sequence() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $func
    local.get 1
    if
      local.get 2
    else
      local.get 3
    end)))
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.selection_range(create_params(
        uri,
        vec![Position {
            line: 7,
            character: 17,
        }],
    ));
    assert_json_snapshot!(response);
}
