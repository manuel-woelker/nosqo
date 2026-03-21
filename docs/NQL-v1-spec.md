# NQL v1 — nosqo Query Language Specification

## 1. Overview

NQL (nosqo Query Language) v1 is a minimal pattern-matching query language designed to operate directly on the nosqo triple-based data model.

A query consists of:

* a **match block**: a set of triple patterns
* a **return block**: a projection of variables

The language is intentionally small to ensure:

* easy formulation
* simple parsing
* straightforward execution

NQL operates purely on triples and does not distinguish between instance data and ontology data.

---

## 2. Core Concepts

### 2.1 Triples

All data is represented as triples:

<subject> <predicate> <object>

NQL queries match against these triples.

### 2.2 Terms

A term is one of:

* **variable**: ?x
* **identifier (ID)**: @id, #Type, ~predicate
* **literal**: "text", i42, d2026-01-01, T, F

Term syntax is identical to the nosqo text format.

### 2.3 Variables

Variables:

* begin with `?`
* represent unknown values
* are bound during matching
* must unify when repeated

Example:

?x label ?name
?x description ?desc

Both patterns refer to the same ?x.

---

## 3. Query Structure

A query has exactly two blocks:

match <pattern> <pattern>
return
<variables | *>

### 3.1 Match Block

The `match` block defines triple patterns.

Each line is one triple pattern:

<subject> <predicate> <object>

### 3.2 Return Block

The `return` block defines which variables are returned.

Options:

* list of variables
* `*` (all bound variables)

---

## 4. Semantics

### 4.1 Pattern Matching

Each pattern constrains variable bindings.

Execution proceeds by finding all bindings that satisfy all patterns simultaneously.

### 4.2 Variable Unification

If a variable appears multiple times, all occurrences must resolve to the same value.

Example:

match
?x label "Berlin"
?x capitalof ?country
return
?country

?x must refer to the same entity in both patterns.

### 4.3 Result Projection

After matching, only variables listed in `return` are output.

Example:

match
?city isA #City
?city label ?label
return
?label

?city is bound but not returned.

### 4.4 Return All Variables

`return *` returns all bound variables.

Order of variables SHOULD follow first appearance in the match block.

---

## 5. Grammar (EBNF)

query        := "match" newline pattern+ "return" newline return_spec

pattern      := term term term newline?

return_spec  := "*" | variable+

term         := variable | id | literal

variable     := "?" name

id           := ("@" | "#" | "~") name

literal      := string | integer | date | boolean

name         := [a-zA-Z_][a-zA-Z0-9_]*

newline      := '\n'

Notes:

* Whitespace separates tokens
* Each pattern SHOULD be on its own line
* Parsers MAY allow flexible whitespace

---

## 6. Execution Model

A conforming implementation SHOULD execute queries as follows:

1. Parse query into an abstract syntax tree (AST)
2. Normalize terms using nosqo parsing rules
3. Initialize an empty set of bindings
4. For each pattern:

    * match triples in the store
    * join results with existing bindings
5. Apply variable unification constraints
6. Project variables specified in `return`

This corresponds to a sequence of joins over triple patterns.

---

## 7. Examples

### 7.1 All Cities

match
?city isA #City
return
?city

### 7.2 City Labels

match
?city isA #City
?city label ?label
return
?city ?label

### 7.3 Capitals

match
?city capitalof ?country
return
?city ?country

### 7.4 Single Fact Lookup

match
@berlin capitalof ?country
return
?country

### 7.5 Ontology Query

match
?p isA #Predicate
?p label ?label
return
?p ?label

---

## 8. Validation Rules

A valid query MUST:

* contain exactly one `match` block
* contain exactly one `return` block
* contain at least one pattern
* only return variables (unless using `*`)

A query SHOULD:

* use variables consistently
* avoid unused variables in `return`

---

## 9. Non-Goals (v1)

The following features are explicitly out of scope for v1:

* filters (e.g., comparisons)
* sorting
* limits
* aggregation (count, group)
* optional patterns
* negation
* path queries
* inference-aware querying

These may be added in future versions.

---

## 10. Design Principles

NQL v1 follows these principles:

* **Minimality**: smallest useful query language
* **Alignment**: mirrors nosqo triple structure
* **Uniformity**: same language for data and ontology
* **Predictability**: no hidden behavior
* **Composability**: extensible without breaking core

---

## 11. Example End-to-End

Query:

match
?city isA #City
?city label ?label
return
?label

Execution:

1. Match triples where predicate is `isA` and object is `#City`
2. Bind each subject to ?city
3. For each ?city, match triples with predicate `label`
4. Bind object to ?label
5. Return ?label

---

## 12. Version

This document defines NQL v1.

Future versions may extend the language while preserving backward compatibility.
