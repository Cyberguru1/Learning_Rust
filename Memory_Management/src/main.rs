use std::rc::Rc;

#[allow(unused_allocation)]
#[allow(unused_assignments)]


// Memeory Management
// Ownership
// Borrow
// Lifetimes
// Reference Counted Variables


#[derive(Debug)]
struct Person {
    name: String
}

#[derive(Debug)]
struct Dog <'a > {
    name: String,
    owner: &'a Person
}

#[derive(Debug)]
struct Car {
    brand: Rc<String>
}

impl Car {
    fn new(brand: Rc<String>) -> Car { Car { brand: brand} }
    fn drive(&self) {
        println!("{} is driving...\n", self.brand);
    }
}

fn main() {
    println!("Hello, world!\n");

    // Ownership
    // transfer through copying
    let i = 5;
    let j = i;

    println!("{}, {}\n", i, j);

    // only one owner per piece of data

    let v = vec![1,2,3,4,5];
    // let m = v;

    // println!("{:?}", v); this invalid
    println!("{:?}", v);

    let foo = |v: Vec<i32> | -> Vec<i32> {
        println!("vector used in foo\n");
        v
    };

    let v = foo(v);
    println!("{:?}\n", v);

    // Borrowing
    // pass to another variable only for a temporary time
    
    // let a = 6;
    // let b = &a;

    let mut a = 6;
    {   
        let b = &mut  a;

        println!("{}\n", *b);

        *b += 2;
    }

    println!("{}\n", a);

    let  mut v = vec![1,2,4,5];
    for i in &v {
        println!("{}", i);
    }

    // LifeTime
    // Sample
    // Struct Object<'lifetime> {
        // field: &'lifetime str
    // }
    
    println!("\n{}\n", get_str());

    let p1 = Person { name: String::from("Rustman") };

    let d1 = Dog { name: String::from("Maxxer"), owner: &p1 };

    println!("\n{:?}", d1);

    // lifetime emission
    let mut aa: &String;

    {
        let p2 = Person { name: String::from("marther")};
        println!("\n{:?}", p2.get_name());
        aa = p1.get_name()

    }

    println!("\n{:?}", aa);

    // Reference Counted Variable

    let brand = Rc::new(String::from("BMW"));
    println!("\npointers: {}\n", Rc::strong_count(&brand));
    {
        let car = Car::new(brand.clone());
        car.drive();
        println!("\npointers: {}", Rc::strong_count(&brand));
    }
    println!("\nMy car is a {}", brand);
    println!("\nPointers: {}", Rc::strong_count(&brand));

    
}

impl Person {
    fn get_name<'a> (&'a self) -> &'a String {
        &self.name
    }
}

fn get_str() -> &'static str {
    "Hello"
}