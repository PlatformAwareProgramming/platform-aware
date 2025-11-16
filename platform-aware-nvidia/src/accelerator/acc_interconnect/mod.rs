use platform_aware_features::{AccInterconnect, Feature, PlatformParameter, insert_feature, create_feature_hierarchy};

/*
pub struct NVLink; // <: AccInterconnect

impl Feature for NVLink {
    fn string(self: &Self) -> &'static str {
        "NVLink"
    }
    fn feature_class(self: &Self) -> std::option::Option<PlatformParameter> {
        Some("acc_interconnect".to_string())
    }
    fn supertype(self: &Self) -> Option<Box<dyn Feature>> {
        Some(Box::new(AccInterconnect))
    }
}

#[allow(non_camel_case_types)]
pub struct NVLink_v1; // <: NVLink

impl Feature for NVLink_v1 {
    fn string(self: &Self) -> &'static str {
        "NVLink_v1"
    }
    fn feature_class(self: &Self) -> std::option::Option<PlatformParameter> {
        Some("acc_interconnect".to_string())
    }
    fn supertype(self: &Self) -> Option<Box<dyn Feature>> {
        Some(Box::new(NVLink))
    }
}

#[allow(non_camel_case_types)]
pub struct NVLink_v2; // <: NVLink

impl Feature for NVLink_v2 {
    fn string(self: &Self) -> &'static str {
        "NVLink_v2"
    }
    fn feature_class(self: &Self) -> std::option::Option<PlatformParameter> {
        Some("acc_interconnect".to_string())
    }
    fn supertype(self: &Self) -> Option<Box<dyn Feature>> {
        Some(Box::new(NVLink))
    }
}

#[allow(non_camel_case_types)]
pub struct NVLink_v3; // <: NVLink

impl Feature for NVLink_v3 {
    fn string(self: &Self) -> &'static str {
        "NVLink_v3"
    }
    fn feature_class(self: &Self) -> std::option::Option<PlatformParameter> {
        Some("acc_interconnect".to_string())
    }
    fn supertype(self: &Self) -> Option<Box<dyn Feature>> {
        Some(Box::new(NVLink))
    }
}

#[allow(non_camel_case_types)]
pub struct NVLink_v4; // <: NVLink

impl Feature for NVLink_v4 {
    fn string(self: &Self) -> &'static str {
        "NVLink_v4"
    }
    fn feature_class(self: &Self) -> std::option::Option<PlatformParameter> {
        Some("acc_interconnect".to_string())
    }
    fn supertype(self: &Self) -> Option<Box<dyn Feature>> {
        Some(Box::new(NVLink))
    }
}

#[allow(non_camel_case_types)]
pub struct NVLink_v5; // <: NVLink

impl Feature for NVLink_v5 {
    fn string(self: &Self) -> &'static str {
        "NVLink_v5"
    }
    fn feature_class(self: &Self) -> std::option::Option<PlatformParameter> {
        Some("acc_interconnect".to_string())
    }
    fn supertype(self: &Self) -> Option<Box<dyn Feature>> {
        Some(Box::new(NVLink))
    }
}

pub struct NVSwitch; // <: NVLink

impl Feature for NVSwitch {
    fn string(self: &Self) -> &'static str {
        "NVSwitch"
    }
    fn feature_class(self: &Self) -> std::option::Option<PlatformParameter> {
        Some("acc_interconnect".to_string())
    }
    fn supertype(self: &Self) -> Option<Box<dyn Feature>> {
        Some(Box::new(NVLink))
    }
}

*/

use ctor::ctor;


create_feature_hierarchy! { 
    NVLink ->
    NVLinkV1 ->
    NVLinkV2 ->
    NVLinkV3 ->
    NVLinkV4 ->
    NVLinkV5 =>
    "acc_interconnect";
}


create_feature_hierarchy! { 
    NVSwitch ->
    NVSwitchG1 ->       // for NVLink 2.0
    NVSwitchG2 ->       // for NVLink 3.0
    NVSwitchG3 ->       // for NVLink 4.0
    NVSwitchG4 =>       // for NVLink 5.0
    "acc_interconnect";
}
    