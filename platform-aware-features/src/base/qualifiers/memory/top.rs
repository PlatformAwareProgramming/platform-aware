use crate::{Feature, PlatformParameter};

pub struct  MemoryType;

impl Feature for MemoryType { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "MemoryType" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some(PlatformParameter::mem_type) } 
}