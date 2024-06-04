#[allow(unused_allocation)]
#[allow(unused_assignments)]
#[allow(non_snake_case)]
use crate::Colors::Red;
use crate::Person::Name;
use crate::RGB::RED;


#[derive(Debug)]
enum RGB<T>{
    RED(T),
    GREEN(T),
    BLUE(T)
}


#[derive(Debug)]
enum Person {
    Name(String), 
    Surname(String),
    Age(u32)
}
#[derive(Debug)]
enum Colors {
    Red,
    Green,
}

#[derive(Debug)]
struct Point2<T, V> {
    x: T,
    y: V
}

// Data types
// Arrays
// Vectors
// Slices
// Tuples
// Structures
// Enums
// Generic
fn main() {
    println!("Hello, Data types!");

    // Arrays

    let primes = [2, 3, 5, 7, 11];
    let doubles: [f64; 4] = [1.0, 2.0, 3.0, 4.0];

    println!("{:?}, {:?}", primes, doubles);

    let mut numbers = [0; 15];

    println!("{:?}", numbers);

    const DEFAULT: i32 = 3;

    let mut numbers = [DEFAULT; 15];

    println!("{:?}", numbers[3]);

    for num in numbers.iter() {
        println!("{}", num);
    }

    // Vectors

    let mut primes: Vec<i32> = Vec::new();

    let mut primes1 = vec![2, 3, 5];

    println!("primes1: {:?}, primes: {:?}", primes1, primes);

    // adding elements
    
    primes.push(7);
    println!("{:?}", primes);

    primes1.remove(2);
    println!("{:?}", primes1);

    // defalut values

    let mut nums = vec![DEFAULT; 10];
    println!("{:?}", nums);

    for num in nums.iter() {
        println!("{}", num);
    }

    // Slices
    // pointer to a block of memory that points to an array/vector

    let numbers = [1, 2, 3, 4];
    let slice = &numbers[2..4];
    println!("{:?}", slice);

    let mut colors = ["red", "green", "blue", "brown"];
    update_colors(&mut colors[1..4]);

    println!("{:?}", colors);

    // Tuples -- limited to just 13 elements
    // colletion of various types

    let person = ("john", "holland", 28);
    println!("{:?}", person);
    println!("\nperson names is {}", person.0);
    // dereferencing is also possible --- destructuring


    // Structures
    // key value pairs or dictionary
    // used for class in rust
    // interface implementation too.

    #[derive(Debug)] // annotate to make it printable
    struct Employee {
        name: String,
        company: String,
        age: i32
    }

    impl Employee {
        fn print_details(&self) -> String {
           format!("MR {} is working in {} with an age of {}", &self.name, &self.company, &self.age)
        }

        fn static_fn_detail() -> String {
            String::from("Details of a person")
        }
    }

    let emp = Employee {
        name: String::from("Rust"),
        company: String::from("DIA"),
        age : 20
    };

    println!("\n{:?}", emp);

    // function or class interface

    println!("\n{}",emp.print_details());

    // static function in structure
    println!("\n{}", Employee::static_fn_detail());

    // Enums
    #[derive(Debug)]
    enum Colors {
        Red,
        Green,
        Blue
    }

    let my_color = Colors::Red;
    println!("\n{:?}", my_color);

    let my_color = Red;

    // Enum with data types
    // #[derive(Debug)]
    // enum Person {
    //     Name(String), 
    //     Surname(String),
    //     Age(u32)
    // }

    let person = Name(String::from("alex"));

    println!("\n{:?}", person);

    // Generics
    // used for variable data types

    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T
    }

    let p1: Point<i32> = Point {x:0, y:33};
    let p2: Point<f64> = Point {x:3.3, y:5.5};

    println!("\n{:?}", (p1, p2));

    // #[derive(Debug)]
    // enum RBG <T> {
    //     RED(T),
    //     BLUE(T),
    //     GREEN(T)
    // }

    let c1 = RED("#f00");
    let c2 = RED(255);

    println!("\n{:?}", (c1, c2));

    // multiple Generics

    let p1: Point2<i32, f32> = Point2 {x:32, y:4.4};
    println!("\n{:?}", (p1));


}

fn update_colors(colors_slice: &mut [&str]){
    colors_slice[0] = "yellow";
    colors_slice[1] = "whilte";
    colors_slice[2] = "purple";
}