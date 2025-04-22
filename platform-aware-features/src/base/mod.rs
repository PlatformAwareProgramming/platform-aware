use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum PlatformParameters {
    /* accelerator features */
    acc_manufacturer,
    acc_memory_type,
    acc_count,
    acc_backend,
    acc_architecture,
    acc_model,
    acc_processor_count,
    acc_processor,
    acc_memory_size,
    acc_tdp,
    acc_type,

    /* processor features */
    cpu_count,
    cpu_core_count,
    cpu_core_threads_count,
    cpu_l1_size,
    cpu_l2_size,
    cpu_l3_size,
    cpu_clock,
    cpu_manufacturer,
    cpu_isa,
    cpu_simd,
    cpu_tdp,
    cpu_model,
    cpu_microarchitecture,

    /* interconnection */
    net_latency,
    net_type,
    net_bandwidth,
    net_startup_time,
    net_rdma,
    net_topology,

    /* storage */
    sto_bandwidth,
    sto_type,
    sto_latency,
    sto_size,
    sto_network_bandwidth,
    sto_interface,

    /* memory */
    mem_type,
    mem_size,
    mem_frequency,
    mem_latency,
    mem_bandwidth,

    /* node (cloud) */
    vm_provider,
    vm_dedicated,
    vm_machinefamily,
    vm_machinetype,
    vm_vcpus_count,
    vm_threads_count,
}


pub const PLATFORM_PARAMETERS:&[PlatformParameters] = &[
                                                    PlatformParameters::cpu_simd,
                                                    PlatformParameters::acc_model, 
                                                    PlatformParameters::acc_backend, 
                                                    PlatformParameters::acc_count,
                                                    PlatformParameters::acc_manufacturer,
                                                    PlatformParameters::acc_memory_type,
                                                    PlatformParameters::acc_count,
                                                    PlatformParameters::acc_backend,
                                                    PlatformParameters::acc_architecture,
                                                    PlatformParameters::acc_model,
                                                    PlatformParameters::acc_processor_count,
                                                    PlatformParameters::acc_processor,
                                                    PlatformParameters::acc_memory_size,
                                                    PlatformParameters::acc_tdp,
                                                    PlatformParameters::acc_type,                                                    
                                                    PlatformParameters::cpu_core_count,
                                                    PlatformParameters::cpu_count,
                                                    PlatformParameters::cpu_core_threads_count,
                                                    PlatformParameters::cpu_simd,
                                                    PlatformParameters::cpu_l1_size,
                                                    PlatformParameters::cpu_l2_size,
                                                    PlatformParameters::cpu_l3_size,
                                                    PlatformParameters::cpu_clock,
                                                    PlatformParameters::cpu_manufacturer,
                                                    PlatformParameters::cpu_isa,
                                                    PlatformParameters::cpu_tdp,
                                                    PlatformParameters::cpu_model,
                                                    PlatformParameters::cpu_microarchitecture,
                                                    PlatformParameters::net_latency,
                                                    PlatformParameters::net_type,
                                                    PlatformParameters::net_bandwidth,
                                                    PlatformParameters::net_startup_time,
                                                    PlatformParameters::net_rdma,
                                                    PlatformParameters::net_topology,
                                                    PlatformParameters::sto_bandwidth,
                                                    PlatformParameters::sto_type,
                                                    PlatformParameters::sto_latency,
                                                    PlatformParameters::sto_size,
                                                    PlatformParameters::sto_network_bandwidth,
                                                    PlatformParameters::sto_interface,
                                                    PlatformParameters::mem_type,
                                                    PlatformParameters::mem_size,
                                                    PlatformParameters::mem_frequency,
                                                    PlatformParameters::mem_latency,
                                                    PlatformParameters::mem_bandwidth,
                                                    PlatformParameters::vm_dedicated,
                                                    PlatformParameters::vm_machinetype,
                                                    PlatformParameters::vm_threads_count,
                                                    PlatformParameters::vm_machinefamily,
                                                    PlatformParameters::vm_provider,
                                                    PlatformParameters::vm_vcpus_count
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
                        QuantifierType::Value => { v1 <= v2 },
                    },
                    QuantifierType::AtMost => match qt_other {
                        QuantifierType::AtLeast => false,
                        QuantifierType::AtMost => v1 >= v2,
                        QuantifierType::Value => v1 >= v2,
                    },
                    QuantifierType::Value => match qt_other {
                        QuantifierType::AtLeast => { v1 >= v2 },
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
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::cpu_simd) } 
}

impl Feature for AccModel { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "acc_model".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_model) } 
}
                            

impl Feature for AccBackend { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "acc_backend".to_string() } 
    fn feature_class(self:&Self) -> std::option::Option<PlatformParameters> { Some(PlatformParameters::acc_backend) } 
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
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { None }
}

