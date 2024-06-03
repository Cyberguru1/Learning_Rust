#[allow(unused_variables)] // linting -- could be used to escape error
#[allow(unused_imports)]
#[allow(unused_assignments)] 

// Language Basics
// Variables
// Scalar data types
// Strings
// Constants
// Operators
// Functions

fn main() {
    println!("Hello, world!");

    // Variables

    let name = "Rust";
    let age = 32;

    // strongly typed language is rust
    let amount:i64 = 847382900909;

    // mutable values
    let mut ages = 34;
    ages = 42; 

    println!("{}", ages);

    // shadowing is possible unlike golang

    let color = "blue";
    let color = 34;
    println!("{}", color);

    // declaring multiple variables

    let (a, b, c) = (2, 3, "rust");
    println!("(a, b, c) <==> ({}, {}, {})", a, b, c);

    // Scalar Data Types

    //Type Cast

    let pi: f32 = 3.0;
    
    // using undersocre

    let million = 1_000_000;

    println!("{}", million);

    // Boolean

    let is_day = true;
    let is_night = false;

    println!("we currently in day right ? {} and what of night ? {}", is_day, is_night);


    // Character

    let char1 = 'A';
    let smiley = '\u{1F601}';

    println!("{}, {}\n", char1, smiley);

    // Strings
    // Strings (smillar to siles in golang--string slices)
    // string slices are immutable

    let cat: &'static str = "Fluffy";
    println!("{}\n", cat);

    // string objects

    let dog = String::new();

    let dog = String::from("Max");

    println!("{}\n", dog);

    // string build 

    let mut owner = format!("Hi, Cyberman is learning {}", "Rust");
    println!("{}\n", owner);

    // length of string
    println!("{}\n", dog.len());

    // push to a string
    owner.push('!');

    owner.push_str("\nGood to know man");
    println!("{}\n", owner);

    // replace

    let new_owner = owner.replace("man", "cyberman");
    println!("{}\n", new_owner);

    // Constants
    // shadoing is not supported

    const URL: &str = "rust.com";
    println!("{}\n", URL);

    // Arithmetric Operators

    let a = 2 / 2;
    let b = 10 * 3;
    
    println!("a={}, b={}", a, b);

    // Rational Operators
    println!("a >= b ? {}", a >= b);

    // Functions
    say_hi();

    // For loops

    for i in 1..5 {
        say_hi();
    }

}

fn say_hi(){
    println!{"\nHEllo There!!"};
}
