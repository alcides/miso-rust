use std::fmt::Debug;
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::cmp::min;

pub trait Transitionable : Clone + Copy + Eq + PartialEq + Debug + Sync + Send {
    fn transition(&mut self);
}

fn advance_world<W: Transitionable>(w:Arc<Mutex<W>>, iters:u64, b:Arc<Barrier>) {
    for _ in 0..iters {
        let mut w = w.lock().unwrap();
        w.transition();
        b.wait();
    }
} 


pub fn miso_runner<W: Transitionable + 'static>(w: W, i:u64) -> W{
    let bw = w;
    let original = Arc::new(Mutex::new(w));
    let backup = Arc::new(Mutex::new(bw));
    
    let copies = 2;
    let check_interval = 5;
    
    let mut iteration = 0;
    loop { /* Overall loop over the iterations, check_interval at a time */
        let next = min(check_interval, i-iteration);  
        loop { /* Loops if faults found */
        
            let mut handles = Vec::with_capacity(copies);
            let barrier = Arc::new(Barrier::new(copies));
        
            let worlds = vec!(original.clone(), backup.clone());
        
            for world in worlds {
                let b = barrier.clone();
                handles.push(thread::spawn( move || {
                    advance_world(world, next, b); 
                }));
            }
            #[allow(unused_must_use)]
            for t in handles {
                t.join();
            }
        
            let _1 = original.lock().unwrap();
            let _2 = backup.lock().unwrap();
    
            if *_1 == *_2 {
                break;
            }
            println!("Fault detected!");
        }
        iteration += next;
        if iteration >= i {
            break;
        }
    }
        
    let k = original.lock().unwrap();
    *k
}