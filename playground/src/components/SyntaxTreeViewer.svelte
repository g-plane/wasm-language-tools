<script lang="ts">
  import { debugSyntaxTree } from '@wasm-language-tools/wasm'
  import { onMount } from 'svelte'
  import { monacoOptions } from '../shared.js'

  const { monaco, sourceCode }: {
    monaco: typeof import('@codingame/monaco-vscode-editor-api'),
    sourceCode: string,
  } = $props()
  let el: HTMLDivElement
  let editor: import('@codingame/monaco-vscode-editor-api').editor.IStandaloneCodeEditor | undefined

  $effect(() => {
    const tree = debugSyntaxTree(sourceCode)
    editor?.setValue(tree)
  })

  onMount(() => {
    editor = monaco.editor.create(el, {
      ...monacoOptions,
      value: debugSyntaxTree(sourceCode),
      language: 'plaintext',
      readOnly: true,
    })
    return () => {
      editor?.dispose()
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
