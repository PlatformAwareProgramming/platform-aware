use configurable_features::create_feature_hierarchy;

create_feature_hierarchy! {  register_features_1 ; "sto_type" : None :> StoType;}
create_feature_hierarchy! {  register_features_2 ; "sto_interface" : None :> StoInterface;}