use platform_aware_features::{
    insert_feature, AccBackend, Feature, PlatformParameter, CURRENT_FEATURES,
};

pub struct CUDA;

pub struct CUDA10;
pub struct CUDA20;
pub struct CUDA30;
pub struct CUDA40;
pub struct CUDA50;
pub struct CUDA60;
pub struct CUDA70;
pub struct CUDA80;
pub struct CUDA90;
pub struct CUDA91;
pub struct CUDA92;

pub struct CUDA100;
pub struct CUDA101;
pub struct CUDA102;

pub struct CUDA110;
pub struct CUDA111;
pub struct CUDA112;
pub struct CUDA113;
pub struct CUDA114;
pub struct CUDA115;
pub struct CUDA116;
pub struct CUDA117;
pub struct CUDA118;

pub struct CUDA120;
pub struct CUDA121;
pub struct CUDA122;

impl Feature for CUDA {
    fn string(&self) -> &'static str {
        "CUDA"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(AccBackend))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA10 {
    fn string(&self) -> &'static str {
        "CUDA10"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA20 {
    fn string(&self) -> &'static str {
        "CUDA20"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA10))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA30 {
    fn string(&self) -> &'static str {
        "CUDA30"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA20))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA40 {
    fn string(&self) -> &'static str {
        "CUDA40"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA30))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA50 {
    fn string(&self) -> &'static str {
        "CUDA50"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA40))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA60 {
    fn string(&self) -> &'static str {
        "CUDA60"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA50))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA70 {
    fn string(&self) -> &'static str {
        "CUDA70"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA60))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA80 {
    fn string(&self) -> &'static str {
        "CUDA80"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA70))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA90 {
    fn string(&self) -> &'static str {
        "CUDA90"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA80))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA91 {
    fn string(&self) -> &'static str {
        "CUDA91"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA90))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA92 {
    fn string(&self) -> &'static str {
        "CUDA92"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA91))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA100 {
    fn string(&self) -> &'static str {
        "CUDA100"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA92))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA101 {
    fn string(&self) -> &'static str {
        "CUDA101"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA100))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA102 {
    fn string(&self) -> &'static str {
        "CUDA102"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA101))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA110 {
    fn string(&self) -> &'static str {
        "CUDA110"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA102))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA111 {
    fn string(&self) -> &'static str {
        "CUDA111"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA110))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA112 {
    fn string(&self) -> &'static str {
        "CUDA112"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA111))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA113 {
    fn string(&self) -> &'static str {
        "CUDA113"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA112))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA114 {
    fn string(&self) -> &'static str {
        "CUDA114"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA113))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA115 {
    fn string(&self) -> &'static str {
        "CUDA115"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA114))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA116 {
    fn string(&self) -> &'static str {
        "CUDA116"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA115))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA117 {
    fn string(&self) -> &'static str {
        "CUDA117"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA116))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA118 {
    fn string(&self) -> &'static str {
        "CUDA118"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA117))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA120 {
    fn string(&self) -> &'static str {
        "CUDA120"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA118))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA121 {
    fn string(&self) -> &'static str {
        "CUDA121"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA120))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

impl Feature for CUDA122 {
    fn string(&self) -> &'static str {
        "CUDA122"
    }
    fn supertype(&self) -> Option<Box<dyn Feature>> {
        Some(Box::new(CUDA121))
    }
    fn feature_class(&self) -> Option<PlatformParameter> {
        Some("acc_backend".to_string())
    }
}

use ctor::ctor;
use std::sync::Arc;

#[ctor]
fn module_initializer() {
    insert_feature(Arc::new(CUDA));
    insert_feature(Arc::new(CUDA10));
    insert_feature(Arc::new(CUDA20));
    insert_feature(Arc::new(CUDA30));
    insert_feature(Arc::new(CUDA40));
    insert_feature(Arc::new(CUDA50));
    insert_feature(Arc::new(CUDA60));
    insert_feature(Arc::new(CUDA70));
    insert_feature(Arc::new(CUDA80));
    insert_feature(Arc::new(CUDA90));
    insert_feature(Arc::new(CUDA91));
    insert_feature(Arc::new(CUDA92));

    insert_feature(Arc::new(CUDA100));
    insert_feature(Arc::new(CUDA101));
    insert_feature(Arc::new(CUDA102));

    insert_feature(Arc::new(CUDA110));
    insert_feature(Arc::new(CUDA111));
    insert_feature(Arc::new(CUDA112));
    insert_feature(Arc::new(CUDA113));
    insert_feature(Arc::new(CUDA114));
    insert_feature(Arc::new(CUDA115));
    insert_feature(Arc::new(CUDA116));
    insert_feature(Arc::new(CUDA117));
    insert_feature(Arc::new(CUDA118));

    insert_feature(Arc::new(CUDA120));
    insert_feature(Arc::new(CUDA121));
    insert_feature(Arc::new(CUDA122));

    // specific parameters for the above features are declared here
}
