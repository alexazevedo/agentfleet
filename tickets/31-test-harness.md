---
id: 31
status: doing
model: haiku
depends_on: [30]
max_attempts: 4
files:
  - package.json
  - package-lock.json
  - vite.config.ts
  - src/lib/theme.test.ts
---
## Goal
`npm test` runs Vitest with jsdom and Svelte 5 component support, proven by one test of `resolveTheme`.

## Context
No test runner exists. Vite 7, Svelte 5, Node 26. Tauri's `invoke` is imported from `@tauri-apps/api/core` in stores; component tests in later tickets will `vi.mock` it.

- Dev deps: `vitest`, `jsdom`, `@testing-library/svelte`, `@testing-library/jest-dom`, `@testing-library/user-event` (latest, `npm i -D`).
- `package.json` scripts: `"test": "vitest run"`, `"test:watch": "vitest"`.
- `vite.config.ts`: add `import { svelteTesting } from "@testing-library/svelte/vite";` to `plugins`, and
  ```ts
  test: { environment: "jsdom", include: ["src/**/*.test.ts"], setupFiles: ["./src/test-setup.ts"] }
  ```
  `src/test-setup.ts`: `import "@testing-library/jest-dom/vitest";`. Add `/// <reference types="vitest/config" />` at the top of vite.config.ts so `defineConfig` accepts `test`.
- `tsconfig.json` `include` already covers `src/**/*.ts`; add `"types": ["vitest/globals"]` only if you enable `globals` (don't; import `describe/it/expect` explicitly).
- `src/lib/theme.test.ts`: `resolveTheme("system", true) === "dark"`, `("system", false) === "light"`, `("light", true) === "light"`, `("dark", false) === "dark"`.

## Checks (run in order — cheapest first)
1. `npm run check`
2. `npm test`
3. `npm run build`

## Done when
- `npm test` prints 1 file, 4 tests passed. `npm run build` unaffected (test files excluded from the bundle).

## Not included
- Component tests (written with their components), coverage config, CI changes.
