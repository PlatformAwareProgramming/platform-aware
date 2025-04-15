use platform_aware_features::{Feature, PlatformParameters, SIMD};


pub struct AVX512;

impl Feature for AVX512 { 
    fn string(self:&Self) -> String { "avx512".to_string() } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(SIMD)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::CPUSIMD) } 
}
