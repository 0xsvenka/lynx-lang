# Lynx programming language overview

Lynx is an expression-oriented language designed around explicit structure, uniform reference semantics, and operator-driven syntax. Its core philosophy emphasizes clarity, predictable semantics, and composable abstractions, while providing powerful tools like pattern matching, first-class types, and macros.

## Basic syntax

### Layout rule

Expressions are terminated with the semicolon; whitespace and indentation are insignificant.

```lynx
a = 1; b =
a;
c = a + b;

println c;
```

#### Block

Multiple expressions can be grouped into a **block** with curly braces, e.g. `{a = 1; b = a;}`. Blocks are a purely syntactic structure existing at compile time and do not carry any semantic meanings on their own; they are usually used as [macro](#macros) arguments.

### Atoms

#### Literals

##### Unit

The unit literal `()` is the only value of type `Unit`.

```lynx
assert_eq (type_of println, Str -> Unit);
assert_eq (println "Hello", ());
```

##### Number (integer & floating-point)

Integer and floating-point literals are typed `Int` and `Float` respectively. They may be of arbitrary size and precision, limited only by memory.

##### Character

Character literals are typed `Char`. They must be be valid UTF-8.

##### String

String literals are typed `Str`. They must be valid UTF-8.

There are two kinds of string literals:

- **Quoted string** is delimited by double quotes. It supports escape sequences but may not span multiple lines, e.g. `"Hello,\nWorld!"`.

- **Raw string** begins with `\\` and extends to the end of the line, with no escape processing, e.g.

  ```lynx
  \\Each line starts with "\\" ...
  ```

Adjacent string literals are concatenated, with line breaks inserted between:

```lynx
s = "This is a multi-line..."
-- Comments do not break it
    \\...string literal.
    ;
```

#### Wildcard (`_`)

#### Name

There are two kinds of names:

- Alphabetic name: `[A-Za-z_][A-Za-z0-9_'!]*`.

- Symbolic name: ``[~`!@#$%^&*+=|:<>.?/]`` **TODO**

Note, however, that their difference is solely lexical, and they are equivalent in functionality.

##### The `mut` modifier

`mut a` declares **mutable name** `a`, i.e. it may be [rebound](#binding-expression) to another value.

### Composite values

#### Tuple

- Syntax: `a, b, c`.

- Typing: `1, 'A'` is typed `Int * Char`.

The composite nature of tuples means that nullary or unary tuples do not exist; neither atoms nor `()` is a tuple.

Due to the relatively low precedence of `,`, tuples often require surrounding parentheses, as in `f (a, b)`. These parentheses serve the sole duty of grouping and are not part of the tuple syntax.

#### List

- Syntax: `[a0, a1, a2]`.

- Typing: `[1, 2]` is typed `List Int`.

  All elements must share the same type.

#### Map

- Syntax: `map [(k0, v0), (k1, v1), (k2, v2)]`.

  Maps are introduced by the `map` [macro](#macros). Key-value pairs are written as binary tuples and wrapped in a list. The order of them is insignificant.

- Typing: `map [(1, 'A'), (2, 'B')]` is typed `Map (Int, Char)`.

  All keys share one type and all values share one type; of course, these two types may differ.

#### Record

- Syntax: `rec (k0 = a, k1 = b, k2 = c)`.

  Records are introduced by the `rec` [macro](#macros).

- Typing: denoted by the `Rec` macro, e.g. `rec (task = "write Lynx", todo = true)` is typed `Rec (task: Str, todo: Bool)`.

Records are represented as tuples at the root level. Nevertheless, since record items are labeled, the order of them is insignificant.

### Binding expression

Syntax: `pattern = expr`.

Example:

```lynx
p = 1, 2; x, _ = p;
println x;
```

Mutable names may be rebound with `:=`. Example:

```lynx
a = 1; mut a' = a;
(a', _) := (2, 0);
println a;  -- 1
println a';  -- 2
```

### Lambda expression

Syntax: `param_pattern => expr`.

The `=>` operator is right‑associative, convenient for creating higher-order functions.

Example:

```lynx
get_x = (x, _) => x;
add = x => y => x + y;
```

Note, however, that most functions are not defined this way directly; instead, the [`fn` macro](#fn-macro-function-definition) is more common.

### Function application

Syntax: `f x` (juxtaposition).

Function application is left-associative: `f x y` is equivalent to `(f x) y`.

### Operator

Lynx syntax relies heavily on operators. During parsing, even symbols like `,` and `=>` are handled as operators; this empowers the parser to follow a unified Pratt algorithm based on [operator precedence and associativity](#precedence-and-associativity-of-standard-operators).

There are three kinds of operators: prefix, infix, and suffix, all of which may be enriched by user-defined ones. This allows for enormous flexibility.

```lynx
infixl * 70;  -- Left associative, precedence set to 70
fn ((*) @(A, B, m: Mul (A, B)) (a: A) (b: B): m.R) {
  m.mul (a, b);
};
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

In a type expression, you may name any parameter for later use. `A ~ B` is the type-context equivalent of value-context `B : A`.

Example:

```lynx
my_map: (Type*Type ~ (A, B)) -> (A -> B) -> List A -> List B) = (_, _) => map;
my_map (Int, Str) to_str [1, 2, 3];
```

### Contextual parameter

Syntax: `@ParamType`.

Contextual parameters do not need to be passed explicitly; instead, they are inferred lexically from the context (e.g. through parameter annotation, trait implementation search, etc). Nevertheless, you may also pass them explicitly with the `@` prefix.

Example:

```lynx
assert_type (map, @(Type*Type ~ (A, B)) -> (A -> B) -> List A -> List B);
my_map = (A, B) => map @(A, B);

assert_type ((==), @((Type~A) * Eq A) -> A -> A -> Bool);
```

### Mutable cell

For any type `T`, the mutable cell type `&T` provides shared mutability. Note that this is a distinct concept from mutable name.

- Create: the `ref` function.

- Inspect: prefix operator `!`.

- Mutate: infix operator `<<`.

Note that the compiler will copy a value if it decides that the latter cannot be shared safely between a `T` and a `&T`.

Example:

```lynx
ra: &Int = ref 1;
rb = ra;
ra << 3;
println !rb;  -- 3
```

## Macros

Macros are compile-time functions that carry out AST transformations. When the Lynx parser encounters a macro call, it applies the macro to the latter's subtree and merges the result into the parent. This process is called **macro expansion**. Of course, macros may be created by the user.

Here are some of the pre-defined macros:

### Control flow macros

#### `do`

Evaluates to the last expression.

```lynx
println (do {
    print "1 + 2 = ";
    1;
} + 2);
```

#### `if`

```lynx
if (n % 3 == 0 && n % 5 == 0) {
    println "FizzBuzz";
} elif (n % 3 == 0) {
    println "Fizz";
} elif (n % 5 == 0) {
    println "Buzz";
} else {
    println n;
};
```

#### `while`

```lynx
mut i = 0;
while (i < 100) {
    println i;
    i := i + 1;
};
```

#### `for`

```lynx
for (k, v) in (map [(1, 'A'), (2, 'B')]) {
  println k;
  println v;
};
```

#### `match`

```lynx
match n {
  1 | 2 => "small";
  _ => "big";
};
```

### `fn`: function definition

```lynx
fn (f (n: Int): Int) {
    if (n == 0) {1}
    else {n * f (n-1)};
};

fn (swap @A (a: &A, a': &A): Unit) {
    temp = !a;
    a << !a';
    a' << temp;
};
```

### `trait` & `impl`: ad-hoc polymorphism

```lynx
trait (Eq (A: Type)) {
  eq: A * A -> Bool;
};

impl (Eq Int) {
  eq = __builtin_eq_int;
};
```

### `data`: ADT & GADT

```lynx
data (Complex (A: Type)) {
  complex @A (_: A, _: A);
};

data (Expr (A: Type)) {
  atom_expr @A (_: A): Expr A;
  eq_expr @(A, _: Eq A) (_: Expr A, _: Expr A): Expr Bool;
};
```

## Module system

**TODO**
