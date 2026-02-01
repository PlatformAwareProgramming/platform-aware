use configurable_features::{create_feature_hierarchy};

use crate::AccInterconnect;

create_feature_hierarchy! {  register_features ;
    "acc_interconnect" :
    AccInterconnect ->
    GPUdirect;
}
