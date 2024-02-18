use std::sync::{Arc, Mutex, RwLock};

pub struct Find {
    pub paused: RwLock<bool>,
    pub triplets: Mutex<Vec<Triplet>>,
    pub current: Mutex<u64>,
    pub stop: RwLock<Stop>,
}
impl Find {
    pub fn new() -> Arc<Self> {
        Arc::new(Find {
            paused: RwLock::new(true),
            triplets: Mutex::new(Vec::new()),
            current: Mutex::new(0u64),
            stop: RwLock::new(Stop::new()),
        })
    }

    pub fn find(&self) -> ! {
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
                        // if !contains_triplet(&triplets, &triplet) {
                        if !triplet.contained_in(&triplets) {
                            triplets.push(triplet);
                        }
                    }
                }
            }

            if self.stop.read().unwrap().stop {
                self.stop.write().unwrap().stopped += 1;
                while self.stop.read().unwrap().stop {}
            }
        }
    }
}

pub struct Stop {
    pub stop: bool,
    pub stopped: u32,
}
impl Stop {
    pub fn new() -> Self {
        Self {
            stop: false,
            stopped: 0,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Triplet {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}

impl Triplet {
    pub fn new(a: u64, b: u64, c: u64) -> Triplet {
        Triplet { a, b, c }
    }

    fn contained_in(&self, triplets: &[Triplet]) -> bool {
        for i in triplets {
            if (((self.a % i.a == 0) && (self.b % i.b == 0))
                || ((self.a % i.b == 0) && (self.b % i.a == 0)))
                && (self.c % i.c == 0)
            {
                return true;
            }
        }

        // TODO: simplify the triplet, then use .contains

        false
    }
}
