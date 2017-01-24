extern crate time;

#[macro_use]
extern crate miso;

use time::{Duration, PreciseTime};
use miso::runner::miso_runner;

define_cell!( FibCell {
    n: u64,
    prev: u64,
    curr: u64
    } => self, previous, world {
        self.n = previous.n + 1;
        self.prev = previous.curr;
        self.curr = previous.curr + previous.prev;
});

define_world!(
    fc: FibCell
);


fn fib_main() -> World {
    let world = World { 
        fc: FibCell { n: 2, prev: 1, curr: 1 },
    };
    
    let w = miso_runner(world, 51-2);
    w
}

#[allow(unused_variables)]
fn main() {
    let mut iterations = 0;
    let mut t = Duration::seconds(0);
    
    let start = PreciseTime::now();
    while t < Duration::seconds(10) {
        for _ in 1..1000 {
            let w = fib_main();
        }
        iterations += 1;
        //println!("fib({:?}) = {:?}", w.fc.n, w.fc.curr);
        t = start.to(PreciseTime::now())
    }
    println!("{}", t.num_milliseconds() as f64 / (1000 * iterations) as f64);
}
