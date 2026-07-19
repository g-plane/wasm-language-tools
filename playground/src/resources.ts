import { initSync } from '@wasm-language-tools/wasm'
import { registerLanguage } from './shared.js'

const vscode = Promise.all([
  import('@codingame/monaco-vscode-api'),
  import('@codingame/monaco-vscode-languages-service-override'),
  import('vscode/localExtensionHost'),
]).then(([{ initialize }, { default: getLanguagesServiceOverride }]) =>
  initialize({
    ...getLanguagesServiceOverride(),
  })
).catch((error: Error) => {
  // HMR workaround
  if (error.message !== 'Services are already initialized') {
    throw error
  }
})

const monaco = vscode.then(() => import('@codingame/monaco-vscode-editor-api'))
monaco.then(registerLanguage)

const wasm = fetch(new URL('@wasm-language-tools/wasm/binding_wasm_bg.wasm', import.meta.url))
  .then((res) => res.arrayBuffer())
wasm.then((bytes) => initSync({ module: bytes }))

const client = vscode.then(() => Promise.all([import('./client.js'), wasm]))
  .then(([{ createLanguageClient }, wasm]) => createLanguageClient(wasm))

const d3Graphviz = import('d3-graphviz')

export const resources = Promise.all([monaco, client, d3Graphviz])
