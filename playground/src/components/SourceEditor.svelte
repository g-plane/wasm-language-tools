<script lang="ts">
  import type { Position } from '@codingame/monaco-vscode-editor-api'
  import { onMount } from 'svelte'
  import {
    type IStandaloneCodeEditor,
    type ITextModel,
    SOURCE_URI,
    monacoOptions,
  } from '../shared.js'

  const { monaco, defaultValue, selectedRange, onValueChange, onCursorPositionChange }: {
    monaco: typeof import('@codingame/monaco-vscode-editor-api'),
    defaultValue: string,
    selectedRange: { start: number, end: number } | null,
    onValueChange: (value: string) => void,
    onCursorPositionChange: (position: Position) => void,
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
    model = monaco.editor.createModel(defaultValue, 'wat', monaco.Uri.parse(SOURCE_URI))
    editor = monaco.editor.create(el, { ...monacoOptions, model })
    const didChangeModelContentListener = editor.onDidChangeModelContent(() => {
      if (model) {
        onValueChange(model.getValue())
      }
    })
    const didChangeCursorPositionListener = editor.onDidChangeCursorPosition((e) => {
      onCursorPositionChange(e.position)
    })
    return () => {
      didChangeModelContentListener.dispose()
      didChangeCursorPositionListener.dispose()
      editor?.dispose()
      model?.dispose()
    }
  })
</script>

<div bind:this={el}></div>

<style>
  div {
    height: var(--editor-height);
  }
</style>
