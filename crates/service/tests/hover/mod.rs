use insta::assert_json_snapshot;
use lspt::{HoverParams, Position, TextDocumentIdentifier};
use wat_service::LanguageService;

mod memo;

fn create_params(uri: String, line: u32, character: u32) -> HoverParams {
    HoverParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line, character },
        work_done_token: Default::default(),
    }
}

#[test]
fn param_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i32)
        (local.get 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 20));
    assert_json_snapshot!(response);
}

#[test]
fn param_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $param i32)
        (local.get $param)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 25));
    assert_json_snapshot!(response);
}

#[test]
fn local_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (local i32)
        (local.get 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 20));
    assert_json_snapshot!(response);
}

#[test]
fn local_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (local $local i32)
        (local.get $local)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 25));
    assert_json_snapshot!(response);
}

#[test]
fn global_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global i32)
    (func (global.get 0))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 23));
    assert_json_snapshot!(response);
}

#[test]
fn global_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global $global (mut i32))
    (func (global.get $global))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 29));
    assert_json_snapshot!(response);
}

#[test]
fn func_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $param i32) (param f32 f64) (result i32 i64)
        (call 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 15));
    assert_json_snapshot!(response);
}

#[test]
fn func_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    ;; This is a comment
    ;;; ## Canendo in adest purpureas
    ;;;
    ;;; Aonius nec adstitit, meo suo *inplevere* ignisque inmeriti **Rex talaria si** tendite?
    ;;; Deum tres essent; dabat, [liquidis per](http://pictis.com/poculamutua) tacta,
    ;;; Samos deum veros aestuat acta necis, sed gestumque.
    ;; This is another comment
    (func $func (param $param i32) (param f32 f64) (result i32 i64)
        (call $func)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 10, 19));
    assert_json_snapshot!(response);
}

#[test]
fn func_type_use_only() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type $type (func (param $param i32) (param f32 f64) (result i32 i64)))
    (func $func (type $type)
        (call $func)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 4, 19));
    assert_json_snapshot!(response);
}

#[test]
fn func_type_use_with_inlined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type $type (func (param $param i32) (param f32 f64) (result i32 i64)))
    (func $func (type $type) (param $p f64) (result i32)
        (call $func)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 4, 19));
    assert_json_snapshot!(response);
}

#[test]
fn func_import_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (import \"\" \"\" (func (param $param i32) (param f32 f64) (result i32 i64)))
    (func (call 0))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 17));
    assert_json_snapshot!(response);
}

#[test]
fn func_import_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (import \"\" \"\" (func $func (param $param i32) (param f32 f64) (result i32 i64)))
    (func (call $func))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 19));
    assert_json_snapshot!(response);
}

#[test]
fn type_use_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type (func (param $param i32) (param f32 f64) (result i32 i64)))
    (func (type 0))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 17));
    assert_json_snapshot!(response);
}

#[test]
fn type_use_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type $type (func (param $param i32) (param f32 f64) (result i32 i64)))
    (func (type $type))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 21));
    assert_json_snapshot!(response);
}

#[test]
fn param_decl() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $param i64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 21));
    assert_json_snapshot!(response);
}

#[test]
fn param_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param $param i64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 15));
    assert_json_snapshot!(response);
}

#[test]
fn local_decl() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (local $local i64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 21));
    assert_json_snapshot!(response);
}

#[test]
fn local_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (local $local i64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 15));
    assert_json_snapshot!(response);
}

#[test]
fn func_decl() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $func (param $param i32) (param f32 f64) (result i32 i64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 14));
    assert_json_snapshot!(response);
}

#[test]
fn func_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $func (param $param i32) (param f32 f64) (result i32 i64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 8));
    assert_json_snapshot!(response);
}

#[test]
fn type_use_only_func() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func $func (type $t))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 14));
    assert_json_snapshot!(response);
}

#[test]
fn type_decl() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type $type (func (param $param i32) (param f32 f64) (result i32 i64)))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 14));
    assert_json_snapshot!(response);
}

#[test]
fn type_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type $type (func (param $param i32) (param f32 f64) (result i32 i64)))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 8));
    assert_json_snapshot!(response);
}

#[test]
fn type_decl_empty() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (type $type (func))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 14));
    assert_json_snapshot!(response);
}

#[test]
fn global_decl() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global $global i64)
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 17));
    assert_json_snapshot!(response);
}

#[test]
fn global_decl_mut() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global $global (mut i64))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 17));
    assert_json_snapshot!(response);
}

#[test]
fn global_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (global $global i64)
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 9));
    assert_json_snapshot!(response);
}

#[test]
fn num_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param i32))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn vec_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param v128))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn abbr_ref_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param funcref))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn instr_name() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (i32.const 0))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn two_slots_instr_op_code() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (v128.store (unreachable)))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn three_slots_instr_op_code() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (f64x2.sqrt (unreachable)))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn select() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (select)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 12));
    assert_json_snapshot!(response);
}

#[test]
fn select_with_result() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (select (result i32))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 12));
    assert_json_snapshot!(response);
}

#[test]
fn ref_test_with_non_null_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    ref.test (ref any)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 8));
    assert_json_snapshot!(response);
}

#[test]
fn ref_test_with_abbr_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    ref.test anyref))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 8));
    assert_json_snapshot!(response);
}

#[test]
fn ref_test_with_null_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    ref.test (ref null any)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 8));
    assert_json_snapshot!(response);
}

#[test]
fn ref_cast_with_non_null_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    ref.cast (ref any)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 8));
    assert_json_snapshot!(response);
}

#[test]
fn ref_cast_with_abbr_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    ref.cast anyref))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 8));
    assert_json_snapshot!(response);
}

#[test]
fn ref_cast_with_null_ref() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    ref.cast (ref null any)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 8));
    assert_json_snapshot!(response);
}

#[test]
fn block_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block (result i32 f32)
      br 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 4, 9));
    assert_json_snapshot!(response);
}

#[test]
fn block_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block $b (param i64 f64) (result i32 f32)
      br $b)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 4, 9));
    assert_json_snapshot!(response);
}

#[test]
fn block_via_type_use() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (func (param i32) (param i32) (result i32)))
  (func
    (block $b (type $t)
      br $b)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 5, 9));
    assert_json_snapshot!(response);
}

#[test]
fn block_decl() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block $b (param i64 f64) (result i32 f32))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 11));
    assert_json_snapshot!(response);
}

#[test]
fn block_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block (result i32 f32))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 6));
    assert_json_snapshot!(response);
}

#[test]
fn loop_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (loop (result i32 f32))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 6));
    assert_json_snapshot!(response);
}

#[test]
fn if_keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (if (result i32 f32))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 6));
    assert_json_snapshot!(response);
}

#[test]
fn ref_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
    (func (param (ref any))
        (local.get 0)
    )
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 20));
    assert_json_snapshot!(response);
}

#[test]
fn struct_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $struct (struct (field) (field i8 (mut anyref)) (field $x (ref $x))))
  (func (param (ref $struct))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 24));
    assert_json_snapshot!(response);
}

#[test]
fn struct_type_empty() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $struct (struct (field)  ))
  (func (param (ref $struct))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 24));
    assert_json_snapshot!(response);
}

#[test]
fn array_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $array (array  (mut nullfuncref) ))
  (func (param (ref $array))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 24));
    assert_json_snapshot!(response);
}

#[test]
fn array_type_empty() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $array (array  ))
  (func (param (ref $array))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 3, 24));
    assert_json_snapshot!(response);
}

#[test]
fn field_decl() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field $x i32))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 24));
    assert_json_snapshot!(response);
}

#[test]
fn field_with_mut() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field (mut i32)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 26));
    assert_json_snapshot!(response);
}

#[test]
fn field_with_ref_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field (ref 0)))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 2, 26));
    assert_json_snapshot!(response);
}

#[test]
fn field_int_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field (mut i32))))
  (func
    struct.get 0 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 4, 17));
    assert_json_snapshot!(response);
}

#[test]
fn field_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field $x (mut i32))))
  (func
    struct.get 0 $x))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.hover(create_params(uri, 4, 18));
    assert_json_snapshot!(response);
}
