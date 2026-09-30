// Every code block in the book is a real file under /snippets that was compiled
// by snippets/verify.py. This module loads them as raw text at build time.
import hljs from 'highlight.js/lib/core';
import rust from 'highlight.js/lib/languages/rust';
import ini from 'highlight.js/lib/languages/ini';
import json from 'highlight.js/lib/languages/json';
import javascript from 'highlight.js/lib/languages/javascript';
import bash from 'highlight.js/lib/languages/bash';
import diff from 'highlight.js/lib/languages/diff';
import meta from '../../snippets/meta.json';

hljs.registerLanguage('rust', rust);
hljs.registerLanguage('toml', ini);
hljs.registerLanguage('json', json);
hljs.registerLanguage('javascript', javascript);
hljs.registerLanguage('bash', bash);
hljs.registerLanguage('diff', diff);

const files = import.meta.glob(
  [
    '/snippets/*/src/**/*.rs',
    '/snippets/*/build.rs',
    '/snippets/*/Cargo.toml',
    '/snippets/*/ui/*.slint',
    '/snippets/*/capabilities/*.json',
    '/snippets/*/tauri.conf.json',
    '/snippets/extra/*'
  ],
  { query: '?raw', import: 'default', eager: true }
);

const LANG = { rs: 'rust', toml: 'toml', json: 'json', js: 'javascript', sh: 'bash', txt: 'plaintext', slint: 'plaintext', diff: 'diff' };

export function highlight(code, lang) {
  if (lang && lang !== 'plaintext' && hljs.getLanguage(lang)) {
    return hljs.highlight(code, { language: lang, ignoreIllegals: true }).value;
  }
  return code.replace(/[&<>]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;' })[c]);
}

/** Load a compiled snippet by its path under /snippets, e.g. "winit-window/src/bin/one.rs". */
export function load(path) {
  const code = files['/snippets/' + path];
  if (code === undefined) throw new Error('Missing snippet: ' + path);
  const member = path.split('/')[0];
  const ext = path.split('.').pop();
  const pkg = meta.packages[member];
  const bin = /src\/bin\/(\w+)\.rs$/.exec(path)?.[1];
  const deps = pkg ? Object.entries(pkg.deps).slice(0, 2).map(([n, v]) => `${n} ${v}`).join(', ') : '';
  return {
    code: code.trimEnd(),
    lang: LANG[ext] ?? 'plaintext',
    label: path,
    verified: member === 'extra' ? null : !!pkg?.ok,
    deps,
    rustc: meta.rustc.replace('rustc ', '').split(' ')[0],
    output: bin && pkg?.output?.[bin] ? pkg.output[bin] : null
  };
}

export const checked = meta.checked;
