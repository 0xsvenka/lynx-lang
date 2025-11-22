# Lynx Programming Language Overview

## 1. Design Philosophy

*Simplicity, generality, flexibility - and being intuitive.*

---

## 2. Basic Syntax & Layout Rules

### Paragraph-based layout

- Blank lines separate paragraphs.
- Within a paragraph:
  - Line breaks and indentation are ignored.
  - Expressions end with `;` or at paragraph end (implicit `;`).
- Encourages one-expression-per-paragraph style, which is natural for functional constructs.

Example:

```lynx
factorial =
  | 0 => 1
  | n => n * factorial (n - 1);
result = factorial 5

println result
```

### Comments

- Line comments: `-- This is a comment`.

### Literals

- Integers: `0b1001`, `-21534789639034149038653713981928045` (arbitrary size
as long as the memory permits).
- Floating-point numbers: `3.14`, `6.626e-34` (arbitrary precision as long as the memory permits).
- Characters: `'a'`, `'\0'`.
- Strings
  - Double-quoted strings: `"Hello, World!\n"` (with escapes).
  - Multi-line strings: `\\Each line starts with "\\" ...` (no escapes).
- Lists: `[a, b, c]` desugars to `cons a (cons b (cons c nil))`.
- Records: `{ name = "Luke" }`, whose type is written as `{ name : Str }`.
- Tuples: `(1, 1.0, 'a')` desugars to `pair 1 (pair 1.0 'a')`, whose type is `Int * Float * Char`, or more verbosely, `Pair Int (Pair Float Char)`.

  *As you might have guessed, this approach, i.e. modeling tuples as composition of pairs, does not recognize singleton tuples. We do have, however, a `()` literal that desugars to `unit` of type `Unit`.*

---

## 3. Core Primitives

### Bindings

Binds an expression to a name.

```lynx
name = expression
```

- Immutable and recursive by default.
- No `let` needed; same syntax for both top-level and local bindings.

### Constructors

Special names that describe patterns.

```lynx
ctor Option : Type -> Type;
ctor some : %~A -> A -> Option A;
ctor none : %~A -> Option A
```

- ADTs and GADTs implemented in identical manner.
- Constructors occupy ordinary types; for instance, `some` is just a polymorphic function that takes a parameter of arbitrary type `A` and returns `Option A`.
- Treated differently during pattern matching.

### Pattern matching lambdas

```lynx
| pattern1 => body1
| pattern2 => body2
```

- `=>` is visually distinct from `->`, which is reserved for function types.
- Patterns: atoms, literals, constructors, wildcards (`_`), alternation (`| 1 | 2 => ...`).
- There is syntactic sugar for multi-argument functions (`| a, b => ...` becomes curried). This helps with complex patterns, e.g. `| _, [x], y+:ys => ...`.
- Used as the primary way to define functions:

  ```lynx
  add = | a, b => a + b
  ```

- Supports case matching naturally via the pipeline operator `|>`, eliminating the need for an additional "match expression".

  ```lynx
  value |>
    | 0 => "zero"
    | _ => "nonzero"
  ```

### Function application & operators

- Application: `f x y` (left-associative, natural currying).
- Infix operators: `(+)`, `(++)`, `(|>)` etc are just ordinary functions defined with surrounding parentheses. Programmers can create new operators at their will.

---

## 4. Type System: Expressive & Innovative

### Types as values

- Structural typing instead of nominal typing.
- Types are not erased; they exist at runtime.
- Types can be stored in lists, passed around to functions...
- Type-level functions are no different from ordinary functions: `Id : Type -> Type`.
- `List`, `Option` etc are just type-level constructors.

### Parameter annotations

`Type ~ value` - name the value of given type for later use in the same type expression, enabling dependent types:

  ```lynx
  make_list : (Type ~ A) -> A -> List A =
    | _, a => [a];
  l = make_list Int 5
  ```

### Contextual parameters

`%T` - inferred from context (e.g., type of arguments), no need to pass the corresponding argument explicitly.

- Syntactic sugar: `%~A` is equivalent to `%(Type ~ A)`, meaning "infer a type; name it `A`".

### Implicit parameters

`#T` - resolved by instance search in current namespace, no need to pass the corresponding argument explicitly.

- Instances are ordinary values of the corresponding type.
- Enables ad-hoc polymorphism without magic:

  ```lynx
  multiply_int : Multiply Int Int =
  {
    R = Int,
    mul = __builtin_mul_int
  }

  (*) : %~A -> %~B -> #((Multiply A B)~m) -> A -> B -> m.R
    = m.mul
  ```

---

## 5. Modules and Namespaces (TODO)

---

## 6. Metaprogramming (TODO)

Lynx does not enforce a strict phase distinction between compile-time and runtime. Types are retained and can be computed, stored, and inspected at runtime, which should pave the way for type-rich metaprogramming.
