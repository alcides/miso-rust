
use std::fmt::Debug;

#[allow(unused_imports)]
use std::sync::{Arc, Barrier, Mutex};

pub trait Transitionable : Clone + Copy + PartialEq + Debug + Sync + Send {
    fn transition(&mut self);
}

#[cfg(feature = "ft")]
fn advance_world<W: Transitionable>(w:Arc<Mutex<W>>, iters:u64, b:Arc<Barrier>) {
    for _ in 0..iters {
        let mut w = w.lock().unwrap();
        w.transition();
        b.wait();
    }
}

#[cfg(feature = "ft")]
pub fn miso_runner<W: Transitionable + 'static>(w: W, i:u64) -> W {
    use std::thread;
    use std::cmp::min;

    let bw = w;
    let mut last_safe_world = w;

    let mut w1 = Arc::new(Mutex::new(w));
    let mut w2 = Arc::new(Mutex::new(bw));

    let copies = 2;
    let check_interval = 5;

    let mut iteration = 0;
    loop { /* Overall loop over the iterations, check_interval at a time */
        let next = min(check_interval, i-iteration);
        loop { /* Loops if faults found */

            let mut handles = Vec::with_capacity(copies);
            let barrier = Arc::new(Barrier::new(copies));

            let worlds = vec!(w1.clone(), w2.clone());

            for world in worlds {
                let b = barrier.clone();
                handles.push(thread::spawn( move || {
                    advance_world(world, next, b);
                }));
            }

            let mut check = true;
            for t in handles {
                let r = t.join();
                match r {
                        Ok(_) => {},
                        Err(_) => { check = false; }
                    }
            }
            {
                let _1 = w1.lock().unwrap();
                let _2 = w2.lock().unwrap();
                if check && *_1 == *_2 {
                    last_safe_world = *_1;
                    break;
                }
            }

            println!("Fault detected!");
            let last_safe_world2 = last_safe_world;
            w1 = Arc::new(Mutex::new(last_safe_world));
            w2 = Arc::new(Mutex::new(last_safe_world2));
        }
        
        iteration += next;
        if iteration >= i {
            break;
        }
    }

    let k = w1.lock().unwrap();
    *k
}


#[cfg(not(feature = "ft"))]
pub fn miso_runner<W: Transitionable + 'static>(w: W, i:u64) -> W {
    let mut w2 = w;
    for _ in 0..i {
        w2.transition();
    }
    w2
}
