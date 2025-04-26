use crate::{Feature, PlatformParameters};

pub struct AccManufacturer;
pub struct AccMemoryType;
pub struct AccArchitecture;
pub struct AccModel;
pub struct AccProcessor;
pub struct AccInterconnect;
pub struct AccBackend;

impl Feature for AccBackend { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "AccBackend" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
}

impl Feature for AccInterconnect { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "AccInterconnect" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
}

impl Feature for AccManufacturer { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "AccManufacturer" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_manufacturer) } 
}
                            
impl Feature for AccMemoryType { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "AccMemoryType" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_memory_type) } 
}

impl Feature for AccArchitecture { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "AccArchitecture" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_model) } 
}

impl Feature for AccModel { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "AccModel" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_model) } 
}

impl Feature for AccProcessor { 
    fn is_top(self:&Self) -> bool { true }
     fn string(self:&Self) -> &'static str { "AccProcessor" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_processor) } 
}




