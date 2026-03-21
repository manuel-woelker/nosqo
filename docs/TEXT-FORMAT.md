Here is a **complete, concise specification** of the nosqo text format (v1), including syntax, inference rules, and the self-describing ontology. The examples prefer **implicit forms**, as requested.

---

# nosqo Text Format (v1)

A human-readable format for representing knowledge as triples:

```text
(subject, predicate, object)
```

The format supports:

* explicit and implicit term notation
* comments
* grouping constructs
* a self-describing ontology defined in the format itself

---

# 1. Statements

A statement is:

```text
<subject> <predicate> <object>
```

Example:

```text
berlin label "Berlin"
```

---

# 2. Terms

Every position (subject, predicate, object) contains a **term**.

A term may be:

* an **ID reference**
* a **literal value**

---

# 3. ID Syntax

All graph nodes are IDs.

## 3.1 Explicit IDs

```text
@name
@_id
@#Type
@~predicate
```

## 3.2 Shorthand Explicit IDs

```text
#Type     → @#Type
~label    → @~label
```

## 3.3 Random IDs

```text
@_k9x2
```

* generated
* opaque
* reserved prefix `_`

---

# 4. Implicit Terms

Terms may omit prefixes and be inferred from position.

## 4.1 Subject position

```text
berlin → @berlin
```

## 4.2 Predicate position

```text
label → @~label
```

## 4.3 Object position

No inference for bare identifiers.

Must use:

```text
@germany
#Type
~label
```

---

# 5. Literals

## Numbers

```text
i42
n3.14
```

## Temporal

```text
d2026-03-21
t2026-03-21T12:00:00Z
```

## Boolean

```text
T
F
```

## Text

```text
"Berlin"
'capital_of'
```

---

# 6. Comments

## Line comment

```text
// comment
```

## Block comment

```text
/* comment */
```

---

# 7. Lists

Comma-separated lists expand to multiple statements.

```text
alice speaks 'en', 'de'
```

→

```text
@alice @~speaks 'en'
@alice @~speaks 'de'
```

---

# 8. Subject Blocks

Group statements sharing a subject:

```text
berlin {
  label "Berlin"
  population i3769000
}
```

→

```text
@berlin @~label "Berlin"
@berlin @~population i3769000
```

---

# 9. Predicate Blocks

Group statements sharing a predicate:

```text
capitalof {
  berlin -> @germany
  paris  -> @france
}
```

→

```text
@berlin @~capitalof @germany
@paris  @~capitalof @france
```

---

# 10. Parsing Model

Parsing proceeds in two phases:

1. **Expansion**

    * shorthand (`#`, `~`) → full `@` form
    * implicit terms → explicit IDs
    * blocks and lists → flat triples

2. **Interpretation**

    * all statements are processed as:

      ```text
      @subject @~predicate <object>
      ```

---

# 11. Self-Describing Ontology (Core Kernel)

The format defines its own schema using the same syntax.

## 11.1 Core Types

```text
#Type {
  isA #Type
  label "Type"
  description "A category of things."
  attribute ~label, ~description, ~isA, ~attribute
}

#Predicate {
  isA #Type
  label "Predicate"
  description "A relation between a subject and an object."
  attribute ~label, ~description, ~isA
}
```

---

## 11.2 Primitive Value Types

```text
#String {
  isA #Type
  label "String"
  description "A human-readable text value."
}

#Integer {
  isA #Type
  label "Integer"
  description "A whole number value."
}

#Decimal {
  isA #Type
  label "Decimal"
  description "A decimal number value."
}

#Date {
  isA #Type
  label "Date"
  description "A calendar date value."
}

#DateTime {
  isA #Type
  label "DateTime"
  description "A date and time value."
}

#Boolean {
  isA #Type
  label "Boolean"
  description "A true or false value."
}

#Symbol {
  isA #Type
  label "Symbol"
  description "An identifier-like literal value."
}
```

---

## 11.3 Core Predicates

```text
~isA {
  isA #Predicate
  label "is a"
  description "Declares that a subject is an instance of a type."
}

~label {
  isA #Predicate
  label "label"
  description "Provides a human-readable name."
  valueType #String
}

~description {
  isA #Predicate
  label "description"
  description "Provides a human-readable description."
  valueType #String
}

~attribute {
  isA #Predicate
  label "attribute"
  description "Declares that a type may use a predicate as an attribute."
  valueType #Predicate
}

~valueType {
  isA #Predicate
  label "value type"
  description "Declares the expected value type of a predicate."
}
```

---

## 11.4 Predicate Capabilities

```text
#Predicate {
  attribute ~valueType
}
```

---

# 12. Example

```text
// define schema
#City {
  isA #Type
  label "City"
}

~capitalof {
  isA #Predicate
  label "capital of"
}

// data
berlin {
  isA #City
  label "Berlin"
  capitalof @germany
  population i3769000
}
```

---

# 13. Design Principles

* **Single ID model**: everything is `@...`
* **Self-describing**: schema is expressed as data
* **Explicit + implicit**: concise but unambiguous
* **Minimal core**: triples only
* **Extensible**: new semantics layered via predicates

---

# 14. Summary of Inference

| Position  | Input    | Expansion      |
| --------- | -------- | -------------- |
| Subject   | `name`   | `@name`        |
| Predicate | `name`   | `@~name`       |
| Any       | `#Type`  | `@#Type`       |
| Any       | `~label` | `@~label`      |
| Object    | `name`   | (not inferred) |

---

This gives you a **compact, expressive, and fully self-hosted knowledge format** with a clear path to validation, inference, and tooling later.
