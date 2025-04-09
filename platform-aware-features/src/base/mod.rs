use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug)]
pub enum PlatformParameters {
    /* accelerator features */
    ACCManufacturer,
    ACCMemoryType,
    ACCCount,
    ACCBackend,
    ACCArchitecture,
    ACCModel,
    ACCProcessorCount,
    ACCProcessor,
    ACCMemorySize,
    ACCTDP,
    ACCType,

    /* processor features */
    CPUCoreCount,
    CPUCount,
    CPUCoreThreadsCount,
    CPUL1Size,
    CPUL2Size,
    CPUL3Size,
    CPUClock,
    CPUManufacturer,
    CPUISA,
    CPUSIMD,
    CPUTDP,
    CPUModel,
    CPUMicroarchtecture,

    /* interconnection */
    NETLatency,
    NETType,
    NETBandwidth,
    NETStartupTime,
    NETRDMA,
    NETTopology,

    /* storage */
    STOBandwith,
    STOType,
    STOLatency,
    STOSize,
    STONetworkBandwith,
    STOInterface,

    /* memory */
    MEMType,
    MEMSize,
    MEMFrequency,
    MEMLatency,
    MEMBandwidth,

    /* node (cloud) */
    VMNDedicated,
    VMNCount,
    VMNVirtual,
    VMNMachineType,
    VMNThreadsCount,
    VMNMachineFamily,
    VMNProvider,
    VMNVCPUSCount
}


pub const PLATFORM_PARAMETERS:&[PlatformParameters] = &[
                                                    PlatformParameters::CPUSIMD,
                                                    PlatformParameters::ACCModel, 
                                                    PlatformParameters::ACCBackend, 
                                                    PlatformParameters::ACCCount,
                                                    PlatformParameters::ACCManufacturer,
                                                    PlatformParameters::ACCMemoryType,
                                                    PlatformParameters::ACCCount,
                                                    PlatformParameters::ACCBackend,
                                                    PlatformParameters::ACCArchitecture,
                                                    PlatformParameters::ACCModel,
                                                    PlatformParameters::ACCProcessorCount,
                                                    PlatformParameters::ACCProcessor,
                                                    PlatformParameters::ACCMemorySize,
                                                    PlatformParameters::ACCTDP,
                                                    PlatformParameters::ACCType,                                                    
                                                    PlatformParameters::CPUCoreCount,
                                                    PlatformParameters::CPUCount,
                                                    PlatformParameters::CPUCoreThreadsCount,
                                                    PlatformParameters::CPUSIMD,
                                                    PlatformParameters::CPUL1Size,
                                                    PlatformParameters::CPUL2Size,
                                                    PlatformParameters::CPUL3Size,
                                                    PlatformParameters::CPUClock,
                                                    PlatformParameters::CPUManufacturer,
                                                    PlatformParameters::CPUISA,
                                                    PlatformParameters::CPUTDP,
                                                    PlatformParameters::CPUModel,
                                                    PlatformParameters::CPUMicroarchtecture,
                                                    PlatformParameters::NETLatency,
                                                    PlatformParameters::NETType,
                                                    PlatformParameters::NETBandwidth,
                                                    PlatformParameters::NETStartupTime,
                                                    PlatformParameters::NETRDMA,
                                                    PlatformParameters::NETTopology,
                                                    PlatformParameters::STOBandwith,
                                                    PlatformParameters::STOType,
                                                    PlatformParameters::STOLatency,
                                                    PlatformParameters::STOSize,
                                                    PlatformParameters::STONetworkBandwith,
                                                    PlatformParameters::STOInterface,
                                                    PlatformParameters::MEMType,
                                                    PlatformParameters::MEMSize,
                                                    PlatformParameters::MEMFrequency,
                                                    PlatformParameters::MEMLatency,
                                                    PlatformParameters::MEMBandwidth,
                                                    PlatformParameters::VMNDedicated,
                                                    PlatformParameters::VMNCount,
                                                    PlatformParameters::VMNVirtual,
                                                    PlatformParameters::VMNMachineType,
                                                    PlatformParameters::VMNThreadsCount,
                                                    PlatformParameters::VMNMachineFamily,
                                                    PlatformParameters::VMNProvider,
                                                    PlatformParameters::VMNVCPUSCount
                                                   ];

impl PartialEq for PlatformParameters {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for PlatformParameters {}

impl std::hash::Hash for PlatformParameters {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}


pub enum QuantifierType {
    AtLeast,
    AtMost,
    Value
}

pub enum FeatureType {
    Qualifier,
    Quantifier,
}

pub trait Feature {

    fn is_top(self:&Self) -> bool { false }

    fn string(self:&Self) -> String;

    fn hash_code(self:&Self) -> u64 {
        let mut s = DefaultHasher::new();
        self.string().hash(&mut s);
        s.finish()
    }

    fn feature_class(self:&Self) -> Option<PlatformParameters>;

    fn feature_type(self:&Self) -> FeatureType { FeatureType::Qualifier }

    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { None } 

    fn quantifier_type(self:&Self) -> Option<QuantifierType> { None } // only for quantifiers
    fn val(self:&Self) -> Option<i32> { None }                        // only for quantifiers
    fn other_val(self:&Self) -> Option<i32> { None }                  // only for quantifiers

    // Subtype relation betwenn qualifier and quantifier features
    fn subtypeof(self:&Self, other:&dyn Feature) -> bool {
        if self.feature_class() != other.feature_class() { return false };
        match self.feature_type() {
            FeatureType::Qualifier => {
                if self.hash_code() == other.hash_code() { return true }
                let mut s = self.supertype();
                loop {
                    match s {
                        Some(ref f) => { 
                            if f.hash_code() == other.hash_code() { break } 
                            else { s = s.expect("unexpected error").supertype() }
                        },
                        None => break
                    };                    
                };
                s.is_some()
            },
            FeatureType::Quantifier => {       

                let v1 = self.val().expect("unexpected error");
                let v2 = other.val().expect("unexpected error");

                let qt_self = self.quantifier_type().expect("quantifer expected");
                let qt_other = other.quantifier_type().expect("quantifier expected");
               
                match qt_self {
                    QuantifierType::AtLeast => match qt_other {
                        QuantifierType::AtLeast => v1 <= v2,
                        QuantifierType::AtMost => false,
                        QuantifierType::Value => { println!("{v1}>={v2}"); v1 <= v2 },
                    },
                    QuantifierType::AtMost => match qt_other {
                        QuantifierType::AtLeast => false,
                        QuantifierType::AtMost => v1 >= v2,
                        QuantifierType::Value => v1 >= v2,
                    },
                    QuantifierType::Value => match qt_other {
                        QuantifierType::AtLeast => { println!("{v1}>={v2}"); v1 >= v2 },
                        QuantifierType::AtMost => v1 <= v2,
                        QuantifierType::Value => v1 == v2,
                    },
                }
            },
        }
    }

}



// Feature types (for protoyping)

pub struct AtLeast {
    pub val:i32
}

pub struct AtMost {
    pub val:i32
}

pub struct SIMD;


pub struct AccModel;

pub struct AccBackend;

impl Feature for SIMD { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "simd".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::CPUSIMD) } 
}

impl Feature for AccModel { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "acc_model".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::ACCModel) } 
}
                            

impl Feature for AccBackend { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "acc_backend".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::ACCBackend) } 
}


impl Feature for AtLeast {  
    fn is_top(self:&Self) -> bool { self.val==std::i32::MAX }
    fn string(self:&Self) -> String { "atleast".to_string() } 
    fn feature_type(self:&Self) -> FeatureType { FeatureType::Quantifier } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { None }
    fn val(self:&Self) -> Option<i32> { Some((*self).val) } 
    fn quantifier_type(self:&Self) -> Option<QuantifierType> { Some(QuantifierType::AtLeast) }    
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> {  Some(Box::new(AtLeast { val:self.val+1 }) as Box<dyn Feature>)  }
}

impl Feature for AtMost {  
    fn is_top(self:&Self) -> bool { self.val==0 }
    fn string(self:&Self) -> String { "atmost".to_string() } 
    fn feature_type(self:&Self) -> FeatureType { FeatureType::Quantifier } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { None }
    fn val(self:&Self) -> Option<i32> { Some((*self).val) } 
    fn quantifier_type(self:&Self) -> Option<QuantifierType> { Some(QuantifierType::AtMost) }
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> {  Some(Box::new(AtMost { val:self.val-1 }) as Box<dyn Feature>)  }
}

impl Feature for i32 {  
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "value".to_string() } 
    fn feature_type(self:&Self) -> FeatureType { FeatureType::Quantifier } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { None }
    fn val(self:&Self) -> Option<i32> { Some(*self) } 
    fn quantifier_type(self:&Self) -> Option<QuantifierType> { Some(QuantifierType::Value) }
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { None  }
}