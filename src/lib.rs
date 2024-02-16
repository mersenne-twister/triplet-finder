// temporary, to make it easier to see the important errors
#![allow(dead_code, unused_variables, unused_imports, unused_assignments)]

mod text;
mod input;
mod find;
mod print;

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
    println!("{}{}", text::MESSAGE, text::HELP);

    let num_threads = if let Some(num) = threads { num } else { 8 };

    let paused = Arc::new(RwLock::new(true));
    let stop = Arc::new(RwLock::new(Stop::new()));
    let print = Arc::new(RwLock::new(true));
    let triplets = Arc::new(Mutex::new(Vec::new()));
    let current = Arc::new(Mutex::new(0u64));
    let print_init = Arc::new(RwLock::new(None));

    for _ in 0..num_threads {
        let paused = Arc::clone(&paused);
        let current = Arc::clone(&current);
        let triplets = Arc::clone(&triplets);
        let stop = Arc::clone(&stop);

        thread::spawn(move || find_triplets(paused, current, triplets, stop));
    }

    {
        // spawn the thread to print the values as they're found
        let num_triplets = triplets.lock().unwrap().len();
        let triplets = Arc::clone(&triplets);
        let print = Arc::clone(&print);
        let init = Arc::clone(&print_init);

        thread::spawn(move || print_triplets(triplets, num_triplets, print, init));
    }

    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        if strict {
            input = input.to_ascii_lowercase();
        }

        match input.trim().split_ascii_whitespace().next().unwrap_or("") {
            "start" => {
                *paused.write().unwrap() = false;
                println!("Program executing.")
            }
            "stop" => {
                *paused.write().unwrap() = true;
                println!("Program suspended.")
            }
            "print" => {
                let arg = input
                    .split_ascii_whitespace()
                    .nth(1)
                    .unwrap_or("")
                    .parse::<bool>();
                if let Ok(arg) = arg {
                    *print.write().unwrap() = arg;
                    println!("Printing {}.", if arg { "enabled" } else { "disabled" })
                } else {
                    println!("{}", text::PRINT_ERROR);
                }
            }
            "exit" if *paused.read().unwrap() => return,
            "save" if *paused.read().unwrap() => {
                let arg = input
                    .split_ascii_whitespace()
                    .nth(1)
                    .unwrap_or("triplets.txt");

                save(
                    arg,
                    num_threads,
                    &print,
                    &paused,
                    &stop,
                    &current,
                    &triplets,
                )
                .unwrap_or_else(|err| {
                    println!("Save error: {}.", arg);
                })
            }
            "load" if *paused.read().unwrap() => {
                // try to load the file, and error if it fails
                let arg = input
                    .split_ascii_whitespace()
                    .nth(1)
                    .unwrap_or("triplets.txt");

                load(arg, &current, &triplets, &print, &print_init).unwrap_or_else(|err| {
                    println!("Load error: {}.\nFile possibly corrupted.", err);
                });
            }
            // show an error instead of silently denying
            "exit" | "save" | "load" => println!("{}", text::RUNNING_ERROR),
            "help" => println!("{}", text::HELP),
            "" => (), // don't show an error if they don't input anything
            _ => println!("{}", text::INPUT_ERROR),
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

fn save(
    arg: &str,
    num_threads: u32,
    print: &Arc<RwLock<bool>>,
    paused: &Arc<RwLock<bool>>,
    stop: &Arc<RwLock<Stop>>,
    current: &Arc<Mutex<u64>>,
    triplets: &Arc<Mutex<Vec<Triplet>>>,
) -> Result<(), Box<dyn Error>> {
    println!("Opening file...");
    let mut file = File::create(arg)?;

    // TODO: move this to function stop_threads
    // don't print new triplets found, as we're closing the threads
    *print.write().unwrap() = false;
    // necessary because it would otherwise never get to the stopping point
    *paused.write().unwrap() = false;
    stop.write().unwrap().stop = true;
    println!("Suspending threads...");
    while stop.read().unwrap().stopped < num_threads {}
    // after everything has stopped, re-pause
    *paused.write().unwrap() = true;
    println!("Saving data...");

    file.write_fmt(format_args!("{}\n", current.lock().unwrap()))?;
    for triplet in (*triplets.lock().unwrap()).iter() {
        file.write_fmt(format_args!("{}-{}-{}\n", triplet.a, triplet.b, triplet.c))?;
    }

    println!("Data saved to {}.", arg);
    stop.write().unwrap().stop = false;
    // reset the counter
    stop.write().unwrap().stopped = 0;

    Ok(())
}

/// routine to load state from a file
fn load(
    arg: &str,
    current: &Arc<Mutex<u64>>,
    triplets: &Arc<Mutex<Vec<Triplet>>>,
    print: &Arc<RwLock<bool>>,
    init: &Arc<RwLock<Option<usize>>>,
) -> Result<(), Box<dyn Error>> {
    // let content = match fs::read_to_string(arg) {
    //     Ok(content) => content.lines(),
    //     Err(err) => {
    //         println!("{}\nError-type: {}", intro::LOAD_ERROR, err);
    //         return;
    //     }
    // };

    println!("Reading {}...", arg);
    let content = fs::read_to_string(arg)?;
    println!("Loading contents...");
    let mut content = content.lines();

    *current.lock().unwrap() = content.next().ok_or("File corrupted")?.parse()?;
    let print_state = *print.read().unwrap();
    *print.write().unwrap() = false;

    let mut triplets = triplets.lock().unwrap();
    let mut nums = Vec::new();
    for triplet in content {
        nums.clear();
        for num in triplet.split('-') {
            nums.push(num.parse()?);
        }
        if nums.len() < 3 {
            return Err("File corrupted".into());
        }

        triplets.push(Triplet::new(nums[0], nums[1], nums[2]));
    }

    println!("Successfully loaded from {}.", arg);

    *init.write().unwrap() = Some(triplets.len());
    *print.write().unwrap() = print_state;

    Ok(())
}

/// TODO: move input into here
fn input() {}

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

fn find_triplets(
    paused: Arc<RwLock<bool>>,
    current: Arc<Mutex<u64>>,
    triplets: Arc<Mutex<Vec<Triplet>>>,
    stop: Arc<RwLock<Stop>>,
) {
    loop {
        let c = {
            let mut current = current.lock().unwrap();
            *current += 1; // increment current, so multiple threads don't check the same number
            *current
        };

        for a in 1..=c {
            for b in 1..=a {
                while *paused.read().unwrap() {}

                if a.pow(2) + b.pow(2) == c.pow(2) {
                    let triplet = Triplet::new(b, a, c);
                    // we only lock triplets when we need to check it
                    let mut triplets = triplets.lock().unwrap();
                    // if !contains_triplet(&triplets, &triple) {
                    if !contains_triplet(&triplets, &triplet) {
                        triplets.push(triplet);
                    }
                }
            }
        }

        if stop.read().unwrap().stop {
            stop.write().unwrap().stopped += 1;
            while stop.read().unwrap().stop {}
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
