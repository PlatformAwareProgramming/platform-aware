use crate::{Feature, PlatformParameters, StoType};

pub struct SSD;  // <: StoType

impl Feature for SSD { 
    fn string(self:&Self) -> String { "SSD".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(StoType)) }
}

pub struct HDD;  // <: StoType

impl Feature for HDD { 
    fn string(self:&Self) -> String { "HDD".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(StoType)) }
}