<script lang="ts">
  import { onMount } from 'svelte'
  import { createLanguageClient } from '../client.js'
  import { type IStandaloneCodeEditor, type ITextModel, monacoOptions } from '../shared.js'

  const { monaco, wasm, defaultValue, selectedRange, onValueChange }: {
    monaco: typeof import('@codingame/monaco-vscode-editor-api'),
    wasm: ArrayBuffer,
    defaultValue: string,
    selectedRange: { start: number, end: number } | null,
    onValueChange: (value: string) => void,
  } = $props()
  let el: HTMLDivElement
  let model: ITextModel | undefined = $state()
  let editor: IStandaloneCodeEditor | undefined = $state()

  $effect(() => {
    if (model && editor && selectedRange) {
      const start = model.getPositionAt(selectedRange.start)
      const end = model.getPositionAt(selectedRange.end)
      editor.setSelection(monaco.Selection.fromPositions(start, end))
    }
  })

  onMount(() => {
    model = monaco.editor.createModel(
      defaultValue,
      'wat',
      monaco.Uri.parse('file:///main.wat'),
    )
    editor = monaco.editor.create(el, { ...monacoOptions, model })
    const didChangeModelContentListener = editor.onDidChangeModelContent(() => {
      if (model) {
        onValueChange(model.getValue())
      }
    })
    const languageClient = createLanguageClient(wasm)
    return () => {
      languageClient.dispose()
      didChangeModelContentListener.dispose()
      editor?.dispose()
      model?.dispose()
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
