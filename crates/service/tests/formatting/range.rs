use insta::assert_json_snapshot;
use lspt::{
    DocumentRangeFormattingParams, FormattingOptions, Position, Range, TextDocumentIdentifier,
};
use wat_service::{LanguageService, ServiceConfig};

fn create_params(
    uri: String,
    start_line: u32,
    start_character: u32,
    end_line: u32,
    end_character: u32,
) -> DocumentRangeFormattingParams {
    DocumentRangeFormattingParams {
        text_document: TextDocumentIdentifier { uri },
        range: Range {
            start: Position {
                line: start_line,
                character: start_character,
            },
            end: Position {
                line: end_line,
                character: end_character,
            },
        },
        options: FormattingOptions {
            tab_size: 2,
            insert_spaces: true,
            ..Default::default()
        },
        work_done_token: Default::default(),
    }
}

#[test]
fn fully_covered_node() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32)
    ( local.get    0 )
  )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.range_formatting(create_params(uri, 3, 4, 3, 22));
    assert_json_snapshot!(response);
}

#[test]
fn overlap() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
(block $b
    (block     $a
            (block   $b
          (block $c
            (block $b   )

          )

        )
      )
)
    ))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.range_formatting(create_params(uri, 4, 8, 5, 23));
    assert_json_snapshot!(response);
}

#[test]
fn format_comments() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    ;;comment
  )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    service.set_config(
        uri.clone(),
        ServiceConfig {
            format: wat_formatter::config::LanguageOptions {
                format_comments: true,
                ..Default::default()
            },
            ..Default::default()
        },
    );
    let response = service.range_formatting(create_params(uri, 3, 4, 3, 13));
    assert_json_snapshot!(response);
}
