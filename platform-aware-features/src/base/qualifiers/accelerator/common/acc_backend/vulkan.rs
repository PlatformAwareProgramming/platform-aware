use configurable_features::{create_feature_hierarchy};

use crate::AccBackend;

create_feature_hierarchy! {  register_features ;
    "acc_backend" :
    AccBackend :>
    Vulkan :>
    Vulkan_1_1 :>
    Vulkan_1_2 :>
    Vulkan_1_3;
}
