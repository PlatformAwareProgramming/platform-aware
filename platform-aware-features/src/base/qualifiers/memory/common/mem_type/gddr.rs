

use configurable_features::create_feature_hierarchy;

use crate::SDRAM;

create_feature_hierarchy! {  register_features_1 ; "mem_type" : SDRAM :> GDDR2 :> GDDR3 :> GDDR4 :> GDDR5 :> GDDR6 :> GDDR7;}
create_feature_hierarchy! {  register_features_2 ; "mem_type" : GDDR6 :> GDDR6X; }
create_feature_hierarchy! {  register_features_3 ; "mem_type" : GDDR5 :> GDDR5X; }
