
use crate::{Feature, PlatformParameters, CPUSIMD};

pub struct AVX;  // <: ProcessorSIMD 

impl Feature for AVX { 
    fn string(self:&Self) -> String { "AVX".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CPUSIMD)) }
}

pub struct AVX2;  // <: AVX 

impl Feature for AVX2 { 
    fn string(self:&Self) -> String { "AVX2".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AVX)) }
}
pub struct AVX512;  // <: AVX2   # https://en.wikipedia.org/wiki/AVX-512

impl Feature for AVX512 { 
    fn string(self:&Self) -> String { "AVX512".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AVX2)) }
}
