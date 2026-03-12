use serde::Serialize;
use serde_wasm_bindgen::{Error, Serializer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
import type * as lsp from 'vscode-languageserver-protocol'

export type LintLevel = 'deny' | 'warn' | 'hint' | 'allow'
export interface ServiceConfig {
    format: {
        splitClosingParens: boolean
        wrapBeforeLocals: 'never' | 'overflow' | 'multiOnly' | 'always'
        wrapBeforeFields: 'never' | 'overflow' | 'multiOnly' | 'always'
        wrapBeforeConstExpr: 'never' | 'overflow' | 'multiOnly' | 'always'
        multiLineLocals: 'never' | 'overflow' | 'smart' | 'wrap' | 'always'
        multiLineFields: 'never' | 'overflow' | 'smart' | 'wrap' | 'always'
        formatComments: boolean
        ignoreCommentDirective: string
    }
    lints: {
        unused: LintLevel
        unread: LintLevel
        shadow: LintLevel
        implicitModule: LintLevel
        multiModules: LintLevel
        unreachable: LintLevel
        deprecated: LintLevel
        needlessMut: LintLevel
        needlessTryTable: LintLevel
        uselessCatch: LintLevel
    }
    inlayHint: {
        types: boolean
        ending: boolean
        index: boolean
    }
}
"#;

#[wasm_bindgen]
pub struct LanguageService {
    inner: wat_service::LanguageService,
    serializer: Serializer,
}

#[wasm_bindgen(js_class = LanguageService)]
impl LanguageService {
    #[wasm_bindgen(constructor)]
    pub fn new() -> LanguageService {
        LanguageService {
            inner: wat_service::LanguageService::default(),
            serializer: Serializer::json_compatible(),
        }
    }

    #[wasm_bindgen(js_name = "getOpenedUris", unchecked_return_type = "string[]")]
    pub fn get_opened_uris(&self) -> Result<JsValue, Error> {
        self.inner.get_opened_uris().serialize(&self.serializer)
    }

    #[wasm_bindgen(js_name = "setConfig")]
    pub fn set_config(
        &mut self,
        uri: String,
        #[wasm_bindgen(unchecked_param_type = "ServiceConfig")] config: JsValue,
    ) -> Result<(), Error> {
        serde_wasm_bindgen::from_value(config).map(|config| self.inner.set_config(uri, config))
    }
    #[wasm_bindgen(js_name = "setGlobalConfig")]
    pub fn set_global_config(
        &mut self,
        #[wasm_bindgen(unchecked_param_type = "ServiceConfig")] config: JsValue,
    ) -> Result<(), Error> {
        serde_wasm_bindgen::from_value(config).map(|config| self.inner.set_global_config(config))
    }

    #[wasm_bindgen]
    pub fn commit(&mut self, uri: String, text: String) {
        self.inner.commit(uri, text);
    }

    #[wasm_bindgen(unchecked_return_type = "lsp.InitializeResult")]
    pub fn initialize(
        &mut self,
        #[wasm_bindgen(unchecked_param_type = "lsp.InitializeParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.initialize(params).serialize(&self.serializer))
    }

    #[wasm_bindgen(js_name = "didOpen")]
    pub fn did_open(
        &mut self,
        #[wasm_bindgen(unchecked_param_type = "lsp.DidOpenTextDocumentParams")] params: JsValue,
    ) -> Result<(), Error> {
        serde_wasm_bindgen::from_value(params).map(|params| self.inner.did_open(params))
    }

    #[wasm_bindgen(js_name = "didChange")]
    pub fn did_change(
        &mut self,
        #[wasm_bindgen(unchecked_param_type = "lsp.DidChangeTextDocumentParams")] params: JsValue,
    ) -> Result<(), Error> {
        serde_wasm_bindgen::from_value(params).map(|params| self.inner.did_change(params))
    }

    #[wasm_bindgen(js_name = "didClose")]
    pub fn did_close(
        &mut self,
        #[wasm_bindgen(unchecked_param_type = "lsp.DidCloseTextDocumentParams")] params: JsValue,
    ) -> Result<(), Error> {
        serde_wasm_bindgen::from_value(params).map(|params| self.inner.did_close(params))
    }

    #[wasm_bindgen(
        js_name = "prepareCallHierarchy",
        unchecked_return_type = "lsp.CallHierarchyItem[] | null"
    )]
    pub fn prepare_call_hierarchy(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.CallHierarchyPrepareParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.prepare_call_hierarchy(params).serialize(&self.serializer))
    }
    #[wasm_bindgen(
        js_name = "callHierarchyIncomingCalls",
        unchecked_return_type = "lsp.CallHierarchyIncomingCall[] | null"
    )]
    pub fn call_hierarchy_incoming_calls(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.CallHierarchyIncomingCallsParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params).and_then(|params| {
            self.inner
                .call_hierarchy_incoming_calls(params)
                .serialize(&self.serializer)
        })
    }
    #[wasm_bindgen(
        js_name = "callHierarchyOutgoingCalls",
        unchecked_return_type = "lsp.CallHierarchyOutgoingCall[] | null"
    )]
    pub fn call_hierarchy_outgoing_calls(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.CallHierarchyOutgoingCallsParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params).and_then(|params| {
            self.inner
                .call_hierarchy_outgoing_calls(params)
                .serialize(&self.serializer)
        })
    }

    #[wasm_bindgen(js_name = "codeAction", unchecked_return_type = "lsp.CodeAction[] | null")]
    pub fn code_action(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.CodeActionParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.code_action(params).serialize(&self.serializer))
    }

    #[wasm_bindgen(js_name = "codeLens", unchecked_return_type = "lsp.CodeLens[] | null")]
    pub fn code_lens(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.CodeLensParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.code_lens(params).serialize(&self.serializer))
    }
    #[wasm_bindgen(js_name = "codeLensResolve", unchecked_return_type = "lsp.CodeLens")]
    pub fn code_lens_resolve(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.CodeLens")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.code_lens_resolve(params).serialize(&self.serializer))
    }

    #[wasm_bindgen(unchecked_return_type = "lsp.CompletionItem[] | null")]
    pub fn completion(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.CompletionParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.completion(params).serialize(&self.serializer))
    }

    #[wasm_bindgen(js_name = "gotoDefinition", unchecked_return_type = "lsp.Definition | null")]
    pub fn goto_definition(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.DefinitionParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.goto_definition(params).serialize(&self.serializer))
    }
    #[wasm_bindgen(js_name = "gotoTypeDefinition", unchecked_return_type = "lsp.Definition | null")]
    pub fn goto_type_definition(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.TypeDefinitionParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.goto_type_definition(params).serialize(&self.serializer))
    }
    #[wasm_bindgen(js_name = "gotoDeclaration", unchecked_return_type = "lsp.Declaration | null")]
    pub fn goto_declaration(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.DeclarationParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.goto_declaration(params).serialize(&self.serializer))
    }

    #[wasm_bindgen(
        js_name = "pullDiagnostics",
        unchecked_return_type = "lsp.RelatedFullDocumentDiagnosticReport"
    )]
    pub fn pull_diagnostics(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.DocumentDiagnosticParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.pull_diagnostics(params).serialize(&self.serializer))
    }
    #[wasm_bindgen(
        js_name = "publishDiagnostics",
        unchecked_return_type = "lsp.PublishDiagnosticsParams"
    )]
    pub fn publish_diagnostics(&self, uri: String) -> Result<JsValue, Error> {
        self.inner.publish_diagnostics(uri).serialize(&self.serializer)
    }

    #[wasm_bindgen(
        js_name = "documentHighlight",
        unchecked_return_type = "lsp.DocumentHighlight[] | null"
    )]
    pub fn document_highlight(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.DocumentHighlightParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.document_highlight(params).serialize(&self.serializer))
    }

    #[wasm_bindgen(js_name = "documentSymbol", unchecked_return_type = "lsp.DocumentSymbol[] | null")]
    pub fn document_symbol(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.DocumentSymbolParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.document_symbol(params).serialize(&self.serializer))
    }

    #[wasm_bindgen(js_name = "foldingRange", unchecked_return_type = "lsp.FoldingRange[] | null")]
    pub fn folding_range(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.FoldingRangeParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.folding_range(params).serialize(&self.serializer))
    }

    #[wasm_bindgen(unchecked_return_type = "lsp.TextEdit[] | null")]
    pub fn formatting(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.DocumentFormattingParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.formatting(params).serialize(&self.serializer))
    }
    #[wasm_bindgen(js_name = "rangeFormatting", unchecked_return_type = "lsp.TextEdit[] | null")]
    pub fn range_formatting(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.DocumentRangeFormattingParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.range_formatting(params).serialize(&self.serializer))
    }

    #[wasm_bindgen(unchecked_return_type = "lsp.Hover | null")]
    pub fn hover(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.HoverParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params).and_then(|params| self.inner.hover(params).serialize(&self.serializer))
    }

    #[wasm_bindgen(js_name = "inlayHint", unchecked_return_type = "lsp.InlayHint[] | null")]
    pub fn inlay_hint(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.InlayHintParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.inlay_hint(params).serialize(&self.serializer))
    }

    #[wasm_bindgen(js_name = "findReferences", unchecked_return_type = "lsp.Location[] | null")]
    pub fn find_references(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.ReferenceParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.find_references(params).serialize(&self.serializer))
    }

    #[wasm_bindgen(js_name = "prepareRename", unchecked_return_type = "lsp.PrepareRenameResult | null")]
    pub fn prepare_rename(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.PrepareRenameParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.prepare_rename(params).serialize(&self.serializer))
    }
    #[wasm_bindgen(unchecked_return_type = "lsp.WorkspaceEdit | null")]
    pub fn rename(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.RenameParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params).and_then(|params| self.inner.rename(params).serialize(&self.serializer))
    }

    #[wasm_bindgen(js_name = "selectionRange", unchecked_return_type = "lsp.SelectionRange[] | null")]
    pub fn selection_range(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.SelectionRangeParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.selection_range(params).serialize(&self.serializer))
    }

    #[wasm_bindgen(js_name = "semanticTokensFull", unchecked_return_type = "lsp.SemanticTokens | null")]
    pub fn semantic_tokens_full(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.SemanticTokensParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.semantic_tokens_full(params).serialize(&self.serializer))
    }
    #[wasm_bindgen(js_name = "semanticTokensRange", unchecked_return_type = "lsp.SemanticTokens | null")]
    pub fn semantic_tokens_range(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.SemanticTokensRangeParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.semantic_tokens_range(params).serialize(&self.serializer))
    }

    #[wasm_bindgen(js_name = "signatureHelp", unchecked_return_type = "lsp.SignatureHelp | null")]
    pub fn signature_help(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.SignatureHelpParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.signature_help(params).serialize(&self.serializer))
    }

    #[wasm_bindgen(
        js_name = "prepareTypeHierarchy",
        unchecked_return_type = "lsp.TypeHierarchyItem[] | null"
    )]
    pub fn prepare_type_hierarchy(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.TypeHierarchyPrepareParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.prepare_type_hierarchy(params).serialize(&self.serializer))
    }
    #[wasm_bindgen(
        js_name = "typeHierarchySupertypes",
        unchecked_return_type = "lsp.TypeHierarchyItem[] | null"
    )]
    pub fn type_hierarchy_supertypes(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.TypeHierarchySupertypesParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.type_hierarchy_supertypes(params).serialize(&self.serializer))
    }
    #[wasm_bindgen(
        js_name = "typeHierarchySubtypes",
        unchecked_return_type = "lsp.TypeHierarchyItem[] | null"
    )]
    pub fn type_hierarchy_subtypes(
        &self,
        #[wasm_bindgen(unchecked_param_type = "lsp.TypeHierarchySubtypesParams")] params: JsValue,
    ) -> Result<JsValue, Error> {
        serde_wasm_bindgen::from_value(params)
            .and_then(|params| self.inner.type_hierarchy_subtypes(params).serialize(&self.serializer))
    }
}
