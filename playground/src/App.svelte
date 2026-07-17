<script lang="ts">
  import { initSync } from '@wasm-language-tools/wasm'
  import SourceEditor from './components/SourceEditor.svelte'
  import SyntaxTreeViewer from './components/SyntaxTreeViewer.svelte'
  import { registerLanguage } from './shared.js'

  const monaco = import('@codingame/monaco-vscode-editor-api')
  monaco.then(registerLanguage)
  const wasm = fetch(new URL('@wasm-language-tools/wasm/binding_wasm_bg.wasm', import.meta.url))
    .then((res) => res.arrayBuffer())
  wasm.then((bytes) => initSync({ module: bytes }))

  let sourceCode = $state('')
</script>

{#await Promise.all([monaco, wasm])}
  Loading editor and language server...
{:then [monaco, wasm]}
  <main>
    <SourceEditor {monaco} {wasm} onValueChange={(value) => sourceCode = value} />
    <SyntaxTreeViewer {monaco} {sourceCode} />
  </main>
{/await}

<style>
  main {
    display: flex;
  }
</style>
