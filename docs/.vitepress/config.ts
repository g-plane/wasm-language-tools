import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'WebAssembly Language Tools',
  description:
    'The set of tools that provides and improves the editing experience of WebAssembly Text Format.',
  head: [
    ['link', { rel: 'icon', href: '/logo.svg' }],
  ],
  lastUpdated: true,
  themeConfig: {
    siteTitle: 'WASM Language Tools',
    logo: '/logo.svg',
    nav: [
      { text: 'Guide', link: '/guide/introduction', activeMatch: '/guide/' },
      { text: 'Config', link: '/config/overview', activeMatch: '/config/' },
    ],
    sidebar: [
      {
        text: 'Guide',
        items: [
          { text: 'Introduction', link: '/guide/introduction' },
          {
            text: 'Getting Started',
            items: [
              {
                text: 'Editors',
                link: '/guide/getting-started/editors',
              },
              {
                text: 'Server Binary',
                link: '/guide/getting-started/binary',
              },
            ],
          },
        ],
      },
      {
        text: 'Config',
        items: [
          { text: 'Overview', link: '/config/overview', docFooterText: 'Config Overview' },
          { text: 'Lint', link: '/config/lint' },
          { text: 'Format', link: '/config/format' },
        ],
      },
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/g-plane/wasm-language-tools' },
    ],
    editLink: {
      pattern: 'https://github.com/g-plane/wasm-language-tools/edit/main/docs/:path',
    },
    search: {
      provider: 'local',
      options: {
        detailedView: true,
      },
    },
  },
})
