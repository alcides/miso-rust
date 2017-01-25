

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

fn extract(c : FactorialCell) -> BigUint {
    BigUint::from(c.value) + (BigUint::from(c.overflows) * BigUint::from(MAX))
}

define_world!(
    fc1: FactorialCell,
    fc2: FactorialCell,
    fc3: FactorialCell,
    fc4: FactorialCell,
    fc5: FactorialCell,
    fc6: FactorialCell,
    fc7: FactorialCell,
    fc8: FactorialCell
);


fn fib_main() -> BigUint {
    let world = World { 
        fc1: FactorialCell { lower: 1, upper: 10000, value: 1, overflows: 0},
        fc2: FactorialCell { lower: 10000, upper: 20000, value: 1, overflows: 0},
        fc3: FactorialCell { lower: 20000, upper: 30000, value: 1, overflows: 0},
        fc4: FactorialCell { lower: 30000, upper: 40000, value: 1, overflows: 0},
        fc5: FactorialCell { lower: 40000, upper: 50000, value: 1, overflows: 0},
        fc6: FactorialCell { lower: 50000, upper: 60000, value: 1, overflows: 0},
        fc7: FactorialCell { lower: 60000, upper: 70000, value: 1, overflows: 0},
        fc8: FactorialCell { lower: 70000, upper: 80000, value: 1, overflows: 0},
    };
    
    let w = miso_runner(world, 1);
    extract(w.fc1) * extract(w.fc2) * extract(w.fc3) * extract(w.fc4)
     * extract(w.fc5) * extract(w.fc6) * extract(w.fc7) * extract(w.fc8)
}

#[allow(unused_variables)]
fn main() {
    benchmark::benchmark(|| {
        let r = fib_main();
        println!("80000! = {}", r);
    });
}
