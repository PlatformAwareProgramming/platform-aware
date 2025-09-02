use platform_aware_features::{insert_feature, AccModel, Feature, PlatformParameter};

pub struct NVidiaH100;

impl Feature for NVidiaH100 { 
     fn string(self:&Self) -> &'static str { "NVidiaH100" } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccModel)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("acc_model".to_string()) } 
}

pub struct NVidiaA100;

impl Feature for NVidiaA100 { 
     fn string(self:&Self) -> &'static str { "NVidiaA100" } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccModel)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("acc_model".to_string()) } 
}


pub struct NVidiaRTX4000Ada;

impl Feature for NVidiaRTX4000Ada { 
     fn string(self:&Self) -> &'static str { "NVidiaRTX4000Ada" } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccModel)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("acc_model".to_string()) } 
}


pub struct NVidiaMTX570A;

impl Feature for NVidiaMTX570A { 
    fn string(self:&Self) -> &'static str { "NVidiaMTX570A" } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccModel)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("acc_model".to_string()) } 
}

pub struct NvidiaGeForce940MX;

impl Feature for NvidiaGeForce940MX { 
    fn string(self:&Self) -> &'static str { "NvidiaGeForce940MX" } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccModel)) } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some("acc_model".to_string()) } 
}

use std::sync::Arc;
use ctor::ctor;

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(NVidiaA100));
    insert_feature(Arc::new(NVidiaH100));
    insert_feature(Arc::new(NVidiaMTX570A));
    insert_feature(Arc::new(NVidiaRTX4000Ada));
    insert_feature(Arc::new(NvidiaGeForce940MX));
}