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