use insta::assert_json_snapshot;
use lsp_types::{
    Position, ReferenceContext, ReferenceParams, TextDocumentIdentifier,
    TextDocumentPositionParams, Uri,
};
use wat_service::LanguageService;

fn create_params(uri: Uri, position: Position, include_declaration: bool) -> ReferenceParams {
    ReferenceParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
        context: ReferenceContext {
            include_declaration,
        },
    }
}

#[test]
fn ignored_tokens() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $func (export \"func\")
        (unreachable $func)
        (cal 0) ;; typo
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    assert!(service
        .find_references(create_params(uri.clone(), Position::new(1, 4), true))
        .is_none());
    assert!(service
        .find_references(create_params(uri.clone(), Position::new(2, 29), true))
        .is_none());
    assert!(service
        .find_references(create_params(uri.clone(), Position::new(3, 7), true))
        .is_none());
    assert!(service
        .find_references(create_params(uri.clone(), Position::new(3, 25), true))
        .is_none());
    assert!(service
        .find_references(create_params(uri.clone(), Position::new(4, 14), true))
        .is_none());
    assert!(service
        .find_references(create_params(uri.clone(), Position::new(4, 23), true))
        .is_none());
}

#[test]
fn func_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func
        (call 0)
    )
)
(module (func))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(2, 9), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(2, 9), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn func_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $func
        (call 0) (call $func)
    )
)
(module (func $func))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(2, 15), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(2, 15), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn param_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param i64)
        (local.get 0)
    )
    (func (param i64))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(2, 20), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(2, 20), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn param_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $param i64)
        (local.get 0) (local.get $param)
    )
    (func (param $param i64))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(2, 23), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(2, 23), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn local_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local i64)
        (local.get 0)
    )
    (func (local i64))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(2, 20), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(2, 20), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn local_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local $local i64)
        (local.get 0) (local.get $local)
    )
    (func (local $local i64))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(2, 23), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(2, 23), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn call_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func
        (call 0)
    )
    (export \"\" (func 0))
)
(module (func))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 15), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 15), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn call_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func $func
        (call 0) (call $func)
    )
    (export \"\" (func $func))
)
(module (func $func))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 28), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 28), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn param_access_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param i64)
        (local.get 0)
    )
    (func (param i64))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 20), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 20), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn param_access_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $param i64) (local $local i64)
        (local.get 0) (local.get $param)
    )
    (func (param $param i64) (local $local i64))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 38), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 38), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn local_access_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local i64)
        (local.get 0)
    )
    (func (local i64))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 20), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 20), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn local_access_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $param i64) (local $local i64)
        (local.get 1) (local.get $local)
    )
    (func (param $param i64) (local $local i64))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 38), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 38), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn local_ref_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param i64) (local i64)
        (local.get 1)
    )
    (func (param i64) (local i64))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 20), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 20), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn local_ref_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $name i32) (local $name i64)
        (local.get $name)
    )
    (func (param $name i32) (local $name i64))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 24), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 24), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn func_type_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type (func))
    (func (type 0))
)
(module (type (func)))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(2, 9), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(2, 9), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn func_type_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type $type (func))
    (func (type 0))
    (func (type $type))
)
(module (type $type))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(2, 15), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(2, 15), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn type_use_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type (func))
    (func (type 0))
)
(module (type (func)))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 17), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 17), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn type_use_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (type $type (func))
    (func (type 0))
    (func (type $type))
)
(module (type $type))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(4, 21), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(4, 21), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn global_def_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global i32)
    (func (global.get 0))
)
(module (global i32))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(2, 11), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(2, 11), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn global_def_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global $global i32)
    (func (global.get 0) (global.get $global))
)
(module (global $global i32))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(2, 19), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(2, 19), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn global_ref_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global i32)
    (func (global.get 0))
)
(module (global i32))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 23), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 23), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn global_ref_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (global $global i32)
    (func (global.get 0) (global.get $global))
)
(module (global $global i32))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 44), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 44), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn memory_def_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (memory (data))
    (export \"\" (memory 0))
)
(module (memory))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(2, 11), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(2, 11), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn memory_def_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (memory $memory (data))
    (export \"\" (memory $memory))
)
(module (memory $memory))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(2, 19), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(2, 19), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn memory_ref_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (memory (data))
    (export \"\" (memory 0))
)
(module (memory))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 24), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 24), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn memory_ref_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (memory $memory (data))
    (export \"\" (memory $memory))
)
(module (memory $memory))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 30), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 30), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn block_def_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block
      (block
        (br_table 1))
      (br_table 0))
    (block
      (br_table 0))))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 10), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 10), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn block_def_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block $block
      (block
        (br_table $block))
      (br_table $block))
    (block $block
      (br_table $block))))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(3, 14), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(3, 14), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn block_ref_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block
      (block
        (br_table 1))
      (br_table 0))
    (block
      (br_table 0))))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(5, 19), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(5, 19), false));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn block_ref_ident_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
  (func
    (block $block
      (block
        (br_table $block))
      (br_table $block))
    (block $block
      (br_table $block))))
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let include_decl =
        service.find_references(create_params(uri.clone(), Position::new(5, 21), true));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(uri, Position::new(5, 21), false));
    assert_json_snapshot!(exclude_decl);
}
