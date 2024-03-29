use {
    crate::{
        find::{Find, Triplet},
        input,
        print::Print,
        text,
    },
    std::{
        error::Error,
        fs::{self, File},
        io::{self, Write},
        sync::Arc,
    },
};

pub fn input(find: Arc<Find>, print: Arc<Print>, strict: bool, num_threads: u32) {
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        if !strict {
            input = input.to_ascii_lowercase();
        }

        match input.trim().split_ascii_whitespace().next().unwrap_or("") {
            "start" => {
                *find.paused.write().unwrap() = false;
                println!("Program executing.")
            }
            "stop" => {
                *find.paused.write().unwrap() = true;
                println!("Program suspended.")
            }
            "print" => {
                let arg = input
                    .split_ascii_whitespace()
                    .nth(1)
                    .unwrap_or("")
                    .parse::<bool>();
                if let Ok(arg) = arg {
                    *print.print.write().unwrap() = arg;
                    println!("Printing {}.", if arg { "enabled" } else { "disabled" })
                } else {
                    println!("{}", text::PRINT_ERROR);
                }
            }
            "exit" if *find.paused.read().unwrap() => return,
            "save" if *find.paused.read().unwrap() => {
                let arg = input
                    .split_ascii_whitespace()
                    .nth(1)
                    .unwrap_or("triplets.txt");

                input::save(&find, &print, arg, num_threads).unwrap_or_else(|err| {
                    println!("Save error: {}.", err);
                })
            }
            "load" if *find.paused.read().unwrap() => {
                // try to load the file, and error if it fails
                let arg = input
                    .split_ascii_whitespace()
                    .nth(1)
                    .unwrap_or("triplets.txt");

                input::load(&find, &print, arg).unwrap_or_else(|err| {
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
}

pub fn save(
    find: &Arc<Find>,
    print: &Arc<Print>,
    arg: &str,
    num_threads: u32,
) -> Result<(), Box<dyn Error>> {
    println!("Opening file...");
    let mut file = File::create(arg)?;

    // TODO: move this to function stop_threads
    // don't print new triplets found, as we're closing the threads
    *print.print.write().unwrap() = false;
    // necessary because it would otherwise never get to the stopping point
    *find.paused.write().unwrap() = false;
    find.stop.write().unwrap().stop = true;
    println!("Suspending threads...");
    while find.stop.read().unwrap().stopped < num_threads {}
    // after everything has stopped, re-pause
    *find.paused.write().unwrap() = true;
    println!("Saving data...");

    file.write_fmt(format_args!("{}\n", find.current.lock().unwrap()))?;
    for triplet in (*find.triplets.lock().unwrap()).iter() {
        file.write_fmt(format_args!("{}-{}-{}\n", triplet.a, triplet.b, triplet.c))?;
    }

    println!("Data saved to {}.", arg);
    find.stop.write().unwrap().stop = false;
    // reset the counter
    find.stop.write().unwrap().stopped = 0;

    Ok(())
}

/// routine to load state from a file
pub fn load(find: &Arc<Find>, print: &Arc<Print>, arg: &str) -> Result<(), Box<dyn Error>> {
    println!("Reading {}...", arg);
    let content = fs::read_to_string(arg)?;
    println!("Loading contents...");
    let mut content = content.lines();

    *find.current.lock().unwrap() = content.next().ok_or("File corrupted")?.parse()?;
    let print_state = *print.print.read().unwrap();
    *print.print.write().unwrap() = false;

    let mut triplets = find.triplets.lock().unwrap();
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

    *print.init.write().unwrap() = Some(triplets.len());
    *print.print.write().unwrap() = print_state;

    Ok(())
}
