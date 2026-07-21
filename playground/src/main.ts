import { mount } from 'svelte'
import App from './App.svelte'

self.MonacoEnvironment = {
  getWorker() {
    return new Worker(
      new URL(
        '@codingame/monaco-vscode-editor-api/esm/vs/editor/editor.worker.js',
        import.meta.url,
      ),
      { type: 'module' },
    )
  },
}

mount(App, {
  target: document.querySelector('#app')!,
})
