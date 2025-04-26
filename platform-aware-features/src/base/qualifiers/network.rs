use crate::{Feature, PlatformParameters};

pub struct  NetTopology;

impl Feature for NetTopology { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "NetTopology" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::net_topology) } 
}

pub struct  NETType;

impl Feature for NETType { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "NETType" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::net_type) } 
}

