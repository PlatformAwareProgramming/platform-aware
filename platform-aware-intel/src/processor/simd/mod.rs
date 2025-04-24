use platform_aware_features::{Feature, PlatformParameters, CPUSIMD};


pub struct AVX512;

impl Feature for AVX512 { 
    fn string(self:&Self) -> String { "AVX512".to_string() } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CPUSIMD)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::cpu_simd) } 
}
