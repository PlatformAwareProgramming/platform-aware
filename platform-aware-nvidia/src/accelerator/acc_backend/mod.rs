use platform_aware_features::{insert_feature, AccBackend, Feature, PlatformParameter};
use ctor::ctor;
use std::sync::Arc;

macro_rules! define_cuda {
    ( $($name:ident => $super:ident),* ) => {
        $(
            pub struct $name;

            impl Feature for $name {
                fn string(&self) -> &'static str { stringify!($name) }
                fn supertype(&self) -> Option<Box<dyn Feature>> {
                    Some(Box::new($super))
                }
                fn feature_class(&self) -> Option<PlatformParameter> {
                    Some("acc_backend".to_string())
                }
            }
        )*

        #[ctor]
        fn register_cuda() {
            $(
                insert_feature(std::sync::Arc::new($name));
            )*
        }
    };
}

#[rustfmt::skip]
define_cuda!(
    CUDA => AccBackend, CUDA10 => CUDA, CUDA20 => CUDA10, CUDA30 => CUDA20,
    CUDA40 => CUDA30, CUDA50 => CUDA40, CUDA60 => CUDA50, CUDA70 => CUDA60,
    CUDA80 => CUDA70, CUDA90 => CUDA80, CUDA91 => CUDA90, CUDA92 => CUDA91,
    CUDA100 => CUDA92, CUDA101 => CUDA100, CUDA102 => CUDA101, CUDA110 => CUDA102,
    CUDA111 => CUDA110, CUDA112 => CUDA111, CUDA113 => CUDA112, CUDA114 => CUDA113,
    CUDA115 => CUDA114, CUDA116 => CUDA115, CUDA117 => CUDA116, CUDA118 => CUDA117,
    CUDA120 => CUDA118, CUDA121 => CUDA120, CUDA122 => CUDA121
);
