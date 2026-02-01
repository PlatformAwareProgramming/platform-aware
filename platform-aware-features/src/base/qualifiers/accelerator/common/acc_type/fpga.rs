use configurable_features::create_feature_hierarchy;

use crate::AccType;


create_feature_hierarchy! {  register_features ;
    "acc_type" :
    AccType ->
    FPGA;
 }
