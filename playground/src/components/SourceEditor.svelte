<script lang="ts">
  import { onMount } from 'svelte'
  import { createLanguageClient } from '../client.js'
  import { monacoOptions } from '../shared.js'

  const { monaco, wasm, onValueChange }: {
    monaco: typeof import('@codingame/monaco-vscode-editor-api'),
    wasm: ArrayBuffer,
    onValueChange: (value: string) => void,
  } = $props()
  let el: HTMLDivElement

  onMount(() => {
    const editor = monaco.editor.create(el, {
      ...monacoOptions,
      value: '',
      language: 'wat',
    })
    const didChangeModelContentListener = editor.onDidChangeModelContent(() => {
      onValueChange(editor.getValue())
    })
    const languageClient = createLanguageClient(wasm)
    return () => {
      languageClient.dispose()
      didChangeModelContentListener.dispose()
      editor.dispose()
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
