use configurable_features::create_feature_hierarchy;

use crate::AccType;

create_feature_hierarchy! {  register_features_1 ;"acc_type" : AccType -> XPU; }
create_feature_hierarchy! {  register_features_2 ;"acc_type" : XPU -> GPU & TPU & IPU; }

