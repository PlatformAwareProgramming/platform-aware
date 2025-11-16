mod accelerator;
mod platformfile;
mod processor;

use std::sync::Arc;

pub use accelerator::*;
use platform_aware_features::{AtLeast, CURRENT_FEATURES, insert_parameter, readplatformfeatures};
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
    let m1 = readplatformfeatures_nvidia();

    m0.extend(m1.into_iter());
}

