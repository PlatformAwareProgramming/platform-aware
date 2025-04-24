use crate::{Feature, PlatformParameters, AccInterconnect};

pub struct GPUDirect; // <: AccInterconnect

impl Feature for GPUDirect { 
    fn string(self:&Self) -> String { "GPUDirect".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccInterconnect)) }
}