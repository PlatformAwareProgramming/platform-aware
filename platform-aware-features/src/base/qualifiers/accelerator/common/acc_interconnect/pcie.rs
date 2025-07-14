use crate::{Feature, PlatformParameters, AccInterconnect};

pub struct PCIe;

impl Feature for PCIe { 
     fn string(self:&Self) -> &'static str { "PCIe" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccInterconnect)) }
}

#[allow(non_camel_case_types)]
pub struct PCIe_v1;

impl Feature for PCIe_v1 { 
     fn string(self:&Self) -> &'static str { "PCIe_v1" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(PCIe)) }
}

#[allow(non_camel_case_types)]
pub struct PCIe_v2;

impl Feature for PCIe_v2 { 
     fn string(self:&Self) -> &'static str { "PCIe_v2" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(PCIe_v1)) }
}

#[allow(non_camel_case_types)]
pub struct PCIe_v3;

impl Feature for PCIe_v3 { 
     fn string(self:&Self) -> &'static str { "PCIe_v3" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(PCIe_v2)) }
}

#[allow(non_camel_case_types)]
pub struct PCIe_v4;

impl Feature for PCIe_v4 { 
     fn string(self:&Self) -> &'static str { "PCIe_v4" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(PCIe_v3)) }
}

#[allow(non_camel_case_types)]
pub struct PCIe_v5;

impl Feature for PCIe_v5 { 
     fn string(self:&Self) -> &'static str { "PCIe_v5" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(PCIe_v4)) }
}

#[allow(non_camel_case_types)]
pub struct PCIe_v6;

impl Feature for PCIe_v6 { 
     fn string(self:&Self) -> &'static str { "PCIe_v6" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(PCIe_v5)) }
}

#[allow(non_camel_case_types)]
pub struct PCIe_v7;

impl Feature for PCIe_v7 { 
     fn string(self:&Self) -> &'static str { "PCIe_v7" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_interconnect) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(PCIe_v6)) }
}

use std::sync::Arc;
use ctor::ctor;
use crate::insert_feature;

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(PCIe));
    insert_feature(Arc::new(PCIe_v1));
    insert_feature(Arc::new(PCIe_v2));
    insert_feature(Arc::new(PCIe_v3));
    insert_feature(Arc::new(PCIe_v4));
    insert_feature(Arc::new(PCIe_v5));
    insert_feature(Arc::new(PCIe_v6));
    insert_feature(Arc::new(PCIe_v7));
}