use crate::find::Find;
use std::sync::{Arc, RwLock};

pub struct Print {
    pub print: RwLock<bool>,
    pub init: RwLock<Option<usize>>,
}
impl Print {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            print: RwLock::new(true),
            init: RwLock::new(None),
        })
    }

    pub fn print(&self, find: Arc<Find>) {
        let mut num_printed = 0;
        loop {
            if !*self.print.read().unwrap() {
                // this prevents a deadlock, if we get rid of do_init then the lock is still held
                // when we call `init.write()`
                let do_init = *self.init.read().unwrap();
                if let Some(init_amount) = do_init {
                    num_printed = init_amount;
                    *self.init.write().unwrap() = None;
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
}
