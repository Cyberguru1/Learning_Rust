
#[allow(unused_allocation)]
#[allow(unused_assignments)]
#[allow(non_snake_case)]

// Functions and Scope
// Closures
// Higher Order Functions
// Macros

// global variable with static keyword

static mut A: i32 = 0;

// creating macros

macro_rules! my_macro {
    () => {println!("first macro\n")};
}

macro_rules! name {
    ($name: expr) => {
        println!("Hey {}", $name);
    };
}

macro_rules! names {
    ($($name: expr), *) => { $(println!("\nHey {}", $name);)*};
}

macro_rules! xy {
    (x => $e: expr) => (println!("\nX is {}\n", $e));
    (y => $e: expr) => (println!("Y is {}\n", $e));
}

macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name() {
            println!("{:?} was created", stringify!($fn_name));
        }
    }
}

fn main() {
    println!("Hello, Functions!");

    // Functions basics

    let mut name = "Rust";

    let out = say_hi(&mut name);

    println!("{}", out);

    // Scopes

    {
        let a = 3;
        println!("\na: {}", a);
    }

    // global unsafe
    unsafe {
        A = 4;
        println!("\nA: {}", A);
    }
    
    unsafe {
        println!("\nA: {}", A);
    }

    // Closures -- anonymous functions
    // functions without names
    // also called lambda expressions

    let summed =|a: i32, b: i32| println!("\nA({}) + B({}) = {}",a, b, a+b);
    let sum = |a: i32, b: i32| -> i32 { a + b };
    println!("\nSumm result := {}", sum(2, 3));
    summed(4, 5);

    // simple example
    let a = |a: i32| a+1;
    println!("\nA({} + 1) = {}",6, a(6));

    // Generic Clousers
    let gen = |x| println!("\n{}", x);
    gen(7);

    // HOFs
    // function that takes another function as parameter
    let square = |x| -> i32 { x * x};
    apply(square, 3);

    //

    let limit = 300;
    let mut sum = 0;

    for i in 0.. {
        let isq = i * i;
        if isq > limit { break; }
        else {
            if is_even(isq) {
                sum += isq;
            }
        }
    }

    println!("\nThe sum is {}", sum);

    //With HOFs

    let sum2 = 
        (0..).map(|x| x * x)
            .take_while(|&x| x <= limit)
            .filter(|x| is_even(*x))
            .fold(0, |sum, x| sum + x);

    println!("\nThe sum using HOFs is {}\n", sum2);

    // Macros
    // Write code in a short hand way
    // Meta programming, code that write code
    // fn! -- format

    my_macro!();
    name!("Young john");
    names!("Alex", "Hary", "Carol");
    xy!(x => 5);
    xy!(y => 10);

    build_fn!(hey);
    hey();


}


fn say_hi(name: &mut &str) -> String {
    *name = "Alex";
    format!("\nHello, {}!", name)
}

fn apply(f: fn(i32) -> i32, a: i32){
    println!("\nRESULT: {}", f(a));
}

fn is_even(a: i32) -> bool {
    a % 2 == 0
}