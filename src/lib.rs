// temporary, to make it easier to see the important errors
#![allow(dead_code, unused_variables, unused_imports)]

mod intro;

use std::{
    fs::File,
    io,
    io::Write,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn run(threads: Option<i32>, strict: bool) {
    println!("{}", intro::MESSAGE);

    let num_threads = if let Some(num) = threads { num } else { 8 };

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

fn contains_triplet(arr: &[Triplet], triple: &Triplet) -> bool {
    for i in arr {
        if (((triple.a % i.a == 0) && (triple.b % i.b == 0))
            || ((triple.a % i.b == 0) && (triple.b % i.a == 0)))
            && (triple.c % i.c == 0)
        {
            return true;
        }
    }

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
