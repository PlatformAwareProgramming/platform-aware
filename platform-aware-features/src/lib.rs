mod base; 
mod platformfile;

pub use configurable_features::*;
pub use base::*;

pub use platformfile::{readplatformfeatures, readplatform, readqualifierlist, readqualifier, readquantifier};

use ctor::ctor;

#[ctor]
fn module_initializer() {
    register_feature_loader(readplatformfeatures);
}

