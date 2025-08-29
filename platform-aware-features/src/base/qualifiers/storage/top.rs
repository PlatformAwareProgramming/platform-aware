use crate::{Feature, PlatformParameter};

pub struct StoType;
pub struct StoInterface;

impl Feature for StoType { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "StoType" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("sto_type".to_string()) } 
}

impl Feature for StoInterface { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "StoInterface" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("sto_interface".to_string()) } 
}