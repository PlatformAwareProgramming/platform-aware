use crate::{Feature, PlatformParameters, AccType};

pub struct XPU;

impl Feature for XPU { 
    fn string(self:&Self) -> String { "XPU".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_type) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccType)) }
}

pub struct GPU;
pub struct TPU;
pub struct IPU;

impl Feature for GPU { 
    fn string(self:&Self) -> String { "GPU".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_type) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(XPU)) }
}

impl Feature for TPU { 
    fn string(self:&Self) -> String { "TPU".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_type) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(XPU)) }
}

impl Feature for IPU { 
    fn string(self:&Self) -> String { "IPU".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_type) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(XPU)) }
}