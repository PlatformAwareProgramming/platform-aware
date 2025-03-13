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

pub enum FeatureType {
    Qualifier,
    Quantifier
}

pub trait Feature {
    fn feature_type(self:&Self) -> FeatureType {
        FeatureType::Qualifier
    }
}

pub trait Quantifier {
    fn val(self:&Self) -> &Self where Self: Sized {
        self
    }
}

pub trait Qualifier {
    fn supertype() -> &'static dyn Feature; 
}

pub struct top;

pub struct simd;
pub struct avx512;
pub struct accmodel;
pub struct nvidia_H100;
pub struct accbackend;
pub struct cuda10;

impl Feature for top {}
impl Feature for simd {}
impl Feature for avx512 {}
impl Feature for accmodel {}
impl Feature for nvidia_H100 {}
impl Feature for accbackend {}
impl Feature for cuda10 {}
impl Feature for i32 { fn feature_type(self:&Self) -> FeatureType { FeatureType::Quantifier } }
impl Feature for f32 { fn feature_type(self:&Self) -> FeatureType { FeatureType::Quantifier } }

impl Qualifier for simd { fn supertype() -> &'static dyn Feature { &top } }
impl Qualifier for avx512 { fn supertype() -> &'static dyn Feature { &simd } }
impl Qualifier for accmodel { fn supertype() -> &'static dyn Feature { &top } }
impl Qualifier for nvidia_H100 { fn supertype() -> &'static dyn Feature { &accmodel } }
impl Qualifier for accbackend { fn supertype() -> &'static dyn Feature { &top } }
impl Qualifier for cuda10 { fn supertype() -> &'static dyn Feature { &accbackend } }

impl Quantifier for i32 {}
impl Quantifier for f32 {}


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

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! { static ref currentfeatures: Mutex<HashMap<PlatformParameters, &'static dyn Feature>> = Mutex::new(readplatformfeatures()); }

// TODO
pub fn readplatformfeatures() -> HashMap<PlatformParameters, &'static dyn Feature>
{
    let mut m: HashMap<PlatformParameters, &dyn Feature> = HashMap::new();

    println!("readplatformfeatures at startup"); 

    m.insert(PlatformParameters::SIMD, &avx512);
    m.insert(PlatformParameters::SIMD, &0);

    return m;
}

use std::mem::transmute;

// TODO
pub fn resolve(features:Vec<HashMap<PlatformParameters, &dyn Feature>> ) -> i32
{
    let currentfeatures2: HashMap<PlatformParameters, &dyn Feature> = readplatformfeatures();
    let x = currentfeatures2.get(&PlatformParameters::SIMD).expect("unexpected error").to_owned();

    let xx = x.feature_type();
    let xxx = match xx {
        FeatureType::Qualifier => 0,
        FeatureType::Quantifier => x.val()
    };

    //let xxx = *xx;

    return 0;
}

