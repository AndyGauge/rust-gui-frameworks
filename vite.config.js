import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [sveltekit()],
  server: {
    // snippets/ and content/ hold the compiled code and page prose that the
    // book imports as raw text; the dev server must be allowed to serve them.
    fs: { allow: ['snippets', 'content'] }
  }
});
