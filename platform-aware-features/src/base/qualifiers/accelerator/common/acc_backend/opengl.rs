use crate::{AccBackend, Feature, PlatformParameter};

pub struct OpenGL; // <: AcceleratorBackend

impl Feature for OpenGL {
    fn string(self: &Self) -> &'static str {
        "OpenGL"
    }
    fn feature_class(self: &Self) -> std::option::Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
    fn supertype(self: &Self) -> Option<Box<dyn Feature>> {
        Some(Box::new(AccBackend))
    }
}

#[allow(non_camel_case_types)]
pub struct OpenGL_4_6; // <: OpenGL

impl Feature for OpenGL_4_6 {
    fn string(self: &Self) -> &'static str {
        "OpenGL_4_6"
    }
    fn feature_class(self: &Self) -> std::option::Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
    fn supertype(self: &Self) -> Option<Box<dyn Feature>> {
        Some(Box::new(OpenGL))
    }
}

use crate::insert_feature;
use ctor::ctor;
use std::sync::Arc;

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(OpenGL));
    insert_feature(Arc::new(OpenGL_4_6));
}
