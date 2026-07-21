<script lang="ts">
  import { snippets } from '../shared.js'

  const { onExampleChange }: {
    onExampleChange: (code: string) => void,
  } = $props()

  function handleExampleChange(e: Event) {
    const target = e.currentTarget as HTMLSelectElement
    const code = snippets.get(target.value)
    if (code) {
      onExampleChange(code)
    }
  }
</script>

<div class="form-item">
  <span>Examples</span>
  <select onchange={handleExampleChange}>
    <option value="empty">Empty</option>
    <option value="messy">Messy Code</option>
    <option value="cf">Control Flow</option>
    <option value="mutability">Mutability</option>
    <option value="type-check">Type Checking</option>
  </select>
</div>

<style>
  .form-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
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
