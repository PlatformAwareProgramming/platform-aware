use platform_aware_features::{AccModel, Feature, PlatformParameters};

pub struct NVidiaH100;

impl Feature for NVidiaH100 { 
     fn string(self:&Self) -> &'static str { "nvidia_H100" } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccModel)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_model) } 
}
