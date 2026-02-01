
use configurable_features::create_feature_hierarchy;

create_feature_hierarchy! {  register_features_1 ;"vm_provider" : None -> VMprovider;}
create_feature_hierarchy! {  register_features_2 ; "vm_machinefamily" : None -> VMmachinefamily;}
create_feature_hierarchy! {  register_features_3 ; "vm_machinetype" : None -> VMmachinetype;}

// Added qualifiers for Virtual Machine features (by copilot)
create_feature_hierarchy! {  register_features_4 ;"vm_cpuarch" : None -> VMcpuarch;}
create_feature_hierarchy! {  register_features_5 ;"vm_osfamily" : None -> VMosfamily;}
create_feature_hierarchy! {  register_features_6 ;"vm_osname" : None -> VMosname;}
create_feature_hierarchy! {  register_features_7 ;"vm_oshypervisor" : None -> VMoshypervisor;}
create_feature_hierarchy! {  register_features_8 ;"vm_virtualizationtype" : None -> VMvirtualizationtype;}