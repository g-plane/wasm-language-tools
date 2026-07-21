<script lang="ts">
  import { isDarkMode } from '../color.js'

  function toggleDarkMode() {
    $isDarkMode = !$isDarkMode
  }
</script>

<header>
  <div class="left">
    <h1>WebAssembly Language Tools Playground</h1>
    <nav>
      <a href="https://wasm-language-tools.netlify.app/" target="_blank" rel="noopener noreferrer">
        Docs
      </a>
      <a
        href="https://github.com/g-plane/wasm-language-tools"
        target="_blank"
        rel="noopener noreferrer"
      >
        GitHub
      </a>
    </nav>
  </div>
  <button onclick={toggleDarkMode}>{$isDarkMode ? 'Dark' : 'Light'}</button>
</header>

<style>
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 2.75rem;
    padding: 0 1.25rem;
  }
  .left {
    display: flex;
    align-items: center;
    column-gap: 1.5rem;
  }
  h1 {
    font-size: 1.25rem;
  }

  nav {
    display: flex;
    align-items: center;
    column-gap: 0.8rem;
  }
  a, button {
    color: oklch(0 0 0);
    padding: 0.35rem 0.7rem;
    border-radius: 0.5rem;
    transition: background-color 0.2s, color 0.2s;
  }
  :global(.dark) :is(a, button) {
    color: var(--dark-text-color);
  }
  :is(a, button):hover {
    background-color: oklch(0.9 0.02 281.96);
  }
  :global(.dark) :is(a, button):hover {
    background-color: oklch(0.35 0.02 281.96);
  }
</style>
