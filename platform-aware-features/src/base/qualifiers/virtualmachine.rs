
use crate::{Feature, PlatformParameter};

pub struct VMprovider;
pub struct VMmachinefamily;
pub struct VMmachinetype;

impl Feature for VMprovider { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "VMprovider" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some(PlatformParameter::vm_provider) } 
}

impl Feature for VMmachinefamily { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "VMmachinefamily" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some(PlatformParameter::vm_machinefamily) } 
}

impl Feature for VMmachinetype { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "VMmachinetype" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some(PlatformParameter::vm_machinetype) } 
}
