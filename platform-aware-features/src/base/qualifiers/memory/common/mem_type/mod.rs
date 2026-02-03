mod ddr;
mod gddr; 
mod hbm;


use configurable_features::create_feature_hierarchy;

use crate::MemoryType;


create_feature_hierarchy! {  register_features ; "mem_type" : MemoryType :> RAM :> SDRAM; }
