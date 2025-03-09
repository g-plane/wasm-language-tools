use insta::assert_json_snapshot;
use lspt::{DocumentHighlightParams, Position, TextDocumentIdentifier};
use wat_service::LanguageService;

fn create_params(uri: String, line: u32, character: u32) -> DocumentHighlightParams {
    DocumentHighlightParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line, character },
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    }
}

#[test]
fn keyword() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func)
  (func $func (export \"func\")
    (call $func)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 3, 6));
    assert_json_snapshot!(response);
}

#[test]
fn instr_name() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (local.get) (local.get)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 13));
    assert_json_snapshot!(response);
}

#[test]
fn num_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param $i32 i32) (i32))
  (func (param $i32 i32) (i32))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 3, 21));
    assert_json_snapshot!(response);
}

#[test]
fn vec_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param $v128 v128) (v128))
  (func (param $v128 v128) (v128))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 3, 24));
    assert_json_snapshot!(response);
}

#[test]
fn ref_type() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param $funcref funcref) (funcref))
  (func (param $funcref funcref) (funcref))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 3, 26));
    assert_json_snapshot!(response);
}

#[test]
fn non_idx_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (param i32)
    (local.get 0)
    (f32.const 0)
    (f64.const 0)))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 5, 16));
    assert_json_snapshot!(response);
}

#[test]
fn float() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $1.0 (f32.const 1.0) (f64.const 1.0))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 41));
    assert_json_snapshot!(response);
}

#[test]
fn func() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f (param $f i32) (call 0) (call $f))
)
(module (func $f))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 9));
    assert_json_snapshot!(response);
}

#[test]
fn call_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f (call 0) (call $f))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 18));
    assert_json_snapshot!(response);
}

#[test]
fn call_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f (call 0) (call $f))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 17));
    assert_json_snapshot!(response);
}

#[test]
fn call_undefined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (call $f))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 15));
    assert!(response.is_none());
}

#[test]
fn param() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f (param $f i32) (local.get 0) (local.get $f))
  (func (param $f i32) (local.get 0) (local.get $f))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn local() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f (local $f i32) (local.get 0) (local.get $f) (local.set 0) (local.set $f))
  (func (local $f i32) (local.get 0) (local.get $f))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 19));
    assert_json_snapshot!(response);
}

#[test]
fn local_ref_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f (param $f i32) (local.get 0) (local.get $f) (local.set 0) (local.set $f))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 38));
    assert_json_snapshot!(response);
}

#[test]
fn local_ref_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f (param $f i32) (local.get 0) (local.get $f) (local.set 0) (local.set $f))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 52));
    assert_json_snapshot!(response);
}

#[test]
fn local_ref_undefined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func $f (local.get 0) (local.get $f))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 37));
    assert!(response.is_none());
}

#[test]
fn type_def() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (func))
  (func (type 0))
  (func (type $t))
)
(module (func $t))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 9));
    assert_json_snapshot!(response);
}

#[test]
fn type_use_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (func))
  (func (type 0))
  (func (type $t))
)
(module (func $t))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 3, 15));
    assert_json_snapshot!(response);
}

#[test]
fn type_use_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type $t (func))
  (func (type 0))
  (func (type $t))
)
(module (func $t))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 4, 15));
    assert_json_snapshot!(response);
}

#[test]
fn type_use_undefined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (type $t))
)
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 15));
    assert!(response.is_none());
}

#[test]
fn global() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global $g)
  (func (global.get 0) (global.get $g) (global.set 0) (global.set $g))
)
(module (global $g))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 11));
    assert_json_snapshot!(response);
}

#[test]
fn global_ref_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global $g)
  (func (global.get 0) (global.get $g) (global.set 0) (global.set $g))
)
(module (global $g))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 3, 52));
    assert_json_snapshot!(response);
}

#[test]
fn global_ref_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (global $g)
  (func (global.get 0) (global.get $g) (global.set 0) (global.set $g))
)
(module (global $g))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 3, 68));
    assert_json_snapshot!(response);
}

#[test]
fn global_ref_undefined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (global.get 0) (global.get $g) (global.set 0) (global.set $g))
)
(module (global $g))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 36));
    assert!(response.is_none());
}

#[test]
fn memory() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m)
  (data (memory 0))
  (data (memory $m))
)
(module (memory $m))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 11));
    assert_json_snapshot!(response);
}

#[test]
fn memory_ref_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m)
  (data (memory 0))
  (data (memory $m))
)
(module (memory $m))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 3, 17));
    assert_json_snapshot!(response);
}

#[test]
fn memory_ref_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (memory $m)
  (data (memory 0))
  (data (memory $m))
)
(module (memory $m))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 4, 17));
    assert_json_snapshot!(response);
}

#[test]
fn memory_ref_undefined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (data (memory $m))
)
(module (memory $m))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 17));
    assert!(response.is_none());
}

#[test]
fn table() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $t)
  (func
    (table.get 0) (table.get $t)
    (table.set 0) (table.set $t)
    (table.size 0) (table.size $t)
    (table.init 0) (table.init $t)
    (table.grow 0) (table.grow $t)
    (table.fill 0) (table.fill $t)
    (table.copy 0 $t) (table.copy $t 0))
)
(module (table $t))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 10));
    assert_json_snapshot!(response);
}

#[test]
fn table_ref_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $t)
  (func
    (table.get 0) (table.get $t)
    (table.set 0) (table.set $t)
    (table.size 0) (table.size $t)
    (table.init 0) (table.init $t)
    (table.grow 0) (table.grow $t)
    (table.fill 0) (table.fill $t)
    (table.copy 0 $t) (table.copy $t 0))
)
(module (table $t))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 7, 17));
    assert_json_snapshot!(response);
}

#[test]
fn table_ref_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (table $t)
  (func
    (table.get 0) (table.get $t)
    (table.set 0) (table.set $t)
    (table.size 0) (table.size $t)
    (table.init 0) (table.init $t)
    (table.grow 0) (table.grow $t)
    (table.fill 0) (table.fill $t)
    (table.copy 0 $t) (table.copy $t 0))
)
(module (table $t))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 4, 30));
    assert_json_snapshot!(response);
}

#[test]
fn table_ref_undefined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func (table.get 0) (table.size $t))
)
(module (table $g))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 20));
    assert!(response.is_none());
}

#[test]
fn block() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block $a
      br_table 0
      (block $b
        br_table 0 1 $a $b))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 3, 12));
    assert_json_snapshot!(response);
}

#[test]
fn block_ref_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block $b)
    (block $a
      br_table 0
      (block $b
        br_table 0 1 $a $b))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 7, 20));
    assert_json_snapshot!(response);
}

#[test]
fn block_ref_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block $b)
    (block $a
      br_table 0
      (block $b
        br_table 0 1 $a $b))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 7, 25));
    assert_json_snapshot!(response);
}

#[test]
fn block_ref_undefined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (func
    (block $b)
    (block $a
      br_table 0
      (block $b
        br_table 2))))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 7, 18));
    assert!(response.is_none());
}

#[test]
fn field() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field $x i32)))
  (type (struct (field $x i32)))
  (func
    struct.get 0 $x
    struct.set 0 $x))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 2, 24));
    assert_json_snapshot!(response);
}

#[test]
fn field_ref_int() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field i32)))
  (type (struct (field i32)))
  (func
    struct.get 0 0
    struct.set 0 0))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 6, 17));
    assert_json_snapshot!(response);
}

#[test]
fn field_ref_ident() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field $x i32)))
  (type (struct (field $x i32)))
  (func
    struct.get 0 $x
    struct.set 0 $x))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 5, 18));
    assert_json_snapshot!(response);
}

#[test]
fn field_ref_undefined() {
    let uri = "untitled:test".to_string();
    let source = "
(module
  (type (struct (field i32)))
  (func
    struct.get 0 $x))
";
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let response = service.document_highlight(create_params(uri, 4, 18));
    assert!(response.is_none());
}
