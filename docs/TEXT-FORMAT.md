Here’s the **current nosqo textual format (v1 draft)** clearly written down.

---

# 🧱 Core Structure

A statement is a **triple of atoms**:

```text
<subject> <predicate> <object>
```

Each statement is typically **one line**:

```text
@berlin @capitalof @germany
```

---

# 🔤 Atom Format

Every value in the system is an **atom**, and every atom is **explicitly typed**.

General shape:

```text
<type-indicator><payload>
```

Exceptions:

* `T` and `F` are complete atoms (no payload)

---

# 🧩 Type System

## 1. References (graph nodes)

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

## 2. Numeric values

### Integer

```text
i<integer>
```

```text
i42
i-3
```

### Decimal (number)

```text
n<decimal>
```

```text
n3.14
n-0.01
n19.99
```

---

## 3. Temporal values

### Date

```text
d<YYYY-MM-DD>
```

```text
d2026-03-21
```

### DateTime

```text
t<ISO-8601 datetime>
```

```text
t2026-03-21T12:34:56Z
```

---

## 4. Boolean values

```text
T   # true
F   # false
```

Examples:

```text
@alice @verified T
@user42 @active F
```

---

## 5. Textual values

### String (human-readable text)

```text
"..."
```

```text
"Berlin"
"capital of Germany"
```

---

### Symbol (identifier-like literal)

```text
'...'
```

```text
'capital_of'
'en'
'kg'
```

---

# ✨ Summary Table

| Type      | Syntax    | Example              |
| --------- | --------- | -------------------- |
| Reference | `@...`    | `@berlin`            |
| Integer   | `i...`    | `i42`                |
| Decimal   | `n...`    | `n3.14`              |
| Date      | `d...`    | `d2026-03-21`        |
| DateTime  | `t...`    | `t2026-03-21T12:00Z` |
| Boolean   | `T` / `F` | `T`                  |
| String    | `"..."`   | `"Berlin"`           |
| Symbol    | `'...'`   | `'capital_of'`       |

---

# 🧾 Example

```text
@berlin @capitalof @germany
@berlin @label "Berlin"
@berlin @population i3769000
@berlin @growthrate n0.012
@berlin @founded d1237-01-01
@berlin @iscapital T

@alice @birthdate d1990-05-14
@alice @verified F

@product42 @price n19.99

@capitalof @label "capital of"
@capitalof @alias 'capital_of'

@text1 @language 'en'
```

---

# 🧠 Key Design Principles (captured)

* **Single id type** → everything in the graph is `@...`
* **Explicit typing everywhere** → no ambiguity
* **Minimal syntax** → single-character prefixes
* **Human-readable** → quotes for text, ISO formats for time
* **Extensible** → new prefixes can be added later

