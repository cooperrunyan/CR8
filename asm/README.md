# `asm`

Custom `CR8` Assembly compiler.

Note: [`core`](./src/builtin/core/README.md) is automatically imported into
every invocation of `compile`.

## Syntax

Conventionally, files end with the `.asm` prefix and follow the following
syntax.

```cr8
; this is a line comment

; example of a meta attribute to tell the compiler certain information
; that is not an actual operation like `mov` or `add`
#[static(CONST: 5)]

single_word_instruction
another %b, %v

label:
    instruction_with_args %a, 2

    .scoped_label:
        inst %a, CONST + 12 ; comment
```

Each file contains 0 or more items. An item is one of:

- [label](#labels)
- [instruction](#instructions)
- [meta](#meta-attributes)

## Labels

Labels are defined with `label_name:`. They can be used in expressions to
calculate its address. By convention, anything inside the label should be
indented. Labels can be thought of as functions in typical languages.

```cr8
label:
    mov %a, %b
    ; ...

jmp label
```

If a label starts with `.` (ex: `.loop`), it will be a sub-label of the
previously defined label. In the following example, the labels `.one` and `.two`
can only be accessed from within `outer` or one of its sublabels.

```cr8
outer:
    jmp .one

    .one:
        jmp .two

    .two:
        ret

other:
    jmp outer
```

> Note that these `jmp` calls would be pointless in an actual program because
> they just jump to the following instruction.

## Instructions

Can have 0 or more [values](#values) as arguments. Arguments are comma-separated
(no trailing comma allowed) and cannot span multiple lines.

```cr8
inst val, val ; comment
inst val ; comment
inst ; comment
```

There are a limited amount of native instructions. To solve this problem,
[macros](#macros) can be used to create new instructions or extend the
functionality of builtin instructions. See [core](./src/builtin/core/README.md).

### Instruction Encoding

- Instructions are 2-4 bytes long.
- First byte of the instruction looks like `XXXXYZZZ`, where:
  - `X`: Instruction code (see above)
  - `Y`: Is-Immediate (Denotes whether the instruction uses an immediate or
    register value)
  - `Z`: Register (the first register argument)

```text
   imm
    |
00000000    00000000    00000000
└──┘ └─┘    └──────┘    └──────┘
op   reg    imm(low)    imm2(high)
```

## Values

An instruction's arguments can each be one of the following:

- [literal](#literals)
- [register](#registers)
- [constant](#constants)
- [label](#labels)
- [expr](#expressions)
- [macro variable](#arguments)

### Literals

Values like `1`, `0b0010`, or `0xF000`.

> Can contain `_` to separate number characters visually.

#### Bases

- Base ten (default)
- Hexadecimal (starts with `0x`)
- Binary (starts with `0b`)

### Registers

Registers are written as `%a` for `A`.

See [`registers`](../README.md#registers).

### Constants

Constant values should be in SCREAMING_SNAKE_CASE and can be set with the
`static` meta attribute. It tells the compiler that any use can be replaced with
the static value.

### Expressions

Values that can be computed during compile-time. Expressions follow the
following syntax:

```cr8
_ (1 + 3) >> (3 * 2) + 2
_ 1 + 2
_ CONST + 2
```

#### Operations

Valid operations in expressions are:

- `*`: Multiplication
- `/`: Division
- `-`: Subtraction
- `+`: Addition
- `<<`: Left shift
- `>>`: Right shift
- `|`: Logical OR
- `&`: Logical AND

#### Terms

Expressions can operate on any [`value`](#values) except for
[`registers`](#registers).

```cr8
_ CONST + label - $macro_variable
```

A common thing to see is:

```cr8
_ label >> 8   ; high byte
_ label & 0xFF ; low byte
```

> Doing this will allow the programmer to work with addresses.

## Meta Attributes

Items that tell the compiler extra information.

- [`main`](#main)
- [`use`](#use)
- [`static`](#static)
- [`const`](#const)
- [`dyn`](#dyn)
- [`macro`](#macro)

### `#[main]`

Informs the compiler to jump to a label at the beginning of the program.

```cr8
; The computer starts executing at PC = 0, so it would normally begin here.
routine:
    mov %a, %b
    ret

; This tells the compiler to insert a `jmp main` at the very beginning of
; the binary.
#[main]
main: ; Conventionally, this label is called `main`
    call routine
```

### `#[use]`

Import the contents of another `.asm` file. Argument can be either a
(rust-inspired) module path to a [`builtin`](./src/builtin/core/README.md)
module or a quoted string.

```cr8
; builtin modules
#[use(std::gfx)]
#[use(core::sym)]

; custom import
#[use("hello")]
#[use("./hello/mod.asm")]
#[use("./hello")]
#[use("./hello.asm")]
```

> Note that the three `use` calls for `hello` can all match the file
> `./hello/mod.asm` and they all (except "... /mod.asm") can match the file
> `./hello.asm`.

The compiler tracks all files that get used to prevent circular-imports. This
prevents things like `ifndef` because files can import any dependencies they
have without worrying about whether or not they have already been imported.

> TLDR: `use` can also be thought of as `import-if-not-imported-already`.

```cr8
; a.asm
#[use("shared")]

; b.asm
#[use("shared")]

; shared.asm
; ...

; c.asm
#[use("a")]
#[use("b")]
; Even though "a" and "b" both import "shared", "shared.asm" only gets
; included into the compiler context once.
```

#### Filesystem

```cr8
; path/to/program.asm
#[use("hello")]
```

The compiler will check for and use any of the following files:

- `path/to/hello`
- `path/to/hello.asm`
- `path/to/hello/mod.asm`
- `path/to/hello/main.asm`
- `$PWD/hello`
- `$PWD/hello.asm`
- `$PWD/hello/mod.asm`
- `$PWD/hello/main.asm`

### `#[static]`

```cr8
#[static(X: 12)]
#[static(Y: 0b0010_0100)]
#[static(Z: 0xFF00)]

; later

; move low byte of Z into %a and high byte into %b
mov %a, Z & 0xFF
mov %b, Z >> 8
```

Variables are not scoped; if `A` is defined in `a.asm` and `b.asm` imports
`a.asm`, `b.asm` can access `A`.

> Variables names may not be re-used.

### `#[const]`

```cr8
#[const(DATA)] { 0, 0, 0 }
#[const(DATA2)] {
   0xff, 0xff, 0xff, ; Trailing commas are allowed
}
```

Exactly where `const` is called, the compiler will insert its bytes into the
binary. This is used for data that will be stored on ROM and never changed.

For functionality that allows values to change, see [`dyn`](#dyn).

### `#[dyn]`

```cr8
; Specify where in ram to store variables at
#[dyn(&0xC000)]

; variable name: byte-length it occupies in memory
#[dyn(VAR:  2)] ; VAR  = 0xC000
#[dyn(VAR2: 3)] ; VAR1 = 0xC002
#[dyn(VAR3: 1)] ; VAR3 = 0xC005
```

`dyn` vs `static` vs `const`:

- `dyn`: Store data in a specified location in RAM.
- `static`: Immutable data used only at compile-time.
- `const`: Immutable data stored in ROM exactly where `const` was called.

### `#[macro]`

Define a [`macro`](#macros)

## Macros

Instruction-set is extremely minimal but the assembler offers extensibility with
rust-inspired macros. See `asm/src/builtin` for the `std` and `core` libraries
which contain macros and other functions.

Macros can be used to either create new instructions or extend the capabilities
of native ones.

After its definition, a macro cannot be redefined elsewhere.

### Definition

```cr8
#[macro] macro_name: {
    ; First capture
    ($arg: reg, ...) => {
        ; ...
    }
    ; another capture
    (...) => {}
}
```

Inside the macro declaration block contains macro captures (inspired by
[Rust](https://rust-lang.org)). Captures are not separated by commas. A capture
takes the form: `(` [`arguments`](#arguments) `)` `=>` `{` [`body`](#body) `}`.

#### Arguments

The arguments of the capture appear similar to the arguments of a function
definition in common programming languages.

Example:

- None
- `$name` `:` [`type`](#types)
- `$arg0` `:` [`type`](#types) `,` `$arg1` `:` [`type`](#types)

#### Types

Each macro variable can be one of:

- `reg`: Any [`register`](../README.md#registers) (ex: `%a`)
- `lit`: A literal number (ex: `5`)
- `expr`: An [`expression`](#expressions)
- `any`: Either a [`register`](../README.md#registers) or a literal number

#### Body

A macro's body consists of any instruction or macro between `{` and `}`.

> A macro can call other variants of itself in its definition. When doing this,
> beware of accidental infinite recursion.

### Usage

The macros can be used the same as
[Native Instructions](../README.md#instructions).

```cr8
macro_name %a
macro_name
macro_name %a, %b
macro_name 12 + 3
; etc.
```
