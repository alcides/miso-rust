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
                   pub fn transition(&mut $sel, &$prev: &$dest_name, &$world: &World) {
                       $code
                   }
               }
           
           }
    }
    
}