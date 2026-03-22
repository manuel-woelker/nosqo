# What is this document for?

This document describes the browser UI stack for `nosqo` and the design guidelines that should shape future UI work.
Use it when adding screens, components, styles, frontend tooling, or frontend documentation in `ui/`.

# Where does the browser UI live?

The browser UI lives in `ui/`.
Do not create additional frontend roots unless there is a very strong reason.

# What is the current UI stack?

The current UI stack is:

- React 19
- TypeScript
- Vite
- TanStack Router
- Zustand
- Immer
- Vitest
- React Testing Library
- `happy-dom` for DOM-oriented tests
- `oxlint` for linting
- `oxfmt` for formatting

The UI currently talks to the backend through a thin `fetch`-based API client in `ui/src/lib/api-client.ts`.
In local development, Vite proxies `/api` requests to `http://127.0.0.1:3000`.
If a different backend origin is needed, set `VITE_API_BASE_URL`.

# How should UI code be organized?

Keep the architecture boring and obvious.
This UI is early, and overengineering it now would be a mistake.

Prefer these boundaries:

- pages and route components handle page composition, route-specific concerns, and user flows
- common UI building blocks live in `src/common/components/`
- use-case-specific code lives in `src/usecases/`
- cross-cutting technical concerns live in `src/infrastructure/`
- styles stay centralized and shared until there is enough complexity to justify splitting them

Prefer small files with one clear responsibility.
If a component becomes hard to scan, split it before adding abstraction layers.

# What source directory structure should be preferred?

Prefer a use-case-driven layout instead of grouping most code by technical type.
That keeps related UI, state, and interaction code close to the workflow it serves.

Use this structure as the default:

- `src/common/components/` for shared UI primitives and common components such as buttons, inputs, dialogs, tables, and layout helpers
- `src/usecases/` for product workflows and feature areas
- `src/usecases/administration/ontology/` for administration and ontology-specific screens, state, and support code
- `src/infrastructure/` for error handling, connectivity, API plumbing, configuration, and other cross-cutting technical concerns

Use-case directories should own the code for their workflow, including:

- route components
- feature-specific stores
- feature-specific view models or helper modules
- tests that are specific to that workflow

Do not dump everything into a giant shared components folder.
If code mainly exists for one workflow, keep it with that workflow.

# How should UI components depend on a component library?

Use a wrapper or abstraction layer around core UI components.
The application should not depend directly on the implementation details of a third-party component library.

Core shared components such as buttons, inputs, selects, dialogs, tables, and similar building blocks should be exposed through `src/common/components/`.
That layer should define the interface the rest of the app uses.
The current implementation is bespoke React components and plain CSS, but feature code should still treat the underlying implementation as an implementation detail.

Use these guidelines:

- wrap third-party primitives instead of scattering direct library imports throughout feature code
- keep the wrapper API aligned with product needs, not the library's quirks
- centralize styling, accessibility fixes, and behavior conventions in the wrapper layer
- make it possible to replace or evolve the underlying component library without rewriting the whole app

Feature code should depend on the `nosqo` component layer, not on whichever UI library happens to be underneath it.
Otherwise the library choice leaks everywhere and migration becomes a slow-motion mess.

# How should routes and navigation work?

All pages should have usable and navigable routes.
If a screen matters enough to exist, it should usually be reachable through the router in a way a user can understand and navigate to again.

Use these guidelines:

- define a concrete route for each real page-level workflow
- keep route paths readable and stable
- ensure navigation between important pages is visible in the UI
- make it possible to deep-link to useful screens when practical
- avoid burying core workflows behind transient UI state only

Do not build page-shaped components that are effectively unreachable.
That is dead product surface, and it gets stale fast.

# How should data fetching and state be handled?

Use Zustand and Immer as the default client-state tools.
That should be the standard path for UI state that needs to live beyond a single local input or callback.

Use these guidelines:

- keep request logic behind small API helper functions
- use Zustand stores for page and application state that benefits from a clear shared model
- use Immer to keep store updates readable and safe as state shape grows
- keep loading, error, and success states explicit in the UI
- avoid scattering the same state logic across multiple components
- avoid adding a separate server-state library until caching, invalidation, or background refresh becomes a real need

Local React state is still fine for truly local transient concerns.
Do not force everything into a store when `useState` is enough.
The default is Zustand plus Immer, not state-management cosplay.

# How should the UI be designed?

The UI should feel direct, legible, and useful.
It should help users work with knowledge, not distract them with decorative chrome.

Design principles:

- prioritize task clarity over visual novelty
- make important actions and states obvious
- keep pages calm and scannable
- prefer smaller, tighter layouts over oversized spacing and decorative breathing room
- treat empty, loading, and error states as first-class UI
- prefer real workflows over placeholder surfaces

The current visual language is intentionally strong but restrained:

- dark, high-contrast panels over a gradient background
- IBM Plex Sans for interface copy and IBM Plex Mono for code-like content
- rounded panels and controls
- a restrained accent color for interactive emphasis
- navigation should feel integrated into the shell, not like a stack of bordered boxes

New screens should extend this visual language instead of introducing unrelated styles.
If the design system changes later, change it deliberately across the app rather than screen by screen.

# How should styling be handled?

Use plain CSS in `ui/src/styles.css` unless there is a clear reason to do otherwise.
The current styling approach is simple, fast, and easy to inspect.

Prefer these styling rules:

- reuse existing tokens and utility classes before adding new one-off styles
- use shared semantic class names such as `panel`, `stack`, `field`, and `empty-state`
- keep spacing, border, and color decisions consistent with the existing tokens
- default to tighter spacing and smaller layout rhythms unless the content clearly needs more room
- do not render primary navigation as bordered cards or boxed tiles
- prefer composition through a small number of readable class names over deeply nested selectors
- design mobile-first and then expand layouts with media queries

Do not introduce CSS-in-JS, utility-first frameworks, or a component styling library without a concrete need.
That would add churn without solving a current problem.

# What accessibility and UX expectations should apply?

Accessibility is part of the baseline quality bar.
The current UI already uses semantic forms, labels, headings, and screen-reader-only text, and new work should keep that standard.

At minimum:

- use semantic HTML before reaching for ARIA
- label every form control
- ensure keyboard access for interactive elements
- expose loading and error states clearly
- keep copy plain and specific
- preserve contrast and readable text sizing
- make empty states informative rather than dead ends

If a user can make a mistake, the UI should help them recover instead of silently failing.

# How should UI tests be written?

Follow the repository testing strategy in `docs/TESTING.md`.
Prefer small component and page tests over large full-app integration tests.

In practice:

- test route components directly when router behavior is not under test
- use `happy-dom` only for tests that actually need DOM APIs
- stub `fetch` at the test boundary for UI request flows
- verify user-visible outcomes such as rendered data, empty states, and error messages

Keep UI tests focused on behavior that users can observe.
Avoid brittle tests that overfit implementation details.

# What should the development workflow be?

Use the `ui/` workspace directly for frontend work.

Common commands:

```bash
pnpm install
pnpm dev
pnpm check
```

`pnpm check` runs formatting, linting, type-checking, tests, and a production build.
When completing a broader unit of work for the repository, also run `nao check`.

# What should be avoided?

Avoid these common mistakes:

- adding abstractions before repeated complexity appears
- creating bespoke visual patterns for every page
- hiding backend errors behind vague frontend copy
- building placeholder UI that does not exercise real backend workflows
- adding frontend tooling that is not clearly buying its keep
- creating page-level UI without a real route and navigation path

The right default for this codebase is simple, explicit, and testable.
