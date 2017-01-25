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
    cs: CellArray<MatMulCell>
);


fn mm_main() -> World {
    let world = World { 
        cs: CellArray {
            cells: [ MatMulCell { n: 2, prev: 1, curr: 1 } ; 8 ]
        }
    };
    
    let w = miso_runner(world, 51-2);
    w
}

#[allow(unused_variables)]
fn main() {
    benchmark::benchmark(|| {
        let _ = mm_main();
        1
    });
}