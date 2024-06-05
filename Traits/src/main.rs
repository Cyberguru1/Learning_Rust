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

}
