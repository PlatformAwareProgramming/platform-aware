use configurable_features::create_feature_hierarchy;

use crate::AccBackend;

create_feature_hierarchy! {  register_features ;
    "acc_backend" :
    AccBackend ->
    OpenCL ->
    OpenCL_1_0 ->
    OpenCL_1_1 ->
    OpenCL_1_2 ->
    OpenCL_2_0 ->
    OpenCL_2_1 ->
    OpenCL_3_0 ->
    OpenCL_3_1;
}

