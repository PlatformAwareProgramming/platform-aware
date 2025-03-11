use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;


pub enum PlatformParameters {
    simd,
    gpu_backend
}

impl PartialEq for PlatformParameters {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for PlatformParameters {}

impl std::hash::Hash for PlatformParameters {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

//static mut currentfeatures:HashMap<PlatformParameters, i32> = HashMap::new();

lazy_static! { static ref currentfeatures: HashMap<PlatformParameters, i32> = readplatformfeatures(); }

pub fn readplatformfeatures() -> HashMap<PlatformParameters, i32>
{
    let mut m = HashMap::new();

    println!("readplatformfeatures at startup"); //TODO

    m.insert(PlatformParameters::simd, 0);

    return m;
}

pub fn resolve(features:Vec<HashMap<PlatformParameters, i32>> ) -> i32
{
    let x = currentfeatures.get(&PlatformParameters::simd);

    return 0;
}

