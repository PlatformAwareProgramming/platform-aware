
use configurable_features::create_feature_hierarchy;

create_feature_hierarchy! {  register_features_1 ;"cpu_manufacturer" : None :> CPUManufacturer;}
create_feature_hierarchy! {  register_features_2 ;"cpu_isa" : None :> CPUISA;}
create_feature_hierarchy! {  register_features_3 ;"cpu_simd" : None :> CPUSIMD;}
create_feature_hierarchy! {  register_features_4 ;"cpu_model" : None :> CPUModel;}
create_feature_hierarchy! {  register_features_5 ;"cpu_microarchitecture" : None :> CPUMicroarchitecture;}