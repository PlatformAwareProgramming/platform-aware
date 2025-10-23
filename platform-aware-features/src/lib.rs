mod base; 
mod platformfile;

pub use configurable_features::*;
pub use base::*;

use platformfile::readplatformfeatures;

use ctor::ctor;

#[ctor]
fn module_initializer() {
    
    let mut m0 = CURRENT_FEATURES.lock().unwrap();
    let m = readplatformfeatures();
   
    m0.extend(m.into_iter());

}

