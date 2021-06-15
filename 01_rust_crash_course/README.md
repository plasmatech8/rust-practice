# Rust Crash Course

See:
* [Rust Crash Course | Rustlang](https://www.youtube.com/watch?v=zF34dRivLOw) by Traversy Media on youtube

Contexnts:
- [Rust Crash Course](#rust-crash-course)
  - [Introduction](#introduction)
  - [Setup](#setup)
  - [01. Creating Rust File & Compile](#01-creating-rust-file--compile)
  - [02. Initialise a project with Cargo](#02-initialise-a-project-with-cargo)
  - [03. Creating and importing module/function](#03-creating-and-importing-modulefunction)
  - [04. Print formatting & debug](#04-print-formatting--debug)
  - [05. Variables](#05-variables)
  - [06. Types](#06-types)

## Introduction

Rust is a systems programming language.

Features:
* Unlike C/C++, Rust is type and thread safe!
* Good for WebAssembly and web projects.
* Rust does not have garbage collection, but you do not need to manage memory.
  * The memory heap is checked on-demand when memory is needed

## Setup

Install VSCode extension:
* Rust
* Better TOML
* If langauge server does not work try: `"rust-client.rustupPath": "~/.cargo/bin/rustup",`


Follow Rustinstallation instructions:
* https://www.rust-lang.org/tools/install
* Restart terminal.

Try:
* `rustup --version`
* `cargo --version`

Add terminal/bash completions by:
```bash
mkdir -p $(brew --prefix)/etc/bash_completion.d
$ rustup completions bash > $(brew --prefix)/etc/bash_completion.d/rustup.bash-completion
```

## 01. Creating Rust File & Compile

In `hello.rs`:
```rs
fn main() {
    println!("Hello World");
}
```

Compile: `rustc hello.rs`

Run: `./hello`

## 02. Initialise a project with Cargo

Init: `cargo init example_02`

Project:
* `src/main.rs` source file
* `cargo.toml` config file

Commands:
* Compile and run: `cargo run`
* Compile: `cargo build`
* Compile: `cargo build --release` (optimised/release)

Target:
* The executable is under `target/debug/<EXE>`

## 03. Creating and importing module/function

In `print.rs` we can create a public function:
```rs
pub fn run() {
    // Print to console
    println!("Hello from the print.rs file")
}
```

In `main.rs` we can import the module and run the function:
```rs
mod print;
//...

print::run();
```

## 04. Print formatting & debug

Use curly braces to print stuff.

Supports:
* Position, index, and named parameters .
* Placeholder traits (i.e. decimal places, padding, etc) using `:`
* Printing objects/arrays using `"{:?}"`

```rs
pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Print Formatting
    println!("Number: {}", 1);
    println!("Number: {} {} {}", 0, 1, 2);
    println!("Number: {1} {0} {1}", 0, 1);
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Basketball"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10)
    println!("Pi: {:.3}", 3.14159265359);
}
```

## 05. Variables

All variables are immutable. They hold primitive data or references.

The variables cannot be changed unless you use the `mut` keyword.

In `vars.rs`:
```rs
pub fn run() {
    // Immutable/mutable variables
    let name = "Brad";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // Contants
    const ID: i32 = 2;
    println!("ID: {}", ID);

    // Assign multiple vars (unpacking)
    let (my_name, my_age) = ("Brad", 37);
    println!("{}, {}", my_name, my_age)
}
```

Rust is cool because the error messages are helpful, and if a pointless variable exists
(i.e. age is never used while it is 37) then it will show a warning.

## 06. Types

Rust will automatically infer types to a default types (i32, etc).

If we need a different type, we can use explicit typing: `:i64`

```rs
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn run() {
    // Primitive types
    /*
    Integers:   u8, i8, ... , u128, i128
    Floats:     f32, f64
    Boolean:    bool
    Character:  char
    Tuples:     (,)
    Arrays:     [,]
    */
    // Integers
    println!("Max for i32: {}", std::i32::MAX);
    let x: i64 = 2147483648;
    print_type_of(&x);

    // Arrays & tuples
    let arr1 = [1, 2, 3, 4, 5, 6];
    let arr2: [i32; 3] = [0; 3];
    println!("{:?}", arr1);
    println!("{:?}", arr2);
    println!("{:?}", (1, true, "hello"));

    // Strings and Chars
    let string = "Hello World!";
    let character = 'c';
    let unicode_char = '\u{1F600}';
    println!("{} {} {}", string, character, unicode_char);
    let mut string_builder = String::from("Hello"); // alloc::string::String
    println!("{}", string_builder);
    string_builder.push_str(" World!");
    println!("{}", string_builder);
    for word in string.split_whitespace() {
        println!("{}", word);
    }
}
```