use super::*;
use insta::assert_json_snapshot;
use lsp_types::{Position, Uri};
use wat_service::LanguageService;

#[test]
fn types() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local ))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 17)));
    assert_json_snapshot!(response);
}

#[test]
fn types_following_incomplete_ident() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local $))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 18)));
    assert_json_snapshot!(response);
}

#[test]
fn types_following_ident() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local $p))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 19)));
    assert_json_snapshot!(response);
}

#[test]
fn types_after_ident() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local $p ))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 20)));
    assert_json_snapshot!(response);
}

#[test]
fn types_multiple_types() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local i32 ))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 21)));
    assert_json_snapshot!(response);
}

#[test]
fn types_incomplete_type() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (local i))
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(2, 18)));
    assert_json_snapshot!(response);
}

#[test]
fn locals_and_params() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $p i32) (param f32 f64) (local $l i32) (local f32 f64)
        (local.get )
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(3, 19)));
    assert_json_snapshot!(response);
}

#[test]
fn locals_and_params_following_int_idx() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $p i32) (param f32 f64) (local $l i32) (local f32 f64)
        (local.get 1)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(3, 20)));
    assert_json_snapshot!(response);
}

#[test]
fn locals_and_params_following_dollar() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $p i32) (param f32 f64) (local $l i32) (local f32 f64)
        (local.get $)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(3, 20)));
    assert_json_snapshot!(response);
}

#[test]
fn locals_and_params_following_ident() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $p i32) (param f32 f64) (local $l i32) (local f32 f64)
        (local.get $x)
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(3, 21)));
    assert_json_snapshot!(response);
}

#[test]
fn locals_and_params_in_different_funcs() {
    let uri = "untitled:test".parse::<Uri>().unwrap();
    let source = "
(module
    (func (param $param i32) (local $local i32))
    (func (param $p i32) (local $l i32)
        (local.get )
    )
)
";
    let mut service = LanguageService::default();
    service.commit_file(uri.clone(), source.into());
    let response = service.completion(create_params(uri, Position::new(4, 19)));
    assert_json_snapshot!(response);
}