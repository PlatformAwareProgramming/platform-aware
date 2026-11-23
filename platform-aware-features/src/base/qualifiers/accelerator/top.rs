
use configurable_features::create_feature_hierarchy;


create_feature_hierarchy! { register_features_1 ; "acc_backend" : None :> AccBackend;}
create_feature_hierarchy! { register_features_2 ; "acc_type" : None :> AccType;}
create_feature_hierarchy! { register_features_3 ; "acc_interconnect" : None :> AccInterconnect;}
create_feature_hierarchy! { register_features_4 ; "acc_manufacturer" : None :> AccManufacturer;}
pub use crate::MemoryType as AccMemoryType;
create_feature_hierarchy! { register_features_6 ; "acc_architecture" : None :> AccArchitecture;}
create_feature_hierarchy! { register_features_7 ; "acc_model" : None :> AccModel;}
create_feature_hierarchy! { register_features_8 ; "acc_processor" : None :> AccProcessor;}

// Additional accelerator qualifiers (co-pilot generated)

create_feature_hierarchy! { register_features_9 ; "acc_usage" : None :> AccUsage;}
create_feature_hierarchy! { register_features_10 ; "acc_power_source" : None :> AccPowerSource;}
create_feature_hierarchy! { register_features_11 ; "acc_form_factor" : None :> AccFormFactor;}
create_feature_hierarchy! { register_features_12 ; "acc_connection_interface" : None :> AccConnectionInterface;}
create_feature_hierarchy! { register_features_13 ; "acc_virtualization_support" : None :> AccVirtualizationSupport;}
create_feature_hierarchy! { register_features_14 ; "acc_cooling_type" : None :> AccCoolingType;}
create_feature_hierarchy! { register_features_15 ; "acc_fp_precision_support" : None :> AccFPPrecisionSupport;}
create_feature_hierarchy! { register_features_16 ; "acc_tensor_core_support" : None :> AccTensorCoreSupport;}

create_feature_hierarchy! { register_features_17 ; "acc_ray_tracing_support" : None :> AccRayTracingSupport;}  