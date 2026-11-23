use std::sync::Mutex;

use once_cell::sync::Lazy;
use platform_aware_features::{PlatformFeatures, readplatform, readquantifier};
use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct Platform {
    pub accelerator: Option<PlatformAccelerator>,
}

#[derive(Deserialize)]
pub struct PlatformAccelerator {
    pub acc_cudacc: Option<i32>,
    pub acc_cudatoolkit: Option<i32>,
    pub acc_cudadriver: Option<i32>,
}

pub static CURRENT_PLATFORM_NVIDIA: Lazy<Mutex<Platform>> = Lazy::new(|| {
    let t: Platform = readplatform().unwrap_or_default();
    Mutex::new(t)
});

pub fn readplatformfeatures_nvidia() -> PlatformFeatures {
    let t = CURRENT_PLATFORM_NVIDIA.lock().unwrap();

    let mut m: PlatformFeatures = ::std::collections::HashMap::new();

    // Load NVIDIA-specific parameters; common accelerator qualifiers (acc_backend,
    // acc_interconnect, acc_model) are loaded lazily by platform-aware-features
    // after all vendor #[ctor] registrations complete.
    if let Some(u) = &t.accelerator {
        readquantifier(u.acc_cudacc, "acc_cudacc", &mut m);
        readquantifier(u.acc_cudatoolkit, "acc_cudatoolkit", &mut m);
        readquantifier(u.acc_cudadriver, "acc_cudadriver", &mut m);
    }

    m
}
