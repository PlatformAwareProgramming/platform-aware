
use crate::{Feature, PlatformParameters};

pub struct StoType;
pub struct StoInterface;

impl Feature for StoType { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "StoType".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::sto_type) } 
}

impl Feature for StoInterface { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "StoInterface".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::sto_interface) } 
}