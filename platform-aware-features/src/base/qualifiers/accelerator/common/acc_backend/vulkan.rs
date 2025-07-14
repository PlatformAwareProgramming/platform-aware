use crate::{Feature, PlatformParameters, AccBackend};

pub struct Vulkan;  // <: AcceleratorBackend

impl Feature for Vulkan { 
     fn string(self:&Self) -> &'static str { "Vulkan" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccBackend)) }
}

#[allow(non_camel_case_types)]
pub struct Vulkan_1_1;  // <: Vulkan

impl Feature for Vulkan_1_1 { 
     fn string(self:&Self) -> &'static str { "Vulkan_1_1" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(Vulkan)) }
}

#[allow(non_camel_case_types)]
pub struct Vulkan_1_2;  // <: Vulkan_1_1  

impl Feature for Vulkan_1_2 { 
     fn string(self:&Self) -> &'static str { "Vulkan_1_2" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(Vulkan_1_1)) }
}

#[allow(non_camel_case_types)]
pub struct Vulkan_1_3;  // <: Vulkan_1_2

impl Feature for Vulkan_1_3 { 
     fn string(self:&Self) -> &'static str { "Vulkan_1_3" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(Vulkan_1_2)) }
}


use std::sync::Arc;
use ctor::ctor;
use crate::insert_feature;

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(Vulkan));
    insert_feature(Arc::new(Vulkan_1_1));
    insert_feature(Arc::new(Vulkan_1_2));
    insert_feature(Arc::new(Vulkan_1_3));
}