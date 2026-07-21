import type * as monaco from '@codingame/monaco-vscode-editor-api'

export type IStandaloneCodeEditor = monaco.editor.IStandaloneCodeEditor
export type ITextModel = monaco.editor.ITextModel

export const SOURCE_URI = 'file:///main.wat'
export const RE_NODE_RANGE = /(\w+)@(\d+)\.\.(\d+)/

export const monacoOptions: monaco.editor.IStandaloneEditorConstructionOptions = {
  minimap: { enabled: false },
  fontFamily: 'var(--monospace)',
  fontSize: 16,
  lineHeight: 28,
  tabSize: 2,
  codeLens: false,
  automaticLayout: true,
}

export function registerLanguage(monaco: typeof import('@codingame/monaco-vscode-editor-api')) {
  monaco.languages.register({ id: 'wat', extensions: ['.wat'] })
  monaco.languages.setMonarchTokensProvider('wat', {
    brackets: [{ open: '(', close: ')', token: 'delimiter.parenthesis' }],
    keywords: [
      'array',
      'block',
      'catch',
      'catch_ref',
      'catch_all',
      'catch_all_ref',
      'data',
      'declare',
      'elem',
      'else',
      'end',
      'export',
      'field',
      'final',
      'func',
      'global',
      'if',
      'import',
      'item',
      'local',
      'loop',
      'memory',
      'module',
      'mut',
      'null',
      'offset',
      'pagesize',
      'param',
      'rec',
      'ref',
      'result',
      'shared',
      'start',
      'struct',
      'sub',
      'table',
      'tag',
      'then',
      'try_table',
      'type',
      'unshared',
    ],
    typeKeywords: [
      'i32',
      'i64',
      'f32',
      'f64',
      'v128',
      'i8',
      'i16',
      'any',
      'eq',
      'i31',
      'none',
      'nofunc',
      'exn',
      'noexn',
      'extern',
      'noextern',
      'cont',
      'nocont',
      'anyref',
      'eqref',
      'i31ref',
      'structref',
      'arrayref',
      'nullref',
      'funcref',
      'nullfuncref',
      'exnref',
      'nullexnref',
      'externref',
      'nullexternref',
      'contref',
      'nullcontref',
    ],
    tokenizer: {
      root: [
        [
          /[a-z_$][\w.$]*/,
          {
            cases: {
              '@typeKeywords': 'type.identifier',
              '@keywords': 'keyword',
              '@default': 'operators',
            },
          },
        ],
        [/\$[\w.$-_]+/, 'variable.name'],
        { include: '@whitespace' },
        [/[()]/, '@brackets'],
        [/\d*\.\d+([eE][\-+]?\d+)?/, 'number.float'],
        [/0[xX][0-9a-fA-F]+/, 'number.hex'],
        [/\d+/, 'number'],
        [/"([^"\\]|\\.)*$/, 'string.invalid'],
        [/"/, { token: 'string.quote', bracket: '@open', next: '@string' }],
      ],
      comment: [
        [/[^;)]+/, 'comment'],
        [/;\)/, 'comment', '@pop'],
        [/[;)]/, 'comment'],
      ],
      string: [
        [/[^\\"]+/, 'string'],
        [/"/, { token: 'string.quote', bracket: '@close', next: '@pop' }],
      ],
      whitespace: [
        [/[ \t\r\n]+/, 'white'],
        [/\(;/, 'comment', '@comment'],
        [/;;.*$/, 'comment'],
      ],
    },
  })
  monaco.languages.setLanguageConfiguration('wat', {
    comments: {
      lineComment: ';;',
      blockComment: ['(;', ';)'],
    },
    brackets: [['(', ')']],
    autoClosingPairs: [
      { open: '{', close: '}' },
      { open: '[', close: ']' },
      { open: '(', close: ')' },
      { open: '"', close: '"' },
      { open: '(*', close: '*)', notIn: ['string'] },
    ],
    surroundingPairs: [
      { open: '{', close: '}' },
      { open: '[', close: ']' },
      { open: '(', close: ')' },
      { open: '"', close: '"' },
      { open: '(*', close: '*)' },
    ],
    indentationRules: {
      increaseIndentPattern: /^((?!;;).)*\([^)"'`]*$/,
      decreaseIndentPattern: /^((?!.*?\(;).*;\))?\s*\).*$/,
    },
    onEnterRules: [
      {
        beforeText: /^.*\([^\)]*$/,
        afterText: /^\s*\).*$/,
        action: {
          indentAction: monaco.languages.IndentAction.IndentOutdent,
          appendText: '\t',
        },
      },
      {
        beforeText: /^\s*;;; /,
        action: {
          indentAction: monaco.languages.IndentAction.None,
          appendText: ';;; ',
        },
      },
      {
        beforeText: /^\s*;;;/,
        action: {
          indentAction: monaco.languages.IndentAction.None,
          appendText: ';;;',
        },
      },
    ],
  })
}

export const snippets = new Map([
  ['empty', '(module)'],
  [
    'messy',
    `(module
  (func      $f
    block    $b
    end     nop


    nop     nop
  )
  (global     (mut   i32) (i32.const     0))
)`,
  ],
  [
    'cf',
    `(module
  (global i32
    i32.const 0)
  (func (param (ref any)) (result (ref any)) (local (ref any))
    block $b
      loop $loop
        global.get 0
        if
          local.get 0
          local.set 1
          br $b
        else
          br $loop
        end
      end
    end
    local.get 1)
  (func (param (ref any)) (result (ref any)) (local (ref any))
    (block $b
      (loop $loop
        (if
          (global.get 0)
          (then
            (br $b)
            (local.set 1
              (local.get 0)))
          (else
            (local.set 1
              (local.get 0))
            (br $loop)))))
    (local.get 1))
  (func (param (ref any)) (result (ref any)) (local (ref any))
    (block $b
      (loop $loop
        (if
          (global.get 0)
          (then
            (br $loop
              (local.set 1
                (local.get 0))))
          (else
            (br $b)))))
    (local.get 1))
  (func (local (ref any))
    (loop
      br 0
      local.get 0
      drop)))`,
  ],
  [
    'mutability',
    `(module
  (global i32
    i32.const 0)
  (global (mut i32)
    i32.const 0)
  (func
    (global.set 0
      (i32.const 0))))`,
  ],
  [
    'type-check',
    `(module
  (rec (type $f1 (sub (func))) (type (struct (field (ref $f1)))))
  (rec (type $f2 (sub (func))) (type (struct (field (ref $f1)))))
  (rec (type $g1 (sub $f1 (func))) (type (struct)))
  (rec (type $g2 (sub $f2 (func))) (type (struct)))
  (func (param (ref $g2)) (result (ref $g1))
    (local.get 0)))`,
  ],
])
