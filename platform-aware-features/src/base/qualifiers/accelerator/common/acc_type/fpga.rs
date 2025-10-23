use crate::{AccType, PlatformParameter};

pub struct FPGA;

impl Feature for FPGA {
    fn string(self: &Self) -> &'static str {
        "FPGA"
    }
    fn feature_class(self: &Self) -> std::option::Option<PlatformParameter> {
        Some("acc_type".to_string())
    }
    fn supertype(self: &Self) -> Option<Box<dyn Feature>> {
        Some(Box::new(AccType))
    }
}

use crate::insert_feature;
use configurable_features::Feature;
use ctor::ctor;
use std::sync::Arc;

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(FPGA));
}
