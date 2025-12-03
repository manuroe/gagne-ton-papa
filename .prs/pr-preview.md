# PR Description Template

## Title
Add PR Preview Deployments + QR code in PR comment

## Body
This PR adds a new GitHub Actions workflow `/.github/workflows/pr-preview.yml` that:
- Builds the web app and WASM for each pull request.
- Deploys the build to GitHub Pages under a PR-specific subdirectory: `pr-<PR_NUMBER>`.
- Posts a comment on the PR including the preview URL and an embedded QR code.

Implementation notes:
- Reuses existing setup from `deploy.yml` (Node cache, Rust cache, composite action `./.github/actions/build-wasm`).
- Generates `preview-qr.png` with `npx qrcode` into `web/build` so it is published alongside the preview.
- Uses `peaceiris/actions-gh-pages@v3` to publish to `gh-pages`.
- Uses `actions/github-script@v7` to comment the URL and QR.

Resulting preview URL format:
- `https://manuroe.github.io/gagne-ton-papa/pr-<PR_NUMBER>/`
- QR image: `https://manuroe.github.io/gagne-ton-papa/pr-<PR_NUMBER>/preview-qr.png`

## Prompts Used
- "Is there a way to experiment the web app from a PR? I'd like to have a temporary deployment for it"
- "Start implementation"
- "can you post a QR code too?"
- "yes, create a PR for it"
