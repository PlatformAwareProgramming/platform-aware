mod processor;
mod accelerator;

use std::sync::Arc;

use platform_aware_features::{insert_parameter, AtLeast, PlatformParameter};
#[allow(unused_imports)]
pub use processor::*;
pub use accelerator::*;

use ctor::ctor;

#[ctor]
fn module_initializer() {
    insert_parameter(PlatformParameter::acc_cudacc, Arc::new(AtLeast{val:10}));
    insert_parameter(PlatformParameter::acc_cudatoolkit, Arc::new(AtLeast{val:10}));
    insert_parameter(PlatformParameter::acc_cudadriver, Arc::new(AtLeast{val:100}));
}