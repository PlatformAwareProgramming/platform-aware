use configurable_features::create_feature_hierarchy;

use crate::CPUISA;

create_feature_hierarchy! {  register_features_1 ; "cpu_isa" : CPUISA :> ISA_x86_32 :> ISA_x86_64 :> ISA_x86_64_v1 :> ISA_x86_64_v2 :> ISA_x86_64_v3 :> ISA_x86_64_v4;}
create_feature_hierarchy! {  register_features_2 ; "cpu_isa" : CPUISA :> ISA_IA_64; }

pub use ISA_x86_32 as x86_32;
pub use ISA_x86_64 as x86_64;
pub use ISA_x86_64_v1 as x86_64_v1;
pub use ISA_x86_64_v2 as x86_64_v2;
pub use ISA_x86_64_v3 as x86_64_v3;
pub use ISA_x86_64_v4 as x86_64_v4;
pub use ISA_IA_64 as IA_64;


