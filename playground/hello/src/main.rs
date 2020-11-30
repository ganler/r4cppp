// name an unused var like `_*` can supress compiler warning;
fn foo(_x: &'static str) -> &'static str {
                    // `-> T` : return type;;; 
    let world: &'static str = "world"; // or just : let world = ...
    // C++:  T x;
    // Rust: x: T;
    world // = `return world;`
}

fn main() {
    println!("Hello, {}!", foo("bar"));
    // `println!` is Rust's equivalent of printf.
    // `!` means this is a macro.
    //  A subset of the standard library is available without needing to
    // be explicitly imported/included (the prelude). The `println!` macro is included
    // as part of that subset.
}

/*
Steps: 

cargo new --bin hello

# Inside the module.
cargo build
cargo run
*/