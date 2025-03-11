use std::collections::HashMap;
use lazy_static::lazy_static;

#[allow(unused_variables)]

pub enum PlatformParameters {
    SIMD,
    ACCBackend,
    ACCCount
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

// TODO
pub fn readplatformfeatures() -> HashMap<PlatformParameters, i32>
{
    let mut m = HashMap::new();

    println!("readplatformfeatures at startup"); 

    m.insert(PlatformParameters::SIMD, 0);

    return m;
}

// TODO
pub fn resolve(features:Vec<HashMap<PlatformParameters, i32>> ) -> i32
{
    let x = currentfeatures.get(&PlatformParameters::SIMD);

    return 0;
}

