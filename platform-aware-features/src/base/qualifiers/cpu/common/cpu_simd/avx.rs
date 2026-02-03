use configurable_features::create_feature_hierarchy;

use crate::CPUSIMD;

create_feature_hierarchy! {  register_features_1 ;"cpu_simd" : CPUSIMD :> AVX :> AVX2 :> AVX512;}
create_feature_hierarchy! {  register_features_2 ;"cpu_simd" : AVX512 :> AVX512_F & AVX512_CD & AVX512_PF & AVX512_ER & AVX512_VL & AVX512_BW & AVX512_DQ & AVX512_IFMA & AVX512_VBMI & AVX512_VBMI2 & AVX512_VNNI & AVX512_BITALG & AVX512_BF16; }
