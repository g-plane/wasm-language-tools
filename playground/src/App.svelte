<script lang="ts">
  import FormatterViewer from './components/FormatterViewer.svelte'
  import SourceEditor from './components/SourceEditor.svelte'
  import SyntaxTreeViewer from './components/SyntaxTreeViewer.svelte'
  import Tabs from './components/Tabs.svelte'
  import { resources } from './resources.js'

  let sourceCode = $state('')
  let selectedRange: { start: number, end: number } | null = $state(null)
</script>

{#await resources}
  Loading editor and language server...
{:then [monaco]}
  <main>
    <div>
      <Tabs
        tabs={[
          { name: 'Source', value: 'source', content: source },
        ]}
      />
      {#snippet source()}
        <SourceEditor
          {monaco}
          defaultValue={sourceCode}
          {selectedRange}
          onValueChange={(value) => sourceCode = value}
        />
      {/snippet}
    </div>
    <div>
      <Tabs
        tabs={[
          { name: 'Syntax Tree', value: 'cst', content: cst },
          { name: 'Formatter', value: 'formatter', content: formatter },
        ]}
      />
      {#snippet cst()}
        <SyntaxTreeViewer
          {monaco}
          {sourceCode}
          onNodeRangeChange={(start, end) => selectedRange = { start, end }}
        />
      {/snippet}
      {#snippet formatter()}
        <FormatterViewer {monaco} {sourceCode} options={{}} />
      {/snippet}
    </div>
  </main>
{/await}

<style>
  main {
    display: flex;
  }
  main > div {
    width: 50vw;
  }
</style>
