
use crate::{Feature, PlatformParameters, AccType};

pub struct FPGA;

impl Feature for FPGA { 
     fn string(self:&Self) -> &'static str { "FPGA" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_type) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccType)) }
}

use std::sync::Arc;
use ctor::ctor;
use crate::insert_feature;

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(FPGA));
}
  