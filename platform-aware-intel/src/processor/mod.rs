mod simd;
mod microarchitecture;
mod model;

pub use simd::*;
pub use microarchitecture::*;
pub use model::*;

use platform_aware_features::{CPUManufacturer, create_feature_hierarchy};

create_feature_hierarchy! { register_features_manufacturer ; "cpu_manufacturer" : CPUManufacturer :> Intel; }







