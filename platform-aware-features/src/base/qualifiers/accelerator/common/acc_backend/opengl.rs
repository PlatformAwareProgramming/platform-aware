use crate::{Feature, PlatformParameters, AccBackend};

pub struct OpenGL;  // <: AcceleratorBackend

impl Feature for OpenGL { 
    fn string(self:&Self) -> String { "OpenGL".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccBackend)) }
}

pub struct OpenGL_4_6;  // <: OpenGL

impl Feature for OpenGL_4_6 { 
    fn string(self:&Self) -> String { "OpenGL_4_6".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(OpenGL)) }
}