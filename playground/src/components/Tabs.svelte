<script lang="ts">
  import type { Snippet } from 'svelte'

  interface Tab {
    name: string
    value: string
    content: Snippet
  }

  const { tabs }: { tabs: Tab[] } = $props()
  // svelte-ignore state_referenced_locally -- default value here
  let active = $state(tabs[0]?.value ?? '')
  const currentTab = $derived(tabs.find((tab) => tab.value === active))
</script>

<ul role="tablist">
  {#each tabs as tab (tab.value)}
    <li role="tab" aria-selected={tab.value === active}>
      <button class:selected={tab.value === active} onclick={() => active = tab.value}>
        {tab.name}
      </button>
    </li>
  {/each}
</ul>
{#if currentTab}
  <div role="tabpanel">
    {@render currentTab.content()}
  </div>
{/if}

<style>
  ul {
    display: flex;
    justify-content: center;
    column-gap: 0.75rem;
    list-style: none;
    padding: 0.5rem 0;
    font-size: 0.875rem;
  }

  button {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 0.5rem;
    cursor: pointer;
    background-color: var(--light-color);
    transition: background-color 0.2s, color 0.2s;
  }
  :global(.dark) button {
    background-color: var(--dark-color);
    color: var(--dark-text-color);
  }
  button:is(.selected, :hover) {
    background-color: var(--primary-color);
    color: #fff;
  }
</style>
