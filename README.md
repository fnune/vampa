# Vampa

## MVP

Since the MVP will only deal with integers, I can skip implementing type-checking because everything is an `i32`. Perhaps I will need to implement a type for functions.

It should include:

- Comments
- All the arithmetic operations
- The ability to declare and define variables of type `i32`
- The ability to declare functions (with and without body brackets)
- The ability to call functions

The following program should compile and run:

```
let first: i32 = 20;
let second: i32 = 22;

fun sum of a: i32 and b: i32 returning i32 = + a b;
fun sum_with_brackets of a: i32 and b: i32 returning i32 = { + a b };

sum first second; # 42
sum 10 12; # 22
sum { 10 } 10; # 20
sum { + 2 2 } 5; # 9
```

## General characteristics

- Expression-oriented
- Immutable by default
- Strictly typed
- Supporting type inference
- Compiled using LLVM as a backend

## Basic syntax

Vampa is case-sensitive and uses the Unicode character set.

- Comments can be inline or block
  - Inline comments
  - Block commments

```
# An inline comment

#[
# Hey there
# This is my block comment
#
# #[
# # This is a nested block comment
# # Hey there
# #]
#]
```

- Statements are terminated by a semicolon
- Makes use of curly brackets `{}` for blocks
- Identifiers must start with a letter and must not include `:`
- Variables are declared using `let`
  - They must be initialized at the time of their declaration
  - They are either in the scope of their module or in the scope of a block

```
let poem = "The revery alone will do, if bees are few";
```

- Functions are declared using `of` to initiate a list of arguments, and `and` to separate arguments, and then `=` to start the body of the function, which can have a body `{}` or a body without brackets

```
# Using a body without brackets
fun sum of first_number and second_number = + first_number second_number;

# Using a body with brackets, implicitly returning the last expression
fun sum of first_number and second_number = { + first_number second_number };
```

- Functions calls and arguments in function calls are delimited with spaces:

```
let aggregate = sum 5 7;
```

- Type hints are indicated using a colon `:`, must be placed next to the identifier they refer to, and can be used:
  - Next to variable identifiers to indicate their type
  - Next to function identifiers to indicate their return type

```
let poem: string = "The revery alone will do, if bees are few";
fun sum of first: i32 and second: i32 returning i32 = first + second;
```

- Types are declared using the `type` keyword.

```
type Uuid = string;
```

- Generic types are indicated using square brackets `[]`:

```
type Five[T] = [5 of T];
```

## Data structures

### Scalar types

- Boolean: `true` and `false`

#### Integer types

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

#### Floating-point types

- `f32`
- `f64`

#### The boolean type

- `true`
- `false`

#### The character type

It's four bytes in size.

- `char`

#### The string type

- `string`

### Compound types

#### The tuple type

- `[char, i32]`: a tuple of a `char` and an `i32`

#### The array type

- `[5 of char]`: an array of length 5 containing `char`s

## Arithmetic operations

These are used in prefix position and can be combined using curly brackets `{}` to indicate precedence explicitly.

- `+`
- `-`
- `/`
- `%`
- `*`

## Reserved keywords

- Any variable starting with any symbol that's not a letter
- All the types listed above
- `fun`
- `let`
- `and`
- `of`
- `type`
