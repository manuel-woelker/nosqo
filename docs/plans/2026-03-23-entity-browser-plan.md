# What problem are we solving?

The administration UI can inspect ontology and raw statements, but it still lacks a practical way to find concrete entities by type and attribute values.
Right now users either write NQL by hand in `Query Explorer` or dig through statement text in `Statement Browser`.

That is fine for debugging, but it is bad product UX for a common task:

- pick a type
- search for matching entities
- inspect one entity in detail

The first version should add a read-only `Entity Browser` that makes this workflow obvious and fast.

# What should this first phase deliver?

The first phase should deliver a read-only entity browser inside the Administration area with a fixed three-pane layout:

- query pane on the left
- result table in the top-right
- entity detail pane in the bottom-right

The first phase should include:

- a new `Entity Browser` administration surface alongside the existing `Query Explorer`
- a route dedicated to entity browsing
- a query form that starts from ontology type selection
- dynamic attribute inputs based on the selected type
- a search action that returns matching entities
- a result table that shows entity names or labels for scanning
- a detail view that shows all attributes for the selected entity
- clear loading, empty, and error states

The first phase should not include:

- editing entity data
- saved searches
- pagination or virtualized result sets unless needed by observed data size
- free-form NQL authoring as the main workflow
- multi-entity compare views

# What is the current gap?

The current UI already has a route at `/administration/query-explorer`, but that page is a raw NQL form.
It is useful for debugging, but it is the wrong abstraction for users who want to browse entities.

There is also already an ontology viewer at `/administration/ontology`.
That means the browser can and should reuse ontology metadata instead of forcing users to type predicate names from memory like it is 2007.

The backend currently exposes:

- `GET /api/v1/ontology`
- `POST /api/v1/query`

There is no dedicated entity-browser read model yet.
The plan should therefore make the backend contract explicit instead of hiding all of the logic in the frontend.

# What should the Entity Browser experience be?

The experience should optimize for progressive disclosure:

1. choose a type
2. enter one or more attribute values
3. run search
4. scan matching entities in a compact table
5. inspect the selected entity in the detail pane

The UI should feel like a browser for graph entities, not like a thin wrapper around text queries.

Recommended layout behavior:

- left pane has fixed purpose: query construction
- right pane is vertically split between results and details
- selecting a new result updates the detail pane without navigating away
- resizing can be deferred if the initial split layout is otherwise responsive and usable

# How should the query pane work?

The query pane should be driven by ontology data for the selected type.

Recommended controls:

- a required type selector populated from ontology entity types
- attribute inputs for the selected type
- a search button
- a reset or clear action

Recommended attribute behavior:

- show only attributes defined for the selected type
- render inputs in a stable, readable order
- allow partial querying so users can fill only the fields they care about
- treat blank inputs as omitted filters

For the first phase, all attribute inputs can be rendered as text inputs unless the ontology already provides stable scalar metadata worth using.
Do not overengineer typed form controls before the search workflow is proven.

# How should search be executed?

There are two viable approaches:

- build NQL on the server from structured search criteria
- build NQL in the browser and post it to the existing query endpoint

The better first implementation is a dedicated backend endpoint that accepts structured criteria.

Recommended contract:

- `POST /api/v1/entities/search`

Recommended request shape:

```json
{
  "type": "#Person",
  "filters": {
    "~label": "Alice",
    "~email": "example.com"
  }
}
```

For phase one, matching should be exact:

- type matching is exact
- attribute value matching is exact
- multi-valued attributes match when any stored value matches the requested value
- blank inputs are ignored rather than treated as empty-string matches

If partial, fuzzy, or subtype-aware search is needed later, it should be introduced intentionally as a separate semantics change rather than implied by the first implementation.

Recommended response shape:

```json
{
  "columns": ["id", "label"],
  "rows": [
    {
      "id": "@alice",
      "label": "Alice"
    }
  ]
}
```

This keeps the browser UI simple and preserves freedom to change query compilation rules later.

If the backend team wants to move faster by temporarily compiling NQL in the frontend, keep that logic isolated inside the entity-browser use case and treat it as a transitional adapter, not the permanent architecture.

# Why prefer a dedicated search endpoint over reusing raw NQL in the UI?

Because raw NQL in the browser leaks too much backend detail into the UI.

The browser should know:

- selected type
- chosen attribute filters
- selected result

It should not need to know:

- how to alias variables safely
- how to choose return columns
- how to derive preferred label fields
- how to handle future search semantics changes

Pushing structured criteria to the server keeps query semantics in one place and avoids turning the UI into a string-templating machine with a polite face.

# How should the result table behave?

The result table should optimize for quick scanning and selection.

Recommended first columns:

- entity identifier
- preferred label or name

If both a label-like attribute and a raw id exist, prefer showing the human-readable value prominently and keep the technical id visible but secondary.

Recommended behavior:

- auto-select the first row after a successful search when results are non-empty
- preserve selection while refreshing details for the same entity id when possible
- clear the detail pane when a new search returns no rows
- show an explicit empty state when nothing matches

Do not dump every attribute into the table.
That would wreck scanability and duplicate the purpose of the detail pane.

# How should the detail pane work?

Selecting an entity should show a read-only detail view containing all known attributes for that entity.

Recommended detail sections:

- overview with entity id, type, and preferred label
- attributes list or table
- raw technical values only when useful for debugging

Recommended attribute behavior:

- group repeated values under the same attribute
- use ontology labels for attribute names when available
- show raw predicate ids when no friendly label exists
- omit empty sections instead of rendering decorative emptiness

The detail pane should answer a simple question:
"What do we know about this entity?"

# How should details be retrieved?

The cleanest design is a dedicated entity detail endpoint rather than reconstructing details from the search result row.

Recommended contract:

- `GET /api/v1/entities/:entityId`

Recommended response qualities:

- stable entity id
- explicit type list or primary type
- all attributes with support for repeated values
- preferred label if derivable

If the backend does not add this immediately, the search endpoint may temporarily return enough data for the first detail view, but that should be treated as a temporary shortcut.
Search results and entity details have different optimization goals and should not stay coupled forever.

# How should this fit into the existing administration navigation?

This feature should add a new `Entity Browser` administration entry without replacing `Query Explorer`.

Recommended route shape:

- `/administration/entity-browser`

Navigation and breadcrumbs should use `Entity Browser` consistently.

`Query Explorer` should remain available as the existing free-form NQL tool.
The two pages serve different jobs and should not be collapsed together.

# What frontend architecture should this use?

Keep the implementation inside a focused use-case module:

- `ui/src/usecases/administration/entity-browser/`

Recommended responsibilities:

- page layout and orchestration
- ontology-driven form model
- entity-browser state store
- DTO-to-view-model mapping
- tests for state, rendering, and interactions

Recommended infrastructure responsibilities:

- API functions in `ui/src/infrastructure/api/api-client.ts` or adjacent entity-browser API files
- route and navigation updates in routing modules

State to model explicitly:

- selected type
- available attributes for that type
- entered filter values
- search request state
- search results
- selected entity id
- selected entity detail state

This is a good fit for a small Zustand store if the page is split across multiple components.

# What backend work is needed?

The backend should expose a read model tailored to entity browsing.

Recommended backend tasks:

- add structured search request handling for entities
- resolve type-constrained attribute matching
- choose a deterministic preferred label for result rows
- add entity detail retrieval
- return stable JSON contracts with explicit ids and attributes

The server can implement these using the existing knowledge and query engine internally.
What matters is that the browser receives a stable entity-oriented contract instead of inferring everything from raw statements or ad hoc query text.

# What implementation order makes sense?

Recommended order:

1. define the backend request and response contracts
2. implement backend entity search and detail endpoints with tests
3. rename routing and navigation from `Query Explorer` to `Entity Browser`
3. add routing, navigation, and breadcrumbs for `Entity Browser`
4. scaffold the three-pane page layout with loading and empty states
5. wire ontology type loading into the query pane
6. render dynamic attribute inputs for the selected type
7. connect search execution and populate the result table
8. connect entity selection and detail loading
9. add focused UI tests and run repository-wide verification

Do not start with polish animations or complex table features.
Get the search and inspection loop correct first.

# What should be verified?

Verification should cover both behavior and user flow.

Recommended automated checks:

- backend tests for structured search filtering by type and attribute values
- backend tests for entity detail retrieval including repeated attributes
- frontend tests for route rendering and navigation label changes
- frontend tests for dynamic attribute inputs when a type is selected
- frontend tests for successful search, empty search, and failed search states
- frontend tests for table row selection updating the detail pane
- frontend tests for showing all returned attributes in the detail pane
- repository-wide verification via `nao check`

# What assumptions, risks, and open questions should stay explicit?

Assumptions:

- ontology data includes enough type-to-attribute metadata to drive the query form
- entities have at least one stable identifier even when they do not have a label
- first-phase search result sizes are small enough for a simple table without pagination

Risks:

- ontology attribute metadata may be incomplete or inconsistent for some types
- deriving a single preferred label may be fuzzy if multiple label-like predicates exist
- entity detail payloads may become noisy if raw low-value attributes are returned without curation

Open questions:

- should type selection include subtypes automatically or require exact type matches?

These questions should be answered before backend contract finalization, because they affect both API semantics and test expectations.

# What concrete work should be tracked?

- [ ] Add `Entity Browser` to routes, navigation, and breadcrumbs without changing `Query Explorer`.
- [ ] Define JSON contracts for entity search and entity detail responses.
- [ ] Implement `POST /api/v1/entities/search` on the server with tests.
- [ ] Implement `GET /api/v1/entities/:entityId` on the server with tests.
- [ ] Add frontend API client functions for entity search and entity detail loading.
- [ ] Create `ui/src/usecases/administration/entity-browser/` as a new use case.
- [ ] Build the three-pane layout with a query pane, result table pane, and detail pane.
- [ ] Populate the type selector from ontology data.
- [ ] Render dynamic attribute inputs for the selected type.
- [ ] Execute entity search from structured form values using exact matching semantics.
- [ ] Render entity results with human-readable labels and stable ids.
- [ ] Auto-select a sensible default result after successful search.
- [ ] Load and render the selected entity's full attribute set.
- [ ] Add frontend tests for loading, empty, success, and error states.
- [ ] Run `nao check`.
