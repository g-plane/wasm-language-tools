<script lang="ts">
  import type { Position } from '@codingame/monaco-vscode-editor-api'
  import { configureDarkMode } from './color.js'
  import ControlFlowGraph from './components/ControlFlowGraph.svelte'
  import DiagnosticsList from './components/DiagnosticsList.svelte'
  import FormatterViewer from './components/FormatterViewer.svelte'
  import Header from './components/Header.svelte'
  import Sidebar from './components/Sidebar.svelte'
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
</script>

<Header />
{#await resources}
  Loading editor and language server...
{:then [monaco, client, d3Graphviz]}
  <main>
    <aside>
      <Sidebar onExampleChange={(code) => sourceCode = code} />
    </aside>
    <section>
      <Tabs
        tabs={[
          { name: 'Source', value: 'source', content: source },
        ]}
      />
      {#snippet source()}
        <SourceEditor
          {monaco}
          value={sourceCode}
          {selectedRange}
          onValueChange={(value) => sourceCode = value}
          onCursorPositionChange={(position) => cursorPos = position}
        />
      {/snippet}
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
  aside {
    width: 14vw;
  }
  main {
    display: flex;
  }
  section {
    width: 43vw;
  }
</style>
