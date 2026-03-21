# nosqo

`nosqo` is a knowledge database for arbitrary knowledge.

The core idea is simple: represent knowledge as statements in the form of a triple:

`subject -> predicate -> object`

This statement-based model makes it possible to express a wide range of facts, relationships, and metadata in a consistent way without forcing everything into a rigid domain-specific schema.

## Why this exists

Most knowledge systems become painful when they assume too much about the shape of the data. `nosqo` takes the opposite approach:

- Keep the core knowledge model small.
- Let arbitrary knowledge be expressed with the same primitive.
- Build higher-level meaning on top of simple statements.

That gives the system a few practical advantages:

- Flexible enough for many domains.
- Easy to reason about.
- Easy to extend with indexing, inference, validation, and query features later.

## Core model

The primary unit of knowledge is a statement:

```text
(subject, predicate, object)
```

Examples:

```text
("Berlin", "isCapitalOf", "Germany")
("Rust", "isA", "ProgrammingLanguage")
("Axum", "isWrittenIn", "Rust")
```

This model is intentionally minimal. Everything else can be layered on top:

- identifiers and namespaces
- typed values
- metadata
- provenance
- timestamps
- confidence or trust
- derived or inferred statements

## Technical stack

The project is intended to use:

- Rust for the core implementation
- Axum for the HTTP server / API
- A web-based UI for browsing and editing knowledge

## Planned architecture

At a high level, the system will likely consist of:

- a Rust core for the statement model and query logic
- an Axum server exposing APIs for ingesting, querying, and managing knowledge
- a web UI for exploring entities, relationships, and statements

## Early goals

Reasonable first milestones for the project:

1. Define the statement data model.
2. Support storing and retrieving statements.
3. Expose a basic HTTP API.
4. Build a simple UI for viewing and creating statements.
5. Add query and filtering capabilities.

## Example use cases

`nosqo` should be able to support use cases like:

- personal knowledge bases
- structured notes and facts
- lightweight semantic graph exploration
- domain knowledge modeling
- relationship mapping between entities

## Project status

This project is in its early setup phase.

The current repository does not yet contain the implementation, but the intended direction is:

- statement-based knowledge representation
- Rust backend
- Axum API server
- web UI

## Possible project structure

One reasonable starting layout:

```text
.
├── crates/
│   ├── core/        # statement model, validation, query primitives
│   └── server/      # axum application and HTTP API
├── web/             # browser-based UI
└── README.md
```

## Design principles

- Clarity over cleverness
- A minimal core model
- Consistent representation of knowledge
- Room for richer semantics without bloating the foundation

## Improvements / Things to consider

- Decide early whether `object` is always another entity or can also be a typed literal value.
- Define how identity works: plain strings are fine for examples, but real data needs stable IDs.
- Treat provenance as a first-class concept if this will store human-entered or imported knowledge.
- Be careful not to overengineer query language v1. CRUD plus filtering is the right place to start.
- If the UI is meant for graph exploration, design API responses around traversal and pagination early. Graph UIs get slow fast if the backend shape is sloppy.

## License

TBD
