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
    fn string(self:&Self) -> String { "AccBackend".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
}

impl Feature for AccInterconnect { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "AccInterconnect".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
}

impl Feature for AccManufacturer { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "AccManufacturer".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_manufacturer) } 
}
                            
impl Feature for AccMemoryType { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "AccMemoryType".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_memory_type) } 
}

impl Feature for AccArchitecture { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "AccArchitecture".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_model) } 
}

impl Feature for AccModel { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "AccModel".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_model) } 
}

impl Feature for AccProcessor { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "AccProcessor".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_processor) } 
}




