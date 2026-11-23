mod accelerator;
mod platformfile;
mod processor;

use std::sync::Arc;

pub use accelerator::*;
use platform_aware_features::{
    AccManufacturer, AtLeast, create_feature_hierarchy, insert_parameter, register_feature_loader,
};
pub use platformfile::*;
#[allow(unused_imports)]
pub use processor::*;

create_feature_hierarchy! {
    register_features_manufacturer_nvidia ; "acc_manufacturer" :
    AccManufacturer :> NVIDIA;
}

use ctor::ctor;

#[ctor]
fn module_initializer() {
    insert_parameter("acc_cudacc".to_string(), Arc::new(AtLeast { val: 10 }));
    insert_parameter("acc_cudatoolkit".to_string(), Arc::new(AtLeast { val: 10 }));
    insert_parameter("acc_cudadriver".to_string(), Arc::new(AtLeast { val: 100 }));

    register_feature_loader(readplatformfeatures_nvidia);
}

#[cfg(test)]
mod tests {
    use super::*;
    use platform_aware_features::{AccBackend, AccManufacturer, AccModel, Feature};

    #[test]
    fn test_nvidia_taxonomies_and_listing_9_qualifiers() {
        assert!(NVIDIA_A100_80GB.feature_obj().subtypeof(&NVIDIA_A100.feature_obj()));
        assert!(NVIDIA_A100_80GB.feature_obj().subtypeof(&NVIDIAAccelerator.feature_obj()));
        assert!(NVIDIA_A100_80GB.feature_obj().subtypeof(&AccModel.feature_obj()));
        assert!(NVIDIAGeForce_MX570A.feature_obj().subtypeof(&NVIDIAGeForce_MX570.feature_obj()));
        assert!(NVIDIAGeForce_MX570A.feature_obj().subtypeof(&NVIDIAGeForce.feature_obj()));
        assert!(CUDA_12_0.feature_obj().subtypeof(&CUDA120.feature_obj()));
        assert!(CUDA_12_0.feature_obj().subtypeof(&CUDA.feature_obj()));
        assert!(CUDA_12_0.feature_obj().subtypeof(&AccBackend.feature_obj()));
        assert!(NVIDIA.feature_obj().subtypeof(&AccManufacturer.feature_obj()));
    }
}

