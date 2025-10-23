mod base; 
mod platformfile;
mod featuremap;
mod resolve;

pub use base::*;
use ctor::ctor;
pub use platformfile::*;
pub use resolve::*;
pub use featuremap::*;

 
#[ctor]
fn module_initializer() {
    
    let mut m0 = CURRENT_FEATURES.lock().unwrap();
    let m = readplatformfeatures();
   
    m0.extend(m.into_iter());

}

