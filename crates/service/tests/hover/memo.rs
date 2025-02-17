use super::create_params;
use lspt::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn param_type_changing() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(
        uri.clone(),
        "
(module
    (func (param $param i32)
        (local.get $param)
    )
)
"
        .into(),
    );
    let response1 = service.hover(create_params(uri.clone(), Position { line: 2, character: 20 }));

    service.commit(
        uri.clone(),
        "
(module
    (func (param $param f32)
        (local.get $param)
    )
)
"
        .into(),
    );
    let response2 = service.hover(create_params(uri, Position { line: 2, character: 20 }));

    assert_ne!(response1, response2);
}

#[test]
fn param_name_changing() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(
        uri.clone(),
        "
(module
    (func (param $p1 i32)
        (local.get $param)
    )
)
"
        .into(),
    );
    let response1 = service.hover(create_params(uri.clone(), Position { line: 2, character: 20 }));

    service.commit(
        uri.clone(),
        "
(module
    (func (param $p2 f32)
        (local.get $param)
    )
)
"
        .into(),
    );
    let response2 = service.hover(create_params(uri, Position { line: 2, character: 20 }));

    assert_ne!(response1, response2);
}

#[test]
fn func_name_changing() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(
        uri.clone(),
        "
(module
    (func $f1 (param $param i32))
)
"
        .into(),
    );
    let response1 = service.hover(create_params(uri.clone(), Position { line: 2, character: 12 }));

    service.commit(
        uri.clone(),
        "
(module
    (func $f2 (param $param i32))
)
"
        .into(),
    );
    let response2 = service.hover(create_params(uri, Position { line: 2, character: 12 }));

    assert_ne!(response1, response2);
}

#[test]
fn func_name_and_param_changing() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(
        uri.clone(),
        "
(module
    (func $f1 (param $p1 i32))
)
"
        .into(),
    );
    let response1 = service.hover(create_params(uri.clone(), Position { line: 2, character: 12 }));

    service.commit(
        uri.clone(),
        "
(module
    (func $f2 (param $p2 i32))
)
"
        .into(),
    );
    let response2 = service.hover(create_params(uri, Position { line: 2, character: 12 }));

    assert_ne!(response1, response2);
}
