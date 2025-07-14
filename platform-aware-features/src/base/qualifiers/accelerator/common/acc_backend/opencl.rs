use crate::{Feature, PlatformParameters, AccBackend};

pub struct OpenCL;  // <: AcceleratorBackend

impl Feature for OpenCL { 
     fn string(self:&Self) -> &'static str { "OpenCL" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccBackend)) }
}

#[allow(non_camel_case_types)]
pub struct OpenCL_1_0;  // <: OpenCL

impl Feature for OpenCL_1_0 { 
     fn string(self:&Self) -> &'static str { "OpenCL_1_0" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(OpenCL)) }
}

#[allow(non_camel_case_types)]
pub struct OpenCL_1_1;  // <: OpenCL_1_0

impl Feature for OpenCL_1_1 { 
     fn string(self:&Self) -> &'static str { "OpenCL_1_1" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(OpenCL_1_0)) }
}

#[allow(non_camel_case_types)]
pub struct OpenCL_1_2;  // <: OpenCL_1_1  

impl Feature for OpenCL_1_2 { 
     fn string(self:&Self) -> &'static str { "OpenCL_1_2" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(OpenCL_1_1)) }
}

#[allow(non_camel_case_types)]
pub struct OpenCL_2_0;  // <: OpenCL_1_2

impl Feature for OpenCL_2_0 { 
     fn string(self:&Self) -> &'static str { "OpenCL_2_0" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(OpenCL_1_2)) }
}

#[allow(non_camel_case_types)]
pub struct OpenCL_2_1;  // <: OpenCL_2_0

impl Feature for OpenCL_2_1 { 
     fn string(self:&Self) -> &'static str { "OpenCL_2_1" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(OpenCL_2_0)) }
}

#[allow(non_camel_case_types)]
pub struct OpenCL_2_2;  // <: OpenCL_2_1

impl Feature for OpenCL_2_2 { 
     fn string(self:&Self) -> &'static str { "OpenCL_2_2" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(OpenCL_2_1)) }
}

#[allow(non_camel_case_types)]
pub struct OpenCL_3_0;  // <: OpenCL_2_2

impl Feature for OpenCL_3_0 { 
     fn string(self:&Self) -> &'static str { "OpenCL_3_0" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(OpenCL)) }
}

use std::sync::Arc;
use ctor::ctor;
use crate::insert_feature;

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(OpenCL));
    insert_feature(Arc::new(OpenCL_1_0));
    insert_feature(Arc::new(OpenCL_1_1));
    insert_feature(Arc::new(OpenCL_1_2));    
    insert_feature(Arc::new(OpenCL_2_0));
    insert_feature(Arc::new(OpenCL_2_1));
    insert_feature(Arc::new(OpenCL_2_2));
    insert_feature(Arc::new(OpenCL_3_0));

}