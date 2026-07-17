<script lang="ts">
  import { type FormatOptions, format } from '@wasm-language-tools/wasm'
  import { onMount } from 'svelte'
  import { type IStandaloneCodeEditor, monacoOptions } from '../shared.js'

  const { monaco, sourceCode, options }: {
    monaco: typeof import('@codingame/monaco-vscode-editor-api'),
    sourceCode: string,
    options: FormatOptions,
  } = $props()
  let el: HTMLDivElement
  let editor: IStandaloneCodeEditor | undefined = $state()

  $effect(() => {
    editor?.setValue(format(sourceCode, options))
  })

  onMount(() => {
    editor = monaco.editor.create(el, {
      ...monacoOptions,
      value: format(sourceCode, options),
      language: 'wat',
      readOnly: true,
      inlayHints: { enabled: 'off' },
    })
    return () => {
      editor?.dispose()
    }
  })
</script>

<div bind:this={el}></div>

<style>
  div {
    height: var(--editor-height);
  }
</style>
