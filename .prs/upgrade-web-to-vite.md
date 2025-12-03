# Title
Migrate web app to Vite, upgrade deps, and fix CI

# Body
This PR modernizes the `web/` app tooling and dependencies to eliminate `--legacy-peer-deps` and align with current React/TypeScript ecosystem. It also updates CI to deploy Vite’s output.

## Changes
- Replace Create React App (`react-scripts`) with Vite
  - Add `web/vite.config.ts` with dev server `fs.allow` for `../lib-wasm/pkg`
  - Create Vite entry `web/index.html`
  - Update TypeScript env types to `vite/client`
- Upgrade dependencies
  - React `^18.3.1`, TypeScript `^5.4.5`, Testing Library packages, Web Vitals
  - Add dev deps: `vite`, `@vitejs/plugin-react`, `vitest`, `jsdom`, `eslint`
  - Update scripts: `start`, `build`, `preview`, `test`
- CI
  - Remove `--legacy-peer-deps` from `npm ci`
  - Change publish dir to `./web/dist`
- Repo hygiene
  - Add `web/dist/` to `.gitignore`
- Tests
  - Add Vitest configuration via Vite config
  - Add `URL.createObjectURL` polyfill in `web/src/setupTests.ts`
  - Update `web/src/App.test.tsx` to avoid WASM in unit tests
  - Add `web/src/App.interaction.test.tsx` to simulate piece selection and assert missing cells UI

## Rationale
- `i18next` and `react-i18next` introduce a peer dependency on TypeScript `^5`. CRA (`react-scripts@5`) prefers TS `^3.2.1 || ^4`, causing frequent peer resolution conflicts. Migrating to Vite removes those constraints, speeds up dev/startup, and simplifies config.
- Vite’s dev server requires explicit allow-listing to serve files outside `web/`; WASM artifacts live in `../lib-wasm/pkg`, hence `server.fs.allow`.

## Verification
- `npm install` and `npm run build` succeed; output is in `web/dist`.
- Vitest passes (2 tests): `App.test.tsx` and `App.interaction.test.tsx`.

## Notes
- If preferred, we can alternatively copy `lib-wasm_bg.wasm` under `web/public/` instead of relaxing `server.fs.allow`.

# Prompts Used
- "npm dependency seems to be a mess in `web/`. I want to use the latest versions of the components. We shoud not have to use `--legacy-peer-deps`. Can you fix this?"
- "What is vite, compared to react?"
- "why I did not need it before?"
- "should we gitignore web/dist?"
- "Start implementation"
- "yes"
- "I have this error \"The request url \"/Users/manu-ai/dev/gagne-ton-papa/lib-wasm/pkg/lib_wasm_bg.wasm\" is outside of Vite serving allow list.\""
- "do it"
- "PR it"
