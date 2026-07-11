use insta::assert_json_snapshot;
use lspt::{
    DocumentRangeFormattingParams, DocumentRangesFormattingParams, FormattingOptions, Position, Range,
    TextDocumentIdentifier,
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
    service.commit(&uri, source.into());
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
    service.commit(&uri, source.into());
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
    service.commit(&uri, source.into());
    service.set_config(
        &uri,
        Some(ServiceConfig {
            format: wat_formatter::config::LanguageOptions {
                format_comments: true,
                ..Default::default()
            },
            ..Default::default()
        }),
    );
    let response = service.range_formatting(create_params(uri, 3, 4, 3, 13));
    assert_json_snapshot!(response);
}

#[test]
fn space1() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func



    ;;

    block      $b


    end    $b



    nop     nop
  )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.range_formatting(create_params(uri, 11, 6, 15, 5));
    assert_json_snapshot!(response);
}

#[test]
fn space2() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func



    ;;

    block      $b


    end    $b



    nop     nop
  )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.range_formatting(create_params(uri, 11, 6, 15, 7));
    assert_json_snapshot!(response);
}

#[test]
fn space3() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func



    ;;

    block      $b


    end    $b



    nop     nop
  )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.range_formatting(create_params(uri, 11, 6, 15, 10));
    assert_json_snapshot!(response);
}

#[test]
fn space4() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func



    ;;

    block      $b


    end    $b



    nop     nop
  )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.range_formatting(create_params(uri, 11, 6, 15, 12));
    assert_json_snapshot!(response);
}

#[test]
fn space5() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func



    ;;

    block      $b


    end    $b



    nop     nop
  )
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.range_formatting(create_params(uri, 11, 6, 15, 15));
    assert_json_snapshot!(response);
}

#[test]
fn ranges() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func      $f
    block    $b
    end     nop


    nop     nop
  )
  (global  (mut   i32) (i32.const     0))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.ranges_formatting(DocumentRangesFormattingParams {
        text_document: TextDocumentIdentifier { uri },
        ranges: vec![
            Range {
                start: Position { line: 3, character: 6 },
                end: Position { line: 4, character: 12 },
            },
            Range {
                start: Position { line: 9, character: 13 },
                end: Position { line: 9, character: 30 },
            },
            Range {
                start: Position { line: 4, character: 12 },
                end: Position { line: 7, character: 6 },
            },
        ],
        options: FormattingOptions {
            tab_size: 2,
            insert_spaces: true,
            ..Default::default()
        },
        work_done_token: Default::default(),
    });
    assert_json_snapshot!(response);
}

#[test]
fn ranges_with_overlap() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func      $f
    block    $b
    end     nop


    nop     nop
  )
  (global  (mut   i32) (i32.const     0))
)
";
    let mut service = LanguageService::default();
    service.commit(&uri, source.into());
    let response = service.ranges_formatting(DocumentRangesFormattingParams {
        text_document: TextDocumentIdentifier { uri },
        ranges: vec![
            Range {
                start: Position { line: 3, character: 6 },
                end: Position { line: 4, character: 12 },
            },
            Range {
                start: Position { line: 9, character: 13 },
                end: Position { line: 9, character: 30 },
            },
            Range {
                start: Position { line: 4, character: 9 },
                end: Position { line: 7, character: 6 },
            },
        ],
        options: FormattingOptions {
            tab_size: 2,
            insert_spaces: true,
            ..Default::default()
        },
        work_done_token: Default::default(),
    });
    assert_json_snapshot!(response);
}
