use crate::{Feature, PlatformParameters, AccBackend};

pub struct DirectX;  // <: AcceleratorBackend

impl Feature for DirectX { 
    fn string(self:&Self) -> String { "DirectX".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccBackend)) }
}

pub struct DirectX_11_0;  // <: DirectX

impl Feature for DirectX_11_0 { 
    fn string(self:&Self) -> String { "DirectX_11_0".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DirectX)) }
}

pub struct DirectX_11_1;  // <: DirectX_1_1  

impl Feature for DirectX_11_1 { 
    fn string(self:&Self) -> String { "DirectX_11_1".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DirectX_11_0)) }
}

pub struct DirectX_11_2;  // <: DirectX_1_2

impl Feature for DirectX_11_2 { 
    fn string(self:&Self) -> String { "DirectX_11_2".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DirectX_11_1)) }
}
