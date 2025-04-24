
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
