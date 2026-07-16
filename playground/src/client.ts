import { MonacoLanguageClient } from 'monaco-languageclient'
import { BrowserMessageReader, BrowserMessageWriter } from 'vscode-languageclient/browser'

export function createLanguageClient(wasm: ArrayBuffer) {
  const worker = new Worker(new URL('./server.ts', import.meta.url), { type: 'module' })
  const languageClient = new MonacoLanguageClient({
    name: 'WebAssembly Language Tools',
    clientOptions: {
      documentSelector: [{ language: 'wat' }],
    },
    messageTransports: {
      reader: new BrowserMessageReader(worker),
      writer: new BrowserMessageWriter(worker),
    },
  })
  worker.addEventListener('message', () => {
    languageClient.start()
  }, { once: true })
  worker.postMessage(wasm)
  return languageClient
}
