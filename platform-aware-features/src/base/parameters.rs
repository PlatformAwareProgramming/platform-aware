use std::sync::Arc;

use configurable_features::insert_parameter;
use ctor::ctor;

use crate::*;

#[ctor]
fn module_initializer() {
    // Accelerator parameters (Table 3 + vendor extensions)
    insert_parameter("acc_manufacturer".to_string(), Arc::new(AccManufacturer));
    insert_parameter("acc_memorytype".to_string(), Arc::new(AccMemoryType));
    insert_parameter("acc_count".to_string(), Arc::new(AtLeast { val: 0 }));
    insert_parameter("acc_backend".to_string(), Arc::new(AccBackend));
    insert_parameter("acc_architecture".to_string(), Arc::new(AccArchitecture));
    insert_parameter("acc_model".to_string(), Arc::new(AccModel));
    insert_parameter("acc_processorcount".to_string(), Arc::new(AtLeast { val: 0 }));
    insert_parameter("acc_processor".to_string(), Arc::new(AccProcessor));
    insert_parameter("acc_memorysize".to_string(), Arc::new(AtLeast { val: 0 }));
    insert_parameter("acc_tdp".to_string(), Arc::new(AtMost { val: i32::MAX }));
    insert_parameter("acc_type".to_string(), Arc::new(AccType));
    insert_parameter("acc_interconnect".to_string(), Arc::new(AccInterconnect));

    // Main processor parameters (Table 3)
    insert_parameter("cpu_count".to_string(), Arc::new(AtLeast { val: 1 }));
    insert_parameter("cpu_core_count".to_string(), Arc::new(AtLeast { val: 1 }));
    insert_parameter("cpu_core_threads_count".to_string(), Arc::new(AtLeast { val: 1 }));
    insert_parameter("cpu_core_clock".to_string(), Arc::new(AtLeast { val: 0 }));
    insert_parameter("cpu_L1_size".to_string(), Arc::new(AtLeast { val: 0 }));
    insert_parameter("cpu_L2_size".to_string(), Arc::new(AtLeast { val: 0 }));
    insert_parameter("cpu_L3_size".to_string(), Arc::new(AtLeast { val: 0 }));
    insert_parameter("cpu_tdp".to_string(), Arc::new(AtMost { val: i32::MAX }));
    insert_parameter("cpu_manufacturer".to_string(), Arc::new(CPUManufacturer));
    insert_parameter("cpu_isa".to_string(), Arc::new(CPUISA));
    insert_parameter("cpu_simd".to_string(), Arc::new(CPUSIMD));
    insert_parameter("cpu_model".to_string(), Arc::new(CPUModel));
    insert_parameter("cpu_microarchitecture".to_string(), Arc::new(CPUMicroarchitecture));

    // Interconnection parameters (Table 3)
    insert_parameter("net_latency".to_string(), Arc::new(AtMost { val: i32::MAX }));
    insert_parameter("net_type".to_string(), Arc::new(NetType));
    insert_parameter("net_bandwidth".to_string(), Arc::new(AtLeast { val: 0 }));
    insert_parameter("net_startuptime".to_string(), Arc::new(AtMost { val: i32::MAX }));
    insert_parameter("net_RDMA".to_string(), Arc::new(NetRDMA));
    insert_parameter("net_topology".to_string(), Arc::new(NetTopology));

    // Storage parameters (Table 3)
    insert_parameter("sto_bandwidth".to_string(), Arc::new(AtLeast { val: 0 }));
    insert_parameter("sto_type".to_string(), Arc::new(StoType));
    insert_parameter("sto_latency".to_string(), Arc::new(AtMost { val: i32::MAX }));
    insert_parameter("sto_size".to_string(), Arc::new(AtLeast { val: 0 }));
    insert_parameter("sto_startuptime".to_string(), Arc::new(AtMost { val: i32::MAX }));
    insert_parameter("sto_networkbandwidth".to_string(), Arc::new(AtLeast { val: 0 }));
    insert_parameter("sto_interface".to_string(), Arc::new(StoInterface));

    // Memory parameters (Table 3)
    insert_parameter("mem_type".to_string(), Arc::new(MemoryType));
    insert_parameter("mem_size".to_string(), Arc::new(AtLeast { val: 0 }));
    insert_parameter("mem_frequency".to_string(), Arc::new(AtLeast { val: 0 }));
    insert_parameter("mem_latency".to_string(), Arc::new(AtMost { val: i32::MAX }));
    insert_parameter("mem_bandwidth".to_string(), Arc::new(AtLeast { val: 0 }));
}
