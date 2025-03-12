use std::any::type_name;
use std::collections::HashMap;
use std::fmt::Display;

#[allow(unused_variables)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]

pub enum PlatformParameters {
    SIMD,
    ACCBackend,
    ACCCount
}

pub trait Feature {}
pub trait Subtype {
    fn subtypeof(self, t:impl Feature) -> bool;
}

pub struct simd;
pub struct avx512;

pub struct accmodel;
pub struct nvidia_H100;

pub struct accbackend;
pub struct cuda10;

pub struct Quantifier {
    pub val:i32
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

impl Subtype for simd {
    fn subtypeof(self, x:impl Feature) -> bool {
        let t1 = type_of(self);
        let t2 = type_of(x);
        t1 == t2 
    }
}
impl Subtype for avx512 {
    fn subtypeof(self, x:impl Feature) -> bool {
        let t1 = type_of(self);
        let t2 = type_of(x);
        t1 == t2 || t2 == "simd"
    }
}
impl Subtype for accmodel {
    fn subtypeof(self, x:impl Feature) -> bool {
        let t1 = type_of(self);
        let t2 = type_of(x);
        t1 == t2
    }
}
impl Subtype for nvidia_H100 {
    fn subtypeof(self, x:impl Feature) -> bool {
        let t1 = type_of(self);
        let t2 = type_of(x);
        t1 == t2 || t2 == "accmodel"
    }
}
impl Subtype for accbackend {
    fn subtypeof(self, x:impl Feature) -> bool {
        let t1 = type_of(self);
        let t2 = type_of(x);
        t1 == t2 
    }
}impl Subtype for cuda10 {
    fn subtypeof(self, x:impl Feature) -> bool {
        let t1 = type_of(self);
        let t2 = type_of(x);
        t1 == t2 || t2 == "accbackend"
    }
}
impl Subtype for Quantifier {
    fn subtypeof(self, x:impl Feature) -> bool {
        let t1 = type_of(self);
        let t2 = type_of(x);
        t1 == t2 
    }
}

impl Display for simd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl Display for avx512 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl Display for accmodel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl Display for nvidia_H100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl Display for accbackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl Display for cuda10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl Display for Quantifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl Feature for simd {}
impl Feature for avx512 {}
impl Feature for accmodel {}
impl Feature for nvidia_H100 {}
impl Feature for accbackend {}
impl Feature for cuda10 {}
impl Feature for Quantifier {}



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



// lazy_static! { static ref currentfeatures: HashMap<PlatformParameters, Box<dyn Feature>> = readplatformfeatures(); }

// TODO
pub fn readplatformfeatures() -> HashMap<PlatformParameters, Box<dyn Feature>>
{
    let mut m: HashMap<PlatformParameters, Box<dyn Feature>> = HashMap::new();

    println!("readplatformfeatures at startup"); 

    m.insert(PlatformParameters::SIMD, Box::new(avx512));

    return m;
}

// TODO
pub fn resolve(features:Vec<HashMap<PlatformParameters, Box<dyn Feature>>> ) -> i32
{
    let currentfeatures2: HashMap<PlatformParameters, Box<dyn Feature>> = readplatformfeatures();
    let x = currentfeatures2.get(&PlatformParameters::SIMD);

    let r:bool = match x {
        Some(v) => true,
        None => false,
    };

    return 0;
}

