// temporary, to make it easier to see the important errors
#![allow(dead_code, unused_variables, unused_imports, unused_assignments)]

mod intro;

use core::num;
use std::{
    fs::File,
    hint::spin_loop,
    io::{self, Write},
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};

pub fn run(threads: Option<i32>, strict: bool) {
    println!("{}{}", intro::MESSAGE, intro::HELP);

    let num_threads = if let Some(num) = threads { num } else { 8 };

    // TODO: use `RwLock` for `paused`
    let paused = Arc::new(Mutex::new(true));
    let print = Arc::new(RwLock::new(true));
    let triplets = Arc::new(Mutex::new(Vec::new()));
    let current = Arc::new(Mutex::new(0u64));

    for _ in 0..num_threads {
        let paused = Arc::clone(&paused);
        let current = Arc::clone(&current);
        let triplets = Arc::clone(&triplets);
        thread::spawn(move || check(paused, current, triplets));
    }

    { // spawn the thread to print the values as they're found
        let num_triplets = triplets.lock().unwrap().len();
        let triplets = Arc::clone(&triplets);
        let print = Arc::clone(&print);
        thread::spawn(move || print_triplets(triplets, num_triplets, print));
    }

    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        match input.to_lowercase().trim().split_ascii_whitespace().next().unwrap() {
            "start" => {
                *paused.lock().unwrap() = false;
                println!("Program executing.")
            }
            "stop" => {
                *paused.lock().unwrap() = true;
                println!("Program suspended.")
            }
            "print" => {
                println!("{}", input.split_ascii_whitespace().nth(1).unwrap_or(intro::PRINT_ERROR));
            }
            "exit" => {
                let mut ret = false;
                run_if_paused(&paused, || {
                    ret = true;
                });
                if ret {
                    return;
                }
            }
            "save" => run_if_paused(&paused, || {
                todo!();
            }),
            "load" => run_if_paused(&paused, || {
                todo!();
            }),
            "help" => println!("{}", intro::HELP),
            "" => (), // don't show an error if they don't input anything
            _ => println!("{}", intro::INPUT_ERROR),
        }
    }

    // create mutices for paused, triplets (Vec<Triplet>)

    // spawn threads
    // if !paused
    // get the next number
    // compute it
    // if triplet is found, check it, and if it's simplified,
    // check found_triplets, and if it's new, push it to found

    // loop for input
    // if exit, close threads
}

fn run_if_paused<F>(paused: &Arc<Mutex<bool>>, f: F)
where
    F: FnOnce(),
{
    if *paused.lock().unwrap() {
        f();
    } else {
        println!("{}", intro::RUNNING_ERROR);
    }
}

fn print_triplets(triplets: Arc<Mutex<Vec<Triplet>>>, starting_size: usize, print: Arc<RwLock<bool>>) {
    let num_printed = starting_size;
    loop {
        let num_found = (*triplets.lock().unwrap()).len();
        if num_found > num_printed {
            for num in num_printed..num_found {
                let triplet = &(*triplets.lock().unwrap())[num];
                println!("{}-{}-{}", triplet.a, triplet.b, triplet.c);
            }
        }
    }
}

fn check(paused: Arc<Mutex<bool>>, current: Arc<Mutex<u64>>, triplets: Arc<Mutex<Vec<Triplet>>>) {
    loop {
        if !*paused.lock().unwrap() {
            // get the next number and increment the counter

            // check all possible variations

            // if one is found, check that it is simplified, then check if it already is in the vec
            // if not, add it to it

            let mut current = current.lock().unwrap();
            *current += 1; // increment current, so multiple threads don't check the same number
            let c = *current;
            drop(current); // unlock the mutex

            for a in 1..(c + 1) {
                for b in 1..(a + 1) {
                    if a.pow(2) + b.pow(2) == c.pow(2) {
                        let triplet = Triplet::new(b, a, c);
                        // we only lock triplets when we need to check it
                        let mut triplets = triplets.lock().unwrap();
                        // if !contains_triplet(&triplets, &triple) {
                        if !contains_triplet(&triplets, &triplet) {
                            triplets.push(triplet);

                            // file //TODO: implement display for triplet
                            // .write_fmt(format_args!("{}, {}, {}\n", b, a, c))
                            // .expect("failed to write to file");
                            // println!("{}, {}, {}", b, a, c);
                        }
                    }
                }
            }
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
