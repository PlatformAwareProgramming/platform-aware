use configurable_features::create_feature_hierarchy;

use crate::StoInterface;

create_feature_hierarchy! { register_features ; "sto_interface" : StoInterface -> ATA & IDE & SAS & SCSI & FC; }

