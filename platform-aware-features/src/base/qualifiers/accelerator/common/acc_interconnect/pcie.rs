use crate::{Feature, PlatformParameters, AccInterconnect};

pub struct PCIe;

impl Feature for PCIe { 
     fn string(self:&Self) -> &'static str { "PCIe" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccInterconnect)) }
}

pub struct PCIe_v1;

impl Feature for PCIe_v1 { 
     fn string(self:&Self) -> &'static str { "PCIe_v1" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(PCIe)) }
}
pub struct PCIe_v2;

impl Feature for PCIe_v2 { 
     fn string(self:&Self) -> &'static str { "PCIe_v2" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(PCIe_v1)) }
}
pub struct PCIe_v3;

impl Feature for PCIe_v3 { 
     fn string(self:&Self) -> &'static str { "PCIe_v3" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(PCIe_v2)) }
}
pub struct PCIe_v4;

impl Feature for PCIe_v4 { 
     fn string(self:&Self) -> &'static str { "PCIe_v4" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(PCIe_v3)) }
}
pub struct PCIe_v5;

impl Feature for PCIe_v5 { 
     fn string(self:&Self) -> &'static str { "PCIe_v5" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(PCIe_v4)) }
}
pub struct PCIe_v6;

impl Feature for PCIe_v6 { 
     fn string(self:&Self) -> &'static str { "PCIe_v6" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(PCIe_v5)) }
}
pub struct PCIe_v7;

impl Feature for PCIe_v7 { 
     fn string(self:&Self) -> &'static str { "PCIe_v7" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(PCIe_v6)) }
}
