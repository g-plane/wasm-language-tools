import { writable } from 'svelte/store'
import { monaco } from './resources.js'

const colorScheme = window.matchMedia('(prefers-color-scheme: dark)')

export const isDarkMode = writable(colorScheme.matches, (set) => {
  function updateDarkMode(event: MediaQueryListEvent) {
    set(event.matches)
  }
  colorScheme.addEventListener('change', updateDarkMode)
  return () => {
    colorScheme.removeEventListener('change', updateDarkMode)
  }
})

export function configureDarkMode() {
  return isDarkMode.subscribe((isDarkMode) => {
    if (isDarkMode) {
      document.body.classList.add('dark')
    } else {
      document.body.classList.remove('dark')
    }
    monaco.then((monaco) => {
      monaco.editor.setTheme(isDarkMode ? 'vs-dark' : 'vs')
    })
  })
}
