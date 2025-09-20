use platform_aware_features::{add_quantifier, readplatform, PlatformFeatures};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Platform {
    pub accelerator: Option<PlatformAccelerator>,
}

#[derive(Deserialize)]
pub struct PlatformAccelerator {
    pub acc_cudacc: Option<i32>,
    pub acc_cudatoolkit: Option<i32>,
    pub acc_cudadriver: Option<i32>,
}

pub fn readplatformfeatures_nvidia() -> PlatformFeatures {
    let t: Platform = match readplatform() {
        Ok(t) => t,
        Err(_) => todo!(),
    };

    let mut m: PlatformFeatures = ::std::collections::HashMap::new();

    match t.accelerator {
        Some(u) => {
            match u.acc_cudacc {
                Some(v) => {
                    add_quantifier(&mut m, "acc_cudacc".to_string(), v);
                }
                None => {}
            }
            match u.acc_cudatoolkit {
                Some(v) => {
                    add_quantifier(&mut m, "acc_cudatoolkit".to_string(), v);
                }
                None => {}
            }
            match u.acc_cudadriver {
                Some(v) => {
                    add_quantifier(&mut m, "acc_cudadriver".to_string(), v);
                }
                None => {}
            }
        }
        None => {}
    }

    return m;
}
