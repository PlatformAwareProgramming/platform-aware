
use crate::{Feature, PlatformParameters, CPUSIMD};

pub struct AVX;  // <: ProcessorSIMD 

impl Feature for AVX { 
     fn string(self:&Self) -> &'static str { "AVX" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::cpu_simd) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CPUSIMD)) }
}

pub struct AVX2;  // <: AVX 

impl Feature for AVX2 { 
     fn string(self:&Self) -> &'static str { "AVX2" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::cpu_simd) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AVX)) }
}
pub struct AVX512;  // <: AVX2   # https://en.wikipedia.org/wiki/AVX-512

impl Feature for AVX512 { 
     fn string(self:&Self) -> &'static str { "AVX512" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::cpu_simd) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AVX2)) }
}

use crate::insert_feature;
use std::sync::Arc;
use ctor::ctor;

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(AVX));
    insert_feature(Arc::new(AVX2));
    insert_feature(Arc::new(AVX512));
}