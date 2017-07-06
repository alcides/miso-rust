#[macro_use]
extern crate miso;

use miso::runner::miso_runner;
use std::env::args;

mod benchmark;

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

    let w = miso_runner(world, 500000-2);
    w
}

#[allow(unused_variables)]
fn main() {
    benchmark::benchmark(|| {
        let w = fib_main();
        if args().count() > 1 {
            println!("fib({:?}) = {:?}", w.fc.n, w.fc.curr);
        }
        w.fc.curr
    });
}
