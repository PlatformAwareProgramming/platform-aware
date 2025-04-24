use crate::{Feature, PlatformParameters};

pub struct AccType;

impl Feature for AccType { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "AccType".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_type) } 
}