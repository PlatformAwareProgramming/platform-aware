use platform_aware_features::{insert_feature, AccBackend, Feature, PlatformParameters};

pub struct CUDA;

pub struct CUDA4;

pub struct CUDA6;

pub struct CUDA8;

pub struct CUDA10;

impl Feature for CUDA { 
   fn string(self:&Self) -> &'static str { "CUDA" } 
   fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccBackend)) } 
   fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
}


impl Feature for CUDA4 { 
    fn string(self:&Self) -> &'static str { "CUDA4" } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CUDA)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
}

impl Feature for CUDA6 { 
     fn string(self:&Self) -> &'static str { "CUDA6" } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CUDA4)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
}

impl Feature for CUDA8 { 
     fn string(self:&Self) -> &'static str { "CUDA8" } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CUDA6)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
}

impl Feature for CUDA10 { 
     fn string(self:&Self) -> &'static str { "CUDA10" } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CUDA8)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
}


use std::sync::Arc;
use ctor::ctor;

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(CUDA));
    insert_feature(Arc::new(CUDA4));
    insert_feature(Arc::new(CUDA6));
    insert_feature(Arc::new(CUDA8));
    insert_feature(Arc::new(CUDA10));
}