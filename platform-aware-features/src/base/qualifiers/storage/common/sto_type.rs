use crate::{Feature, PlatformParameter, StoType};

pub struct SSD;  // <: StoType

impl Feature for SSD { 
     fn string(self:&Self) -> &'static str { "SSD" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some(PlatformParameter::sto_type) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(StoType)) }
}

pub struct HDD;  // <: StoType

impl Feature for HDD { 
     fn string(self:&Self) -> &'static str { "HDD" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some(PlatformParameter::sto_type) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(StoType)) }
}

use crate::insert_feature;
use std::sync::Arc;
use ctor::ctor;

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(SSD));
    insert_feature(Arc::new(HDD));
}