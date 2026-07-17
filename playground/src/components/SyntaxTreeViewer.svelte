<script lang="ts">
  import { debugSyntaxTree } from '@wasm-language-tools/wasm'
  import { onMount } from 'svelte'
  import { type IStandaloneCodeEditor, monacoOptions } from '../shared.js'

  const RE_NODE_RANGE = /(\w+)@(\d+)\.\.(\d+)/

  const { monaco, sourceCode, onNodeRangeChange }: {
    monaco: typeof import('@codingame/monaco-vscode-editor-api'),
    sourceCode: string,
    onNodeRangeChange: (start: number, end: number) => void,
  } = $props()
  let el: HTMLDivElement
  let editor: IStandaloneCodeEditor | undefined = $state()

  $effect(() => {
    editor?.setValue(debugSyntaxTree(sourceCode))
  })

  onMount(() => {
    editor = monaco.editor.create(el, {
      ...monacoOptions,
      value: debugSyntaxTree(sourceCode),
      language: 'plaintext',
      readOnly: true,
      occurrencesHighlight: 'off',
    })
    const listenerDidChangeCursorPosition = editor.onDidChangeCursorPosition((e) => {
      sendNodeRange(e.position.lineNumber)
    })
    return () => {
      listenerDidChangeCursorPosition.dispose()
      editor?.dispose()
    }
  })

  function sendNodeRange(lineNumber: number) {
    if (!editor) {
      return
    }
    const line = editor.getModel()?.getLineContent(lineNumber) ?? ''
    const matches = line.match(RE_NODE_RANGE)
    if (matches && matches[1] !== 'ROOT') {
      onNodeRangeChange(+matches[2]!, +matches[3]!)
    }
  }
</script>

<div bind:this={el}></div>

<style>
  div {
    height: var(--editor-height);
  }
</style>
