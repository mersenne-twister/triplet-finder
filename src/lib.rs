// temporary, to make it easier to see the important errors
#![allow(dead_code, unused_variables, unused_imports, unused_assignments)]

mod find;
mod input;
mod print;
mod text;

use {
    core::num,
    find::Find,
    std::{
        error::Error,
        fs::{self, File},
        hint::spin_loop,
        io::{self, Write},
        sync::{Arc, Mutex, RwLock},
        thread,
        time::Duration,
    },
};

pub fn run(threads: Option<u32>, strict: bool) {
    println!("{}\n\n{}", text::MESSAGE, text::HELP);

    let num_threads = if let Some(num) = threads { num } else { 8 };

    let find = Find::new();

    let print = Arc::new(RwLock::new(true));
    let print_init = Arc::new(RwLock::new(None));

    for _ in 0..num_threads {
        let find = Arc::clone(&find);

        thread::spawn(move || find.find());
    }

    {
        // spawn the thread to print the values as they're found
        let num_triplets = find.triplets.lock().unwrap().len();
        let find = find.clone();
        let print = Arc::clone(&print);
        let init = Arc::clone(&print_init);

        thread::spawn(move || print::print_triplets(find, num_triplets, print, init));
    }

    input::input(strict, print, num_threads, print_init, find);
}

#[cfg(test)]
mod test {
    use super::*;

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
