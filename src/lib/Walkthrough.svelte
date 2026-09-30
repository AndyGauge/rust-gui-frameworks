<script>
  import { md, mdBlock } from 'sveltekitbook/md';
  import { load, highlight, checked } from './snippets.js';

  let { parts = [] } = $props();

  // The markdown helper leaves `backticks` alone, so turn them into <code> here.
  const inlineCode = (html) => html.replace(/`([^`\n]+)`/g, '<code>$1</code>');

  // A part is { text } or { code: 'path' } or { inline, lang, label } (illustrative).
  const resolved = $derived(
    parts.map((p) => {
      if (p.text !== undefined) return { kind: 'text', html: inlineCode(mdBlock(p.text, {})) };
      if (p.code) {
        const s = load(p.code);
        return {
          kind: 'code', label: p.label ?? s.label, verified: s.verified, deps: s.deps, rustc: s.rustc,
          html: highlight(s.code, p.lang ?? s.lang), output: p.output === false ? null : s.output, lang: s.lang
        };
      }
      return {
        kind: 'code', label: p.label ?? '', verified: false, deps: '', rustc: '',
        html: highlight(p.inline, p.lang ?? 'plaintext'), output: p.output ?? null, lang: p.lang
      };
    })
  );
</script>

<section class="walk">
  {#each resolved as p}
    {#if p.kind === 'text'}
      <div class="wtext">{@html p.html}</div>
    {:else}
      <figure class="wcode">
        <figcaption>
          <span class="file">{p.label}</span>
          {#if p.verified}
            <span class="badge ok" title="Compiled by snippets/verify.py on {checked}">✔ compiled{p.deps ? ' · ' + p.deps : ''} · rustc {p.rustc}</span>
          {:else if p.verified === false}
            <span class="badge" title="Not compiled as part of this book">illustrative</span>
          {/if}
        </figcaption>
        <pre><code class="hljs">{@html p.html}</code></pre>
        {#if p.output}
          <div class="out-label">output</div>
          <pre class="out"><code>{p.output}</code></pre>
        {/if}
      </figure>
    {/if}
  {/each}
</section>

<style>
  .walk { grid-column: 2; max-width: 74ch; margin-top: 2.2rem; display: flex; flex-direction: column; gap: 1.4rem; }
  .wtext { font-family: var(--serif); font-weight: 300; font-size: clamp(0.95rem, 1.05vw, 1.05rem); line-height: 1.6; color: var(--ink); max-width: 60ch; padding-left: 1.3rem; }
  .wtext :global(p) { margin: 0 0 0.8em; }
  .wtext :global(p:last-child) { margin-bottom: 0; }
  .wtext :global(code) { font-family: 'JetBrains Mono', ui-monospace, monospace; font-size: 0.86em; background: rgba(20, 17, 13, 0.06); padding: 0 0.3em; border-radius: 3px; }

  .wcode { margin: 0; border: 1px solid var(--rule); border-radius: 6px; overflow: hidden; background: rgba(255, 255, 255, 0.55); }
  figcaption { display: flex; justify-content: space-between; gap: 1rem; align-items: center; flex-wrap: wrap; padding: 0.45rem 0.8rem; border-bottom: 1px solid var(--rule); font-family: var(--sans); font-size: 0.68rem; letter-spacing: 0.06em; color: var(--muted); background: rgba(20, 17, 13, 0.035); }
  .file { font-family: 'JetBrains Mono', ui-monospace, monospace; letter-spacing: 0; }
  .badge { text-transform: uppercase; letter-spacing: 0.12em; font-size: 0.6rem; border: 1px solid currentColor; border-radius: 999px; padding: 0.05rem 0.5rem; }
  .badge.ok { color: #1e7a4d; }
  pre { margin: 0; padding: 0.9rem 1rem; overflow-x: auto; font-family: 'JetBrains Mono', ui-monospace, Menlo, monospace; font-size: 0.78rem; line-height: 1.5; tab-size: 4; }
  pre code { font-family: inherit; background: none; padding: 0; white-space: pre; }
  .out-label { padding: 0.25rem 1rem 0; font-family: var(--sans); font-size: 0.6rem; text-transform: uppercase; letter-spacing: 0.2em; color: var(--muted); border-top: 1px dashed var(--rule); }
  pre.out { padding-top: 0.3rem; color: var(--muted); }

  /* highlight.js tokens, muted to sit on the paper background */
  .wcode :global(.hljs-keyword), .wcode :global(.hljs-built_in), .wcode :global(.hljs-literal) { color: #8a3b8f; }
  .wcode :global(.hljs-title), .wcode :global(.hljs-title.function_), .wcode :global(.hljs-attr), .wcode :global(.hljs-section) { color: #1f5fa8; }
  .wcode :global(.hljs-type), .wcode :global(.hljs-title.class_) { color: #a8651f; }
  .wcode :global(.hljs-string), .wcode :global(.hljs-regexp) { color: #2b7a3f; }
  .wcode :global(.hljs-number), .wcode :global(.hljs-symbol) { color: #b5541c; }
  .wcode :global(.hljs-comment), .wcode :global(.hljs-doctag) { color: rgba(20, 17, 13, 0.5); font-style: italic; }
  .wcode :global(.hljs-meta), .wcode :global(.hljs-attribute) { color: #6a6a6a; }
  .wcode :global(.hljs-addition) { color: #1e7a4d; background: rgba(48, 164, 108, 0.12); }
  .wcode :global(.hljs-deletion) { color: #b03034; background: rgba(229, 72, 77, 0.1); }

  @media (max-width: 720px) { .walk { grid-column: 1; } .wtext { padding-left: 0; } }
</style>
