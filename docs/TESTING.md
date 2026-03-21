# What is this document for?

This document describes the testing expectations for `nosqo`.
Use it when adding features, fixing bugs, or changing behavior that should stay stable over time.

# Where should tests live?

Tests should be colocated with the code they exercise.

In practice, that means:

- unit tests live in the same Rust source file behind `#[cfg(test)]`
- crate-level integration tests are acceptable when the behavior is easier to verify from the public API
- documentation-only changes do not need Rust tests unless they describe behavior that is also changing

# What testing style should be preferred?

Prefer small black-box tests that verify observable behavior.
If behavior is easier to understand as a rendered string or artifact file, assert on that output directly.

Prefer:

- data-driven tests when several inputs should exercise the same rule
- snapshot-style assertions when the output is long and readability matters
- regression tests for every bug fix that changed behavior

Avoid mocking when a real in-memory or file-backed path is practical.

# When should `PalMock` be used?

Use `nosqo_pal::pal_mock::PalMock` when the code depends on platform behavior such as:

- reading or writing files
- process execution
- timestamps or system time
- terminal interactivity

`PalMock` is the default test double for backend and infrastructure code that depends on the PAL boundary.

# What should be verified for backend and API changes?

Backend and API changes should usually verify:

- statement parsing or validation behavior
- storage and retrieval behavior
- query behavior
- failure messages
- API responses when endpoint behavior changes

If a bug fix changed a query result, API payload, or validation rule, add a focused test for that exact behavior.

# What repository-wide checks should be run?

When completing a unit of work, run:

```bash
./scripts/check-code.sh
```

That script runs formatting, build, clippy, and the test suite.

# What should happen when a change is not tested?

Call it out explicitly in the final summary or commit context.
If a change is intentionally left without automated coverage, explain why the normal testing approach was not practical.
