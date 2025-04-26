use crate::{Feature, PlatformParameters, AccBackend};

pub struct DirectX;  // <: AcceleratorBackend

impl Feature for DirectX { 
     fn string(self:&Self) -> &'static str { "DirectX" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccBackend)) }
}

pub struct DirectX_11_0;  // <: DirectX

impl Feature for DirectX_11_0 { 
     fn string(self:&Self) -> &'static str { "DirectX_11_0" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DirectX)) }
}

pub struct DirectX_11_1;  // <: DirectX_1_1  

impl Feature for DirectX_11_1 { 
     fn string(self:&Self) -> &'static str { "DirectX_11_1" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DirectX_11_0)) }
}

pub struct DirectX_11_2;  // <: DirectX_1_2

impl Feature for DirectX_11_2 { 
     fn string(self:&Self) -> &'static str { "DirectX_11_2" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DirectX_11_1)) }
}

use std::sync::Arc;
use ctor::ctor;
use crate::insert_feature;

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(DirectX));
    insert_feature(Arc::new(DirectX_11_0));
    insert_feature(Arc::new(DirectX_11_1));
    insert_feature(Arc::new(DirectX_11_2));
}