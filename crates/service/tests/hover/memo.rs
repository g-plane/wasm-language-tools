use super::create_params;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn param_type_changing() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let mut service = LanguageService::default();
    service.commit_file(
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
    let response1 = service.hover(create_params(uri.clone(), Position::new(2, 20)));

    service.commit_file(
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
    let response2 = service.hover(create_params(uri, Position::new(2, 20)));

    assert_ne!(response1, response2);
}

#[test]
fn param_name_changing() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let mut service = LanguageService::default();
    service.commit_file(
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
    let response1 = service.hover(create_params(uri.clone(), Position::new(2, 20)));

    service.commit_file(
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
    let response2 = service.hover(create_params(uri, Position::new(2, 20)));

    assert_ne!(response1, response2);
}

#[test]
fn func_name_changing() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let mut service = LanguageService::default();
    service.commit_file(
        uri.clone(),
        "
(module
    (func $f1 (param $param i32))
)
"
        .into(),
    );
    let response1 = service.hover(create_params(uri.clone(), Position::new(2, 12)));

    service.commit_file(
        uri.clone(),
        "
(module
    (func $f2 (param $param i32))
)
"
        .into(),
    );
    let response2 = service.hover(create_params(uri, Position::new(2, 12)));

    assert_ne!(response1, response2);
}

#[test]
fn func_name_and_param_changing() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let mut service = LanguageService::default();
    service.commit_file(
        uri.clone(),
        "
(module
    (func $f1 (param $p1 i32))
)
"
        .into(),
    );
    let response1 = service.hover(create_params(uri.clone(), Position::new(2, 12)));

    service.commit_file(
        uri.clone(),
        "
(module
    (func $f2 (param $p2 i32))
)
"
        .into(),
    );
    let response2 = service.hover(create_params(uri, Position::new(2, 12)));

    assert_ne!(response1, response2);
}
