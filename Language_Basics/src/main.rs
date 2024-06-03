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

    println!("{}, {}", char1, smiley);

    // Strings (smillar to siles in golang--string slices)



}
