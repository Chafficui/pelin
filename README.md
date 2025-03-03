# Pelin Programming Language

[![Rust CI](https://github.com/Chafficui/pelin/actions/workflows/testBuild.yml/badge.svg)](https://github.com/Chafficui/pelin/actions/workflows/testBuild.yml)

> **Note:** This is a learning project created to explore language implementation concepts and as a way to practice Rust programming. While functional, it's not intended for production use and may contain imperfections or unoptimized elements.

Pelin is a lightweight, modular programming language implemented in Rust, designed for extensibility through the "Feather" system.

## Overview

Pelin is a dynamically typed language with a simple and expressive syntax. The core design principle of Pelin is modularity - standard libraries are implemented as "Feathers" that can be easily imported and extended.

## Features

- Simple, clean syntax inspired by modern programming languages
- Dynamically typed with support for numbers, strings, booleans, and null values
- First-class functions with closures
- Modular design through the extensible Feather system
- Seamless integration with Rust for performance-critical code
- Built-in standard library modules for common operations

## Language Basics

### Types

Pelin supports the following basic types:
- `num` - Floating-point numbers
- `str` - Text strings
- `bool` - Boolean values (true/false)
- `nun` - Null value (similar to `null` or `None` in other languages)
- `any` - Generic type for functions that accept any type

### Functions

Functions are defined using the `fn` keyword:

```
fn num add(num a, num b) {
    RUST[std_func::add](a, b)
}
```

### Importing Feathers

Standard modules and custom libraries are imported using the `imp` keyword:

```
imp std_num
imp std_io

std_io.print(std_num.add(5, 10))
```

### Calling Rust Functions

One of Pelin's powerful features is direct integration with Rust code through the `RUST` keyword:

```
RUST[std_func::add](5, 10)
```

## Standard Feathers

### Math Operations (`std_num`)
```
add, subtract, multiply, divide, sqrt
```

### Mathematical Functions (`std_math`)
```
sin, cos
```

### Logic Operations (`std_logic`)
```
and, not, or, xor
```

### Comparison (`std_comp`)
```
eq, neq, lt, lte, gt, gte
```

### Type Conversion (`std_convert`)
```
to_num, to_str
```

### Input/Output (`std_io`)
```
print
```

### File Operations (`std_file`)
```
read_file, write_file
```

## Example Program

```
imp std_file
imp std_io
imp std_comp

fn nun write_file(str path, str content) {
    std_file.write_file(path, content)
}

write_file("test.txt", "Hello, World!")
std_io.print(std_comp.eq(3, 3))
```

## Building and Running

### Prerequisites

- Rust 1.50 or higher
- Cargo

### Building the Interpreter

```bash
cargo build --release
```

### Running a Pelin Program

```bash
./pelin your_program.pl
```

### Running Tests

```bash
cargo test
```

## Extending Pelin

### Creating Custom Feathers

Feathers are Pelin's way of organizing modules and libraries. To create a custom feather:

1. Create a `.pl` file in the `feathers` directory or at a custom path
2. Define your functions using the standard Pelin syntax
3. Import your feather using `imp your_feather_name`

### Creating Custom Rust Extensions

Pelin can be extended with Rust code for performance-critical operations:

1. Create a Rust library with exported functions
2. Build the library and place it in the `rust_libs` directory
3. Call your Rust functions using the `RUST[your_lib::your_function]` syntax

## Architecture

Pelin's architecture consists of several key components:

- **Lexer** (`lexer.rs`): Tokenizes source code
- **Parser** (`parser.rs`): Parses tokens into abstract syntax tree
- **Interpreter** (`interpreter.rs`): Executes the parsed expressions
- **FeatherManager** (`feather.rs`): Manages feather modules and Rust integration

## License

[LICENSE](LICENSE.md)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.