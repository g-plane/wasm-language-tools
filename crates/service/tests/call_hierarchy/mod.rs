use insta::assert_json_snapshot;
use lspt::{
    CallHierarchyIncomingCallsParams, CallHierarchyOutgoingCallsParams, CallHierarchyPrepareParams, Position,
    TextDocumentIdentifier,
};
use wat_service::LanguageService;

const SOURCE: &str = "
(module
    (func $f1)
    (func $f2 (call $f1) (call $f2))
    (func $f3 (call $f1) (call $f2) (call $f3))
)
";

#[test]
fn f1_incoming() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(&uri, SOURCE.into());
    let prepare = service.prepare_call_hierarchy(CallHierarchyPrepareParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line: 2, character: 12 },
        work_done_token: Default::default(),
    });
    assert_json_snapshot!(prepare);
    let incoming_calls = service.call_hierarchy_incoming_calls(CallHierarchyIncomingCallsParams {
        item: prepare.unwrap().first().unwrap().clone(),
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    });
    assert_json_snapshot!(incoming_calls);
}

#[test]
fn f2_incoming() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(&uri, SOURCE.into());
    let prepare = service.prepare_call_hierarchy(CallHierarchyPrepareParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line: 3, character: 33 },
        work_done_token: Default::default(),
    });
    assert_json_snapshot!(prepare);
    let incoming_calls = service.call_hierarchy_incoming_calls(CallHierarchyIncomingCallsParams {
        item: prepare.unwrap().first().unwrap().clone(),
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    });
    assert_json_snapshot!(incoming_calls);
}

#[test]
fn f3_incoming() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(&uri, SOURCE.into());
    let prepare = service.prepare_call_hierarchy(CallHierarchyPrepareParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line: 4, character: 12 },
        work_done_token: Default::default(),
    });
    assert_json_snapshot!(prepare);
    let incoming_calls = service.call_hierarchy_incoming_calls(CallHierarchyIncomingCallsParams {
        item: prepare.unwrap().first().unwrap().clone(),
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    });
    assert_json_snapshot!(incoming_calls);
}

#[test]
fn f1_outgoing() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(&uri, SOURCE.into());
    let prepare = service.prepare_call_hierarchy(CallHierarchyPrepareParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line: 2, character: 12 },
        work_done_token: Default::default(),
    });
    assert_json_snapshot!(prepare);
    let outgoing_calls = service.call_hierarchy_outgoing_calls(CallHierarchyOutgoingCallsParams {
        item: prepare.unwrap().first().unwrap().clone(),
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    });
    assert_json_snapshot!(outgoing_calls);
}

#[test]
fn f2_outgoing() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(&uri, SOURCE.into());
    let prepare = service.prepare_call_hierarchy(CallHierarchyPrepareParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line: 3, character: 33 },
        work_done_token: Default::default(),
    });
    assert_json_snapshot!(prepare);
    let outgoing_calls = service.call_hierarchy_outgoing_calls(CallHierarchyOutgoingCallsParams {
        item: prepare.unwrap().first().unwrap().clone(),
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    });
    assert_json_snapshot!(outgoing_calls);
}

#[test]
fn f3_outgoing() {
    let uri = "untitled:test".to_string();
    let mut service = LanguageService::default();
    service.commit(&uri, SOURCE.into());
    let prepare = service.prepare_call_hierarchy(CallHierarchyPrepareParams {
        text_document: TextDocumentIdentifier { uri },
        position: Position { line: 4, character: 12 },
        work_done_token: Default::default(),
    });
    assert_json_snapshot!(prepare);
    let outgoing_calls = service.call_hierarchy_outgoing_calls(CallHierarchyOutgoingCallsParams {
        item: prepare.unwrap().first().unwrap().clone(),
        work_done_token: Default::default(),
        partial_result_token: Default::default(),
    });
    assert_json_snapshot!(outgoing_calls);
}
