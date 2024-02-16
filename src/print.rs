use crate::find::Find;
use std::sync::{Arc, RwLock};

pub fn print_triplets(
    find: Arc<Find>,
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
        let num_found = (*find.triplets.lock().unwrap()).len();
        if num_found > num_printed {
            for num in num_printed..num_found {
                let triplet = &(*find.triplets.lock().unwrap())[num];
                println!("{}-{}-{}", triplet.a, triplet.b, triplet.c);
            }
            num_printed = num_found;
        }
    }
}
