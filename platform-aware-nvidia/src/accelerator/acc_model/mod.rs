use platform_aware_features::{AccModel, create_feature_hierarchy};


create_feature_hierarchy! { register_features ;
    "acc_model" :
    AccModel :>
    NVidiaA100 &
    NVidiaH100 &
    NVidiaMTX570A &
    NVidiaRTX4000Ada &
    NvidiaGeForce940MX ;
}