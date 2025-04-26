
use crate::{Feature, PlatformParameters};

pub struct CPUManufacturer;
pub struct CPUISA;
pub struct CPUSIMD;
pub struct CPUModel;
pub struct CPUMicroarchitecture;

impl Feature for CPUManufacturer { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "CPUManufacturer" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::cpu_manufacturer) } 
}

impl Feature for CPUISA { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "CPUISA" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::cpu_isa) } 
}

impl Feature for CPUSIMD { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "CPUSIMD" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::cpu_simd) } 
}

impl Feature for CPUModel { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "CPUModel" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::cpu_model) } 
}

impl Feature for CPUMicroarchitecture { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "CPUMicroarchitecture" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::cpu_microarchitecture) } 
}
