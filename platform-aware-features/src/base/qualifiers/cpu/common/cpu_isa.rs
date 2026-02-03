use configurable_features::create_feature_hierarchy;

use crate::CPUISA;

create_feature_hierarchy! {  register_features_1 ; "cpu_isa" : CPUISA :> x86_32 :> x86_64 :> x86_64_v1 :> x86_64_v2 :> x86_64_v3 :> x86_64_v4;}
create_feature_hierarchy! {  register_features_2 ; "cpu_isa" : CPUISA :> IA_64; }


