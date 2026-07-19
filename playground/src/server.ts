import { LanguageService, initSync } from '@wasm-language-tools/wasm'
import { type Connection, DidChangeConfigurationNotification } from 'vscode-languageserver'
import {
  BrowserMessageReader,
  BrowserMessageWriter,
  createConnection,
} from 'vscode-languageserver/browser'

self.addEventListener('message', (event) => {
  initSync({ module: event.data })
  const service = new LanguageService()
  const connection = createConnection(
    new BrowserMessageReader(self),
    new BrowserMessageWriter(self),
  )
  bindConnection(service, connection)
  connection.listen()
  self.postMessage('loaded')
}, { once: true })

export function bindConnection(service: LanguageService, connection: Connection) {
  connection.onInitialize((params) => service.initialize(params))
  connection.onInitialized(() => {
    connection.client.register(DidChangeConfigurationNotification.type, undefined)
  })
  connection.onDidOpenTextDocument(async (params) => {
    service.didOpen(params)
    const config = await connection.workspace.getConfiguration({
      scopeUri: params.textDocument.uri,
      section: 'wasmLanguageTools',
    })
    service.setConfig(params.textDocument.uri, config)
    connection.sendDiagnostics(service.publishDiagnostics(params.textDocument.uri))
    connection.languages.inlayHint.refresh()
  })
  connection.onDidChangeTextDocument((params) => {
    service.didChange(params)
    connection.sendDiagnostics(service.publishDiagnostics(params.textDocument.uri))
  })
  connection.onDidCloseTextDocument((params) => service.didClose(params))
  connection.onDidChangeConfiguration(async () => {
    const uris = service.getOpenedUris()
    const configs = await connection.workspace.getConfiguration(
      uris.map((uri) => ({ scopeUri: uri, section: 'wasmLanguageTools' })),
    )
    uris.forEach((uri, i) => {
      service.setConfig(uri, configs[i])
      connection.sendDiagnostics(service.publishDiagnostics(uri))
    })
    connection.languages.inlayHint.refresh()
  })
  connection.languages.callHierarchy.onPrepare((params) => service.prepareCallHierarchy(params))
  connection.languages.callHierarchy.onIncomingCalls((params) =>
    service.callHierarchyIncomingCalls(params)
  )
  connection.languages.callHierarchy.onOutgoingCalls((params) =>
    service.callHierarchyOutgoingCalls(params)
  )
  connection.onCodeAction((params) => service.codeAction(params))
  connection.onCodeLens((params) => service.codeLens(params))
  connection.onCodeLensResolve((params) => service.codeLensResolve(params))
  connection.onCompletion((params) => service.completion(params))
  connection.onDefinition((params) => service.gotoDefinition(params))
  connection.onTypeDefinition((params) => service.gotoTypeDefinition(params))
  connection.onDeclaration((params) => service.gotoDeclaration(params))
  connection.languages.diagnostics.on((params) => service.pullDiagnostics(params))
  connection.onDocumentHighlight((params) => service.documentHighlight(params))
  connection.onDocumentSymbol((params) => service.documentSymbol(params))
  connection.onFoldingRanges((params) => service.foldingRange(params))
  connection.onDocumentFormatting((params) => service.formatting(params))
  connection.onDocumentRangeFormatting((params) => service.rangeFormatting(params))
  connection.onExecuteCommand((params) => service.executeCommand(params))
  connection.onHover((params) => service.hover(params))
  connection.languages.inlayHint.on((params) => service.inlayHint(params))
  connection.onReferences((params) => service.findReferences(params))
  connection.onPrepareRename((params) => service.prepareRename(params))
  connection.onRenameRequest((params) => service.rename(params))
  connection.onSelectionRanges((params) => service.selectionRange(params))
  connection.languages.semanticTokens.on((params) => service.semanticTokensFull(params))
  connection.languages.semanticTokens.onRange((params) => service.semanticTokensRange(params))
  connection.onSignatureHelp((params) => service.signatureHelp(params))
  connection.languages.typeHierarchy.onPrepare((params) => service.prepareTypeHierarchy(params))
  connection.languages.typeHierarchy.onSupertypes((params) =>
    service.typeHierarchySupertypes(params)
  )
  connection.languages.typeHierarchy.onSubtypes((params) => service.typeHierarchySubtypes(params))
}
