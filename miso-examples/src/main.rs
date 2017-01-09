#[macro_use]
extern crate miso;

use std::cell::RefCell;
use std::rc::Rc;

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
    
    let world = Rc::new(RefCell::new(World { 
        fc: FibCell { n: 2, prev: 1, curr: 1 },
        fc2: FibCell { n: 2, prev: 1, curr: 1 },
        }));
    
    let w2 = miso_runner(world.clone(), 51-2);
    
    let w = world.borrow_mut();
    println!("fib({:?}) = {:?}", w.fc.n, w.fc.curr);
    println!("fib({:?}) = {:?}", w2.fc.n, w2.fc.curr);

}
