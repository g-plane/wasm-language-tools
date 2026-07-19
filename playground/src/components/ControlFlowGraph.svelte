<script lang="ts">
  import type { Position } from '@codingame/monaco-vscode-editor-api'
  import type { MonacoLanguageClient } from 'monaco-languageclient'
  import { onMount } from 'svelte'
  import { RE_NODE_RANGE, SOURCE_URI } from '../shared.js'

  const { d3Graphviz, client, position, onNodeRangeChange }: {
    d3Graphviz: typeof import('d3-graphviz'),
    client: MonacoLanguageClient,
    position: Position | null,
    onNodeRangeChange: (start: number, end: number) => void,
  } = $props()
  let el: HTMLDivElement
  let graphviz: ReturnType<typeof import('d3-graphviz')['graphviz']> | undefined = $state()
  let hasGraph = $state(false)

  $effect(() => {
    if (position) {
      renderGraph(position)
    } else {
      hasGraph = false
    }
  })

  onMount(() => {
    graphviz = d3Graphviz.graphviz(el, { useWorker: false })
    return () => {
      ;(graphviz as { destroy(): unknown } | undefined)?.destroy()
    }
  })

  async function renderGraph(position: Position) {
    if (!el || !graphviz) {
      return
    }
    const dot = await client.sendRequest<string | null>('workspace/executeCommand', {
      command: 'wasmLanguageTools.__generateControlFlowGraphDot',
      arguments: [SOURCE_URI, { line: position.lineNumber - 1, character: position.column - 1 }],
    })
    if (dot) {
      hasGraph = true
      graphviz.renderDot(dot)
    } else {
      hasGraph = false
    }
  }

  function handleClick(e: MouseEvent) {
    const target = e.target as Element | null
    const text = target?.closest('g.node')?.querySelector('text')?.textContent
    const matches = text?.match(RE_NODE_RANGE)
    if (matches) {
      onNodeRangeChange(+matches[2]!, +matches[3]!)
    }
  }
</script>

<div bind:this={el} role="none" style:display={hasGraph ? 'block' : 'none'} onclick={handleClick}>
</div>
{#if !hasGraph}
  <section>
    <p>No control flow graph available for the current cursor position in the left-side editor.</p>
  </section>
{/if}

<style>
  div > :global(svg) {
    width: 100%;
    height: var(--editor-height);
  }
  section {
    height: var(--editor-height);
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>
