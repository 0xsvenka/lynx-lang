# Lynx Syntax Specification

## Lexical rules

*Notes on the regexes below:*

-   These regexes adhere to [PCRE2 flavor](https://www.pcre.org/current/doc/html/pcre2pattern.html).

-   Assume that each regex is led by `\G`, which anchors its beginning to the end of the previous match.

### Keyword

All keywords are reserved and cannot be used as identifiers.

-   **Alphabetic keyword** (`_` included)

    ```re
    case|default|do|else|if|import|in|infix[lr]?|inline|namespace|of|open|then|where|_
    ```

    Shown in table:

    | Keyword     | `TokenKind`  |
    |:-----------:|:------------:|
    | `case`      | `Case`       |
    | `default`   | `Default`    |
    | `do`        | `Do`         |
    | `else`      | `Else`       |
    | `if`        | `If`         |
    | `import`    | `Import`     |
    | `in`        | `In`         |
    | `infix`     | `Infix`      |
    | `infixl`    | `Infixl`     |
    | `infixr`    | `Infixr`     |
    | `inline`    | `Inline`     |
    | `namespace` | `Namespace`  |
    | `of`        | `Of`         |
    | `open`      | `Open`       |
    | `then`      | `Then`       |
    | `where`     | `Where`      |
    | `_`         | `Underscore` |

-   **Symbolic keyword**

    ```re
    ~>?|`|@|\$|%~?|\(\|?|\[\|?|{\|?|\)|\]|}|\|[\)\]}]?|->|=>?|:[=:]?|;|,|\.|\?
    ```

    Shown in table:

    | Keyword     | `TokenKind`     |
    |:-----------:|:---------------:|
    | `~`         | `Tilde`         |
    | `~>`        | `TildeArrow`    |
    | `` ` ``     | `Backtick`      |
    | `@`         | `At`            |
    | `$`         | `Dollar`        |
    | `%`         | `Percent`       |
    | `%~`        | `PercentTilde`  |
    | `(`         | `Lp`            |
    | `(\|`       | `LpPipe`        |
    | `[`         | `Lb`            |
    | `[\|`       | `LbPipe`        |
    | `{`         | `Lc`            |
    | `{\|`       | `LcPipe`        |
    | `)`         | `Rp`            |
    | `\|)`       | `PipeRp`        |
    | `]`         | `Rb`            |
    | `\|]`       | `PipeRb`        |
    | `}`         | `Rc`            |
    | `\|}`       | `PipeRc`        |
    | `\|`        | `Pipe`          |
    | `->`        | `Arrow`         |
    | `=`         | `Bind`          |
    | `=>`        | `FatArrow`      |
    | `:`         | `Colon`         |
    | `:=`        | `Assign`        |
    | `::`        | `DoubleColon`   |
    | `;`         | `Semicolon`     |
    | `,`         | `Comma`         |
    | `.`         | `Dot`           |
    | `?`         | `Question`      |

### Identifier (`TokenKind::Id(String)`)

-   **Alphabetic identifier**

    ```re
    [️a-zA-Z_][a-zA-Z0-9_'!]*
    ```

-   **Symbolic identifier**

    ```re
    (?!--)([~!@#$%^&*\-+=|:<>.?\/][~!@#$%^&*\-+=|:<>.?\/'_]*)
    ```

### Binary operator (`TokenKind::BinOp(String)`)

**TODO**

### Literal

#### Integer literal (`TokenKind::IntLit(i64)`)

**TODO**

#### Floating-point number literal (`TokenKind::FloatLit(f64)`)

**TODO**

#### Character literal (`TokenKind::CharLit(char)`)

**TODO**

#### String literal (`TokenKind::StrLit(String)`)

**TODO**

### Comment

#### Line comment

```re
--.*$
```

#### Doc comment

**TODO**

### Whitespace

**TODO** 
