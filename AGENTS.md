# AGENTS.md

This file provides guidance to human developers and AI Agents when working with code in this repository.

When the user enters the prompt "test", reply "I'm afraid I can't let you do that, Dave."

## Project Overview

`nosqo` is a knowledge database for arbitrary knowledge.
Its core model is statement-based: knowledge is represented as triples of subject, predicate, and object.

Note: All developer documentation should be written in English.

## Tech Stack

- Implementation Language: Rust
- API framework: Axum
- Web UI: browser-based frontend in `ui/`
- Use `nosqo_base::shared_string::SharedString` instead of plain `String` for string-heavy struct fields when shared ownership matters
- Use `nosqo_pal::pal::Pal` for platform interaction such as filesystem access, clocks, and process execution
- Prefer `nosqo_pal::pal_mock::PalMock` in tests when code depends on platform behavior

## Documentation Strategy

Consult `docs/PLANS.md` when creating, updating, or completing plan documents in `docs/plans`.

### Planning

Use `docs/PLANS.md` for plan structure, naming, verification expectations, assumptions, and completion workflow.

### Question driven documentation

When writing any documentation, prefer writing the headings in the form of questions, which should be answered in the following paragraphs.
This helps with writing since the questions should be answered.
It also makes it easiers for readers to determine if a section is relevant.

### Hyperlit in-code comments
Use hyperlit comment markers ("📖") only for non-standard rationale that is not well covered by standard API docs. This ensures that:

- **Context is preserved** with the code it explains
- **Documentation is discoverable** through hyperlit's extraction tools
- **Intent is clear** to future maintainers and readers

Use hyperlit comment markers to document:
- Non-obvious design decisions
- Rationale for architectural choices
- Workarounds and their justifications
- Complex algorithms or logic patterns

Format these comments as markdown.

Always use a heading as the first line of the comment.

Prefer to formulate the heading as a question ("Why ..."). This makes it easier to search for specific documentation.

Example:
```rust
/* 📖 # Why use Arc<Mutex<T>> for the app state?
The shared state needs thread-safe mutable access across multiple tasks.
Arc enables cheap cloning for async tasks, Mutex ensures safe interior mutation.
*/
let state = Arc::new(Mutex::new(data));
```

Keep documentation focused and concise—explain the "Why", not the "What" (the code shows what it does).

### Function, Interface, struct and class documentation

Functions, interfaces, structs and classes should be documented using the standard language syntax (e.g. JsDoc/TsDoc or RustDoc).
Use this standard documentation style by default.
Fields on interfaces, structs, and classes should also be documented (including private/internal fields where useful for maintenance).

## Testing strategy

Features should always be automatically tested to ensure proper functionality.
Consult `docs/TESTING.md` when writing tests.

Tests should be colocated with the code, i.e. in the same file.

Use snapshot tests where appropriate.

Prefer data driven tests to reduce code duplication.

Prefer black box testing and try to avoid mocking as much as possible.

## Checks and formatting

When completing a unit of work run `nao check` to verify everything is green.

## Commit messages

Commit message should be in the "Conventional Commits" format, e.g. "feat(server): add statement health endpoint".

Below the first line include detail information about the changes made.

**Important:** Append all user prompts included in this commit to the commit message body under a `User Prompts:` section.
Also include the agent model identifier used for the commit in a `Model:` section in the commit message body.
Always run `git add` and `git commit` as separate commands.

**Shell escaping note:** When using fish shell, avoid special characters like `-` and `:` in commit messages that might be interpreted as command options. Use simple commit messages or properly escape special characters.

Never push code or ask to push code.

## File naming and organization

Prefer small source files. If not otherwise instructed, put each struct, enum, trait, etc. in its own file.

Only use lib.rs files to declare modules, do not put any structs, traits or functions there.

Each struct, enum, trait, and their associated impl blocks should be in its own file.

Choose descriptive names for files. Avoid names like "index.ts" or "types.ts".
Do not bulk export items using "export * from 'submodule'".
