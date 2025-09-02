use crate::{Feature, PlatformParameter, AccInterconnect};

pub struct GPUDirect; // <: AccInterconnect

impl Feature for GPUDirect { 
     fn string(self:&Self) -> &'static str { "GPUDirect" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("acc_interconnect".to_string()) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccInterconnect)) }
}


use std::sync::Arc;
use ctor::ctor;
use crate::insert_feature;

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(GPUDirect));
}