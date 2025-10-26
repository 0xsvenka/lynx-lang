# **Lynx Language Specification**

_"Unify, simplify, generalize - and make it intuitive."_

---

## **1. Design Philosophy & Journey**

Lynx emerged from a deliberate critique of existing functional languages:

- **ML-style** type syntax (`'a list`) reverses natural function application order.
- **Scala** bifurcates type and term parameters (`List[A]` vs `f(a)`), creating cognitive overhead.
- **Haskell** offers conceptual clarity but requires **non-native extensions** (e.g., type families, `ConstraintKinds`) for advanced type-level programming.

After experimenting with Haskell-, ML-, and JavaScript-style lambda syntax, I converged on a **unified pattern-matching lambda** (`| pattern => body`) that:

- Is **concise yet readable**,
- Serves as **both function definition and match expression** (via pipelining),
- Eliminates the need for separate `case`, `match`, or `fun` keywords.

The guiding principles became:

> **Unification**: No artificial divide between terms and types.  
> **Simplicity**: Few core primitives; no redundant syntax.  
> **Expressiveness**: Enable dependent-like features without complexity.  
> **Ergonomics**: Minimize boilerplate; maximize clarity.

Crucially, Lynx **blurs traditional compilation stages**-types are **first-class, runtime-accessible values**, paving the way for **Lisp-style metaprogramming** with static safety.

---

## **2. Syntax & Layout**

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

## **3. Core Expression Primitives**

### **Bindings**

```lynx
name = expression
```

- Recursive by default at top level.
- Local bindings via `do (x = e1; e2)`.

### **Lambda with Pattern Matching**

```lynx
| pattern => body
```

- Multi-case, multi-argument (`|a, b => ...` ≡ curried).
- Patterns: literals, constructors (`x+:xs`), wildcards (`_`), tuples, alternation (`| 1 | 2 =>`).
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

- Types are **not erased**; they exist at runtime.
- Type constructors are **functions**: `List : Type -> Type`.
- Application order is **natural**: `List Int`.

### **(Generalized) Algebraic Data Types**

```lynx
data List : Type -> Type
  | nil  : %~A -> List A
  | cons : %~A -> A -> List A -> List A
```

- Constructors are functions returning the type.
- Polymorphism via **inferable parameters** (`%~A`).

### **Parameter Annotations (`~`)**

- Bind the **value** of a type component for use in the same type expression:
  ```lynx
  make_list : %(Type ~ A) -> A -> List A
  ```

### **Inferable Parameters (`%`)**

- `%T`: inferred from context (e.g., type of arguments).
- Sugar: `%~A` ≡ `%(Type ~ A)` - "infer a type, name it `A`".

### **Implicit Parameters (`#`)**

- `#T`: resolved by **instance search** in current namespace.
- Instances are **ordinary values** of record type.
- Enables **ad-hoc polymorphism** without magic:
  ```lynx
  (*) : %~A -> %~B -> #((Multiply A B) ~ m) -> A -> B -> m.R
    = m.mul
  ```

---

## **5. Ad-Hoc Polymorphism**

- **Type classes = records**.
- **Instances = named values** of those record types.
- **Resolution is lexical**, not global-no orphan instances.
- Full **introspection**: access associated types (`m.R`) and methods (`e.(==)`).

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

## **6. Other Built-in Syntax**

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

## **7. Runtime & Metaprogramming Vision**

Lynx **does not enforce a strict phase distinction** between compile-time and runtime:

- **Types are retained** and can be **computed, stored, and inspected at runtime**.
- This enables:
  - **Generic programming** that adapts to dynamic type information.
  - **Reflective metaprogramming** (future: quotation/splicing).
  - **Lisp-like fluidity** with **Haskell-like safety**.

This positions Lynx as a **multi-stage functional language** where **code, data, and types coexist uniformly**.

---

Lynx is not merely a language-it’s a **coherent vision** for how programming _could feel_: **simple at the surface, powerful at the core, and unified throughout**.
