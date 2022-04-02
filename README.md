# RustSandbox
Playing around Rust

https://doc.rust-lang.org/book/index.html

# Commands:

`cargo new hello_cargo` : Start a new project called `hello_cargo`

`cargo build` : Creates an executable file in target/debug/hello_cargo (or target\debug\hello_cargo.exe on Windows) rather than in your current directory. When the code is finished it can be compiled with optimization using `cargo build --release`

`cargo run` : Compile the code (if didn't change) and then run the resulting executable all in one command.

`cargo check` : Check if compiles without compiling it

`rustfmt filename.rs` : Format rust file.

`cargo update` : Update crates packages of your project

`cargo doc --open` : Build documentation from all dependencies

## Variables:

Variables immutable by default, use `mut` otherwise. `mut` cannot alter variable **type**

## Constants 
Constants are:
    - Aways immutable
    - Can be declared in any scope
    - May be set only to a constant expression `const`.

## Shadowing: 
You can declare a new variable with the same name as a previous variable which means that the second variable’s value is what the program sees when the variable is used

```
let x = 5;

let x = x + 1;
```

Shadowing is different from marking a variable as `mut`, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword. We’re effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name

This works (shadowing):

```
let spaces = "   ";
let spaces = spaces.len();
```

This dont (we’re not allowed to mutate a variable’s type)

```
let mut spaces = "   ";
spaces = spaces.len();
```

