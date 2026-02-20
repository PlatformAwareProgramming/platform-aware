use platform_aware_features::{AccInterconnect, create_feature_hierarchy};


create_feature_hierarchy! { register_features_1 ;
    "acc_interconnect" : 
    AccInterconnect :> 
    NVLink :>
    NVLinkV1 :>
    NVLinkV2 :>
    NVLinkV3 :>
    NVLinkV4 :>
    NVLinkV5;
}


create_feature_hierarchy! { register_features_2 ;
    "acc_interconnect" :
    AccInterconnect :>
    NVSwitch :>
    NVSwitchG1 :>       // for NVLink 2.0
    NVSwitchG2 :>       // for NVLink 3.0
    NVSwitchG3 :>       // for NVLink 4.0
    NVSwitchG4;
}
    