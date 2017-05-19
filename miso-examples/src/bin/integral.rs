#[macro_use]
extern crate miso;

use miso::runner::miso_runner;
use std::env::args;

mod benchmark;

const RESOLUTION:f64 = 100000.0;


define_cell!( IntegralCell {
    x_start: f64,
    x_end: f64,
    int: f64
    } => self, previous, world {
        self.int = 0.0;
        let mut pos = self.x_start;
        while pos < self.x_end {
           let bl: f64 = std::f64::consts::E.powf(pos.sin() as f64);
           let bu: f64 = std::f64::consts::E.powf((pos+1.0/RESOLUTION).sin() as f64);
           self.int += (bl+bu) / 2.0;
           pos += 1.0/RESOLUTION;   
        }
});

define_world!(
    cs: CellArray<IntegralCell>
);


fn integral_main() -> f64 {
    let world = World { 
        cs: CellArray {
            cells: [
                IntegralCell { x_start: 000.0, x_end: 100.0, int: 0.0},
                IntegralCell { x_start: 100.0, x_end: 200.0, int: 0.0},
                IntegralCell { x_start: 200.0, x_end: 300.0, int: 0.0},
                IntegralCell { x_start: 300.0, x_end: 400.0, int: 0.0},
                IntegralCell { x_start: 400.0, x_end: 500.0, int: 0.0},
                IntegralCell { x_start: 500.0, x_end: 600.0, int: 0.0},
                IntegralCell { x_start: 600.0, x_end: 700.0, int: 0.0},
                IntegralCell { x_start: 700.0, x_end: 800.0, int: 0.0},
            ]
        }
    };
    
    let w = miso_runner(world, 1);
    let rs = w.cs.cells.iter().map(|x| (*x).int);
    rs.fold(0.0, |p, val| p + val)
}

#[allow(unused_variables)]
fn main() {
    
    benchmark::benchmark(move || {
        let m3 = integral_main();
        if args().count() > 1 {
            println!("check = {:?}", m3);
        }
        m3.to_string()
    });
}