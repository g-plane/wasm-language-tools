<script lang="ts">
  import { onMount } from 'svelte'
  import { createLanguageClient } from '../client.js'
  import { monacoOptions } from '../shared.js'

  const { monaco, wasm, defaultValue, onValueChange }: {
    monaco: typeof import('@codingame/monaco-vscode-editor-api'),
    wasm: ArrayBuffer,
    defaultValue: string,
    onValueChange: (value: string) => void,
  } = $props()
  let el: HTMLDivElement

  onMount(() => {
    const model = monaco.editor.createModel(
      defaultValue,
      'wat',
      monaco.Uri.parse('file:///main.wat'),
    )
    const editor = monaco.editor.create(el, { ...monacoOptions, model })
    const didChangeModelContentListener = editor.onDidChangeModelContent(() => {
      onValueChange(model.getValue())
    })
    const languageClient = createLanguageClient(wasm)
    return () => {
      languageClient.dispose()
      didChangeModelContentListener.dispose()
      editor.dispose()
      model.dispose()
    }
  })
</script>

<div bind:this={el}></div>

<style>
  div {
    width: 50vw;
    height: 90vh;
  }
</style>
