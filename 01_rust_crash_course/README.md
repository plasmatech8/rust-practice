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
  - [06. Assertions](#06-assertions)
  - [07. Tuples, Arrays, Vectors](#07-tuples-arrays-vectors)
  - [08. Conditionals](#08-conditionals)

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

An exclamation mark `!` function is a macro. A macro is a special type of function.
It can take a variable number of arguments.

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

## 06. Assertions

We can do runtime assertions:
```rs
assert_eq!(2, 2);
assert_eq!(2, 3);
```
```
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `2`,
 right: `3`', src/assert.rs:3:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## 07. Tuples, Arrays, Vectors

Tuples:
* Max 12 elements
* Each element can have any type
* Access elements using `person.0`
* Stack allocated

Arrays:
* Fixed length
* Only one type
* Use `[i32; 3]` to specify the type and length
* Use `[42; 3]` to create an array of length 3, containing 42s
* Stack allocated
* Slice references are possible (but you cannot mutate which are referenced by the slice)

Vectors:
* Growable arrays

Note: we can write `mem::size_of_val` instead of `std::mem::size_of_val` if we use `use std::mem`.

```rs
pub fn run() {
    // Tuples
    println!("--- Tuples ---");

    let person = ("Brad", "Mass", 37);
    let person_again: (&str, &str, i8) = ("Brad", "Mass", 37);
    println!("{:?}", person);
    println!("{:?}", person_again);
    println!("{} | {} | {}", person.0, person.1, person.2); // indexing

    // Arrays
    println!("--- Arrays ---");

    let mut arr1 = [1, 2, 3, 4, 5, 6];
    arr1[0] = 42; // Assignment
    println!("{:?}", arr1);

    let arr2: [i32; 5] = [1, 2, 3, 4, 5]; // Same element N times
    let arr3: [i32; 3] = [10; 3];
    println!("{:?}", arr1);
    println!("{:?}", arr2);
    println!("{:?}", arr3);

    let arr4: [[i32; 3]; 3] = [[1; 3]; 3]; // 2D array
    println!("{:?}", arr4);
    println!("{:?}", arr4[2][2]); // Indexing (&arr4[2][2] also works)
    println!("{:?}", arr4.len()); // Length
    println!(
        "[[i32; 3] is {:?} bytes (i32*3*3 = 4bytes*3*3)",
        std::mem::size_of_val(&arr4)
    );

    let slice: &[i32] = &arr1[1..3]; // Slice (reference)
    println!("{:?}", slice);

    // Vectors
    println!("--- Vectors ---");

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    println!("{:?}, {}", numbers, numbers.len());
    numbers.push(1337);
    println!("{:?}", numbers);
    numbers.pop();
    println!("{:?}", numbers);
    // Iteration
    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    // Mutable iteration
    for x in numbers.iter_mut() {
        *x *= 2
    }
    println!("{:?}", numbers);
}
```

## 08. Conditionals

