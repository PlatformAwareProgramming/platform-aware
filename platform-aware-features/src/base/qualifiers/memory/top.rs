use crate::{Feature, PlatformParameters};

pub struct  MemoryType;

impl Feature for MemoryType { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "MemoryType".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::mem_type) } 
}