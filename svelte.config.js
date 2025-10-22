import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

export default {
  preprocess: vitePreprocess(),
  compilerOptions: {
    // Enable Svelte 5 compatibility mode for components
    compatibility: {
      componentApi: 5
    }
  }
};
