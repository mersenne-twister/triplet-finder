// temporary, to make it easier to see the important errors
#![allow(dead_code, unused_variables, unused_imports, unused_assignments)]

mod find;
mod input;
mod print;
mod text;

use core::num;
use std::{
    error::Error,
    fs::{self, File},
    hint::spin_loop,
    io::{self, Write},
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};
use find::Find;

struct Stop {
    stop: bool,
    stopped: u32,
}
impl Stop {
    fn new() -> Self {
        Self {
            stop: false,
            stopped: 0,
        }
    }
}

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
        let num_triplets = triplets.lock().unwrap().len();
        let triplets = Arc::clone(&triplets);
        let print = Arc::clone(&print);
        let init = Arc::clone(&print_init);

        thread::spawn(move || print_triplets(triplets, num_triplets, print, init));
    }

    input::input(
        strict,
        paused,
        print,
        num_threads,
        stop,
        current,
        triplets,
        print_init,
    );
}

fn print_triplets(
    triplets: Arc<Mutex<Vec<Triplet>>>,
    starting_size: usize,
    print: Arc<RwLock<bool>>,
    init: Arc<RwLock<Option<usize>>>,
) {
    let mut num_printed = starting_size;
    loop {
        if !*print.read().unwrap() {
            // this prevents a deadlock, if we get rid of do_init then the lock is still held
            // when we call `init.write()`
            let do_init = *init.read().unwrap();
            if let Some(init_amount) = do_init {
                num_printed = init_amount;
                *init.write().unwrap() = None;
            }
            continue;
        }
        let num_found = (*triplets.lock().unwrap()).len();
        if num_found > num_printed {
            for num in num_printed..num_found {
                let triplet = &(*triplets.lock().unwrap())[num];
                println!("{}-{}-{}", triplet.a, triplet.b, triplet.c);
            }
            num_printed = num_found;
        }
    }
}



#[derive(PartialEq, Debug)]
struct Triplet {
    a: u64,
    b: u64,
    c: u64,
}

impl Triplet {
    fn new(a: u64, b: u64, c: u64) -> Triplet {
        Triplet { a, b, c }
    }
}

fn contains_triplet(triplets: &[Triplet], triplet: &Triplet) -> bool {
    for i in triplets {
        if (((triplet.a % i.a == 0) && (triplet.b % i.b == 0))
            || ((triplet.a % i.b == 0) && (triplet.b % i.a == 0)))
            && (triplet.c % i.c == 0)
        {
            return true;
        }
    }

    // TODO: simplify the triplet, then use .contains

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detects_multiples() {
        assert!(contains_triplet(
            &[Triplet::new(3, 4, 5)],
            &Triplet::new(6, 8, 10)
        ));
    }

    #[test]
    fn detects_distincts() {
        assert!(!contains_triplet(
            &[Triplet::new(3, 4, 5)],
            &Triplet::new(5, 15, 17)
        ));
    }
}
