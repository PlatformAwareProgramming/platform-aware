use std::any::type_name;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};

#[allow(unused_variables)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]

pub enum PlatformParameters {
    SIMD,
    ACCModel,
    ACCBackend,
    ACCCount
}

pub enum FeatureType {
    Qualifier,
    QuantifierUp,
    QuantifierDown,
}


pub trait Feature {

    fn string(self:&Self) -> String;

    fn hash_code(self:&Self) -> u64 {
        let mut s = DefaultHasher::new();
        self.string().hash(&mut s);
        let h = s.finish();
        h
    }

    fn feature_class(self:&Self) -> PlatformParameters;

    fn feature_type(self:&Self) -> FeatureType {
        FeatureType::Qualifier
    }

    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { 
        match self.feature_type() {
            FeatureType::Qualifier => None,
            FeatureType::QuantifierUp => {   
                let v = self.val().expect("unexpected error");        
                Some(Box::new(acc_count { val:v+1 }) as Box<dyn Feature>)                         
            },
            FeatureType::QuantifierDown => {   
                let v = self.val().expect("unexpected error");        
                Some(Box::new(acc_count { val:v-1 }) as Box<dyn Feature>)                         
            },
        }
    } 

    fn subtypeof(self:&Self, other:&dyn Feature) -> bool {
        if self.feature_class() != other.feature_class() { return false };
        match self.feature_type() {
            FeatureType::Qualifier => {
                let mut s = self.supertype();
                loop {
                    match s {
                        Some(ref f) => { 
                            if f.hash_code() == other.hash_code() { break } 
                            else { s = s.expect("unexpected error").supertype() }
                        },
                        None => break
                    };                    
                };
                s.is_some()
            },
            FeatureType::QuantifierUp => {       
                let v1 = self.val().expect("unexpected error");
                let v2 = other.val().expect("unexpected error");
                v1 <= v2
            },
            FeatureType::QuantifierDown => {
                let v1 = self.val().expect("unexpected error");
                let v2 = other.val().expect("unexpected error");
                v1 >= v2
            },
        }
    }

    fn val(self:&Self) -> Option<i32> { None }
}

pub struct simd;
pub struct avx512;
pub struct acc_model;
pub struct nvidia_H100;
pub struct acc_backend;
pub struct cuda10;
#[derive(Clone)]
pub struct acc_count{
    pub val:i32
}

impl Feature for simd { fn string(self:&Self) -> String { "simd".to_string() } 
                        fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::SIMD } }
impl Feature for avx512 { fn string(self:&Self) -> String { "avx512".to_string() } 
                          fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(simd)) } 
                          fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::SIMD } }
impl Feature for acc_model { fn string(self:&Self) -> String { "acc_model".to_string() } 
                             fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::ACCModel } }
impl Feature for nvidia_H100 { fn string(self:&Self) -> String { "nvidia_H100".to_string() } 
                               fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(acc_model)) } 
                               fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::ACCModel } }
impl Feature for acc_backend { fn string(self:&Self) -> String { "acc_backend".to_string() } 
                               fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::ACCBackend } }
impl Feature for cuda10 { fn string(self:&Self) -> String { "cuda10".to_string() } 
                          fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(acc_backend)) } 
                             fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::ACCBackend } }
impl Feature for acc_count { fn string(self:&Self) -> String { "acc_count".to_string() } 
                             fn feature_type(self:&Self) -> FeatureType { FeatureType::QuantifierUp } 
                             fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::ACCCount}
                             fn val(self:&Self) -> Option<i32> { Some((*self).val) } }






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

// lazy_static! { static ref currentfeatures: Mutex<HashMap<PlatformParameters, &'static dyn Feature>> = Mutex::new(readplatformfeatures()); }

// TODO
pub fn readplatformfeatures() -> HashMap<PlatformParameters, &'static dyn Feature>
{
    let mut m: HashMap<PlatformParameters, &dyn Feature> = HashMap::new();

    println!("readplatformfeatures at startup"); 

    m.insert(PlatformParameters::SIMD, &avx512);
    m.insert(PlatformParameters::ACCCount, &acc_count{ val:10 });

    return m;
}


// TODO
pub fn resolve(features:Vec<HashMap<PlatformParameters, &dyn Feature>> ) -> i32
{
    let currentfeatures2: HashMap<PlatformParameters, &dyn Feature> = readplatformfeatures();
    let x = currentfeatures2.get(&PlatformParameters::ACCCount).expect("unexpected error").to_owned();

    let xx = x.feature_type();
    let xxx = match xx {
        FeatureType::Qualifier => 0,
        FeatureType::QuantifierUp => x.val().expect("error"),
        FeatureType::QuantifierDown => x.val().expect("error")
    };

    println!("V = {}", xxx);

    let xxxx = match xx {
        FeatureType::Qualifier => { let y = x.supertype(); &*y.expect("error") },
        FeatureType::QuantifierUp => &acc_count { val:x.val().expect("error") } as &dyn Feature,
        FeatureType::QuantifierDown => &acc_count { val:x.val().expect("error") } as &dyn Feature
    };
    
    
        //let xxx = *xx;

    return 0;
}

