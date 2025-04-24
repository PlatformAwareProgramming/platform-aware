use crate::{Feature, PlatformParameters, MemoryType};


pub struct RAM;  // <: MemoryType

impl Feature for RAM { 
    fn string(self:&Self) -> String { "RAM".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(MemoryType)) }
}

pub struct SDRAM;  // <: RAM

impl Feature for SDRAM { 
    fn string(self:&Self) -> String { "SDRAM".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(RAM)) }
}

pub struct DDR2;  // <: SDRAM

impl Feature for DDR2 { 
    fn string(self:&Self) -> String { "DDR2".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(SDRAM)) }
}
pub struct DDR3;  // <: DDR2

impl Feature for DDR3 { 
    fn string(self:&Self) -> String { "DDR3".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DDR2)) }
}

pub struct DDR3L;  // <: DDR3

impl Feature for DDR3L { 
    fn string(self:&Self) -> String { "DDR3L".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DDR3)) }
}
pub struct DDR4;  // <: DDR3

impl Feature for DDR4 { 
    fn string(self:&Self) -> String { "DDR4".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DDR3)) }
}
pub struct LPDDR4;  // <: DDR4

impl Feature for LPDDR4 { 
    fn string(self:&Self) -> String { "LPDDR4".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DDR4)) }
}
pub struct LPDDR4X;  // <: LPDDR4

impl Feature for LPDDR4X { 
    fn string(self:&Self) -> String { "LPDDR4X".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(LPDDR4)) }
}
pub struct DDR5;  // <: DDR4

impl Feature for DDR5 { 
    fn string(self:&Self) -> String { "DDR5".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DDR4)) }
}
pub struct LPDDR5;  // <: DDR5

impl Feature for LPDDR5 { 
    fn string(self:&Self) -> String { "LPDDR5".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(DDR5)) }
}


pub struct GDDR2;  // <: SDRAM

impl Feature for GDDR2 { 
    fn string(self:&Self) -> String { "GDDR2".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(SDRAM)) }
}
pub struct GDDR3;  // <: GDDR2

impl Feature for GDDR3 { 
    fn string(self:&Self) -> String { "GDDR3".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(GDDR2)) }
}
pub struct GDDR4;  // <: GDDR3

impl Feature for GDDR4 { 
    fn string(self:&Self) -> String { "GDDR4".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(GDDR3)) }
}
pub struct GDDR5;  // <: GDDR4

impl Feature for GDDR5 { 
    fn string(self:&Self) -> String { "GDDR5".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(GDDR4)) }
}
pub struct GDDR5X;  // <: GDDR5

impl Feature for GDDR5X { 
    fn string(self:&Self) -> String { "GDDR5X".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(GDDR5)) }
}
pub struct GDDR6;  // <: GDDR5

impl Feature for GDDR6 { 
    fn string(self:&Self) -> String { "GDDR6".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(GDDR5)) }
}
pub struct GDDR6X;  // <: GDDR6

impl Feature for GDDR6X { 
    fn string(self:&Self) -> String { "GDDR6X".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(GDDR6)) }
}
pub struct GDDR7;  // <: GDDR6

impl Feature for GDDR7 { 
    fn string(self:&Self) -> String { "GDDR7".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(GDDR6)) }
}

pub struct HBM2;  // <: SDRAM

impl Feature for HBM2 { 
    fn string(self:&Self) -> String { "HBM2".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(SDRAM)) }
}
pub struct HBM2e;  // <: HBM2

impl Feature for HBM2e { 
    fn string(self:&Self) -> String { "HBM2e".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(HBM2)) }
}
pub struct HBM3;  // <: HBM2

impl Feature for HBM3 { 
    fn string(self:&Self) -> String { "HBM3".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(HBM2)) }
}
pub struct HBM_PIM;  // <: SDRAM

impl Feature for HBM_PIM { 
    fn string(self:&Self) -> String { "HBM_PIM".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(SDRAM)) }
}
