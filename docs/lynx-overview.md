# Lynx programming language overview

## Basic syntax

### Paragraph-based layout

Expressions end with a semicolon or a blank line; whitespace and indentation are insignificant. Example:

```lynx
a = 1; b = a;
c = a + b

println c
```

### Literals

#### Interger

Arbitrary size as long as the memory permits.

#### Floating-point number

Arbitrary precision as long as the memory permits.

#### Character

#### String

- Quoted string

  Supports escape sequences, e.g. `"Hello,\nWorld!"`; cannot span multiple lines.

- Raw string

  Begins with `\\` and extends to the end of the line, with no escape processing, e.g. `\\Each line starts with "\\" ...`.

Adjacent string literals are concatenated, with line breaks inserted between. Example:

```lynx
s = "This is a multi-line..."
  -- Comments do not break the multi-line string
  \\...string literal.
```

### Patterns

#### Atoms

- Literal

- Wildcard (`_`)

- Constructor

  Syntax: `` `A`` for the constructor named `A`.

- Identifier
  - Alphabetic identifier

    Syntax: `[A-Za-z_][A-Za-z0-9_'!]*`.

  - Symbolic identifier

    Syntax: ``[~`!@#$%^&*+=|:<>.?/]``

  `mut a` indicates that the identifier `a` is a mutable binding, i.e. it can be [rebound](#binding) to another value.

#### Constructor expression

Examples: `` `cons x xs``, `` `some a``.

#### Tuple

Items are separated by commas. `1, 'A'` is of type `Int * Char`. Note that the `,` operator has relatively low precedence, therefore, tuples often require surrounding parentheses, e.g. `f (a, b)`.

#### List

Wrapping a tuple in a pair of square brackets yields a list, e.g. `[a, b, c]`. The compiler ensures that list items are of the same type.

#### Map

Syntax: `#{k1, v1; k2, v2}`.

#### Record

`rec {name = }`, of type `Rec {name: Str}`.

## Core primitives

### Binding

Syntax: `pattern = expr`.

Example:

```lynx
p = 1, 2; x, _ = p;
println x
```

Mutable bindings can be rebound with `:=`. Example:

```lynx
a = 1; mut a' = a;
a' := 2;
println a;  -- 1
println a'  -- 2
```

### Lambda expression

Syntax: `param_pattern => expr`.

The `=>` operator is right‑associative, convenient for creating higher-order functions.

Example:

```lynx
get_x = x, _ => x

add = x => y => x + y
```

Note, however, that most functions are not defined this way directly; instead, the [`fn` macro](#fn-macro-function-definition) is more common.

### Function application

Syntax: `f x` (juxtaposition).

Function application is left-associative: `f x y` is equivalent to `(f x) y`.

### Operator

Lynx syntax relies heavily on operators. During parsing, even symbols like `,` and `=>` are handled as operators; in this way, the parser can follow a unified Pratt algorithm based on [operator precedence and associativity](#precedence-and-associativity-of-standard-operators).

There are three kinds of operators: prefix, infix, and suffix, all of which can be enriched by user-defined ones. This allows for enormous flexibility.

```lynx
infixl * 70;  -- Left associative, precedence set to 70
fn (*) @(A, B, m: Mul (A, B)) (a: A) (b: B): m.R {
  m.mul (a, b)
}
```

#### Precedence and associativity of standard operators

**TODO**

## Type system

Types are first-class values.

### Type annotation

Syntax: `value: Type`.

Type annotation helps the compiler with type inference as well as type checking. It may be employed on any expression, although the most common usage is at function definition.

### Parameter annotation

Syntax: `ParamType ~ param_pattern`.

In a type expression, you can name any parameter for later use. `A ~ B` is the type-context equivalent of value-context `B : A`.

Example:

```lynx
map: (Type*Type ~ (A, B)) -> (A -> B) -> List A -> List B;
map (Int, Str) to_str [1, 2, 3]
```

### Contextual parameter

Syntax: `@ParamType`.

Contextual parameters do not need to be passed explicitly; instead, they are inferred lexically from the context (e.g. through parameter annotation, trait implementation search, etc).

Example:

```lynx
map: @(Type*Type ~ (A, B)) -> (A -> B) -> List A -> List B

(==): @((Type~A) * Eq A) -> A -> A -> Bool
```

### Interior mutability

For any type `T`, the "mutable cell" type `&T` provides interior mutability. Note that this is a fundamentally distinct concept from mutable binding.

- Create: the `ref` function.

- Inspect: prefix operator `!`.

- Mutate: infix operator `<<`.

Note that the compiler will copy a value if it decides that the latter cannot be shared safely between a `T` and a `&T`.

Example:

```lynx
ra: &Int = ref 1;
ra << 3;
println !ra
```

## Macros

Macros are essentially compile-time functions that takes a token stream and returns an AST. When the Lynx parser encounters a macro invocation, it applies the custom parsing logic defined by that macro on the remaining token stream, after which it merges the resulting AST into the existing one. This process is called _macro expansion_. Macros may be created by the user.

Here are some of the macros pre-defined by the language:

### Control flow macros

- `do`

Evaluates to the last expression.

```lynx
println (do {
    print "1 + 2 = ";
    1
} + 2)
```

- `if`

```lynx
if n % 3 == 0 && n % 5 == 0 {
    println "FizzBuzz"
} elif n % 3 == 0 {
    println "Fizz"
} elif n % 5 == 0 {
    println "Buzz"
} else {
    println n
}
```

- `while`

```lynx
mut i = 0;
while i < 100 {
    println i;
    i := i + 1
}
```

- `for`

```lynx
for k, v in #{1, 'A'; 2, 'B'} {
  println k; println v
}
```

- `match`

```lynx
match n {
  1 | 2 => "small";
  _ => "big"
}
```

### `fn`: function definition

```lynx
fn f (n: Int): Int {
    if n == 0 {1}
    else {n * f (n-1)}
}

fn swap @A (a: &A, a': &A): Unit {
    temp = !a;
    a << !a';
    a' << temp
}
```

### `trait` & `impl`: ad-hoc polymorphism

```lynx
trait Eq (A: Type) {
  eq: A * A -> Bool
}

impl Eq Int {
  eq = __builtin_eq_int
}
```

### `data`: ADT & GADT

```lynx
data Complex (A: Type) {
  complex @A (_: A, _: A)
}

data Expr (A: Type) {
  atom_expr @A (_: A): Expr A;
  eq_expr @(A, _: Eq A) (_: Expr A, _: Expr A): Expr Bool
}
```

## Modules & namespaces

**TODO**
