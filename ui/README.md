# ui

This directory is reserved for the browser-based `nosqo` UI.

The UI is built with:

- React
- TanStack Router
- Vite
- Vitest
- happy-dom
- oxlint
- oxfmt

## Development

Install dependencies:

```bash
pnpm install
```

Run the dev server:

```bash
pnpm dev
```

Set `VITE_API_BASE_URL` if the UI should talk to a backend origin other than the local Vite proxy.

Run the UI checks:

```bash
pnpm check
```
