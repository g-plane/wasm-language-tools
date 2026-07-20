<script lang="ts">
  import type { MonacoLanguageClient } from 'monaco-languageclient'
  import type { Diagnostic, DocumentDiagnosticReport } from 'vscode-languageserver-protocol'
  import { SOURCE_URI } from '../shared.js'

  const { monaco, client }: {
    monaco: typeof import('@codingame/monaco-vscode-editor-api'),
    client: MonacoLanguageClient,
  } = $props()
  let diagnostics: Diagnostic[] = $state([])

  updateDiagnostics()
  $effect(() => {
    const listener = monaco.editor.onDidChangeMarkers(() => {
      updateDiagnostics()
    })
    return () => {
      listener.dispose()
    }
  })

  async function updateDiagnostics() {
    const response: DocumentDiagnosticReport = await client.sendRequest('textDocument/diagnostic', {
      textDocument: { uri: SOURCE_URI },
    })
    if (response.kind === 'full') {
      diagnostics = response.items
    }
  }
</script>

{#if diagnostics.length > 0}
  <ul>
    {#each diagnostics as diagnostic}
      <li data-severity={diagnostic.severity}>
        {diagnostic.code}:{diagnostic.range.start.line + 1}:{diagnostic.range.start.character + 1}:
        {diagnostic.message}
      </li>
    {/each}
  </ul>
{:else}
  <p>No problems.</p>
{/if}

<style>
  ul {
    list-style: none;
    display: flex;
    flex-direction: column;
    row-gap: 0.5rem;
    font-family: var(--monospace);
  }
  li {
    padding: 0.15rem 0.5rem;
  }
  li[data-severity="1"] {
    border-left: 4px solid var(--severity-error);
  }
  li[data-severity="2"] {
    border-left: 4px solid var(--severity-warning);
  }
  li[data-severity="4"] {
    border-left: 4px solid var(--severity-hint);
  }

  ul, p {
    padding: 1rem;
  }
</style>
