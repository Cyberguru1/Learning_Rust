use std::io;

// modules learnt
// printing to stdout using println!
// reading user input with the std::io stdin()
// comments

/// Crate comments
fn main() {
    //! # main funcition <<< header function
    //! 
    //! ```
    //! fn main()
    //! ```
    //! read user input and print it 
    //! to stdout
    //! 
    println!("Hello, world!");

    let mut input = String::new();
    println!("Enter Something>>>:");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You said {}", input);
        },
        Err (e) => {
            println!("Something went wrong {}", e)
        }
    }

    // println! is a macro
    // formatting
    println!("MY name is {} and am running on {}", "rust", "compiler");

    // Positional formatting
    println!("{0}, has two {1}", "rust", "compiler");

    // Key value pairs based formatting
    println!("{name}, is good in {action}", name="rust", action="safe memory");

    // type traits
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 10, 10, 10);

    // debugging
    println!{"Array {:?}, {:?}", [1, 2, 3], "out"};
    //
}
