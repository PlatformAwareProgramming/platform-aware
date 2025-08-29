use crate::{Feature, PlatformParameter, AccBackend};

use std::sync::Arc;
use ctor::ctor;
use crate::insert_feature;

pub struct DirectX;  // <: AcceleratorBackend

impl Feature for DirectX { 
     fn string(self:&Self) -> &'static str { "DirectX" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("acc_backend".to_string()) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccBackend)) }
}

#[allow(non_camel_case_types)]
pub struct DirectX_11_0;  // <: DirectX

impl Feature for DirectX_11_0 { 
     fn string(self:&Self) -> &'static str { "DirectX_11_0" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("acc_backend".to_string()) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DirectX)) }
}

#[allow(non_camel_case_types)]
pub struct DirectX_11_1;  // <: DirectX_1_1  

impl Feature for DirectX_11_1 { 
     fn string(self:&Self) -> &'static str { "DirectX_11_1" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("acc_backend".to_string()) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DirectX_11_0)) }
}

#[allow(non_camel_case_types)]
pub struct DirectX_11_2;  // <: DirectX_1_2

impl Feature for DirectX_11_2 { 
     fn string(self:&Self) -> &'static str { "DirectX_11_2" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("acc_backend".to_string()) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DirectX_11_1)) }
}

#[allow(non_camel_case_types)]
pub struct DirectX_12_0;  // <: DirectX_1_2

impl Feature for DirectX_12_0 { 
     fn string(self:&Self) -> &'static str { "DirectX_12_0" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("acc_backend".to_string()) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DirectX_11_2)) }
}

#[allow(non_camel_case_types)]
pub struct DirectX_12_1;  // <: DirectX_1_2

impl Feature for DirectX_12_1 { 
     fn string(self:&Self) -> &'static str { "DirectX_12_1" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("acc_backend".to_string()) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DirectX_12_0)) }
}

#[allow(non_camel_case_types)]
pub struct DirectX_12_2;  // <: DirectX_1_2

impl Feature for DirectX_12_2 { 
     fn string(self:&Self) -> &'static str { "DirectX_12_2" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("acc_backend".to_string()) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DirectX_12_1)) }
}

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(DirectX));
    insert_feature(Arc::new(DirectX_11_0));
    insert_feature(Arc::new(DirectX_11_1));
    insert_feature(Arc::new(DirectX_11_2));
    insert_feature(Arc::new(DirectX_12_0));
    insert_feature(Arc::new(DirectX_12_1));
    insert_feature(Arc::new(DirectX_12_2));
}