
use configurable_features::create_feature_hierarchy;

use crate::StoType;

create_feature_hierarchy! { register_features ; "sto_type" : StoType :> SSD & HDD; }
create_feature_hierarchy! { register_features_ssd ; "sto_type" : SSD :> SATA_SSD & NVMe_SSD; }

