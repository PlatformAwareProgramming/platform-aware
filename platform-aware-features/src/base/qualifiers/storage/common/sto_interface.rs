use crate::{Feature, PlatformParameters, StoInterface};


pub struct SATA;  // <:StorageInterface

impl Feature for SATA { 
    fn string(self:&Self) -> String { "SATA".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(StoInterface)) }
}

pub struct IDE;  // <:StorageInterface

impl Feature for IDE { 
    fn string(self:&Self) -> String { "IDE".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(StoInterface)) }
}

pub struct SAS;  // <:StorageInterface

impl Feature for SAS { 
    fn string(self:&Self) -> String { "SAS".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(StoInterface)) }
}

pub struct SCSI;  // <:StorageInterface

impl Feature for SCSI { 
    fn string(self:&Self) -> String { "SCSI".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(StoInterface)) }
}

pub struct FC;  // <:StorageInterface

impl Feature for FC { 
    fn string(self:&Self) -> String { "FC".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(StoInterface)) }
}