# What is this document for?

This document specifies a compact JSON format for transmitting `nosqo` statements.
It is intended for machine-to-machine transport where plain nosqo text is too expensive, too ambiguous, or too awkward to process incrementally.

The format keeps the core `nosqo` model intact:

- knowledge is still represented as triples
- statement rows may compactly encode repeated subject and predicate pairs with multiple objects
- literal and identifier syntax stays aligned with nosqo term conventions

# What is the core idea?

The format uses:

- a single indexed value table
- compact statement rows containing indexes into that table

Each statement row contains:

- one subject index
- one predicate index
- one or more object indexes

If a row has more than one object index, it expands to multiple triples with the same subject and predicate.

# What is the top-level structure?

A payload has this shape:

```json
{
  "format": "nosqo-statement-json-v1",
  "values": [],
  "statements": []
}
```

Fields:

- `format`: required format identifier
- `values`: required value table
- `statements`: required array of compact statement rows

Additional metadata such as pagination or cursors may be added later, but they are not part of this v1 core specification.

# What does the `values` table contain?

The `values` table contains the terms referenced by statement rows.
Each entry is one of:

- a bare JSON string
- a single-element JSON array containing one string

Examples:

```json
[
  "#Person",
  "~isA",
  "#Type",
  "~label",
  ["Person"],
  "~population",
  "i3769000",
  "~isCapital",
  "T"
]
```

# How should bare strings in `values` be interpreted?

A bare string is a nosqo token encoded using standard nosqo syntax.

Examples:

- `"#Person"` is a type identifier
- `"~label"` is a predicate identifier
- `"@frodo_baggins"` is an entity identifier
- `"i3769000"` is an integer literal
- `"n3.14"` is a decimal literal
- `"d2026-03-22"` is a date literal
- `"t2026-03-22T12:00:00Z"` is a datetime literal
- `"T"` and `"F"` are boolean literals

The decoder should interpret these exactly as nosqo terms, not as arbitrary untyped JSON strings.

# How should nested single-string arrays in `values` be interpreted?

A single-element array containing one string represents plain text.

Example:

```json
["Person"]
```

This means the text value `Person`.

This wrapper exists so plain text does not compete with bare nosqo tokens in the same table.

Examples:

- `["Person"]` is text
- `["#Person"]` is text containing the characters `#Person`
- `["T"]` is text containing the characters `T`, not a boolean literal

# Why are plain text values wrapped?

Without the wrapper, plain text would be ambiguous with nosqo syntax.

Examples of ambiguity that the wrapper avoids:

- `"#Person"` could mean a type identifier or the text `#Person`
- `"T"` could mean a boolean or the text `T`
- `"i42"` could mean an integer or the text `i42`

The wrapper makes plain text explicit while keeping statement rows compact.

# What is the structure of a statement row?

Each statement row is a JSON array of integers with length at least `3`.

General form:

```json
[subjectIndex, predicateIndex, objectIndex, ...additionalObjectIndexes]
```

Rules:

- the first index references the subject term in `values`
- the second index references the predicate term in `values`
- the remaining indexes reference one or more object terms in `values`
- every index must be a valid zero-based index into `values`

Example:

```json
[0, 1, 2]
```

This is one triple.

Example:

```json
[0, 5, 3, 6]
```

This expands to two triples:

- `values[0] values[5] values[3]`
- `values[0] values[5] values[6]`

# How do statement rows expand into triples?

Statement rows are transport-level compact forms.
They expand into one or more logical triples.

Expansion rule:

```text
[s, p, o1, o2, ..., on]
```

expands to:

```text
(values[s], values[p], values[o1])
(values[s], values[p], values[o2])
...
(values[s], values[p], values[on])
```

So a row with:

- one object index expands to one triple
- multiple object indexes expands to multiple triples

This is equivalent to nosqo comma-separated object lists.

# What is a complete example?

```json
{
  "format": "nosqo-statement-json-v1",
  "values": [
    "#Person",
    "~isA",
    "#Type",
    "~label",
    ["Person"],
    "~attribute",
    "~description",
    "~population",
    "i3769000",
    "~isCapital",
    "T"
  ],
  "statements": [
    [0, 1, 2],
    [0, 3, 4],
    [0, 5, 3, 6],
    [0, 7, 8],
    [0, 9, 10]
  ]
}
```

This expands to:

```text
#Person ~isA #Type
#Person ~label "Person"
#Person ~attribute ~label
#Person ~attribute ~description
#Person ~population i3769000
#Person ~isCapital T
```

# What validation rules should apply?

A valid payload must satisfy all of these rules:

- `format` must equal `nosqo-statement-json-v1`
- `values` must be an array
- every `values` entry must be either:
  - a JSON string
  - a JSON array of length `1` whose only element is a JSON string
- `statements` must be an array
- every statement row must be an array of integers
- every statement row must have length at least `3`
- every referenced index must be within the bounds of `values`

Recommended semantic validation:

- subjects should resolve to valid nosqo subject terms
- predicates should resolve to valid nosqo predicate terms
- objects should resolve to valid nosqo object terms

If a decoder already has access to the nosqo parser, it should reuse it instead of inventing a parallel term parser.

# What should be avoided?

Avoid these mistakes:

- using bare strings for plain text
- allowing nested arrays longer than one element in `values`
- treating multi-object rows as a different graph model instead of compact triple expansion
- introducing a second way to encode plain text such as quoted nosqo strings in bare string entries

Plain text should use the wrapped form consistently.
Bare strings should be reserved for nosqo-syntax terms.

# What are the tradeoffs of this format?

Advantages:

- compact over the wire
- repeated identifiers and literals are deduplicated naturally
- preserves nosqo term syntax for non-text literals
- supports grouped multi-object statements without losing the triple model

Tradeoffs:

- less human-readable than plain nosqo text
- requires explicit decoding before direct inspection
- not ideal as an authoring format

This is a transport format, not a replacement for the nosqo text format.

# When should this format be preferred over nosqo text?

Prefer this format when:

- bandwidth matters
- repeated identifiers appear often
- a client wants fast indexed decoding
- the payload is intended for programmatic consumption rather than direct editing

Prefer plain nosqo text when:

- humans need to read or author the data directly
- debuggability matters more than transport compactness
- preserving the exact textual shape is important

# What is the simplest recommendation?

Use:

- one `values` table
- bare strings for nosqo-syntax terms
- single-string nested arrays for plain text
- statement rows of `[subjectIndex, predicateIndex, ...objectIndexes]`

That gives a compact transport without inventing a second semantic model.
