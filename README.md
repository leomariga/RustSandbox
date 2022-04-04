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

``` rust
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

``` rust
let x = 5;

let x = x + 1;
```

Shadowing is different from marking a variable as `mut`, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword. We’re effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name

This works (shadowing):

``` rust
let spaces = "   ";
let spaces = spaces.len();
```

This dont (we’re not allowed to mutate a variable’s type)

``` rust
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

``` rust
let y = {
    let x = 3;
    x + 1
};
```

This expression returns `4`.

- **Functions with Return values**

There are no function calls, macros, or even `let` statements in the `five` function—just the number `5` by itself

``` rust
fn five() -> i32 {
    5
}
```

# Control flow

**loop** - Loops until something `break` it. you can create labels to specify which `loop` you want to break such as:

``` rust
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


# Ownership


Ownership is a set of rules that governs how a Rust program manages memory. 

First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

Each value in Rust has a variable that’s called its owner.
There can only be one owner at a time. When the owner goes out of scope, the value will be dropped.


- string literal: We know the content in compilation time but immutable.
- With `String` type in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

The memory must be requested from the memory allocator at runtime. We need a way of returning this memory to the allocator when we’re done with our String.

muita coisa pra escrever, ler aqui:

https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

String has its memory freed when passing through a function like this:

``` rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```


The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.


## References

A reference is like a pointer in that it’s an address we can follow to access data stored at that address that is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type


``` rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
```

This code don't work:

``` rust

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

```

This don't:

``` rust

fn main() {
    let reference_to_nothing = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

```

The `&s1` syntax lets us create a reference that refers to the value of `s1` but does not own it. We call the action of creating a reference borrowing. 

Let’s recap what we’ve discussed about references:

    - At any given time, you can have either one mutable reference or any number of immutable references.

    - References must always be valid.


## Slices

``` rust

let s = "Hello, world!";

```

The type of s here is `&str`: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; `&str` is an immutable reference.

# Struct

Rust has this interesting thing that you dont need to specify field with the same name as parameters of a function:

This:

``` rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

Is equal to:


``` rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

We can also create another user2 with parameters from user1 such as:

``` rust
fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

The same as: 

``` rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

### Tuple structs:

``` rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

## Method Systax

Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else. Unlike functions, methods are defined within the context of a struct (or an enum or a trait object, which we cover in Chapters 6 and 17, respectively), and their first parameter is always `self`, which represents the instance of the struct the method is being called on.


# Building a project


- Packages: A Cargo feature that lets you build, test, and share crates

- Crates: A tree of modules that produces a library or executable

- Modules and use: Let you control the organization, scope, and privacy of paths

- Paths: A way of naming an item, such as a struct, function, or module


A **crate** is a binary or library. The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate. A **package** is one or more crates that provide a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.