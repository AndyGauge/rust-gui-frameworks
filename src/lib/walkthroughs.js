// Page walkthroughs are markdown files in /content, one per entry id.
// Prose is plain markdown. Directives on their own line:
//   ::code path/under/snippets            a compiled snippet (see /snippets)
//   ::code path | label=Shown label | output=no
//   ::inline lang | label=Shown label     an uncompiled snippet, closed by ::end
const files = import.meta.glob('/content/*.md', { query: '?raw', import: 'default', eager: true });

function opts(rest) {
  const o = {};
  for (const seg of rest.split('|').slice(1)) {
    const [k, ...v] = seg.split('=');
    o[k.trim()] = v.join('=').trim();
  }
  return o;
}

export function walkFor(id) {
  const raw = files[`/content/${id}.md`];
  if (!raw) return [];
  const parts = [];
  let buf = [];
  let inline = null;
  const flush = () => {
    const t = buf.join('\n').trim();
    if (t) parts.push({ text: t });
    buf = [];
  };
  for (const line of raw.split('\n')) {
    if (inline) {
      if (line.startsWith('::end')) {
        parts.push({ inline: inline.lines.join('\n'), lang: inline.lang, label: inline.label });
        inline = null;
      } else inline.lines.push(line);
    } else if (line.startsWith('::code ')) {
      flush();
      const path = line.slice(7).split('|')[0].trim();
      const o = opts(line);
      parts.push({ code: path, label: o.label, output: o.output === 'no' ? false : undefined });
    } else if (line.startsWith('::inline ')) {
      flush();
      const o = opts(line);
      inline = { lang: line.slice(9).split('|')[0].trim(), label: o.label ?? '', lines: [] };
    } else buf.push(line);
  }
  flush();
  return parts;
}
