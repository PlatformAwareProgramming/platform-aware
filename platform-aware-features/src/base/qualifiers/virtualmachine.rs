use crate::{Feature, PlatformParameter};

pub struct VMprovider;
pub struct VMmachinefamily;
pub struct VMmachinetype;

impl Feature for VMprovider {
    fn is_top(self: &Self) -> bool {
        true
    }
    fn string(self: &Self) -> &'static str {
        "VMprovider"
    }
    fn feature_class(self: &Self) -> std::option::Option<PlatformParameter> {
        Some("vm_provider".to_string())
    }
}

impl Feature for VMmachinefamily {
    fn is_top(self: &Self) -> bool {
        true
    }
    fn string(self: &Self) -> &'static str {
        "VMmachinefamily"
    }
    fn feature_class(self: &Self) -> std::option::Option<PlatformParameter> {
        Some("vm_machinefamily".to_string())
    }
}

impl Feature for VMmachinetype {
    fn is_top(self: &Self) -> bool {
        true
    }
    fn string(self: &Self) -> &'static str {
        "VMmachinetype"
    }
    fn feature_class(self: &Self) -> std::option::Option<PlatformParameter> {
        Some("vm_machinetype".to_string())
    }
}
