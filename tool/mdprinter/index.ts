import it from 'markdown-it';
import anchor from 'markdown-it-anchor';

// @ts-ignore
import katex from 'markdown-it-katex';

const md = it({
  linkify: true,
  typographer: true,
})
  .use(anchor)
  .use(katex);

const file = await Bun.file(Bun.argv[2]).text();

const template = await Bun.file(`${import.meta.dir}/template.html`).text();
const gfm = await Bun.file(`${import.meta.dir}/gfm.css`).text();

const res = md
  .render(file)
  .replaceAll('.md"', '.html"')
  .replaceAll('.md#', '.html#');

const html = template
  .replace('{{CONTENT}}', res)
  .replace('{{GFM_STYLE}}', `<style>${gfm}</style>`);

await Bun.write(Bun.file(Bun.argv[3]), html);
