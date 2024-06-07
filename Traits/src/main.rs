use std::ops::Add;

// Traits
// Generics
// dyn
// Operator overloading
// static dispatch
// dynamic dispatch


// structures to use
struct RustDev {
    awsome: bool
}

struct JavaDev {
    awsome: bool
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}


trait Developer {
    fn new(awsome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) { println!("Hello Dev!!\n") }
}

impl Developer for RustDev {
    fn language(&self) -> &str {
        "Rust\n"
    }

    fn new(awsome: bool) -> Self {
        RustDev { awsome: awsome}
    }

    fn say_hello(&self) {
        println!("println!(\"Hello Dev!\");\n");
    }
}

impl Developer for JavaDev {
    fn language(&self) -> &str {
        "Java"
    }

    fn new(awsome: bool) -> Self {
        JavaDev { awsome: awsome }
    }

    fn say_hello(&self) {
        println!("system.out.println(\"Hello Dev!\");\n");
    }
}

trait Bark {
    fn bark(&self) -> String;
}

struct Dog {
    species: &'static str
}

struct Cat {
    color: &'static str
}

impl Bark for Dog {
    fn bark(&self) -> String {
        return format!("{} barking!!!", self.species)
    }
}

impl Bark for Cat {
    fn bark(&self) -> String {
        return format!("{} barking!!!", self.color)
    }
}


fn bark_it<T: Bark> (b: T) {
    println!("{}", b.bark());
}


fn main() {
    println!("Hello, Traits!\n");

    // Instantiate the interfaces

    let r = RustDev::new(true);
    let j = JavaDev::new(false);
    println!("{}", r.language());
    r.say_hello();
    j.say_hello();

    // Generics in traits

    let dog = Dog { species: "retriever" };
    let cat = Cat { color: "black" };
    bark_it(dog);
    bark_it(cat);

    // Dyn
    // Use for returning traits

    println!("\nThe animal says {}", get_animal(0.4).make_noise());
    println!("\nThe animal says {}", get_animal(1.4).make_noise());

    // operator overloading
    let a = vec![1, 2, 3, 4];
    println!("\nSum of the vector a {:?} is {}", a, a.sum());

    let p1 = Point { x: 0.5, y: 0.5 };
    let p2 = Point { x: 0.5, y: 0.8 };
    let p3 = p1 + p2;
    println!("\nSum of p1 {:?} and p2 {:?} is {:?}", Point { x: 0.5, y: 0.5}, Point { x: 0.5, y: 0.8 }, p3);

    // Static Dispatch
    // process of monomorphization

    let x = 32;
    let b = "Hi John".to_string();

    duplicate(&x);
    duplicate(&b);

    // Dynamic Dispatch





}


trait duplicate {
    fn dupl(&self) -> String;
}

impl duplicate for String {
    fn dupl(&self) -> String {
        format!("{0} {0}", *self)
    }
}

impl duplicate for i32 {
    fn dupl(&self) -> String {
        format!("{}", *self*2)
    }
}

// fn duplicate<T: duplicate>(x: T) {
//     println!("{}", x.dupl());
// }

fn duplicate(x: &dyn duplicate) {
    println!("{}", x.dupl());
}
// adding functionalty to existing structures
trait summable<T> {
    fn sum(&self) -> T;
}

impl summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum = 0;
        for i in self {
            sum += *i;
        }
        sum
    }

    
}
struct  Dog1 {}
struct  Cat1 {}

trait Animal {
    fn make_noise(&self) -> &'static str;
}

impl Animal for Dog1 {
    fn make_noise(&self) -> &'static str {
        "woof"
    }
}

impl Animal for Cat1 {
    fn make_noise(&self) -> &'static str {
        "meow"
    }
}

fn get_animal(rand_number: f64) -> Box<dyn Animal> {
    if rand_number < 1.0 {
        Box::new( Dog1 {} )
    } else {
        Box::new( Cat1 {} )
    }
}
