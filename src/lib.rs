mod intro;

use std::{fs::File, io::Write/*, process*/};

pub fn run(threads: Option<i32>, strict: bool) {
    println!("{}", intro::MESSAGE);
    // let mut file = File::create("triplets").unwrap_or_else(|err| {
    //     panic!("Could not create file: {}", err);
    // });

    // let mut triplets = Vec::new();

    // for c in 1u64..=50 {
    //     for a in 1..(c + 1) {
    //         for b in 1..(a + 1) {
    //             if a.pow(2) + b.pow(2) == c.pow(2) {
    //                 let triple = Triplet::new(b, a, c);

    //                 if !contains_triple(&triplets, &triple) {
    //                     triplets.push(triple);

    //                     file //TODO: implement display for triplet
    //                     .write_fmt(format_args!("{}, {}, {}\n", b, a, c)) 
    //                     .expect("failed to write to file");
    //                     println!("{}, {}, {}", b, a, c);
    //                 }
    //             }
    //         }
    //     }
    // }

    // println!("All triplets up to 18446744073709551615 found.");
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


fn contains_triple(arr: &[Triplet], triple: &Triplet) -> bool {
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
        assert!(contains_triple(
            &[Triplet::new(3, 4, 5)],
            &Triplet::new(6, 8, 10)
        ));
    }
}