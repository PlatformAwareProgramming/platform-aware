mod processor;
mod accelerator;
mod platformfile;

use std::sync::Arc;

use platform_aware_features::{insert_parameter, AtLeast, CURRENT_FEATURES};
#[allow(unused_imports)]
pub use processor::*;
pub use accelerator::*;
pub use platformfile::*;

use ctor::ctor;

#[ctor]
fn module_initializer() {
    insert_parameter("acc_cudacc".to_string(), Arc::new(AtLeast{val:10}));
    insert_parameter("acc_cudatoolkit".to_string(), Arc::new(AtLeast{val:10}));
    insert_parameter("acc_cudadriver".to_string(), Arc::new(AtLeast{val:100}));

    let mut m0 = CURRENT_FEATURES.lock().unwrap();
    let m = readnvidiaplatformfeatures();
   
    m0.extend(m.into_iter());

}