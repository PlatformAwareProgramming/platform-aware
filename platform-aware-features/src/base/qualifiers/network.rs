use crate::{Feature, PlatformParameters};

pub struct  NetTopology;

impl Feature for NetTopology { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "NetTopology".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::net_topology) } 
}