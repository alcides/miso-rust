#[macro_use]
extern crate miso;

use miso::runner::miso_runner;
use std::env::args;

mod benchmark;

const ROW_SIZE:u64 = 800;
const MATRIX_SIZE:usize = 640000;

struct Mat4 {
    dat: [f32; MATRIX_SIZE]
}

const M1:Mat4 = Mat4 {
    dat: [3.0; MATRIX_SIZE]
};

const M2:Mat4 = Mat4 {
    dat: [1.0; MATRIX_SIZE]
};

fn wrap(p:u64, add:bool) -> u64 {
    if p == 0 && !add {
        0
    } else {
        if p == ROW_SIZE-1 && add {
            ROW_SIZE-1
        } else {
            if add {
                p+1
            } else {
                p-1
            }
        }
    }
}

define_cell!( HeatCell {
    x_start: u64,
    x_end: u64,
    y_start: u64,
    y_end: u64,
    even: bool,
    check: f32
    } => self, previous, world {
        self.check = 0.0;

        for i in self.x_start..self.x_end {
            for j in self.y_start..self.y_end {
                if self.even {
                    let n_val = 0.25 * (
                            M1.dat[(wrap(i,false) * ROW_SIZE + j) as usize] +
                            M1.dat[(wrap(i,true) * ROW_SIZE + j) as usize] +
                            M1.dat[(i * ROW_SIZE + wrap(j,false)) as usize] +
                            M1.dat[(i * ROW_SIZE + wrap(j,true)) as usize]
                        );
                    M2.dat[( i * ROW_SIZE + j ) as usize] = n_val;
                    self.check += n_val;
                } else {
                    let n_val = 0.25 * (
                            M2.dat[(wrap(i,false) * ROW_SIZE + j) as usize] +
                            M2.dat[(wrap(i,true) * ROW_SIZE + j) as usize] +
                            M2.dat[(i * ROW_SIZE + wrap(j,false)) as usize] +
                            M2.dat[(i * ROW_SIZE + wrap(j,true)) as usize]
                        );
                    M1.dat[( i * ROW_SIZE + j ) as usize] = n_val;
                    self.check += n_val;
                };
            }
        }
        self.even = !self.even;
});

define_world!(
    cs: CellArray<HeatCell>
);


fn mm_main() -> f32 {
    let world = World {
        cs: CellArray {
            cells: [
                HeatCell { x_start: 000, x_end: 100, y_start: 000, y_end: 800, check: 0.0, even:true },
                HeatCell { x_start: 100, x_end: 200, y_start: 000, y_end: 800, check: 0.0, even:true },
                HeatCell { x_start: 200, x_end: 300, y_start: 000, y_end: 800, check: 0.0, even:true },
                HeatCell { x_start: 300, x_end: 400, y_start: 000, y_end: 800, check: 0.0, even:true },
                HeatCell { x_start: 400, x_end: 500, y_start: 000, y_end: 800, check: 0.0, even:true },
                HeatCell { x_start: 500, x_end: 600, y_start: 000, y_end: 800, check: 0.0, even:true },
                HeatCell { x_start: 600, x_end: 700, y_start: 000, y_end: 800, check: 0.0, even:true },
                HeatCell { x_start: 700, x_end: 800, y_start: 000, y_end: 800, check: 0.0, even:true },
            ]
        }
    };

    // was 10000
    let w = miso_runner(world, 4000);
    let rs = w.cs.cells.iter().map(|x| (*x).check);
    rs.fold(0.0, |p, val| p + val)
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

