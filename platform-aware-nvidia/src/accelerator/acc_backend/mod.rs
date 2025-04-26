use platform_aware_features::{AccBackend, Feature, PlatformParameters};

pub struct CUDA4;

pub struct CUDA6;

pub struct CUDA8;

pub struct CUDA10;


impl Feature for CUDA4 { 
     fn string(self:&Self) -> &'static str { "cuda4" } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccBackend)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
}

impl Feature for CUDA6 { 
     fn string(self:&Self) -> &'static str { "cuda6" } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CUDA4)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
}

impl Feature for CUDA8 { 
     fn string(self:&Self) -> &'static str { "cuda8" } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CUDA6)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
}

impl Feature for CUDA10 { 
     fn string(self:&Self) -> &'static str { "cuda10" } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CUDA8)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
}
