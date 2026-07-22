<script lang="ts">
  import type { Position } from '@codingame/monaco-vscode-editor-api'
  import { compressToEncodedURIComponent, decompressFromEncodedURIComponent } from 'lz-string'
  import { configureDarkMode } from './color.js'
  import ControlFlowGraph from './components/ControlFlowGraph.svelte'
  import DiagnosticsList from './components/DiagnosticsList.svelte'
  import ExampleSelect from './components/ExampleSelect.svelte'
  import FormatterViewer from './components/FormatterViewer.svelte'
  import Header from './components/Header.svelte'
  import SourceEditor from './components/SourceEditor.svelte'
  import SyntaxTreeViewer from './components/SyntaxTreeViewer.svelte'
  import Tabs from './components/Tabs.svelte'
  import { resources } from './resources.js'

  let sourceCode = $state('')
  let cursorPos: Position | null = $state(null)
  let selectedRange: { start: number, end: number } | null = $state(null)

  function changeSelectedRange(start: number, end: number) {
    selectedRange = { start, end }
  }

  $effect(configureDarkMode)

  function handleShare() {
    const url = new URL(window.location.href)
    url.searchParams.set('code', compressToEncodedURIComponent(sourceCode))
    navigator.clipboard.writeText(url.toString())
    window.history.replaceState(null, '', url.toString())
  }
  $effect(() => {
    const params = new URLSearchParams(window.location.search)
    const value = params.get('code')
    if (!value) {
      return
    }
    const code = decompressFromEncodedURIComponent(value)
    if (code) {
      sourceCode = code
    }
  })
</script>

<Header onShare={handleShare} />
{#await resources}
  <div class="loading">
    <p>Loading editor and language server...</p>
  </div>
{:then [monaco, client, d3Graphviz]}
  <main>
    <section>
      <ExampleSelect onExampleChange={(code) => sourceCode = code} />
      <SourceEditor
        {monaco}
        value={sourceCode}
        {selectedRange}
        onValueChange={(value) => sourceCode = value}
        onCursorPositionChange={(position) => cursorPos = position}
      />
    </section>
    <section>
      <Tabs
        tabs={[
          { name: 'Syntax Tree', value: 'cst', content: cst },
          { name: 'Formatter', value: 'formatter', content: formatter },
          { name: 'Control Flow Graph', value: 'cfg', content: cfg },
          { name: 'Diagnostics', value: 'diagnostics', content: diagnostics },
        ]}
      />
      {#snippet cst()}
        <SyntaxTreeViewer {monaco} {sourceCode} onNodeRangeChange={changeSelectedRange} />
      {/snippet}
      {#snippet formatter()}
        <FormatterViewer {monaco} {sourceCode} options={{}} />
      {/snippet}
      {#snippet cfg()}
        <ControlFlowGraph
          {d3Graphviz}
          {client}
          position={cursorPos}
          onNodeRangeChange={changeSelectedRange}
        />
      {/snippet}
      {#snippet diagnostics()}
        <DiagnosticsList {monaco} {client} />
      {/snippet}
    </section>
  </main>
{/await}

<style>
  .loading {
    height: var(--workspace-height);
    display: flex;
    justify-content: center;
    align-items: center;
  }
  main {
    display: flex;
  }
  section {
    width: 50vw;
  }
</style>
