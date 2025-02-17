use super::*;
use insta::assert_json_snapshot;
use lspt::Position;
use wat_service::LanguageService;

#[test]
fn memory_def_int_idx() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
    (memory (data))
    (export "" (memory 0))
    (data (memory 0))
    (func
        (i32.store)
        (memory.size 0)
        (memory.grow 0)
        (memory.fill 0)
        (memory.copy 0 1)
        (memory.init 0 0))
)
(module (memory))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(
        uri.clone(),
        Position {
            line: 2,
            character: 11,
        },
        true,
    ));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(
        uri,
        Position {
            line: 2,
            character: 11,
        },
        false,
    ));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn memory_def_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
    (memory $memory (data))
    (export "" (memory $memory))
    (data (memory $memory))
    (func
        (f32.store $memory)
        (memory.size $memory)
        (memory.grow $memory)
        (memory.fill $memory)
        (memory.copy $memory 1)
        (memory.init $memory 0))
)
(module (memory $memory))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(
        uri.clone(),
        Position {
            line: 2,
            character: 19,
        },
        true,
    ));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(
        uri,
        Position {
            line: 2,
            character: 19,
        },
        false,
    ));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn memory_ref_int_idx() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
    (memory (data))
    (export "" (memory 0))
    (data (memory 0))
    (func
        (i64.store)
        (memory.size 0)
        (memory.grow 0)
        (memory.fill 0)
        (memory.copy 0 1)
        (memory.init 0 0))
)
(module (memory))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(
        uri.clone(),
        Position {
            line: 3,
            character: 24,
        },
        true,
    ));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(
        uri,
        Position {
            line: 3,
            character: 24,
        },
        false,
    ));
    assert_json_snapshot!(exclude_decl);
}

#[test]
fn memory_ref_ident_idx() {
    let uri = "untitled:test".to_string();
    let source = r#"
(module
    (memory $memory (data))
    (export "" (memory $memory))
    (data (memory $memory))
    (func
        (f64.store $memory)
        (memory.size $memory)
        (memory.grow $memory)
        (memory.fill $memory)
        (memory.copy $memory 1)
        (memory.init $memory 0))
)
(module (memory $memory))
"#;
    let mut service = LanguageService::default();
    service.commit(uri.clone(), source.into());
    let include_decl = service.find_references(create_params(
        uri.clone(),
        Position {
            line: 3,
            character: 30,
        },
        true,
    ));
    assert_json_snapshot!(include_decl);
    let exclude_decl = service.find_references(create_params(
        uri,
        Position {
            line: 3,
            character: 30,
        },
        false,
    ));
    assert_json_snapshot!(exclude_decl);
}
