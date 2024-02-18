// temporary, to make it easier to see the important errors
// #![allow(dead_code, unused_variables, unused_imports, unused_assignments)]

pub mod args;
mod find;
mod input;
mod print;
mod text;

use {
    find::Find,
    std::{sync::Arc, thread},
};

use crate::print::Print;

pub fn run(num_threads: u32, strict: bool) {
    println!("{}\n\n{}", text::MESSAGE, text::HELP);

    let find = Find::new();
    let print = Print::new();

    for _ in 0..num_threads {
        let find = Arc::clone(&find);

        thread::spawn(move || find.find());
    }

    {
        // spawn the thread to print the values as they're found
        let print = Arc::clone(&print);
        let find = Arc::clone(&find);

        thread::spawn(move || print.print(find));
    }

    input::input(find, print, strict, num_threads);
}

#[cfg(test)]
mod test {
    // use super::*;

    // #[test]
    // fn detects_multiples() {
    //     assert!(contains_triplet(
    //         &[Triplet::new(3, 4, 5)],
    //         &Triplet::new(6, 8, 10)
    //     ));
    // }

    // #[test]
    // fn detects_distincts() {
    //     assert!(!contains_triplet(
    //         &[Triplet::new(3, 4, 5)],
    //         &Triplet::new(5, 15, 17)
    //     ));
    // }
}
