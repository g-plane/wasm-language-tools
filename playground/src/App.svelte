<script lang="ts">
  import SourceEditor from './components/SourceEditor.svelte'
  import SyntaxTreeViewer from './components/SyntaxTreeViewer.svelte'
  import { resources } from './resources.js'

  let sourceCode = $state('')
  let selectedRange: { start: number, end: number } | null = $state(null)
</script>

{#await resources}
  Loading editor and language server...
{:then [monaco]}
  <main>
    <SourceEditor
      {monaco}
      defaultValue={sourceCode}
      {selectedRange}
      onValueChange={(value) => sourceCode = value}
    />
    <SyntaxTreeViewer
      {monaco}
      {sourceCode}
      onNodeRangeChange={(start, end) => selectedRange = { start, end }}
    />
  </main>
{/await}

<style>
  main {
    display: flex;
  }
</style>
