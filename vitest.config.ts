import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    globals: true,
    environment: "jsdom",
    setupFiles: ["./tests/setup.ts"],
    coverage: {
      provider: "v8",
      reporter: ["text", "json", "html"],
      exclude: [
        "node_modules/",
        "tests/",
        "src-tauri/",
        "*.config.ts",
        "*.config.js",
      ],
    },
    include: ["tests/**/*.test.ts"],
    exclude: ["node_modules", "src-tauri"],
  },
});
