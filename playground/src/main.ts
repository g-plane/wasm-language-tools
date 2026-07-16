import { initialize } from '@codingame/monaco-vscode-api'
import getLanguagesServiceOverride from '@codingame/monaco-vscode-languages-service-override'
import 'vscode/localExtensionHost'
import { mount } from 'svelte'
import App from './App.svelte'

self.MonacoEnvironment = {
  getWorker() {
    return new Worker(
      new URL('@codingame/monaco-vscode-editor-api/esm/vs/editor/editor.worker', import.meta.url),
      { type: 'module' },
    )
  },
}

await initialize({
  ...getLanguagesServiceOverride(),
})

mount(App, {
  target: document.querySelector('#app')!,
})
