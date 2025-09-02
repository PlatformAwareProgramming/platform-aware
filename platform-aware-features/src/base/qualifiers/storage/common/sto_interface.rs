use crate::{Feature, PlatformParameter, StoInterface};


pub struct SATA;  // <:StorageInterface

impl Feature for SATA { 
     fn string(self:&Self) -> &'static str { "SATA" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("sto_interface".to_string()) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(StoInterface)) }
}

pub struct IDE;  // <:StorageInterface

impl Feature for IDE { 
     fn string(self:&Self) -> &'static str { "IDE" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("sto_interface".to_string()) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(StoInterface)) }
}

pub struct SAS;  // <:StorageInterface

impl Feature for SAS { 
     fn string(self:&Self) -> &'static str { "SAS" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("sto_interface".to_string()) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(StoInterface)) }
}

pub struct SCSI;  // <:StorageInterface

impl Feature for SCSI { 
     fn string(self:&Self) -> &'static str { "SCSI" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("sto_interface".to_string()) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(StoInterface)) }
}

pub struct FC;  // <:StorageInterface

impl Feature for FC { 
     fn string(self:&Self) -> &'static str { "FC" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("sto_interface".to_string()) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(StoInterface)) }
}

use crate::insert_feature;
use std::sync::Arc;
use ctor::ctor;

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(SATA));
    insert_feature(Arc::new(IDE));
    insert_feature(Arc::new(SAS));
    insert_feature(Arc::new(SCSI));
    insert_feature(Arc::new(FC));
}