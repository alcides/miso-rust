#[macro_use]
extern crate miso;

use miso::runner::miso_runner;
use std::env::args;

mod benchmark;

struct Mat4 {
    dat: [f32; 640000]
}

const M1:Mat4 = Mat4 {
    dat: [1.0; 640000]
};

const M2:Mat4 = Mat4 {
    dat: [1.0; 640000]
};

static mut M3:Mat4 = Mat4 {
    dat: [1.0; 640000]
};



define_cell!( MatMulCell {
    x_start: u64,
    x_end: u64,
    check: u32
    } => self, previous, world {
        self.check = 0;
        for i in self.x_start..self.x_end {
            for j in 0..800 {
                let mut a = 0.0;
                for k in 0..800 {
                    a += M1.dat[(i * 800 + k) as usize] * M2.dat[(k * 800 + j) as usize];
                }

                unsafe {
                    M3.dat[(i * 800 + j) as usize] = a;
                }
                self.check += a as u32;
            }
        }
});

define_world!(
    cs: CellArray<MatMulCell>
);


fn mm_main() -> u32 {
    let world = World {
        cs: CellArray {
            cells: [
                MatMulCell { x_start: 0, x_end: 100, check: 0},
                MatMulCell { x_start: 100, x_end: 200, check: 0},
                MatMulCell { x_start: 200, x_end: 300, check: 0},
                MatMulCell { x_start: 300, x_end: 400, check: 0},
                MatMulCell { x_start: 400, x_end: 500, check: 0},
                MatMulCell { x_start: 500, x_end: 600, check: 0},
                MatMulCell { x_start: 600, x_end: 700, check: 0},
                MatMulCell { x_start: 700, x_end: 800, check: 0},
            ]
        }
    };

    let w = miso_runner(world, 1);
    let rs = w.cs.cells.iter().map(|x| (*x).check);
    rs.fold(0, |p, val| p + val)
}

#[allow(unused_variables)]
fn main() {

    benchmark::benchmark(move || {
        let m3 = mm_main();
        if args().count() > 1 {
            println!("check = {:?}", m3);
        }
        m3
    });
}
