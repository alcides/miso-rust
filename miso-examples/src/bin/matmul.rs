#[macro_use]
extern crate miso;

use miso::runner::miso_runner;

mod benchmark;

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
        c2: MatMulCell { n: 2, prev: 1, curr: 1 },
        c3: MatMulCell { n: 2, prev: 1, curr: 1 },
        c4: MatMulCell { n: 2, prev: 1, curr: 1 },
        c5: MatMulCell { n: 2, prev: 1, curr: 1 },
        c6: MatMulCell { n: 2, prev: 1, curr: 1 },
        c7: MatMulCell { n: 2, prev: 1, curr: 1 },
        c8: MatMulCell { n: 2, prev: 1, curr: 1 },
    };
    
    let w = miso_runner(world, 51-2);
    w
}

#[allow(unused_variables)]
fn main() {
    benchmark::benchmark(|| {
        let _ = mm_main();
    });
}