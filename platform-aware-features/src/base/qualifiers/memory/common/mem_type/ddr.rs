

use configurable_features::create_feature_hierarchy;

use crate::SDRAM;

create_feature_hierarchy! {  register_features_1 ; "mem_type" : SDRAM :> DDR2 :> DDR3 :> DDR4 :> DDR5;}
create_feature_hierarchy! {  register_features_2 ; "mem_type" : DDR3 :> DDR3L; }
create_feature_hierarchy! {  register_features_3 ; "mem_type" : DDR4 :> LPDDR4 :> LPDDR4X; }
create_feature_hierarchy! {  register_features_4 ; "mem_type" : DDR5 :> LPDDR5; }
