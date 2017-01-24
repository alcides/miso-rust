extern crate time;

#[macro_use]
extern crate miso;

use time::{Duration, PreciseTime};
use miso::runner::miso_runner;

define_cell!( MatMulCell {
    n: u64,
    prev: u64,
    curr: u64
    } => self, previous, world {
        self.n = previous.n + 1;
        self.prev = previous.curr;
        self.curr = previous.curr + previous.prev;
});

define_world_par!(
    c1: MatMulCell,
    c2: MatMulCell,
    c3: MatMulCell,
    c4: MatMulCell,
    c5: MatMulCell,
    c6: MatMulCell,
    c7: MatMulCell,
    c8: MatMulCell
);


fn mm_main() -> World {
    let world = World { 
        c1: MatMulCell { n: 2, prev: 1, curr: 1 },
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
            let w = mm_main();
        }
        iterations += 1;
        //println!("fib({:?}) = {:?}", w.fc.n, w.fc.curr);
        t = start.to(PreciseTime::now())
    }
    println!("{}", t.num_milliseconds() as f64 / (1000 * iterations) as f64);
}
