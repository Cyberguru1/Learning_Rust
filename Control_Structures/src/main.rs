// use rand::Rng;

#[allow(unused_assignments)]
#[allow(unused_allocation)]
#[allow(non_snake_case)]


// Control Structures
// If statements
// Mach statement
// Pattern matching
// For loop 
// while loop

fn main() {
    println!("Hello, Control Structures!");

    // let num = Rng.gen_range(0, 11);
    let num = 5;
    if num >= 5 {
        println!("\nnumber {} is greater than or equal to 5", num);
    } else if num == 4 {
        println!("\nnumber {} is equal to 4", num);
    } else {
        println!("\nnumber {} is less than 5", num);
    }

    // something like tenary

    let res = if num >= 5 { true } else { false };
    println!("\n{}", res);

    // Match statement
    // something simillar to switch
    println!("\n");
    print_choice(suit::Heart);
    print_choice(suit::Club);
    print_choice(suit::Spade);
    print_choice(suit::Diamond);

    // match with return statments
    country(54);
    country(34);
    country(44);
    country(0);

    // Pattern matching

    for i in 0..15 {
        println!("{}. i Have {} oranges\n", i, get_oranges(i));
    }

    // Tuple matching

    let point = (6, 2);

    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("x axis ({}, 0)", x),
        (0, y) => println!("y axis (0, {})", y),
        (x, y) => println!("x and y axis ({}, {})", x, y)
    }

    // for loops

    for i in 1..10 {
        println!("\n{} mul by {} = {}", i, i, i*i);
    }

    let pets = ["cats", "cats", "dogs", "hamster", "bear", "sheep"];

    for pet in pets.iter() {
        if pet == &"cats" {
            println!("\nwe have a cat here mate!!!");
            continue
        }
        println!("\n I love my pet {}", pet);
    }

    for (pos, i) in (1..11).enumerate() {
        let square = i*i;
        let nb = pos*i;
        println!("\n{1} * {1} = {0}", square, pos);
    }

    // while loops

    println!("\nWhile loops: ");

    let mut x = 0;

    while x <= 10 {
        println!("\n{0} * {0} = {1}", x, x*x);
        x += 1;
    }

    // infinite loop
    println!("\n infinite loop: variant of while loop");
    loop {
        println!("\n{0} * {0} = {1}", x, x*x);
        x += 1;
        if x > 200 {
            break
        }
    }


    
}

enum suit {
    Heart,
    Spade,
    Club,
    Diamond
}

fn country(code: i32){
    let country = match code {
        44 => "UK",
        34 => "spain",
        1..=999 => "Unknown",
        _ => "Invalid"
    };

    println!("\n COUNTRY IS {}", country);


}

fn print_choice(choice: suit) {
    match choice {
        suit::Heart => { println!("\u{2665}") }
        suit::Spade => { println!("\u{2660}") }
        suit::Club => { println!("\u{2663}") }
        suit::Diamond => { println!("\u{2666}") }
    }

}

fn get_oranges(amount: i32) -> &'static str {
    return match amount {
        0 => "no",
        1 | 2 => "one or two",
        3..=7 => "a few",
        _ if (amount % 2 == 0) => "an even amount of",
        _ => "lots of"
    }
}