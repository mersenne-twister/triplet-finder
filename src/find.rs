use std::sync::{Arc, Mutex, RwLock};
use crate::{Stop, Triplet};

pub struct Find {
    paused: RwLock<bool>,
    triplets: Mutex<Vec<Triplet>>,
    current: Mutex<u64>,
    stop: RwLock<Stop>,
}
impl Find {
    pub fn new() -> Arc<Self> {
        Arc::new(Find {
            paused: RwLock::new(true),
            triplets: Mutex::new(Vec::new()),
            current: Mutex::new(0u64),
            stop:RwLock::new(Stop::new()),
        })
    }

    pub fn find(&self) {
        loop {
            let c = {
                let mut current = self.current.lock().unwrap();
                *current += 1; // increment current, so multiple threads don't check the same number
                *current
            };
    
            for a in 1..=c {
                for b in 1..=a {
                    while *self.paused.read().unwrap() {}
    
                    if a.pow(2) + b.pow(2) == c.pow(2) {
                        let triplet = Triplet::new(b, a, c);
                        // we only lock triplets when we need to check it
                        let mut triplets = self.triplets.lock().unwrap();
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
}

