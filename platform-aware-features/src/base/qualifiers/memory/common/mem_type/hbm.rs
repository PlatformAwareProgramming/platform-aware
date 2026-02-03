use configurable_features::create_feature_hierarchy;

use crate::SDRAM;

create_feature_hierarchy! {  register_features_1 ;"mem_type" : SDRAM :> HBM2 :> HBM3;}
create_feature_hierarchy! {  register_features_2 ; "mem_type" : HBM2 :> HBM2e; }
create_feature_hierarchy! {  register_features_3 ; "mem_type" : SDRAM :> HBM_PIM; }


