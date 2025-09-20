use crate::{AccType, Feature, PlatformParameter};

pub struct XPU;

impl Feature for XPU {
    fn string(self: &Self) -> &'static str {
        "XPU"
    }
    fn feature_class(self: &Self) -> std::option::Option<PlatformParameter> {
        Some("acc_type".to_string())
    }
    fn supertype(self: &Self) -> Option<Box<dyn Feature>> {
        Some(Box::new(AccType))
    }
}

pub struct GPU;
pub struct TPU;
pub struct IPU;

impl Feature for GPU {
    fn string(self: &Self) -> &'static str {
        "GPU"
    }
    fn feature_class(self: &Self) -> std::option::Option<PlatformParameter> {
        Some("acc_type".to_string())
    }
    fn supertype(self: &Self) -> Option<Box<dyn Feature>> {
        Some(Box::new(XPU))
    }
}

impl Feature for TPU {
    fn string(self: &Self) -> &'static str {
        "TPU"
    }
    fn feature_class(self: &Self) -> std::option::Option<PlatformParameter> {
        Some("acc_type".to_string())
    }
    fn supertype(self: &Self) -> Option<Box<dyn Feature>> {
        Some(Box::new(XPU))
    }
}

impl Feature for IPU {
    fn string(self: &Self) -> &'static str {
        "IPU"
    }
    fn feature_class(self: &Self) -> std::option::Option<PlatformParameter> {
        Some("acc_typ".to_string())
    }
    fn supertype(self: &Self) -> Option<Box<dyn Feature>> {
        Some(Box::new(XPU))
    }
}

use crate::insert_feature;
use ctor::ctor;
use std::sync::Arc;

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(XPU));
    insert_feature(Arc::new(GPU));
    insert_feature(Arc::new(TPU));
    insert_feature(Arc::new(IPU));
}
