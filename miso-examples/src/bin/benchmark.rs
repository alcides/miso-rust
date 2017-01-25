extern crate time;
extern crate energy;

use self::time::{Duration, PreciseTime};
use self::energy::energy::start_recording;


pub fn benchmark<R, F>(mut func: F) where F : FnMut() -> R, R: Eq {
    
    let mut iterations = 0;
    let mut time = Duration::seconds(0);
    let mut energy = 0.0;
    
    let start_e = start_recording();
    let start_t = PreciseTime::now();
    
    let mut default:Option<R> = None;
    
    
    while time < Duration::seconds(10) {
        let r = func();
        time = start_t.to(PreciseTime::now());
        iterations += 1;
        
        match default {
            Some(d) => if d != r {
                panic!("Value Fault not prevented!");
            },
            None => {}
        }
        default = Some(r);
    }
    let en = start_e.stop_recording();
    match en {
        None => {},
        Some(e) => {
            energy = e;
        }
    }
    
    println!("Time: {}", time.num_milliseconds() as f64 / (1000 * iterations) as f64);
    println!("Energy: {}", energy as f64 / (iterations) as f64);
}