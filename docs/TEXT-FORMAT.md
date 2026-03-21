Here is a **concise, implementation-ready syntax document** for the current nosqo text format.

---

# nosqo Text Format (v1 Draft)

A human-readable format for expressing knowledge as triples:

```text
(subject, predicate, object)
```

---

# 1. Basic Structure

A **statement** is a triple:

```text
<subject> <predicate> <object>
```

Example:

```text
@berlin @capitalof @germany
```

A file is a sequence of statements, blocks, and comments.

---

# 2. Atoms

All values are **typed atoms**.

## 2.1 References (IDs)

```text
@<id>
```

Examples:

```text
@berlin
@capitalof
@user:alice
```

---

## 2.2 Numbers

### Integer

```text
i<integer>
```

```text
i42
i-3
```

### Decimal

```text
n<number>
```

```text
n3.14
n-0.01
```

---

## 2.3 Temporal Values

### Date

```text
d<YYYY-MM-DD>
```

### DateTime

```text
t<ISO-8601>
```

---

## 2.4 Boolean

```text
T   // true
F   // false
```

---

## 2.5 Text

### String (human-readable)

```text
"..."
```

### Symbol (identifier-like literal)

```text
'...'
```

---

# 3. Comments

## Line comment

```text
// comment
```

## Block comment

```text
/* comment */
```

Block comments are not nested.

---

# 4. Lists

Comma-separated lists expand into multiple statements.

## Multiple objects

```text
@alice @speaks 'en', 'de'
```

Expands to:

```text
@alice @speaks 'en'
@alice @speaks 'de'
```

## Multiple subjects

```text
@berlin, @paris @instanceof @city
```

Expands to:

```text
@berlin @instanceof @city
@paris @instanceof @city
```

## Multiple subjects and objects

```text
@a, @b @rel @x, @y
```

Expands to all combinations.

---

# 5. Subject Blocks

Group multiple predicates for one subject:

```text
@berlin {
  @capitalof @germany
  @label "Berlin"
  @population i3769000
}
```

Expands to:

```text
@berlin @capitalof @germany
@berlin @label "Berlin"
@berlin @population i3769000
```

---

# 6. Predicate Blocks

Group multiple subject-object pairs for one predicate:

```text
@capitalof {
  @berlin -> @germany
  @paris  -> @france
}
```

Expands to:

```text
@berlin @capitalof @germany
@paris  @capitalof @france
```

---

# 7. Expansion Model

All constructs reduce to plain triples:

```text
@subject @predicate @object
```

The format defines only **syntax sugar** on top of this core model.

---

# 8. Example

```text
// capitals
@capitalof {
  @berlin -> @germany
  @paris  -> @france
}

@berlin {
  @label "Berlin"
  @population i3769000
  @founded d1237-01-01
  @iscapital T
}

@alice {
  @birthdate d1990-05-14
  @speaks 'en', 'de'
  @verified F
}
```

---

# 9. Design Principles

* **Single ID type**: all graph nodes use `@...`
* **Explicit typing**: no untyped values
* **Minimal syntax**: single-character prefixes
* **Composable**: all syntax reduces to triples
* **Human-friendly**: readable and writable by hand

---

# 10. Self-describing Ontology

The ontology of the system is defined in terms of itself using the following schema:

```nosqo
@#Type {
  @~isA @#Type
  @~label "Type"
  @~description "A category of things."
  @~attribute @~label, @~description, @~isA, @~attribute
}

@#Predicate {
  @~isA @#Type
  @~label "Predicate"
  @~description "A relation between a subject and an object."
  @~attribute @~label, @~description, @~isA
}

@#String {
  @~isA @#Type
  @~label "String"
  @~description "A human-readable text value."
}

@#Integer {
  @~isA @#Type
  @~label "Integer"
  @~description "A whole number value."
}

@#Decimal {
  @~isA @#Type
  @~label "Decimal"
  @~description "A decimal number value."
}

@#Date {
  @~isA @#Type
  @~label "Date"
  @~description "A calendar date value."
}

@#DateTime {
  @~isA @#Type
  @~label "DateTime"
  @~description "A date and time value."
}

@#Boolean {
  @~isA @#Type
  @~label "Boolean"
  @~description "A true or false value."
}

@#Symbol {
  @~isA @#Type
  @~label "Symbol"
  @~description "An identifier-like literal value."
}

@~isA {
  @~isA @#Predicate
  @~label "is a"
  @~description "Declares that a subject is an instance of a type."
}

@~label {
  @~isA @#Predicate
  @~label "label"
  @~description "Provides a human-readable name."
  @~valueType @#String
}

@~description {
  @~isA @#Predicate
  @~label "description"
  @~description "Provides a human-readable description."
  @~valueType @#String
}

@~attribute {
  @~isA @#Predicate
  @~label "attribute"
  @~description "Declares that a type may use a predicate as an attribute."
  @~valueType @#Predicate
}

@~valueType {
  @~isA @#Predicate
  @~label "value type"
  @~description "Declares the expected value type of a predicate."
}

@#Predicate {
  @~attribute @~valueType
}
```
