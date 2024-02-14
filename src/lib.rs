// temporary, to make it easier to see the important errors
#![allow(dead_code, unused_variables, unused_imports, unused_assignments)]

mod intro;

use core::num;
use std::{
    fs::File,
    io,
    io::Write,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn run(threads: Option<i32>, strict: bool) {
    println!("{}{}", intro::MESSAGE, intro::HELP);

    let num_threads = if let Some(num) = threads { num } else { 8 };

    let paused = Arc::new(Mutex::new(true));
    let triplets = Arc::new(Mutex::new(Vec::new()));
    let current = Arc::new(Mutex::new(0u64));
    let mut handles = vec![];

    for _ in 0..num_threads {
        let paused = Arc::clone(&paused);
        let handle = thread::spawn(move || check(paused));
        handles.push(handle);
    }

    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        match input.to_lowercase().trim() {
            "start" => *paused.lock().unwrap() = false,
            "stop" => *paused.lock().unwrap() = true,
            "exit" => run_if_paused(&paused, || {
                return;
            }),
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

    /*
    let mut file = File::create("triplets").unwrap_or_else(|err| {
        panic!("Could not create file: {}", err);
    });

    let mut triplets = Vec::new();

    for c in 1u64..=50 {
        for a in 1..(c + 1) {
            for b in 1..(a + 1) {
                if a.pow(2) + b.pow(2) == c.pow(2) {
                    let triple = Triplet::new(b, a, c);

                    if !contains_triple(&triplets, &triple) {
                        triplets.push(triple);

                        file //TODO: implement display for triplet
                        .write_fmt(format_args!("{}, {}, {}\n", b, a, c))
                        .expect("failed to write to file");
                        println!("{}, {}, {}", b, a, c);
                    }
                }
            }
        }
    }

    println!("All triplets up to 18446744073709551615 found.");
    */
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

fn check(paused: Arc<Mutex<bool>>, current: Arc<Mutex<u64>>, triplets: Arc<Mutex<Vec<Triplet>>>) {
    loop {
        if !*paused.lock().unwrap() {
            // get the next number and increment the counter
            
            // check all possible variations

            // if one is found, check that it is simplified, then check if it already is in the vec
            // if not, add it to it

            let current = current.lock().unwrap();
            *current += 1; // increment current, so multiple threads don't check the same number
            let c = *current;
            drop(current); // unlock the mutex

            for a in 1..(c + 1) {
                for b in 1..(a + 1) {
                    if a.pow(2) + b.pow(2) == c.pow(2) {
                        let triplet = Triplet::new(b, a, c);
                        // we only lock triplets when we need to check it
                        let mut triplets = *triplets.lock().unwrap();
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
    for i in arr {
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
    #[test]
    fn foo() {
        assert!(contains_triplet(
            &[Triplet::new(3, 4, 5)],
            &Triplet::new(6, 8, 10)
        ));
    }
}
