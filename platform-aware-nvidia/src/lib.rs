mod accelerator;
mod platformfile;
mod processor;

use std::sync::Arc;

pub use accelerator::*;
use platform_aware_features::{insert_parameter, AtLeast, CURRENT_FEATURES};
pub use platformfile::*;
#[allow(unused_imports)]
pub use processor::*;

use ctor::ctor;

#[ctor]
fn module_initializer() {
    insert_parameter("acc_cudacc".to_string(), Arc::new(AtLeast { val: 10 }));
    insert_parameter("acc_cudatoolkit".to_string(), Arc::new(AtLeast { val: 10 }));
    insert_parameter("acc_cudadriver".to_string(), Arc::new(AtLeast { val: 100 }));

    let mut m0 = CURRENT_FEATURES.lock().unwrap();
    let m = readplatformfeatures_nvidia();

    m0.extend(m.into_iter());
}
