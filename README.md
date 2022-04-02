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

## `match`

A `match` expression is made up of arms. An arm consists of a pattern to `match` against, and the code that should be run if the value given to `match` fits that arm’s pattern.

Example handling error 

```
let myindex: u8 = match guess.trim().parse() {
                Ok(num) => num,
                Err(error) => panic!("Problem converting: {:?}", error),
            };
```

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

# Data types
Rust is a statically typed language, which means that it must know the types of all variables at compile time

Has some cool features when overflowing ints. You can set to:
 - `wrapping_*` to wrap around the overflow
 - `checked_*` to return `None`
 - `overflowing_*` return the value (What value?) 
 - `saturating_*` saturate to max and min value

 int, float, bool, char, tuple, array

### obs

 **tuple:**  A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

**array** must have same type and have the same size. If you’re unsure whether to use an array or a vector, chances are you should use a vector.


# Functions
Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.

- **Statements** are instructions that perform some action and do not return a value.

```
let y = 6;
```
does not make sense to do somethink like:

```
let x = (let y = 6);
```

- **Expressions** evaluate to a resulting value. **Expressions do not include ending semicolons**. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value

```
let y = {
    let x = 3;
    x + 1
};
```

This expression returns `4`.

- **Functions with Return values**

There are no function calls, macros, or even `let` statements in the `five` function—just the number `5` by itself

```
fn five() -> i32 {
    5
}
```

# Control flow

**loop** - Loops until something `break` it. you can create labels to specify which `loop` you want to break such as:

```
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
```

You can return the value of a loop after the break like `break counter * 2;` 

**for** and **while** similar to other languages.