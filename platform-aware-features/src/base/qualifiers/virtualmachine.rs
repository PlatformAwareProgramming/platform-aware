
use crate::{Feature, PlatformParameters};

pub struct VMprovider;
pub struct VMmachinefamily;
pub struct VMmachinetype;

impl Feature for VMprovider { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "VMprovider".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::vm_provider) } 
}

impl Feature for VMmachinefamily { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "VMmachinefamily".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::vm_machinefamily) } 
}

impl Feature for VMmachinetype { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "VMmachinetype".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::vm_machinetype) } 
}
