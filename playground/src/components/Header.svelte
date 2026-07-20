<script lang="ts"></script>

<header>
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
</header>

<style>
  header {
    display: flex;
    align-items: center;
    column-gap: 1.5rem;
    height: 2.75rem;
    padding-left: 1.25rem;
  }
  h1 {
    font-size: 1.25rem;
  }

  nav {
    display: flex;
    align-items: center;
    column-gap: 0.8rem;
  }
  nav > * {
    color: oklch(0 0 0);
    padding: 0.35rem 0.7rem;
    border-radius: 0.5rem;
    transition: background-color 0.2s, color 0.2s;
  }
  :global(.dark) nav > * {
    color: var(--dark-text-color);
  }
  nav > *:hover {
    background-color: oklch(0.9 0.02 281.96);
  }
  :global(.dark) nav > *:hover {
    background-color: oklch(0.35 0.02 281.96);
  }
</style>
