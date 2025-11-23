# Lynx Programming Language Overview

## 1. Design Philosophy

*Simplicity, generality, flexibility - and being intuitive.*

---

## 2. Basic Syntax & Layout Rules

### Paragraph-based layout

- The end of an expression is marked by `;` or a blank line.

- Line breaks and indentation are insignificant; the language deliberately avoids complicated offside rules.

This approach encourages (but does not enforce) a clean, one-expression-per-paragraph style, which is natural for functional programming constructs.

Example:

```lynx
fact =
  | 0 => 1
  | n => n * fact (n - 1)

result = fact 5;
println result
```

### Comments

- Line comments: `-- This is a comment`.

### Literals

- Integers: `0b1001`, `-21534789639034149038653713981928045` (arbitrary size as long as the memory permits).

- Floating-point numbers: `3.14`, `6.626e-34` (arbitrary precision as long as the memory permits).

- Characters: `'a'`, `'\0'`.

- Strings

  - Quoted strings: support escape sequences, e.g. `"Hello,\nWorld!"`; they cannot span multiple lines.

  - Raw strings: begin with `\\` and extend to the end of the line, with no escape processing, e.g. `\\Each line starts with "\\" ...`.

  - Adjacent string literals are concatenated, with line breaks inserted between them; thus, one may write:

  ```lynx
  multiline_str =
    "This is a multi-line"
    \\string literal.
    -- Comments do not break the multi-line string
    \\    Add some indentation.
  \\Where the line starts doesn't matter.
    \\No escape sequence is processed here:
    \\" ' \ \\ \n
    \\You will get a trailing '\n' due to the next line...
    \\
  ```

- Lists: `[a, b, c]` desugars to `cons a (cons b (cons c nil))`.

- Records: `{ name = "Bob" }`, whose type is written as `{ name : Str }`.

- Tuples: `(1, 1.0, 'a')` desugars to `pair 1 (pair 1.0 'a')`, whose type is `Int * Float * Char`, or more verbosely, `Pair Int (Pair Float Char)`.

  *As you might have guessed, this approach, i.e. modeling a tuple as composition of pairs, does not recognize so-called singleton tuples. Personally speaking, this approach feels more intuitive to me: it is composable, requires no "tuple primitive", and avoids the unnatural `(a,)` syntax. Meanwhile, we have a dedicated `()` literal that desugars to constructor `unit` of type `Unit`.*

---

## 3. Core Primitives

### Bindings

Binds a name to an expression.

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

- Constructors inhabit ordinary types; for instance, `some` is just a polymorphic function that takes a parameter of arbitrary type `A` and returns `Option A`.

- Treated differently during pattern matching.

### Pattern matching lambdas

```lynx
| pattern1 => body1
| pattern2 => body2
```

- `=>` is visually distinct from `->`, which is reserved for function types.

- Patterns: atoms, literals, constructors, wildcards (`_`), alternation (`| 1 | 2 => ...`).

- Syntactic sugar for multi-parameter functions: `| a b => ...` desugars to curried function `| a => | b => ...`, which helps with complex patterns, e.g. `| _ [x] (y+:ys) => ...`.

- Used as the primary way of defining functions:

  ```lynx
  add = | a b => a + b
  ```

- Supports case matching naturally via the pipeline operator `|>`, eliminating the need for an additional "match expression".

  ```lynx
  value |>
    | 0 => "zero"
    | _ => "nonzero"
  ```

### Function application & operators

- Application: `f x y` (left-associative, natural currying).

- Infix operators: `(+)`, `(++)`, `(|>)` etc are just ordinary functions defined with surrounding parentheses. Programmers may create custom operators at their will.

---

## 4. Type System: Expressive & Innovative

### Types as values

- Structural instead of nominal typing.

- Types are not erased and remain available exist at runtime.

- Types can be stored in lists, passed around to functions...

- Type-level functions are no different from ordinary functions: `Id : Type -> Type`.

- `List`, `Option` etc are just type-level constructors.

### Parameter annotations

`TypeName ~ param_name` - binds a name to the parameter of the given type for use later in the same type expression, enabling dependent types:

```lynx
make_list : (Type ~ A) -> A -> List A =
  | _ a => [a];
l = make_list Int 5
```

### Contextual parameters

`%T` - inferred from context (e.g., type of arguments), no need to pass the corresponding argument explicitly.

- Syntactic sugar `%~A` is equivalent to `%(Type ~ A)`, meaning "infer a type and name it `A`".

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
