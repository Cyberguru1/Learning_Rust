#[allow(unused_allocation)]
#[allow(unused_assignments)]
#[allow(unused_imports)]
#[allow(non_snake_case)]

mod player;
use crate::archive::arch::arch_file as arc;

// things learnt:
// functions
// modules
// crates
//     Types:
//           Binary Crates <-- has main
//           Library Crates <-- no main
// uses cargo to manage crates

fn main() {
    println!("Hello, world!");
    player::play_movie("I like the way you...");
    player::play_audio("Cashapp");

    clean::perform_clean();

    uni::fac::dept();

    // crates
    arc("somefiles.txt")

}

mod clean {
    pub fn perform_clean() {
        println!("\nCleaning up now ...");
    }
}

mod uni {
    pub mod fac {
        pub fn dept() {
            println!("\nYou currently in the dept level ...");
        }
    }
}