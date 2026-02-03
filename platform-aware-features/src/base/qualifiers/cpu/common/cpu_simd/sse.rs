

use configurable_features::create_feature_hierarchy;

use crate::CPUSIMD;

create_feature_hierarchy! {  register_features_1 ;"cpu_simd" : CPUSIMD :> SSE :> SSE_2 :> SSE_3 :> SSSE_3 :> SSE_4 :> SSE_4_1;}
create_feature_hierarchy! {  register_features_2 ;"cpu_simd" : SSE_4 :> SSE_4_2; }
create_feature_hierarchy! {  register_features_3 ;"cpu_simd" : SSE_3 :> SSE_4a; }


