# What problem are we solving?

`nosqo` has a Rust backend and a reserved `ui/` directory for a browser UI, but there is no actual frontend workspace yet.
That blocks any serious work on query exploration, statement browsing, or basic product feedback loops from a browser.

We need a minimal but production-credible frontend foundation that is fast to iterate on, easy to test, and boring in the right ways.

# What is the current gap?

The repository currently does not include:

- a Node-based frontend workspace
- a package manager configuration
- a Vite development and build setup
- a React application shell
- client-side routing
- frontend test infrastructure
- a defined browser-to-server development workflow

The repository should use `ui/` consistently as the browser UI home.
Creating multiple frontend roots would be needless churn.

# What should the frontend foundation include?

The requested stack is a good baseline:

- Node.js
- `pnpm`
- Vite
- Vitest
- React
- TanStack Router

That is not quite enough on its own.
The initial foundation should also include:

- TypeScript
- `oxlint`
- `oxfmt`
- React Testing Library for component and route-level tests
- `happy-dom` for browser-like Vitest execution
- a small API client boundary for talking to the Axum server
- environment handling for API base URLs
- a Vite dev proxy or explicit backend CORS configuration for local development
- a simple error boundary and not-found route

Nice-to-have but deferrable:

- Playwright for end-to-end smoke tests
- TanStack Query if and when server state becomes non-trivial
- a component system or design system
- Storybook

# What architecture should v1 use?

Keep the first version small and explicit.
Do not overengineer a frontend architecture before there is real UI complexity.

Recommended shape:

- `ui/` as a standalone Node project, not mixed into Cargo
- React with TypeScript
- TanStack Router with file-based or code-based routes, whichever keeps the route tree obvious
- a small `src/lib/api` area for HTTP calls and DTO mapping
- route-oriented feature slices for pages such as query explorer, statement browser, and home
- colocated tests alongside components and route modules

Avoid adding global state management on day one.
Router state, local component state, and direct data loading are enough until proven otherwise.

# What should the initial user experience cover?

The first UI should prove the stack and the backend integration, not chase polish.

Recommended initial routes:

- `/` with a lightweight project landing page
- `/query` for submitting NQL and viewing tabular results
- `/statements` for basic statement browsing against the existing API

Each route should handle:

- loading state
- empty state
- backend error state
- obvious navigation back to the other entry points

If the first shipped screen cannot successfully hit the backend and render a realistic response, the scaffold is not done yet.

# How should local development work?

The development loop should be simple:

1. run the Rust server locally
2. run the Vite dev server locally
3. have browser requests to the API work without manual URL rewriting

The simplest correct setup is usually:

- Vite dev server on a frontend port such as `5173`
- Axum server on its existing local port
- Vite proxying `/api` requests to the backend in development

This avoids premature CORS yak-shaving during the first phase.
If production deployment later requires separate origins, CORS can be added intentionally instead of accidentally.

# What should be tested?

Frontend work should be tested automatically, even at bootstrap time.

At minimum, cover:

- app shell rendering
- route navigation for the initial pages
- query page success state with mocked HTTP responses
- query page error state
- statement page empty-state rendering
- one smoke test that the router mounts without crashing

If end-to-end coverage is included in this phase, keep it to one or two smoke tests.
Do not let test infrastructure become the project.

# What implementation order is recommended?

- [ ] Keep `ui/` as the single documented frontend home
- [ ] Update repository documentation so `ui/` is the single documented frontend home
- [ ] Add Node and `pnpm` version declarations for reproducible local setup
- [ ] Initialize the frontend project with Vite, React, and TypeScript
- [ ] Add TanStack Router and create the initial route tree
- [ ] Add Vitest, `happy-dom`, and React Testing Library
- [ ] Add `oxlint` and `oxfmt`
- [ ] Add a development proxy or equivalent local API wiring
- [ ] Implement the initial app shell with navigation, not-found handling, and error boundaries
- [ ] Implement `/query` against `POST /api/v1/query`
- [ ] Implement `/statements` against the existing statements endpoint
- [ ] Add colocated tests for routing and the initial screens
- [ ] Add frontend package scripts for `dev`, `build`, `test`, and `lint`
- [ ] Decide whether frontend verification should be folded into `scripts/check-code.sh` now or in a follow-up
- [ ] Run the relevant frontend checks
- [ ] Run `./scripts/check-code.sh` after the repository-level integration is in place

# What assumptions and risks should stay explicit?

- The repository now standardizes on `ui/` as the frontend directory.
- The backend API is still early. The first UI should expect some endpoint churn and keep the API client layer thin.
- TanStack Query is intentionally not part of the first bootstrap unless data-fetching complexity appears quickly.
- A component system is intentionally deferred. That is the right call for now.
- If frontend tooling is added without integrating it into repository-wide checks, it will rot fast. That integration should happen early, even if it lands in a follow-up patch.

# What should be decided before implementation starts?

Decide these upfront so the bootstrap does not drift:

- what Node and `pnpm` versions should the repo standardize on?
- should the initial TanStack Router setup use file-based routes or explicit code-based routes?
- should frontend checks be required in `scripts/check-code.sh` immediately?

# What is the simplest recommendation?

Use:

- `ui/`
- Node LTS
- `pnpm`
- Vite + React + TypeScript
- TanStack Router
- Vitest + React Testing Library + `happy-dom`
- `oxlint` + `oxfmt`
- Vite proxying `/api` to the Rust server in development

That gives you a clean v1 without importing a pile of framework fashion.
