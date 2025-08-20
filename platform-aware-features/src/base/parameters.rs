use std::{collections::HashMap, sync::{Arc, Mutex}};

use once_cell::sync::Lazy;

use super::{AccArchitecture, AccBackend, AccInterconnect, AccManufacturer, AccModel, AccProcessor, AccType, AtLeast, AtMost, CPUManufacturer, CPUMicroarchitecture, CPUModel, Feature, MemoryType, NETType, NetTopology, StoInterface, StoType, CPUISA, CPUSIMD};

pub type PlatformParameter = String;

/*
//#[derive(Debug)]
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum PlatformParameter {
    /* accelerator features */
    acc_manufacturer,
    acc_memory_type,
    acc_count,
    acc_backend,
    acc_cudacc,
    acc_cudatoolkit,
    acc_cudadriver,
    acc_architecture,
    acc_model,
    acc_processor_count,
    acc_processor,
    acc_memorysize,
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
*/


pub static PLATFORM_PARAMETERS: Lazy<Mutex<Vec<PlatformParameter>>> = Lazy::new(|| {
    let map: Vec<PlatformParameter> = Vec::new();
    Mutex::new(map)
});

/* pub const PLATFORM_PARAMETERS:&[PlatformParameter] = &[
                                                    PlatformParameter::cpu_simd,
                                                    PlatformParameter::acc_model, 
                                                    PlatformParameter::acc_backend, 
                                                    PlatformParameter::acc_cudacc, 
                                                    PlatformParameter::acc_cudatoolkit,
                                                    PlatformParameter::acc_cudadriver,
                                                    PlatformParameter::acc_count,
                                                    PlatformParameter::acc_manufacturer,
                                                    PlatformParameter::acc_memory_type,
                                                    PlatformParameter::acc_count,
                                                    PlatformParameter::acc_architecture,
                                                    PlatformParameter::acc_processor_count,
                                                    PlatformParameter::acc_processor,
                                                    PlatformParameter::acc_memorysize,
                                                    PlatformParameter::acc_tdp,
                                                    PlatformParameter::acc_type,                                                    
                                                    PlatformParameter::acc_interconnect,                                                    
                                                    PlatformParameter::cpu_core_count,
                                                    PlatformParameter::cpu_count,
                                                    PlatformParameter::cpu_core_threads_count,
                                                    PlatformParameter::cpu_simd,
                                                    PlatformParameter::cpu_l1_size,
                                                    PlatformParameter::cpu_l2_size,
                                                    PlatformParameter::cpu_l3_size,
                                                    PlatformParameter::cpu_clock,
                                                    PlatformParameter::cpu_manufacturer,
                                                    PlatformParameter::cpu_isa,
                                                    PlatformParameter::cpu_tdp,
                                                    PlatformParameter::cpu_model,
                                                    PlatformParameter::cpu_microarchitecture,
                                                    PlatformParameter::net_latency,
                                                    PlatformParameter::net_type,
                                                    PlatformParameter::net_bandwidth,
                                                    PlatformParameter::net_startup_time,
                                                    PlatformParameter::net_rdma,
                                                    PlatformParameter::net_topology,
                                                    PlatformParameter::sto_bandwidth,
                                                    PlatformParameter::sto_type,
                                                    PlatformParameter::sto_latency,
                                                    PlatformParameter::sto_size,
                                                    PlatformParameter::sto_network_bandwidth,
                                                    PlatformParameter::sto_interface,
                                                    PlatformParameter::mem_type,
                                                    PlatformParameter::mem_size,
                                                    PlatformParameter::mem_frequency,
                                                    PlatformParameter::mem_latency,
                                                    PlatformParameter::mem_bandwidth,
                                                    PlatformParameter::vm_dedicated,
                                                    PlatformParameter::vm_machinetype,
                                                    PlatformParameter::vm_threads_count,
                                                    PlatformParameter::vm_machinefamily,
                                                    PlatformParameter::vm_provider,
                                                    PlatformParameter::vm_vcpus_count
                                                   ];
*/

pub static FEATURE_TOP: Lazy<Mutex<HashMap<PlatformParameter, Arc<dyn Feature + Send + Sync>>>> = Lazy::new(|| {
    let map: HashMap<PlatformParameter, Arc<dyn Feature + Send + Sync>> = HashMap::new();


/* 
    map.insert(PlatformParameter::acc_manufacturer, Arc::new(AccManufacturer));
    map.insert(PlatformParameter::acc_memory_type, Arc::new(MemoryType));
    map.insert(PlatformParameter::acc_count, Arc::new(AtLeast{val:0}));
    map.insert(PlatformParameter::acc_backend, Arc::new(AccBackend));
    map.insert(PlatformParameter::acc_cudacc, Arc::new(AtLeast{val:10}));
    map.insert(PlatformParameter::acc_cudatoolkit, Arc::new(AtLeast{val:10}));
    map.insert(PlatformParameter::acc_cudadriver, Arc::new(AtLeast{val:100}));
    map.insert(PlatformParameter::acc_architecture, Arc::new(AccArchitecture));
    map.insert(PlatformParameter::acc_model, Arc::new(AccModel));
    map.insert(PlatformParameter::acc_processor_count, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameter::acc_processor, Arc::new(AccProcessor));
    map.insert(PlatformParameter::acc_memorysize, Arc::new(AtLeast{val:0}));
    map.insert(PlatformParameter::acc_tdp, Arc::new(AtMost{val:i32::MAX}));
    map.insert(PlatformParameter::acc_type, Arc::new(AccType));
    map.insert(PlatformParameter::acc_interconnect, Arc::new(AccInterconnect));
    map.insert(PlatformParameter::cpu_count, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameter::cpu_core_count, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameter::cpu_core_threads_count, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameter::cpu_l1_size, Arc::new(AtLeast{val:0}));
    map.insert(PlatformParameter::cpu_l2_size, Arc::new(AtLeast{val:0}));
    map.insert(PlatformParameter::cpu_l3_size, Arc::new(AtLeast{val:0}));
    map.insert(PlatformParameter::cpu_manufacturer, Arc::new(CPUManufacturer));
    map.insert(PlatformParameter::cpu_isa, Arc::new(CPUISA));
    map.insert(PlatformParameter::cpu_simd, Arc::new(CPUSIMD));
    map.insert(PlatformParameter::cpu_model, Arc::new(CPUModel));
    map.insert(PlatformParameter::cpu_microarchitecture, Arc::new(CPUMicroarchitecture));
    map.insert(PlatformParameter::net_latency, Arc::new(AtMost{val:0}));
    map.insert(PlatformParameter::net_type, Arc::new(NETType));
    map.insert(PlatformParameter::net_bandwidth, Arc::new(AtLeast{val:0}));
    map.insert(PlatformParameter::net_startup_time, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameter::net_rdma, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameter::net_topology, Arc::new(NetTopology));
    map.insert(PlatformParameter::sto_bandwidth, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameter::sto_type, Arc::new(StoType));
    map.insert(PlatformParameter::sto_latency, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameter::sto_size, Arc::new(AtLeast{val:0}));
    map.insert(PlatformParameter::sto_network_bandwidth, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameter::sto_interface, Arc::new(StoInterface));
    map.insert(PlatformParameter::mem_type, Arc::new(MemoryType));
    map.insert(PlatformParameter::mem_size, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameter::mem_frequency, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameter::mem_latency, Arc::new(AtLeast{val:1}));
    map.insert(PlatformParameter::mem_bandwidth, Arc::new(AtLeast{val:1}));
*/

//    map.insert(PlatformParameter::vm_provider, Arc::new(CPUSIMD));
//    map.insert(PlatformParameter::vm_dedicated, Arc::new(CPUSIMD));
//    map.insert(PlatformParameter::vm_machinefamily, Arc::new(CPUSIMD));
//    map.insert(PlatformParameter::vm_machinetype, Arc::new(CPUSIMD));
//    map.insert(PlatformParameter::vm_vcpus_count, Arc::new(CPUSIMD));
//    map.insert(PlatformParameter::vm_threads_count, Arc::new(CPUSIMD));
    
    Mutex::new(map)
});

pub fn insert_parameter(fname:PlatformParameter, fvalue: Arc<dyn Feature + Send + Sync>) {
    let mut dict = FEATURE_TOP.lock().unwrap();
    let mut paramlist = PLATFORM_PARAMETERS.lock().unwrap();
    paramlist.push(fname.clone());
    dict.insert(fname, fvalue);
}

impl PartialEq for PlatformParameter {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for PlatformParameter {}

impl std::hash::Hash for PlatformParameter {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}


use ctor::ctor;

#[ctor]
fn module_initializer() {
    insert_parameter(PlatformParameter::acc_manufacturer, Arc::new(AccManufacturer));
    insert_parameter(PlatformParameter::acc_memory_type, Arc::new(MemoryType));
    insert_parameter(PlatformParameter::acc_count, Arc::new(AtLeast{val:0}));
    insert_parameter(PlatformParameter::acc_backend, Arc::new(AccBackend));
//    insert_parameter(PlatformParameter::acc_cudacc, Arc::new(AtLeast{val:10}));
//    insert_parameter(PlatformParameter::acc_cudatoolkit, Arc::new(AtLeast{val:10}));
//    insert_parameter(PlatformParameter::acc_cudadriver, Arc::new(AtLeast{val:100}));
    insert_parameter(PlatformParameter::acc_architecture, Arc::new(AccArchitecture));
    insert_parameter(PlatformParameter::acc_model, Arc::new(AccModel));
    insert_parameter(PlatformParameter::acc_processor_count, Arc::new(AtLeast{val:1}));
    insert_parameter(PlatformParameter::acc_processor, Arc::new(AccProcessor));
    insert_parameter(PlatformParameter::acc_memorysize, Arc::new(AtLeast{val:0}));
    insert_parameter(PlatformParameter::acc_tdp, Arc::new(AtMost{val:i32::MAX}));
    insert_parameter(PlatformParameter::acc_type, Arc::new(AccType));
    insert_parameter(PlatformParameter::acc_interconnect, Arc::new(AccInterconnect));
    insert_parameter(PlatformParameter::cpu_count, Arc::new(AtLeast{val:1}));
    insert_parameter(PlatformParameter::cpu_core_count, Arc::new(AtLeast{val:1}));
    insert_parameter(PlatformParameter::cpu_core_threads_count, Arc::new(AtLeast{val:1}));
    insert_parameter(PlatformParameter::cpu_l1_size, Arc::new(AtLeast{val:0}));
    insert_parameter(PlatformParameter::cpu_l2_size, Arc::new(AtLeast{val:0}));
    insert_parameter(PlatformParameter::cpu_l3_size, Arc::new(AtLeast{val:0}));
    insert_parameter(PlatformParameter::cpu_manufacturer, Arc::new(CPUManufacturer));
    insert_parameter(PlatformParameter::cpu_isa, Arc::new(CPUISA));
    insert_parameter(PlatformParameter::cpu_simd, Arc::new(CPUSIMD));
    insert_parameter(PlatformParameter::cpu_model, Arc::new(CPUModel));
    insert_parameter(PlatformParameter::cpu_microarchitecture, Arc::new(CPUMicroarchitecture));
    insert_parameter(PlatformParameter::net_latency, Arc::new(AtMost{val:0}));
    insert_parameter(PlatformParameter::net_type, Arc::new(NETType));
    insert_parameter(PlatformParameter::net_bandwidth, Arc::new(AtLeast{val:0}));
    insert_parameter(PlatformParameter::net_startup_time, Arc::new(AtLeast{val:1}));
    insert_parameter(PlatformParameter::net_rdma, Arc::new(AtLeast{val:1}));
    insert_parameter(PlatformParameter::net_topology, Arc::new(NetTopology));
    insert_parameter(PlatformParameter::sto_bandwidth, Arc::new(AtLeast{val:1}));
    insert_parameter(PlatformParameter::sto_type, Arc::new(StoType));
    insert_parameter(PlatformParameter::sto_latency, Arc::new(AtLeast{val:1}));
    insert_parameter(PlatformParameter::sto_size, Arc::new(AtLeast{val:0}));
    insert_parameter(PlatformParameter::sto_network_bandwidth, Arc::new(AtLeast{val:1}));
    insert_parameter(PlatformParameter::sto_interface, Arc::new(StoInterface));
    insert_parameter(PlatformParameter::mem_type, Arc::new(MemoryType));
    insert_parameter(PlatformParameter::mem_size, Arc::new(AtLeast{val:1}));
    insert_parameter(PlatformParameter::mem_frequency, Arc::new(AtLeast{val:1}));
    insert_parameter(PlatformParameter::mem_latency, Arc::new(AtLeast{val:1}));
    insert_parameter(PlatformParameter::mem_bandwidth, Arc::new(AtLeast{val:1}));
}