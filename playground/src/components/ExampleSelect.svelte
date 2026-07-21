<script lang="ts">
  import { snippets } from '../shared.js'

  const { onExampleChange }: {
    onExampleChange: (code: string) => void,
  } = $props()

  function handleExampleChange(e: Event) {
    const target = e.currentTarget as HTMLSelectElement
    const snippet = snippets.get(target.value)
    if (snippet) {
      onExampleChange(snippet.code)
    }
  }
</script>

<div class="form-item">
  <span>Examples:</span>
  <select onchange={handleExampleChange}>
    {#each snippets as snippet (snippet[0])}
      <option value={snippet[0]}>{snippet[1].name}</option>
    {/each}
  </select>
</div>

<style>
  .form-item {
    display: flex;
    align-items: center;
    padding: 1rem;
    column-gap: 0.5rem;
  }

  select {
    border-radius: 0.25rem;
    padding: 0.15rem;
    border: 1px solid oklch(0.85 0 0);
    background-color: oklch(1 0 0);
  }
  :global(.dark) select {
    border: 1px solid oklch(0.4 0 0);
    background-color: oklch(0.3 0 0);
    color: var(--dark-text-color);
  }
</style>
