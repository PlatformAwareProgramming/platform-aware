use crate::{Feature, PlatformParameters, AccBackend};

pub struct OpenCL;  // <: AcceleratorBackend

impl Feature for OpenCL { 
    fn string(self:&Self) -> String { "OpenCL".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccBackend)) }
}

pub struct OpenCL_1_0;  // <: OpenCL

impl Feature for OpenCL_1_0 { 
    fn string(self:&Self) -> String { "OpenCL_1_0".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(OpenCL)) }
}

pub struct OpenCL_1_1;  // <: OpenCL_1_0

impl Feature for OpenCL_1_1 { 
    fn string(self:&Self) -> String { "OpenCL_1_1".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(OpenCL_1_0)) }
}

pub struct OpenCL_1_2;  // <: OpenCL_1_1  

impl Feature for OpenCL_1_2 { 
    fn string(self:&Self) -> String { "OpenCL_1_2".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(OpenCL_1_1)) }
}

pub struct OpenCL_2_0;  // <: OpenCL_1_2

impl Feature for OpenCL_2_0 { 
    fn string(self:&Self) -> String { "OpenCL_2_0".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(OpenCL_1_2)) }
}

pub struct OpenCL_2_1;  // <: OpenCL_2_0

impl Feature for OpenCL_2_1 { 
    fn string(self:&Self) -> String { "OpenCL_2_1".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(OpenCL_2_0)) }
}

pub struct OpenCL_2_2;  // <: OpenCL_2_1

impl Feature for OpenCL_2_2 { 
    fn string(self:&Self) -> String { "OpenCL_2_2".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(OpenCL_2_1)) }
}

pub struct OpenCL_3_0;  // <: OpenCL_2_2

impl Feature for OpenCL_3_0 { 
    fn string(self:&Self) -> String { "OpenCL_3_0".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(OpenCL)) }
}