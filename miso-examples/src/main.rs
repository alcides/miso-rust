#[macro_use]
extern crate miso;

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
    fc: FibCell, 
    fc2: FibCell
);


fn main() {
    
    let world = World { 
        fc: FibCell { n: 2, prev: 1, curr: 1 },
        fc2: FibCell { n: 2, prev: 1, curr: 1 },
    };
    
    let w = miso_runner(world, 51-2);

    println!("fib({:?}) = {:?}", w.fc.n, w.fc.curr);

}
