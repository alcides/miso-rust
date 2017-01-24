#[macro_use]
mod miso {
    

    
    #[macro_export]
    macro_rules! define_world {
        ($($element: ident: $ty: ty),*) => {
            #[derive(Clone, Copy, Eq, PartialEq, Debug)]
            struct World { $($element: $ty),* }
        
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
    
    #[macro_export]
    macro_rules! define_world_par {
        ($($element: ident: $ty: ty),*) => {
            
            extern crate rayon;
            
            #[derive(Clone, Copy, Eq, PartialEq, Debug)]
            struct World { 
                $($element: $ty),*
            }
        
            use miso::runner::Transitionable;
            impl Transitionable for World {
                
                #[allow(unused_assignments)]
                #[allow(path_statements)]
                fn transition(&mut self) {
                    let ref old_world = self.clone();
                    
                    let mut a = rayon::join (|| { () }, || { () }); // dummy
                    
                    $(
                        a = rayon::join( || { a; ()}, || {
                            self.$element.transition(&old_world.$element, &old_world); 
                            ()
                        });
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
               
               #[derive(Clone, Copy, Eq, PartialEq, Debug)]
               struct $dest_name {
                   $( $attr_name : $attr_type ),*
               }
              
               impl $dest_name {
                   #[allow(unused_variables)]
                   fn transition(&mut $sel, &$prev: &$dest_name, &$world: &World) {
                       $code
                   }
               }
           
           }
    }
    
}