use crate::{Feature, PlatformParameters, AccBackend};

pub struct OpenGL;  // <: AcceleratorBackend

impl Feature for OpenGL { 
     fn string(self:&Self) -> &'static str { "OpenGL" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccBackend)) }
}

pub struct OpenGL_4_6;  // <: OpenGL

impl Feature for OpenGL_4_6 { 
     fn string(self:&Self) -> &'static str { "OpenGL_4_6" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(OpenGL)) }
}

use std::sync::Arc;
use ctor::ctor;
use crate::insert_feature;

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(OpenGL));
    insert_feature(Arc::new(OpenGL_4_6));
}