use platform_aware_features::{AccBackend, create_feature_hierarchy}
;

create_feature_hierarchy! {
    register_features ;
    "acc_backend" :
    AccBackend :>
    CUDA :>
    CUDA20 :>       // for CUDA 2.0
    CUDA30 :>       // for CUDA 3.0
    CUDA40 :>       // for CUDA 4.0
    CUDA50 :>       // for CUDA 5.0
    CUDA60 :>       // for CUDA 6.0
    CUDA70 :>       // for CUDA 7.0
    CUDA80 :>       // for CUDA 8.0
    CUDA90 :>       // for CUDA 9.0
    CUDA91 :>       // for CUDA 9.1
    CUDA92 :>       // for CUDA 9.2
    CUDA100 :>       // for CUDA 10.0
    CUDA101 :>       // for CUDA 10.1
    CUDA102 :>       // for CUDA 10.2
    CUDA110 :>       // for CUDA 11.0
    CUDA111 :>       // for CUDA 11.1
    CUDA112 :>       // for CUDA 11.2
    CUDA113 :>       // for CUDA 11.3       
    CUDA114 :>       // for CUDA 11.4       
    CUDA115 :>       // for CUDA 11.5       
    CUDA116 :>       // for CUDA 11.6       
    CUDA117 :>       // for CUDA 11.7       
    CUDA118 :>       // for CUDA 11.8       
    CUDA120 :>       // for CUDA 12.0
    CUDA121 :>       // for CUDA 12.1
    CUDA122;   
}

