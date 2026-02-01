use configurable_features::create_feature_hierarchy;

use crate::AccInterconnect;


create_feature_hierarchy! {  register_features ;
    "acc_interconnect" :
    AccInterconnect ->
    PCIe ->
    PCIe_v1 ->
    PCIe_v2 ->
    PCIe_v3 ->
    PCIe_v4 ->
    PCIe_v5 ->
    PCIe_v6 ->
    PCIe_v7;
}