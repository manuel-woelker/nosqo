# What problem are we solving?

The server currently exposes `GET /api/v1/statements`, which is only a thin statement-pattern filter over subject, predicate, and object query parameters.
That is useful for debugging, but it is not enough to execute real NQL queries with multiple patterns, variable unification, and explicit return projections.

We need a server endpoint that accepts an NQL query, validates it, executes it against the in-memory statement store, and returns results in a stable format that clients can consume.

# What is the current gap?

The repository already has the core NQL syntax pieces:

- `nosqo_model` defines the NQL AST types
- `nosqo_parser::nql::NqlParser` parses NQL text into that AST

The missing pieces are the ones that actually matter for an HTTP query API:

- no execution layer that turns `NqlQuery` into result rows
- no server route for submitting NQL queries
- no query-specific error mapping for invalid queries vs server failures
- no response contract for result rows and projected variables
- no end-to-end tests covering NQL requests through the server boundary

# What endpoint should we add?

Add a `POST /api/v1/query` endpoint.

Using `POST` is the simplest correct choice because NQL is a structured, multiline request body.
Trying to cram this into query parameters would be clumsy, hard to read, and hostile to real client usage.

The initial contract should be:

- request body: raw NQL text with `Content-Type: text/plain`
- success response: JSON
- invalid NQL: `400 Bad Request`
- valid NQL that fails due to an internal execution bug or store failure: `500 Internal Server Error`

The response body should include both the projected variable order and the result rows so clients do not need to reverse-engineer the schema from the first row.

Suggested shape:

```json
{
  "columns": ["?city", "?label"],
  "rows": [
    ["@berlin", "\"Berlin\""],
    ["@paris", "\"Paris\""]
  ]
}
```

This is intentionally boring.
That is good here.
It keeps the API compact, deterministic, and easy to render in both CLI and browser clients.

# Where should the execution logic live?

Keep HTTP handlers thin and put query execution behind server or engine-facing methods, following the existing `ServerState` pattern.

Recommended implementation order:

1. Add a small query execution module in `crates/engine` that accepts `&dyn StatementStore` plus `&NqlQuery`.
2. Introduce explicit result types in `nosqo_model` or `nosqo_engine` for projected query rows.
3. Expose a `ServerState` method that:
   - parses raw NQL text
   - executes the query
   - maps the result into the HTTP response DTO
4. Keep the Axum handler focused on HTTP extraction and status-code mapping.

Do not bury query semantics inside the route handler.
That would be lazy and make testing worse for no benefit.

# How should NQL execution work in v1?

Implement the execution model described in [`docs/NQL-v1-spec.md`](/data/projects/nosqo/docs/NQL-v1-spec.md):

1. Parse the query into `NqlQuery`
2. Evaluate each triple pattern against the statement store
3. Join bindings across patterns
4. Enforce variable unification when a variable repeats
5. Project either:
   - the variables listed in the `return` block, or
   - all bound variables in first-appearance order for `return *`

The first version does not need query optimization.
Correctness and a clean API matter more than being clever.

Implementation details worth making explicit:

- Convert concrete NQL terms into statement-store match constraints where possible
- When a pattern mixes variables and constants, scan matching statements and produce bindings for the variable positions
- Represent a binding row as an ordered mapping from `NqlVariable` to bound value
- Reject or error when `return` references an unbound variable if the parser or executor does not already enforce that invariant

# What response format should the server return?

Return JSON, not nosqo text.

`/api/v1/statements` returning nosqo text is fine because it returns statements.
NQL returns projected bindings, which are tabular results, not statement sets.
Pretending otherwise would produce a weird API.

To keep v1 simple, serialize each projected value in its nosqo string form inside the JSON row arrays.
That avoids inventing a second typed value encoding during the same change.

If typed JSON values become important later, that can be a follow-up endpoint or a versioned response change.

# How should errors be handled?

Split errors into user errors and server errors.

- Parse or validation failure: return `400` with a plain JSON error payload
- Unsupported v1 semantics discovered during execution: return `400`
- Store or unexpected execution failure: return `500` and log the internal error

Suggested error shape:

```json
{
  "error": "query must contain at least one pattern"
}
```

This keeps the client contract simple and is consistent with the current small-server posture.

# What should be tested?

Add colocated tests close to the changed code.

At minimum, cover:

- parser-to-executor integration for a single-pattern query
- multi-pattern joins
- repeated-variable unification
- `return *` column ordering
- explicit `return ?a ?b` projection ordering
- empty result sets
- invalid NQL returning `400`
- server route success response shape
- repository-wide verification with `./scripts/check-code.sh`

Use data-driven tests for query cases instead of copy-pasting one-off examples all over the place.

# What is the recommended implementation order?

- [ ] Add query result types for projected columns and rows
- [ ] Add an NQL execution module in `crates/engine`
- [ ] Cover execution semantics with colocated engine tests
- [ ] Add a `ServerState` method for parsing and executing raw NQL
- [ ] Add `POST /api/v1/query` to the Axum router
- [ ] Add HTTP request and response types for the query endpoint
- [ ] Map invalid queries to `400` and internal failures to `500`
- [ ] Add server tests covering success, empty results, and invalid input
- [ ] Add or update a `.http` example file for manual endpoint exercise
- [ ] Run `./scripts/check-code.sh`

# What assumptions and risks should stay explicit?

- The current codebase has an in-memory `StatementStore` API for statement-pattern lookup, not a dedicated query engine. The plan assumes NQL execution can be layered on top of repeated statement scans for v1.
- The current NQL parser appears to accept the syntax needed for v1, but execution may reveal missing validation rules that should either move into the parser or be enforced in the executor.
- The plan assumes returning nosqo-rendered scalar values inside JSON is acceptable for the first endpoint version. If clients need typed JSON values immediately, the API contract should be revised before implementation.
- The plan assumes `POST /api/v1/query` is the desired public shape. If the product direction wants multiple query languages later, `/api/v1/nql/query` may be cleaner, but that feels premature right now.
