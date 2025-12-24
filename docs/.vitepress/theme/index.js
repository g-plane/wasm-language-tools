import DefaultTheme from 'vitepress/theme'
import CenteredArrowDown from './components/CenteredArrowDown.vue'
import './custom.css'

/** @type {import('vitepress').Theme} */
export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component('CenteredArrowDown', CenteredArrowDown)
  },
}
