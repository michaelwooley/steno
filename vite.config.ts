import { sveltekit } from "@sveltejs/kit/vite";
import type { UserConfig } from "vite";
import { configDefaults } from "vitest/config";

const config: UserConfig = {
  plugins: [sveltekit()],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
  define: {
    // Eliminate in-source test code
    "import.meta.vitest": "undefined",
  },
  build: {
    // Tauri supports es2021
    target: ["es2021", "chrome100", "safari13"],
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  optimizeDeps: {
    exclude: [
      // "codemirror",
      // "@codemirror/lang-markdown",
      // "@codemirror/language-data",
      // "@codemirror/theme-one-dark",
      // "@codemirror/language",
      // "@codemirror/view",
    ],
  },
  test: {
    // jest like globals
    globals: true,
    environment: "jsdom",
    // in-source testing
    includeSource: ["src/**/*.{js,ts,svelte}"],
    // Add @testing-library/jest-dom matchers & setup MSW
    setupFiles: ["./setupTest.js", "./src/mocks/setup.ts"],
    // Exclude files in c8
    coverage: {
      exclude: ["setupTest.js", "src/mocks"],
      provider: "c8",
      reporter: ["cobertura", "text", "json"],
    },
    deps: {
      // Put Svelte component here, e.g., inline: [/svelte-multiselect/, /msw/]
      inline: [/msw/],
    },
    // Exclude playwright tests folder
    exclude: [...configDefaults.exclude, "tests"],
  },
};

export default config;
