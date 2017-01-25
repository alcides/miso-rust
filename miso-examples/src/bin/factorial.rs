#[macro_use]
extern crate miso;
extern crate num;

use miso::runner::miso_runner;
use num::BigUint;
use std::u64::MAX;
use num::ToPrimitive;

mod benchmark;

define_cell!( FactorialCell {
    lower: u64,
    upper: u64,
    value: u64,
    overflows: u64
    } => self, previous, world {
        let mut b:BigUint = BigUint::from(self.value);
        for i in self.lower..self.upper {
            b = BigUint::from(i) * b;
        }
        let b_ = b.clone();
        self.overflows = match (b_ / BigUint::from(MAX)).to_u64() {
            Some(x) => x,
            None => 0
        };
        self.value = match (b % BigUint::from(MAX)).to_u64() {
            Some(x) => x,
            None => 0
        };
});

define_world_par!(
    facts: CellArray<FactorialCell>
);

fn extract(c : FactorialCell) -> BigUint {
    BigUint::from(c.value) + (BigUint::from(c.overflows) * BigUint::from(MAX))
}


fn fib_main() -> BigUint {
    let world = World { 
        facts : CellArray {
            cells : [
                FactorialCell { lower: 1, upper: 10000, value: 1, overflows: 0},
                FactorialCell { lower: 10000, upper: 20000, value: 1, overflows: 0},
                FactorialCell { lower: 20000, upper: 30000, value: 1, overflows: 0},
                FactorialCell { lower: 30000, upper: 40000, value: 1, overflows: 0},
                FactorialCell { lower: 40000, upper: 50000, value: 1, overflows: 0},
                FactorialCell { lower: 50000, upper: 60000, value: 1, overflows: 0},
                FactorialCell { lower: 60000, upper: 70000, value: 1, overflows: 0},
                FactorialCell { lower: 70000, upper: 80000, value: 1, overflows: 0},
            ]
        }
    };
    
    let w = miso_runner(world, 1);
    
    let rs = w.facts.cells.iter().map(|x| extract(*x));
    rs.fold(BigUint::from(1u64), |p, val| p * val)
}

#[allow(unused_variables)]
fn main() {
    benchmark::benchmark(|| {
        let r = fib_main();
        println!("80000! = {}", r);
    });
}
