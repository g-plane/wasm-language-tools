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
                text: 'Server Executable',
                link: '/guide/getting-started/executable',
              },
            ],
          },
          { text: 'Deprecation', link: '/guide/deprecation' },
        ],
      },
      {
        text: 'Config',
        items: [
          { text: 'Overview', link: '/config/overview', docFooterText: 'Config Overview' },
          { text: 'Lint', link: '/config/lint' },
          {
            text: 'Format',
            link: '/config/format',
            items: [
              { text: 'splitClosingParens', link: '/config/format/split-closing-parens' },
              { text: 'wrapBeforeLocals', link: '/config/format/wrap-before-locals' },
              { text: 'wrapBeforeConstExpr', link: '/config/format/wrap-before-const-expr' },
              { text: 'formatComments', link: '/config/format/format-comments' },
            ],
          },
          { text: 'Inlay Hint', link: '/config/inlay-hint' },
        ],
      },
      {
        text: 'Diagnostics',
        items: [
          {
            text: 'Overview',
            link: '/diagnostics/overview',
            docFooterText: 'Diagnostics Overview',
          },
          { text: 'Undefined', link: '/diagnostics/undef' },
          { text: 'Unused', link: '/diagnostics/unused' },
          { text: 'Unreachable', link: '/diagnostics/unreachable' },
          { text: 'Uninitialized', link: '/diagnostics/uninit' },
          { text: 'Mutated Immutable', link: '/diagnostics/mutated-immutable' },
          { text: 'Needless Mutable', link: '/diagnostics/needless-mut' },
          { text: 'Type Checking', link: '/diagnostics/type-check' },
          { text: 'Subtyping', link: '/diagnostics/subtyping' },
          { text: 'Type Misuse', link: '/diagnostics/type-misuse' },
          { text: 'Packed Type', link: '/diagnostics/packing' },
          { text: 'Shadowing', link: '/diagnostics/shadow' },
          { text: 'Constant Expression', link: '/diagnostics/const-expr' },
          { text: 'Duplicated Names', link: '/diagnostics/duplicated-names' },
          { text: 'New Non-defaultable', link: '/diagnostics/new-non-defaultable' },
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
  markdown: {
    codeTransformers: [
      {
        preprocess(_, options) {
          options.decorations = (options.decorations ?? []).concat(
            options.meta?.__raw?.split(' ')
              .map((it) => it.match(/^(error|warning)-(\d+)-(\d+)-(\d+)-(\d+)$/))
              .filter((it) => !!it)
              .map(([, severity, startLine, startChar, endLine, endChar]) => ({
                start: {
                  line: Number.parseInt(startLine) - 1,
                  character: Number.parseInt(startChar) - 1,
                },
                end: {
                  line: Number.parseInt(endLine) - 1,
                  character: Number.parseInt(endChar) - 1,
                },
                properties: { class: `severity severity__${severity}` },
              })) ?? [],
          )
        },
      },
      {
        preprocess(_, options) {
          options.decorations = (options.decorations ?? []).concat(
            options.meta?.__raw?.split(' ')
              .map((it) => it.match(/^faded-(\d+)-(\d+)-(\d+)-(\d+)$/))
              .filter((it) => !!it)
              .map(([, startLine, startChar, endLine, endChar]) => ({
                start: {
                  line: Number.parseInt(startLine) - 1,
                  character: Number.parseInt(startChar) - 1,
                },
                end: {
                  line: Number.parseInt(endLine) - 1,
                  character: Number.parseInt(endChar) - 1,
                },
                properties: { class: `code-faded` },
              })) ?? [],
          )
        },
      },
      {
        preprocess(_, options) {
          options.decorations = (options.decorations ?? []).concat(
            options.meta?.__raw?.split(' ')
              .map((it) => it.match(/^strikethrough-(\d+)-(\d+)-(\d+)-(\d+)$/))
              .filter((it) => !!it)
              .map(([, startLine, startChar, endLine, endChar]) => ({
                start: {
                  line: Number.parseInt(startLine) - 1,
                  character: Number.parseInt(startChar) - 1,
                },
                end: {
                  line: Number.parseInt(endLine) - 1,
                  character: Number.parseInt(endChar) - 1,
                },
                properties: { class: `code-strikethrough` },
              })) ?? [],
          )
        },
      },
    ],
  },
})
