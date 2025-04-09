use platform_aware_features::{AccBackend, Feature, PlatformParameters};

pub struct CUDA4;

pub struct CUDA6;

pub struct CUDA8;

pub struct CUDA10;


impl Feature for CUDA4 { 
    fn string(self:&Self) -> String { "cuda4".to_string() } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccBackend)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::ACCBackend) } 
}

impl Feature for CUDA6 { 
    fn string(self:&Self) -> String { "cuda6".to_string() } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CUDA4)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::ACCBackend) } 
}

impl Feature for CUDA8 { 
    fn string(self:&Self) -> String { "cuda8".to_string() } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CUDA6)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::ACCBackend) } 
}

impl Feature for CUDA10 { 
    fn string(self:&Self) -> String { "cuda10".to_string() } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CUDA8)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::ACCBackend) } 
}
