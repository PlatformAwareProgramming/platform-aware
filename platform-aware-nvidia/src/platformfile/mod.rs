use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use platform_aware_features::{PlatformFeatures, add_qualifier, add_quantifier, readfeaturevector, readplatform, readqualifier, readquantifier};
use serde::Deserialize;
use toml::value::Array;

#[derive(Deserialize)]
pub struct Platform {
    pub accelerator: Option<PlatformAccelerator>,
}

#[derive(Deserialize)]
pub struct PlatformAccelerator {
    pub acc_backend: Option<Array>,              // from platform_aware_features
    pub acc_interconnect: Option<String>,        // from platform_aware_features 
    pub acc_model: Option<String>,               // from platform_aware_features
    pub acc_cudacc: Option<i32>,
    pub acc_cudatoolkit: Option<i32>,
    pub acc_cudadriver: Option<i32>,
}

pub static CURRENT_PLATFORM_NVIDIA: Lazy<Mutex<Platform>> = Lazy::new(|| {
    let t: Platform = match readplatform() {
        Ok(t) => t,
        Err(_) => todo!(),
    };
    Mutex::new(t)
});

pub fn readplatformfeatures_nvidia() -> PlatformFeatures {

   let t = CURRENT_PLATFORM_NVIDIA.lock().unwrap();

    let mut m: PlatformFeatures = ::std::collections::HashMap::new();

    // only modify parameters that has NVIDIA features.
    match &t.accelerator {
        Some(u) => {
            readfeaturevector(u.acc_backend.clone(), "acc_backend",  &mut m);
            readqualifier(u.acc_interconnect.clone(), "acc_interconnect", &mut m);
            readqualifier(u.acc_model.clone(), "acc_model", &mut m);
            readquantifier(u.acc_cudacc.clone(), "acc_cudacc", &mut m);
            readquantifier(u.acc_cudatoolkit.clone(), "acc_cudatoolkit", &mut m);
            readquantifier(u.acc_cudadriver.clone(), "acc_cudadriver", &mut m);
        }
        None => {}
    }

    return m;
}
