use configurable_features::create_feature_hierarchy;

use crate::AccBackend;

create_feature_hierarchy! { register_features ;
    "acc_backend":
    AccBackend :>
    DirectX :>
    DirectX_11_0 :>   
    DirectX_11_1 :>
    DirectX_11_2 :>
    DirectX_12_0 :>
    DirectX_12_1 :>
    DirectX_12_2 ;
}

