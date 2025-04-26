use std::{collections::HashMap, sync::{Arc, Mutex}};

use once_cell::sync::Lazy;

use super::{AccArchitecture, AccBackend, AccInterconnect, AccManufacturer, AccModel, AccProcessor, AccType, AtLeast, AtMost, CPUManufacturer, CPUMicroarchitecture, CPUModel, Feature, MemoryType, NETType, NetTopology, StoInterface, StoType, CPUISA, CPUSIMD};


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
    acc_interconnect,

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
                                                    PlatformParameters::acc_interconnect,                                                    
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


pub static FEATURE_TOP: Lazy<Mutex<HashMap<PlatformParameters, Arc<dyn Feature + Send + Sync>>>> = Lazy::new(|| {
    let mut map: HashMap<PlatformParameters, Arc<dyn Feature + Send + Sync>> = HashMap::new();

    map.insert(PlatformParameters::acc_manufacturer, Arc::new(AccManufacturer));
    map.insert(PlatformParameters::acc_memory_type, Arc::new(MemoryType));
    map.insert(PlatformParameters::acc_count, Arc::new(AtLeast{val:0}));
    map.insert(PlatformParameters::acc_backend, Arc::new(AccBackend));
    map.insert(PlatformParameters::acc_architecture, Arc::new(AccArchitecture));
    map.insert(PlatformParameters::acc_model, Arc::new(AccModel));
    map.insert(PlatformParameters::acc_processor_count, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameters::acc_processor, Arc::new(AccProcessor));
    map.insert(PlatformParameters::acc_memory_size, Arc::new(AtLeast{val:0}));
    map.insert(PlatformParameters::acc_tdp, Arc::new(AtMost{val:i32::MAX}));
    map.insert(PlatformParameters::acc_type, Arc::new(AccType));
    map.insert(PlatformParameters::acc_interconnect, Arc::new(AccInterconnect));
    map.insert(PlatformParameters::cpu_count, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameters::cpu_core_count, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameters::cpu_core_threads_count, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameters::cpu_l1_size, Arc::new(AtLeast{val:0}));
    map.insert(PlatformParameters::cpu_l2_size, Arc::new(AtLeast{val:0}));
    map.insert(PlatformParameters::cpu_l3_size, Arc::new(AtLeast{val:0}));
    map.insert(PlatformParameters::cpu_manufacturer, Arc::new(CPUManufacturer));
    map.insert(PlatformParameters::cpu_isa, Arc::new(CPUISA));
    map.insert(PlatformParameters::cpu_simd, Arc::new(CPUSIMD));
    map.insert(PlatformParameters::cpu_model, Arc::new(CPUModel));
    map.insert(PlatformParameters::cpu_microarchitecture, Arc::new(CPUMicroarchitecture));
    map.insert(PlatformParameters::net_latency, Arc::new(AtMost{val:0}));
    map.insert(PlatformParameters::net_type, Arc::new(NETType));
    map.insert(PlatformParameters::net_bandwidth, Arc::new(AtLeast{val:0}));
    map.insert(PlatformParameters::net_startup_time, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameters::net_rdma, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameters::net_topology, Arc::new(NetTopology));
    map.insert(PlatformParameters::sto_bandwidth, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameters::sto_type, Arc::new(StoType));
    map.insert(PlatformParameters::sto_latency, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameters::sto_size, Arc::new(AtLeast{val:0}));
    map.insert(PlatformParameters::sto_network_bandwidth, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameters::sto_interface, Arc::new(StoInterface));
    map.insert(PlatformParameters::mem_type, Arc::new(MemoryType));
    map.insert(PlatformParameters::mem_size, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameters::mem_frequency, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameters::mem_latency, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameters::mem_bandwidth, Arc::new(AtLeast{val:1}));
//    map.insert(PlatformParameters::vm_provider, Arc::new(CPUSIMD));
//    map.insert(PlatformParameters::vm_dedicated, Arc::new(CPUSIMD));
//    map.insert(PlatformParameters::vm_machinefamily, Arc::new(CPUSIMD));
//    map.insert(PlatformParameters::vm_machinetype, Arc::new(CPUSIMD));
//    map.insert(PlatformParameters::vm_vcpus_count, Arc::new(CPUSIMD));
//    map.insert(PlatformParameters::vm_threads_count, Arc::new(CPUSIMD));
    
    Mutex::new(map)
});


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
