use crate::{Feature, PlatformParameter, CPUISA};

#[allow(non_camel_case_types)]
pub struct x86_32;  // <: CPUISA

impl Feature for x86_32 { 
     fn string(self:&Self) -> &'static str { "x86_32" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some(PlatformParameter::cpu_isa) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CPUISA)) }
}

#[allow(non_camel_case_types)]
pub struct x86_64;  // <: x86_32

impl Feature for x86_64 { 
     fn string(self:&Self) -> &'static str { "x86_64" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some(PlatformParameter::cpu_isa) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(x86_32)) }
}

#[allow(non_camel_case_types)]
pub struct x86_64_v1;  // <: x86_64

impl Feature for x86_64_v1 { 
     fn string(self:&Self) -> &'static str { "x86_64_v1" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some(PlatformParameter::cpu_isa) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(x86_64)) }
}

#[allow(non_camel_case_types)]
pub struct x86_64_v2;  // <: x86_64_v1

impl Feature for x86_64_v2 { 
     fn string(self:&Self) -> &'static str { "x86_64_v2" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some(PlatformParameter::cpu_isa) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(x86_64_v1)) }
}

#[allow(non_camel_case_types)]
pub struct x86_64_v3;  // <: x86_64_v2

impl Feature for x86_64_v3 { 
    fn string(self:&Self) -> &'static str { "x86_64_v3" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some(PlatformParameter::cpu_isa) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(x86_64_v2)) }
}

#[allow(non_camel_case_types)]
pub struct x86_64_v4;  // <: x86_64_v3

impl Feature for x86_64_v4 { 
     fn string(self:&Self) -> &'static str { "x86_64_v4" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some(PlatformParameter::cpu_isa) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(x86_64_v3)) }
}

#[allow(non_camel_case_types)]
pub struct IA_64;  // <: CPUISA

impl Feature for IA_64 { 
     fn string(self:&Self) -> &'static str { "IA_64" } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameter> { Some(PlatformParameter::cpu_isa) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CPUISA)) }
}

use crate::insert_feature;
use std::sync::Arc;
use ctor::ctor;

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(x86_32));
    insert_feature(Arc::new(x86_64));
    insert_feature(Arc::new(x86_64_v1));
    insert_feature(Arc::new(x86_64_v2));
    insert_feature(Arc::new(x86_64_v3));
    insert_feature(Arc::new(x86_64_v4));
    insert_feature(Arc::new(IA_64));
}
  