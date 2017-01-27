#[macro_use]
mod miso {
    
    
    #[cfg(feature = "seq")]
    #[macro_export]
    macro_rules! define_world {
        ($($element: ident: $ty: ty),*) => {
            #[derive(Clone, Copy, Eq, PartialEq, Debug)]
            struct World { $($element: $ty),* }
            
            trait Cell<T> : Copy + Clone + Eq + PartialEq {
                fn transition(&mut self, prev:&T, w:&World);
            }
            
            #[derive(Clone, Copy, Eq, PartialEq, Debug)]
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
            
            #[derive(Clone, Copy, Eq, PartialEq, Debug)]
            struct World { 
                $($element: $ty),*
            }
            
            
            trait Cell<T> : Copy + Clone + Eq + PartialEq {
                fn transition(&mut self, prev:&T, w:&World);
            }
            
            #[derive(Clone, Copy, Eq, PartialEq, Debug)]
            struct CellArray<T> where T : Cell<T> {
                cells : [T; 8]
            }
            
            impl<T> CellArray<T> where T : Cell<T> {
                fn transition(&mut self, &p: &CellArray<T>, &world: &World) {
                    
                    for (n, o) in self.cells.iter_mut().zip(p.cells.iter()) {
                        n.transition(&o, &world);
                    }
                }
            }
            
        
            use std::thread;
            use miso::runner::Transitionable;
            impl Transitionable for World {
                
                #[allow(unused_assignments)]
                #[allow(path_statements)]
                fn transition(&mut self) {
                    let ref old_world = self.clone();
                    let mut handles = Vec::new();
                    {
                        let mut self_clone = self.clone();
                        let ow = old_world.clone();
                    $(
                        handles.push(thread::spawn( move || {
                            self_clone.$element.transition(&ow.$element, &ow);
                        }));
                    ) *
                    }
                    
                    #[allow(unused_must_use)]
                    for t in handles {
                        t.join();
                    }
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
               
               #[derive(Clone, Copy, Eq, PartialEq, Debug)]
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