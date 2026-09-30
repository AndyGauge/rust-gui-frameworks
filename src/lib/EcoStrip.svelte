<script>
  import eco from './eco.json';

  let { crates = [] } = $props();
  const rows = $derived(crates.map((c) => ({ name: c, ...eco.crates[c] })).filter((r) => r.ver));
  const fmt = (n) => n?.toLocaleString('en-US');
</script>

{#if rows.length}
  <aside class="eco" aria-label="Ecosystem facts from crates.io">
    <div class="head">In the ecosystem <span>crates.io, {eco.asOf}</span></div>
    {#each rows as r}
      <dl>
        <dt><code>{r.name}</code> <span class="v">{r.ver}</span></dt>
        <dd>
          <strong>{fmt(r.revdeps)}</strong> crates depend on it
          {#if r.usedBy.length}· used directly by <em>{r.usedBy.join(', ')}</em>{/if}
        </dd>
        {#if r.deps.length}
          <dd class="deps">builds on <em>{r.deps.join(', ')}</em></dd>
        {/if}
      </dl>
    {/each}
  </aside>
{/if}

<style>
  .eco { grid-column: 2; max-width: 74ch; margin-top: 2rem; border-top: 2px solid var(--accent); padding-top: 0.9rem; font-family: var(--sans); font-size: 0.78rem; color: var(--ink); }
  .head { text-transform: uppercase; letter-spacing: 0.22em; font-size: 0.65rem; color: var(--muted); margin-bottom: 0.6rem; display: flex; justify-content: space-between; gap: 1rem; }
  .head span { letter-spacing: 0.06em; text-transform: none; }
  dl { margin: 0 0 0.7rem; }
  dt { font-weight: 600; }
  dt code { font-family: 'JetBrains Mono', ui-monospace, monospace; font-size: 0.95em; }
  .v { font-weight: 400; color: var(--muted); font-variant-numeric: tabular-nums; }
  dd { margin: 0.1rem 0 0; line-height: 1.45; }
  dd.deps { color: var(--muted); }
  em { font-style: normal; }
  @media (max-width: 720px) { .eco { grid-column: 1; } }
</style>
