# What problem are we solving?

The UI now has working routes for the query explorer and statement browser, but it still behaves like a bootstrap app instead of a product shell.
There is no persistent information architecture that helps users understand where they are, how to move between workflows, or how future screens should fit together.

We need a basic application shell that establishes a durable layout:

- a header bar at the top
- breadcrumbs in the header
- a left-hand navigation area
- a main content area in the center

The first navigation group should be `Administration`, with links to the existing query explorer and statement browser.

# What is the current gap?

The current UI has top-level navigation in the page header, but it does not yet provide:

- a stable app-shell layout
- breadcrumbs for location awareness
- a dedicated left navigation region
- grouped navigation that can grow with the product
- a clearer distinction between shell chrome and page content

That is fine for a bootstrap, but it will get messy fast once more screens are added.

# What should the basic UI include?

The first pass should establish the structure, not chase pixel-perfect polish.

Recommended shell:

- a top header bar that is always visible inside the app shell
- breadcrumb rendering in the header for the current route
- a left navigation column with grouped links
- a central main-content region for route content
- responsive behavior that keeps the layout usable on smaller screens

Recommended initial navigation tree:

- `Administration`
- `Administration / Query Explorer`
- `Administration / Statement Browser`

The existing query explorer and statement browser should move into this shell without changing their core behavior.

# How should routing and navigation behave?

The shell should be route-driven, not manually synchronized UI state.
Breadcrumbs and left navigation should be derived from the route structure or route metadata.

Use these guidelines:

- every real page should have a usable route
- the shell should highlight the current navigation item
- breadcrumbs should reflect the actual route hierarchy the user is in
- route labels should be readable product language, not internal code names
- the layout should make it obvious how to move back to sibling pages

Do not hardcode one-off breadcrumb strings in page components if the router can own that information.

# What architecture should this use?

Keep the shell simple and aligned with the documented UI architecture.

Recommended structure:

- shell layout components in `ui/src/common/components/`
- route and use-case code under `ui/src/usecases/`
- cross-cutting route metadata or navigation helpers in `ui/src/infrastructure/`
- Mantine UI wired in as the current underlying component library implementation

The shell should also respect the documented component abstraction rule:

- feature code should depend on shared `nosqo` UI components
- direct dependency on the underlying component library should stay behind the wrapper layer where practical

If a small amount of direct Mantine usage is temporarily needed while the wrapper layer is still thin, keep it localized and treat it as transitional.

# What visual and UX rules should guide the first pass?

The goal is a usable shell, not a giant redesign.
Keep the current visual language, but make the layout feel more product-like and navigable.

The first implementation should:

- preserve clear hierarchy between shell chrome and content
- make breadcrumbs easy to scan
- make left navigation stable and unsurprising
- keep the main content area focused on the active workflow
- avoid squeezing content into a layout that only works on wide screens

If the shell makes the query explorer or statement browser harder to use, the layout is wrong.

# What should be tested?

Add focused tests around the shell behavior rather than only the page internals.

At minimum, cover:

- app-shell rendering with header, breadcrumbs, navigation, and main content
- active navigation state for the current route
- breadcrumb rendering for the query explorer route
- breadcrumb rendering for the statement browser route
- one responsive smoke check if the implementation includes layout-specific branching

Existing page behavior tests for query and statement screens should continue to pass.

# What implementation order is recommended?

- [ ] Define the intended route labels and breadcrumb labels for the initial Administration section
- [ ] Wire Mantine into the app entry point with the required provider and base styles
- [ ] Introduce shared shell components for header, breadcrumbs, left navigation, and main layout regions
- [ ] Move shell-oriented primitives into `ui/src/common/components/` as needed
- [ ] Build the first shared shell primitives on top of the Mantine-backed wrapper layer in `ui/src/common/components/`
- [ ] Add route metadata or a small navigation model that drives breadcrumbs and left navigation
- [ ] Rework the root layout so it renders a top header, left navigation, and central content region
- [ ] Add an `Administration` navigation group with links to Query Explorer and Statement Browser
- [ ] Update the existing query explorer route to render correctly inside the new shell
- [ ] Update the existing statement browser route to render correctly inside the new shell
- [ ] Ensure the shell remains usable on smaller screens
- [ ] Add or update colocated UI tests for shell rendering, breadcrumbs, and navigation state
- [ ] Run `pnpm check` in `ui/`
- [ ] Run `nao check` if repository-level verification includes the frontend at that point

# What assumptions and risks should stay explicit?

- The current app only has a small number of pages, so the initial navigation model should stay intentionally simple.
- Breadcrumbs are only useful if route names remain stable and human-readable.
- A left navigation area can become cluttered quickly if every route is promoted to first-class navigation. Grouping matters.
- The component wrapper layer may still be thin; the first shell pass should not get blocked on building a complete internal design system.
- Mantine is the current component library implementation, but the shell should not let Mantine-specific APIs leak through the app unchecked.
- Responsive navigation behavior may need a follow-up if the first pass prioritizes desktop layout.

# What follow-up questions should stay open?

- should `Administration` eventually become a route of its own or remain a navigation group only?
- should breadcrumbs be fully generated from route metadata, or is a small explicit breadcrumb model good enough for now?
- should mobile navigation collapse into a drawer, or is a stacked layout sufficient for the first pass?

# What is the simplest recommendation?

Build one durable shell now:

- top header with breadcrumbs
- left navigation with an `Administration` group
- central content area
- existing Query Explorer and Statement Browser nested under that group

Keep the implementation route-driven, test the shell behavior, and avoid turning a basic layout job into an accidental framework rewrite.
