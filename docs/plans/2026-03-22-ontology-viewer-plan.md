# What problem are we solving?

`nosqo` needs an ontology UI so users can inspect the conceptual model behind the stored knowledge without dropping straight into raw statements or ad hoc queries.
Right now the browser UI has useful administration tools, but nothing that presents ontology concepts as a first-class product surface.

We should add an ontology viewer/editor area, but keep the first implementation read-only.
Pretending editing is ready before the ontology workflows, validation rules, and write APIs are stable would be fake progress.

# What should this first phase deliver?

The first phase should deliver a read-only ontology viewer inside the Administration area.
It should establish the route, layout, information architecture, and read paths that a future editor can build on.

The first phase should include:

- an `Administration / Ontology` route
- a usable ontology overview screen
- read-only browsing of ontology entities and their relationships
- clear empty, loading, and error states
- UI structure that can later host editing controls without a rewrite

The first phase should not include:

- mutation controls
- inline editing
- create or delete flows
- optimistic UI
- write-side validation UX

# What is the current gap?

The current administration UI only includes:

- Query Explorer
- Statement Browser

There is no ontology-specific page, route, navigation item, or viewer model.
Users who want to understand the ontology are forced back into lower-level tools.

# What should the ontology viewer show?

The first version should prioritize a useful mental model over exhaustive detail.
Do not dump every raw field onto the page if it makes the ontology harder to scan.

Recommended baseline:

- ontology entity list or index
- a detail panel or main detail view for the selected entity
- important ontology metadata that users actually need to understand the model
- relationships such as parent types, properties, or linked concepts when that data exists

# Which ontology entity types should the first viewer support?

The first viewer should support the ontology concepts that are most likely to matter for understanding the graph model.
Do not start with every possible ontology artifact if only a few types are actually stable and useful.

Recommended first-class entity types:

- classes or types
- predicates or properties
- value types or scalar-like concepts when they exist explicitly in the backend model

If the backend ontology model has additional concept types, they can be surfaced later once the primary viewing workflow is solid.

# What information should be shown for each ontology entity?

The viewer should show enough information to answer the question "what does this concept mean and how does it connect to the rest of the ontology?"

For each ontology entity, the detail view should preferably show:

- stable identifier
- human-readable label or display name if one exists
- entity kind such as class, predicate, or value type
- description or documentation text if available
- parent or supertype relationships
- child or subtype relationships when they are easy to compute or already available
- applicable properties or predicates
- domain and range information for predicates when the ontology model supports it
- flags or constraints that materially affect usage
- source metadata only if it helps users understand provenance or generated data

The list or index view should stay lean.
It should show only the information needed for scanning and selection, such as:

- label
- identifier
- entity kind
- one short summary field if available

Do not force users to parse a giant property matrix just to find the concept they care about.

# How should ontology information be grouped in the detail view?

The detail view should be structured into readable sections rather than one long dump.

Recommended sections:

- Overview
- Relationships
- Properties
- Constraints
- Raw identifier or technical metadata

If a section has no meaningful content for the selected entity, omit it instead of rendering empty chrome.

# How should the ontology be retrieved from the server?

The preferred approach is a dedicated read-only ontology endpoint.
The frontend should not reverse-engineer ontology structure from arbitrary statement browsing if a cleaner backend shape can be exposed.

The recommended split is:

- ontology may be stored internally as statements
- the server should project those statements into a dedicated ontology read model for the UI

That keeps the core knowledge representation consistent without forcing the browser UI to reconstruct ontology semantics from raw triples.

Recommended server contract:

- `GET /api/v1/ontology`

Recommended response shape:

- a top-level ontology snapshot payload
- a list of ontology entities
- each entity carrying stable identifiers, kind, labels, descriptions, and relationship references

Recommended payload outline:

```json
{
  "entities": [
    {
      "id": "Person",
      "kind": "class",
      "label": "Person",
      "description": "A human individual.",
      "parents": ["Agent"],
      "children": ["Employee"],
      "properties": ["name", "birthDate"]
    },
    {
      "id": "name",
      "kind": "predicate",
      "label": "name",
      "description": "Human-readable name.",
      "domain": ["Person"],
      "range": ["Text"]
    }
  ]
}
```

The exact wire format can differ, but it should preserve these qualities:

- stable entity identifiers
- explicit entity kind
- explicit relationship references instead of forcing the UI to infer them
- enough data to render both a list view and a detail view without multiple ad hoc requests

If the ontology is large enough that a full snapshot is too heavy, a follow-up design can split this into:

- `GET /api/v1/ontology`
- `GET /api/v1/ontology/:entityId`

But the first version should stay simple unless payload size proves it cannot.

# Should the ontology be transferred to the UI as raw statements?

It can be, but that should not be the default recommendation for this viewer.

Raw statement transfer is appealing because it preserves the universal triple model.
The problem is that the UI would then need to infer ontology semantics that the server already knows or should know how to project.

For this UI, prefer:

- statements as the internal source of truth if that matches the backend model
- a projected ontology API response for browser consumption

Only consider shipping raw ontology statements directly to the browser if there is a strong reason, such as:

- a low-level debugging screen
- an expert-facing tooling mode
- a temporary backend limitation with a clearly documented adapter plan

For the main ontology viewer, raw statements are too low-level.
The browser should receive a structured ontology snapshot, not a bag of triples and a shrug.

# How should the frontend consume the ontology response?

The frontend should treat the server response as a DTO, not as the final UI model.

Recommended flow:

1. fetch the ontology snapshot from a thin infrastructure API client
2. validate or at least structurally normalize the payload at the boundary
3. map DTO entities into a read-only ontology view model inside the ontology use case
4. store selection and filtering state separately from the raw fetched data

This keeps backend churn from leaking everywhere in the UI.

# What should happen if the backend does not yet expose a dedicated ontology endpoint?

That gap should be treated as a real backend issue, not as a permanent frontend responsibility.

If a temporary adapter is unavoidable, keep it behind a clearly named adapter layer that:

- reads from the temporary source
- constructs the same ontology DTO shape the viewer expects
- documents that it is transitional

Do not let ontology rendering logic depend directly on raw statement text.
That would be a janky shortcut and it will rot immediately.

# How should the route and navigation work?

The ontology viewer should live under Administration and have a stable, usable route.

Recommended route shape:

- `/administration/ontology`

If the viewer grows into a master-detail experience, likely follow-up routes would be:

- `/administration/ontology/:entityId`

The first pass does not have to commit to nested detail routes unless they clearly improve usability immediately.
But the route structure should not block that move later.

The left navigation should add:

- `Administration / Ontology`

Breadcrumbs should reflect the route clearly.

# What architecture should this use?

Keep the implementation aligned with the documented frontend architecture.

Recommended structure:

- `ui/src/usecases/administration/ontology/` for the ontology route, read-only state, mapping, and tests
- `ui/src/common/components/` for shared viewer primitives if they are reusable outside ontology
- `ui/src/infrastructure/` for API access, connectivity, and shared error-handling concerns

The viewer should also respect the shared component abstraction layer:

- use Mantine-backed shared `nosqo` components where they already exist
- add wrapper components only when they represent a real reusable UI pattern
- avoid sprinkling direct library-specific markup across the whole use case without a reason

# How should the read-only state be modeled?

This is a good candidate for explicit client state, even in a read-only first pass, if the viewer needs selection state, filtering state, or expanded/collapsed sections.

Recommended approach:

- use a small Zustand store in `ui/src/usecases/administration/ontology/` if state spans multiple viewer components
- use Immer for store updates when the selected-entity view grows beyond trivial booleans
- keep backend fetching behind a thin API client boundary
- keep DTO-to-view-model mapping separate from rendering when the backend payload is not already UI-friendly

Do not add heavy server-state tooling unless the ontology browsing behavior actually needs caching or background refresh semantics.

# What visual and UX rules should guide the first pass?

The ontology viewer should feel like an inspection tool, not a fake editor with disabled controls everywhere.
If the page is read-only, it should say so clearly and then focus on making the information easy to navigate.

The first implementation should:

- make the read-only status explicit
- provide fast scanning of ontology entities
- make the selected entity obvious
- keep relationship information readable
- avoid overloading the page with dense tables if structured sections work better

If the user cannot understand the ontology faster than they can with raw statements, the screen is failing.

# What data and backend questions should stay explicit?

This UI depends heavily on backend read shape.
Before implementation, confirm:

- what ontology read endpoint will back the viewer
- whether ontology entities already have stable identifiers suitable for routes and selection
- what relationships are available in the current backend model
- what minimum ontology payload is worth rendering in v1

If the backend does not yet expose a clean ontology read model, do not bury that problem in frontend hacks.
Call it out and keep any temporary adapter obviously temporary.

# What should be tested?

The ontology viewer should be tested like a real product surface, even in read-only mode.

At minimum, cover:

- route rendering for `Administration / Ontology`
- loading state
- empty state when there is no ontology data
- error state when the ontology request fails
- successful rendering of a representative ontology payload
- selection or detail rendering behavior if the UI supports entity switching
- navigation highlighting and breadcrumb rendering for the ontology route

Prefer focused component and route tests over giant full-app tests.

# What implementation order is recommended?

- [ ] Confirm the backend read source for ontology data and document any stopgap assumptions
- [ ] Add `Administration / Ontology` to the navigation model and breadcrumb model
- [ ] Create `ui/src/usecases/administration/ontology/`
- [ ] Add a thin ontology API client boundary or adapter in the appropriate infrastructure area
- [ ] Define the ontology DTO shape expected from the server
- [ ] Define the read-only ontology view model
- [ ] Implement the ontology route and top-level viewer layout
- [ ] Implement loading, empty, and error states
- [ ] Implement ontology entity browsing and detail rendering
- [ ] Make the read-only status explicit in the UI copy and interaction design
- [ ] Add focused tests for ontology viewer rendering and shell integration
- [ ] Run `pnpm check` in `ui/`
- [ ] Run `nao check` if repository-level verification includes the frontend at that point

# What assumptions and risks should stay explicit?

- The first ontology UI is read-only by design, not by omission.
- The backend may not yet expose a perfect ontology read model.
- Ontology relationships can get dense fast, so the UI must stay selective and scannable.
- If entity identifiers are not stable, nested routes and deep linking will get messy.
- A fake editor with disabled controls would be worse than a clear, capable viewer.

# What follow-up questions should stay open?

- when should the ontology viewer graduate to nested detail routes?
- what ontology operations will define the first real editing phase?
- does the ontology need search or filtering in v1, or is a simple list enough?
- should ontology details support deep links immediately, or can that land in a follow-up?

# What is the simplest recommendation?

Build a read-only ontology viewer first:

- put it under `Administration / Ontology`
- make the route real and navigable
- show ontology entities and useful details
- handle loading, empty, and error states well
- keep the architecture ready for editing later without shipping pretend edit controls now

That gives the product a real ontology surface without lying about write capabilities.
