use crate::{Feature, PlatformParameters, CPUISA};

pub struct x86_32;  // <: CPUISA

impl Feature for x86_32 { 
    fn string(self:&Self) -> String { "x86_32".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CPUISA)) }
}

pub struct x86_64;  // <: x86_32

impl Feature for x86_64 { 
    fn string(self:&Self) -> String { "x86_64".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(x86_32)) }
}

pub struct x86_64_v1;  // <: x86_64

impl Feature for x86_64_v1 { 
    fn string(self:&Self) -> String { "x86_64_v1".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(x86_64)) }
}
pub struct x86_64_v2;  // <: x86_64_v1

impl Feature for x86_64_v2 { 
    fn string(self:&Self) -> String { "x86_64_v2".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(x86_64_v1)) }
}

pub struct x86_64_v3;  // <: x86_64_v2

impl Feature for x86_64_v3 { 
    fn string(self:&Self) -> String { "x86_64_v3".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(x86_64_v2)) }
}

pub struct x86_64_v4;  // <: x86_64_v3

impl Feature for x86_64_v4 { 
    fn string(self:&Self) -> String { "x86_64_v4".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(x86_64_v3)) }
}

pub struct IA_64;  // <: CPUISA

impl Feature for IA_64 { 
    fn string(self:&Self) -> String { "IA_64".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CPUISA)) }
}
