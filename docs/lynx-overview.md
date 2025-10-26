# **Lynx Language Overview**

## **1. Design Philosophy**

_Unify, simplify, generalize - and make it intuitive._

---

## **2. Basic Syntax & Layout**

### **Paragraph-Based Layout**

- **Blank lines** separate paragraphs.
- Within a paragraph:
  - Line breaks and indentation are **ignored**.
  - Expressions end with `;` or at paragraph end (implicit `;`).
- Encourages one-expression-per-paragraph for functional constructs.

Example:

```lynx
factorial =
  | 0 => 1
  | n => n * factorial (n - 1);
result = factorial 5
```

### **Comments**

- Line comments: `-- ...`.

### **Literals**

- Character literals: `'\0'`.
- String literals: `"..."` (with escapes).
- Multi-line strings: `\\...` (no escapes, line-prefix required).

---

## **3. Core Primitives**

### **Bindings**

```lynx
name = expression
```

- Recursive by default.
- No `let` needed; same syntax for both top-level and local bindings.

### **Lambda with Pattern Matching**

```lynx
| pattern => body
```

- Multi-case, multi-argument (`|a, b => ...` ≡ curried).
- Patterns: literals, constructors (`x+:xs`), wildcards (`_`), tuples, records, alternation (`| 1 | 2 =>`).
- Used for **functions** and **case matching** (via the pipeline operator `|>`). Example:
  ```lynx
    value |>
      | 0 => "zero"
      | _ => "nonzero"
  ```


### **Function Application & Operators**

- Application: `f x y` (left-associative).
- Infix operators: `(+)`, `(++)`, `(+:)`, `(|>)` - ordinary functions defined with surrounding parentheses.

---

## **4. Type System: First-Class & Unified**

### **Types as Values**

- Types are not erased; they exist at runtime.
- Type constructors are functions: `List : Type -> Type`.
- Application order is natural: `List Int`.

### **(Generalized) Algebraic Data Types**

```lynx
data List : Type -> Type
  | nil  : %~A -> List A
  | cons : %~A -> A -> List A -> List A
```

- Constructors are functions returning the type.
- GADTs can be implemented in identical manner.

### **Parameter Annotations (`~`)**

- Bind the **value** of a given type for use in the same type expression:
  ```lynx
  make_list : %(Type ~ A) -> A -> List A
  ```

### **Inferable Parameters (`%`)**

- `%T`: **inferred from context** (e.g., type of arguments).
- Sugar: `%~A` ≡ `%(Type ~ A)` - "infer a type, name it `A`".

### **Implicit Parameters (`#`)**

- `#T`: resolved by **instance search** in current namespace.
- Instances are ordinary values of the corresponding type.
- Enables ad-hoc polymorphism without magic:
  ```lynx
  (*) : %~A -> %~B -> #((Multiply A B) ~ m) -> A -> B -> m.R
    = m.mul
  ```

---

## **5. Ad-Hoc Polymorphism**

- **Type classes = records**.
- **Instances = named values** of those record types.
- Resolution is lexical, not global-no orphan instances.
- Full introspection: access associated types (`m.R`) and methods (`e.(==)`).

Example:

```lynx
eq_complex : %~A -> #(Eq A) -> Eq (Complex A) =
  | #e =>
  {
    (==) (complex a1 b1) (complex a2 b2) =
      e.(==) a1 a2 && e.(==) b1 b2,
    (eq_default (==))...
  }
```

---

## **6. Other Builtin Syntax**

| Syntax        | Meaning                 |
| ------------- | ----------------------- |
| `[\|A\|]`     | `List A`                |
| `[]`          | `nil`                   |
| `[x, y]`      | List literal            |
| `(\|A, B\|)`  | Tuple type              |
| `(x, y)`      | Tuple literal           |
| `(x,)`        | Singleton tuple literal |
| `{\|x : A\|}` | Record type             |
| `{ x = a }`   | Record literal          |

### **Record Extension (`...`)**

- `(eq_default (==))...` spreads fields from a helper into a record.
- Reduces boilerplate in instance definitions.

---

## **7. Metaprogramming Vision**

Lynx does not enforce a strict phase distinction between compile-time and runtime. Types are retained and can be computed, stored, and inspected at runtime, which paves the way for **type-rich metaprogramming**.

---

Lynx is not merely a language - it’s a coherent vision for how programming _could feel_: **simple at the surface, powerful at the core, and unified throughout**.
