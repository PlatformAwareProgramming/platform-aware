
use std::{env, error::Error, fs, path::Path};

use platform_aware_features::{add_quantifier, PlatformFeatures};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct Platform {
    pub accelerator:Option<PlatformAccelerator>,
}

#[derive(Deserialize)]
pub struct PlatformAccelerator {
    pub acc_cudacc:Option<i32>,
    pub acc_cudatoolkit:Option<i32>,
    pub acc_cudadriver:Option<i32>,
}

fn readplatform() -> Result<Platform,Box<dyn Error>> {

    let platform_path = match env::var("PLATFORM_DESCRIPTION") {
        Ok(var) => var,
        Err(_) => env::var("PWD")?
    };

    let contents: String = fs::read_to_string(Path::new(&platform_path).join("Platform.toml"))?;

    Ok(toml::from_str(&contents).unwrap())
}

pub fn readnvidiaplatformfeatures() -> PlatformFeatures
{
    let t = match readplatform() {
        Ok(t) => t,
        Err(_) => todo!(),
    };

   let mut m: PlatformFeatures = ::std::collections::HashMap::new();

    match t.accelerator {
        Some(u) => {
            match u.acc_cudacc { Some(v) => { add_quantifier(&mut m, "acc_cudacc".to_string(), v); }, None => {} }
            match u.acc_cudatoolkit { Some(v) => { add_quantifier(&mut m, "acc_cudatoolkit".to_string(), v); }, None => {} }
            match u.acc_cudadriver { Some(v) => { add_quantifier(&mut m, "acc_cudadriver".to_string(), v); }, None => {} }
        },
        None => {},
    }

   return m;
}




