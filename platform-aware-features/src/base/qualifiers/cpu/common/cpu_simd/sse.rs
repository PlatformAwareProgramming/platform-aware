
use crate::{Feature, PlatformParameters, CPUSIMD};

pub struct SSE;  // <: CPUSIMD

impl Feature for SSE { 
    fn string(self:&Self) -> String { "SSE".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CPUSIMD)) }
}

pub struct SSE_2;  // <: SSE

impl Feature for SSE_2 { 
    fn string(self:&Self) -> String { "SSE_2".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(SSE)) }
}

pub struct SSE_3;  // <: SSE_2 

impl Feature for SSE_3 { 
    fn string(self:&Self) -> String { "SSE_3".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(SSE_2)) }
}

pub struct SSSE_3;  // <: SSE_3 

impl Feature for SSSE_3 { 
    fn string(self:&Self) -> String { "SSSE_3".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(SSE_3)) }
}

pub struct SSE_4;  // <: SSSE_3 

impl Feature for SSE_4 { 
    fn string(self:&Self) -> String { "SSE_4".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(SSSE_3)) }
}

pub struct SSE_4_1;  // <: SSE_4

impl Feature for SSE_4_1 { 
    fn string(self:&Self) -> String { "SSE_4_1".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(SSE_4)) }
}

pub struct SSE_4_2;  // <: SSE_4

impl Feature for SSE_4_2 { 
    fn string(self:&Self) -> String { "SSE_4_2".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(SSE_4)) }
}

pub struct SSE_4a;  // <: SSE_3

impl Feature for SSE_4a { 
    fn string(self:&Self) -> String { "SSE_4a".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(SSE_3)) }
}
