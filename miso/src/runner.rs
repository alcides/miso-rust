use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Debug;


pub trait Transitionable : Clone + Copy + Debug {
    fn transition(&mut self);
}


pub fn miso_runner<W: Transitionable>(w:Rc<RefCell<W>>, i:u64) -> W {
    
    let mut original = w.borrow_mut();
    
    let mut backup = (*original).clone();
    
    for _ in 0..i {
        original.transition();
        backup.transition();
    }
    backup
}