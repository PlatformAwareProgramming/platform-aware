use crate::{Feature, PlatformParameters, AccType};

pub struct FPGA;

impl Feature for FPGA { 
    fn string(self:&Self) -> String { "FPGA".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_type) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccType)) }
}
