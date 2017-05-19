#[macro_use]
mod miso {
    
    
    #[cfg(not(feature = "par"))]
    #[macro_export]
    macro_rules! define_world {
        ($($element: ident: $ty: ty),*) => {
            #[derive(Clone, Copy, PartialEq, Debug)]
            struct World { $($element: $ty),* }
            
            trait Cell<T> : Copy + Clone + PartialEq {
                fn transition(&mut self, prev:&T, w:&World);
            }
            
            #[derive(Clone, Copy, PartialEq, Debug)]
            struct CellArray<T> where T : Cell<T> {
                cells : [T; 8]
            }
            
            impl<T> CellArray<T> where T : Cell<T> {
                #[allow(dead_code)] 
                fn transition(&mut self, &p: &CellArray<T>, &world: &World) {
                    
                    for (n, o) in self.cells.iter_mut().zip(p.cells.iter()) {
                        n.transition(&o, &world);
                    }
                }
            }
        
            use miso::runner::Transitionable;
            impl Transitionable for World {
                fn transition(&mut self) {
                    let ref old_world = self.clone();
                
                    $(
                        let mut $element = self.$element.clone();
                        $element.transition(&old_world.$element, &old_world);
                        self.$element = $element;
                    )*
                }
            }
        }
    }
    
    #[cfg(feature = "par")]
    #[macro_export]
    macro_rules! define_world {
        ($($element: ident: $ty: ty),*) => {
            
            #[derive(Clone, Copy, PartialEq, Debug)]
            struct World { 
                $($element: $ty),*
            }
            
            
            trait Cell<T> : Copy + Clone + PartialEq + Sync + Send {
                fn transition(&mut self, prev:&T, w:&World);
            }
            
            #[derive(Clone, Copy, PartialEq, Debug)]
            struct CellArray<T> where T : Cell<T> {
                cells : [T; 8]
            }
            
            use std::sync::{Arc,RwLock};
            
            impl<T> CellArray<T> where T : Cell<T>, T: Send + 'static {
                #[allow(dead_code)]
                fn transition(&mut self, &p: &CellArray<T>, &world: &World) {
                    
                    let old_world = world.clone(); // Copy
                                        
                    let mut handles = Vec::new();
                    let mut refs = Vec::new();
                    
                    for i in 0..8 {
                        let r = Arc::new(RwLock::new(self.cells[i]));
                        refs.push(r);
                        let inner_e = refs[i].clone();
                        {
                            let ow = old_world.clone();

                            handles.push(thread::spawn( move || {
                                let mut inner_e_v = inner_e.write().unwrap();
                                inner_e_v.transition(&p.cells[i], &ow);
                            }));
                        }
                    }
                    
                    #[allow(unused_must_use)]
                    for t in handles {
                        t.join();
                    }
                    
                    for i in 0..8 {
                        {
                            let e = refs[i].read().unwrap();
                            self.cells[i] = e.clone();
                        }
                    }
                }
            }
            
        
            use std::thread;
            use miso::runner::Transitionable;
            impl Transitionable for World {
                
                #[allow(unused_assignments)]
                #[allow(path_statements)]
                fn transition(&mut self) {
                    
                    let old_world = self.clone(); // Copy
                                        
                    let mut handles = Vec::new();
                    
                    $(
                        let $element = Arc::new(RwLock::new(self.$element));
                        {
                            let ow = old_world.clone();
                            
                            let inner_e = ($element).clone();
                            handles.push(thread::spawn( move || {
                                let mut inner_e_v = inner_e.write().unwrap();
                                inner_e_v.transition(&ow.$element, &ow);
                            }));
                        }
                    ) *
                    
                    #[allow(unused_must_use)]
                    for t in handles {
                        t.join();
                    }
                    
                    $(
                        {
                            let e = (*$element).read().unwrap();
                            self.$element = e.clone();
                        }
                    ) *
                  
                }
            }
        }
    }
    
    #[macro_export]
    macro_rules! define_cell {
        ( $dest_name:ident {
               $( $attr_name:ident : $attr_type:ty ),*
               } => $sel:ident, $prev:ident, $world:ident $code:block)
           => {
               
               #[derive(Clone, Copy, PartialEq, Debug)]
               struct $dest_name {
                   $( $attr_name : $attr_type ),*
               }

              
               impl Cell<$dest_name> for $dest_name {
                   #[allow(unused_variables)]
                   fn transition(&mut $sel, &$prev: &$dest_name, &$world: &World) {
                       $code
                   }
               }
           
           }
    }
    
}