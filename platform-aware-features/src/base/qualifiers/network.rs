
use configurable_features::create_feature_hierarchy;

create_feature_hierarchy! {  register_features_1 ; "net_topology" : None :> NetTopology;}
create_feature_hierarchy! {  register_features_2 ; "net_type" : None :> NetType;}
create_feature_hierarchy! {  register_features_3 ; "net_RDMA" : None :> NetRDMA;}
create_feature_hierarchy! {  register_features_4 ; "net_RDMA" : NetRDMA :> NetRDMAYes & NetRDMANo;}
