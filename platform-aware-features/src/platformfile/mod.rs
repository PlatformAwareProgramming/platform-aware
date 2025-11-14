use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use toml::value::Array;

use configurable_features::{lookup_feature, PlatformFeatures, PlatformParameter, Feature};
pub use configurable_features::readplatform;

#[derive(Deserialize)]
pub struct Platform {
    pub accelerator: Option<PlatformAccelerator>,
    pub processor: Option<PlatformProcessor>,
    pub interconnection: Option<PlatformInterconnection>,
    pub storage: Option<PlatformStorage>,
    pub memory: Option<PlatformMemory>,
    pub node: Option<PlatformNode>,
}

#[derive(Deserialize)]
pub struct PlatformAccelerator {
    pub acc_manufacturer: Option<String>,
    pub acc_memory_type: Option<String>,
    pub acc_count: Option<i32>,
    pub acc_backend: Option<Array>,
    pub acc_architecture: Option<String>,
    pub acc_model: Option<String>,
    pub acc_processor_count: Option<i32>,
    pub acc_processor: Option<String>,
    pub acc_memorysize: Option<i32>,
    pub acc_tdp: Option<i32>,
    pub acc_type: Option<String>,
}

#[derive(Deserialize)]
pub struct PlatformProcessor {
    pub cpu_core_count: Option<i32>,
    pub cpu_count: Option<i32>,
    pub cpu_core_threads_count: Option<i32>,
    pub cpu_simd: Option<Array>,
    pub cpu_L2_size: Option<i32>,
    pub cpu_L3_size: Option<i32>,
    pub cpu_L1_size: Option<i32>,
    pub cpu_core_clock: Option<i32>,
    pub cpu_manufacturer: Option<String>,
    pub cpu_isa: Option<String>,
    pub cpu_tdp: Option<i32>,
    pub cpu_model: Option<String>,
    pub cpu_microarchitecture: Option<String>,
}

#[derive(Deserialize)]
pub struct PlatformInterconnection {
    pub net_latency: Option<i32>,
    pub net_type: Option<String>,
    pub net_bandwidth: Option<i32>,
    pub net_startuptime: Option<i32>,
    pub net_RDMA: Option<String>,
    pub net_topology: Option<String>,
}

#[derive(Deserialize)]
pub struct PlatformStorage {
    pub sto_bandwidth: Option<i32>,
    pub sto_type: Option<String>,
    pub sto_latency: Option<i32>,
    pub sto_size: Option<i32>,
    pub sto_networkbandwidth: Option<i32>,
    pub sto_interface: Option<String>,
}

#[derive(Deserialize)]
pub struct PlatformMemory {
    pub mem_frequency: Option<i32>,
    pub mem_size: Option<i32>,
    pub mem_latency: Option<i32>,
    pub mem_type: Option<String>,
    pub mem_bandwidth: Option<i32>,
}

#[derive(Deserialize)]
pub struct PlatformNode {
    pub vm_dedicated: Option<bool>,
    pub vm_count: Option<i32>,
    pub vm_virtual: Option<bool>,
    pub vm_machinetype: Option<String>,
    pub vm_threads_count: Option<i32>,
    pub vm_machinefamily: Option<String>,
    pub vm_provider: Option<String>,
    pub vm_vcpus_count: Option<i32>,
}

pub static CURRENT_PLATFORM: Lazy<Mutex<Platform>> = Lazy::new(|| {
    let t: Platform = match readplatform() {
        Ok(t) => t,
        Err(_) => todo!(),
    };
    Mutex::new(t)
});


pub fn readplatformfeatures() -> PlatformFeatures {
 
    let t = CURRENT_PLATFORM.lock().unwrap();

    let mut m: PlatformFeatures = ::std::collections::HashMap::new();

    match &t.accelerator {
        Some(u) => {
            match &u.acc_backend {
                Some(v) => {
                    let mut fvec = Vec::new();
                    for fv in v {
                        let fs = fv.as_str().unwrap();
                        let static_fs: &'static str = Box::leak(fs.to_string().into_boxed_str());
                        match lookup_feature(static_fs) {
                            Some(f) => fvec.push(f),
                            None => {
                                println!("qualifier {} not found and ignored", static_fs)
                            }
                        };
                    }

                    m.insert("acc_backend".to_string(), Arc::new(fvec));
                }
                None => {}
            }
            match &u.acc_architecture {
                Some(v) => add_qualifier(&mut m, "acc_architecture".to_string(), v.to_string()),
                None => {}
            }
            match &u.acc_memory_type {
                Some(v) => add_qualifier(&mut m, "acc_memory_type".to_string(), v.to_string()),
                None => {}
            }
            match &u.acc_manufacturer {
                Some(v) => add_qualifier(&mut m, "acc_manufacturer".to_string(), v.to_string()),
                None => {}
            }
            match &u.acc_model {
                Some(v) => add_qualifier(&mut m, "acc_model".to_string(), v.to_string()),
                None => {}
            }
            match &u.acc_processor {
                Some(v) => add_qualifier(&mut m, "acc_processor".to_string(), v.to_string()),
                None => {}
            }
            match &u.acc_type {
                Some(v) => add_qualifier(&mut m, "acc_type".to_string(), v.to_string()),
                None => {}
            }
            match &u.acc_count {
                Some(v) => {
                    add_quantifier(&mut m, "acc_count".to_string(), *v);
                }
                None => {}
            }
            match &u.acc_memorysize {
                Some(v) => {
                    add_quantifier(&mut m, "acc_architecture".to_string(), *v);
                }
                None => {}
            }
            match &u.acc_processor_count {
                Some(v) => {
                    add_quantifier(&mut m, "acc_processor_count".to_string(), *v);
                }
                None => {}
            }
            match &u.acc_tdp {
                Some(v) => {
                    add_quantifier(&mut m, "acc_tdp".to_string(), *v);
                }
                None => {}
            }
        }
        None => {}
    }

    match &t.processor {
        Some(u) => {
            match &u.cpu_isa {
                Some(v) => add_qualifier(&mut m, "cpu_isa".to_string(), v.to_string()),
                None => {}
            }
            match &u.cpu_manufacturer {
                Some(v) => add_qualifier(&mut m, "cpu_manufacturer".to_string(), v.to_string()),
                None => {}
            }
            match &u.cpu_microarchitecture {
                Some(v) => add_qualifier(&mut m, "cpu_microarchitecture".to_string(), v.to_string()),
                None => {}
            }
            match &u.cpu_model {
                Some(v) => add_qualifier(&mut m, "cpu_model".to_string(), v.to_string()),
                None => {}
            }
            //match u.cpu_simd { Some(v) => add_qualifier(&mut m,
            // "cpu_simd".to_string(), v), None => {} }
            match &u.cpu_L1_size {
                Some(v) => {
                    add_quantifier(&mut m, "cpu_l1_size".to_string(), *v);
                }
                None => {}
            }
            match &u.cpu_L2_size {
                Some(v) => {
                    add_quantifier(&mut m, "cpu_l2_size".to_string(), *v);
                }
                None => {}
            }
            match &u.cpu_L3_size {
                Some(v) => {
                    add_quantifier(&mut m, "cpu_l3_size".to_string(), *v);
                }
                None => {}
            }
            match &u.cpu_core_clock {
                Some(v) => {
                    add_quantifier(&mut m, "cpu_clock".to_string(), *v);
                }
                None => {}
            }
            match &u.cpu_core_count {
                Some(v) => {
                    add_quantifier(&mut m, "cpu_core_count".to_string(), *v);
                }
                None => {}
            }
            match &u.cpu_core_threads_count {
                Some(v) => {
                    add_quantifier(&mut m, "cpu_core_threads_count".to_string(), *v);
                }
                None => {}
            }
            match &u.cpu_count {
                Some(v) => {
                    add_quantifier(&mut m, "cpu_count".to_string(), *v);
                }
                None => {}
            }
            match &u.cpu_tdp {
                Some(v) => {
                    add_quantifier(&mut m, "cpu_tdp".to_string(), *v);
                }
                None => {}
            }
        }
        None => {}
    }

    match &t.memory {
        Some(u) => {
            match &u.mem_type {
                Some(v) => add_qualifier(&mut m, "mem_type".to_string(), v.to_string()),
                None => {}
            }
            match &u.mem_bandwidth {
                Some(v) => {
                    add_quantifier(&mut m, "mem_bandwidth".to_string(), *v);
                }
                None => {}
            }
            match &u.mem_frequency {
                Some(v) => {
                    add_quantifier(&mut m, "mem_frequency".to_string(), *v);
                }
                None => {}
            }
            match &u.mem_latency {
                Some(v) => {
                    add_quantifier(&mut m, "mem_latency".to_string(), *v);
                }
                None => {}
            }
            match &u.mem_size {
                Some(v) => {
                    add_quantifier(&mut m, "mem_size".to_string(), *v);
                }
                None => {}
            }
        }
        None => {}
    }

    match &t.interconnection {
        Some(u) => {
            match &u.net_RDMA {
                Some(v) => add_qualifier(&mut m, "net_rdma".to_string(), v.to_string()),
                None => {}
            }
            match &u.net_type {
                Some(v) => add_qualifier(&mut m, "net_type".to_string(), v.to_string()),
                None => {}
            }
            match &u.net_topology {
                Some(v) => add_qualifier(&mut m, "net_topology".to_string(), v.to_string()),
                None => {}
            }
            match &u.net_bandwidth {
                Some(v) => {
                    add_quantifier(&mut m, "net_bandwidth".to_string(), *v);
                }
                None => {}
            }
            match &u.net_latency {
                Some(v) => {
                    add_quantifier(&mut m, "net_latency".to_string(), *v);
                }
                None => {}
            }
            match &u.net_startuptime {
                Some(v) => {
                    add_quantifier(&mut m, "net_startup_time".to_string(), *v);
                }
                None => {}
            }
        }
        None => {}
    }

    match &t.storage {
        Some(u) => {
            match &u.sto_interface {
                Some(v) => add_qualifier(&mut m, "sto_interface".to_string(), v.to_string()),
                None => {}
            }
            match &u.sto_type {
                Some(v) => add_qualifier(&mut m, "sto_type".to_string(), v.to_string()),
                None => {}
            }
            match &u.sto_bandwidth {
                Some(v) => {
                    add_quantifier(&mut m, "sto_bandwidth".to_string(), *v);
                }
                None => {}
            }
            match &u.sto_latency {
                Some(v) => {
                    add_quantifier(&mut m, "sto_latency".to_string(), *v);
                }
                None => {}
            }
            match &u.sto_networkbandwidth {
                Some(v) => {
                    add_quantifier(&mut m, "sto_network_bandwidth".to_string(), *v);
                }
                None => {}
            }
            match &u.sto_size {
                Some(v) => {
                    add_quantifier(&mut m, "sto_size".to_string(), *v);
                }
                None => {}
            }
        }
        None => {}
    }

    match &t.node {
        Some(u) => {
            match &u.vm_machinefamily {
                Some(v) => add_qualifier(&mut m, "vm_machinefamily".to_string(), v.to_string()),
                None => {}
            }
            match &u.vm_machinetype {
                Some(v) => add_qualifier(&mut m, "vm_machinetype".to_string(), v.to_string()),
                None => {}
            }
            match &u.vm_provider {
                Some(v) => add_qualifier(&mut m, "vm_provider".to_string(), v.to_string()),
                None => {}
            }
            //match u.vm_count { Some(v) => { add_quantifier(&mut m,
            // "vm_count".to_string(), v); }, None => {} }
            match &u.vm_threads_count {
                Some(v) => {
                    add_quantifier(&mut m, "vm_threads_count".to_string(), *v);
                }
                None => {}
            }
            match &u.vm_vcpus_count {
                Some(v) => {
                    add_quantifier(&mut m, "vm_vcpus_count".to_string(), *v);
                }
                None => {}
            }
            //match u.vm_dedicated { Some(v) => { add_quantifier(&mut m,
            // "vm_dedicated".to_string(), v); }, None => {} } match
            // u.vm_virtual { Some(v) => { add_quantifier(&mut m,
            // "vm_virtual".to_string(), v); }, None => {} }
        }
        None => {}
    }

    return m;
}

pub fn add_qualifier(
    m: &mut HashMap<PlatformParameter, Arc<dyn Feature + Send + Sync>>,
    par: PlatformParameter,
    v: String,
) {
    let f = lookup_feature(Box::leak(v.into_boxed_str()));
    match f {
        Some(f) => {
            m.insert(par, f);
        }
        None => {}
    }
}

pub fn add_quantifier(
    m: &mut HashMap<PlatformParameter, Arc<dyn Feature + Send + Sync>>,
    par: PlatformParameter,
    v: i32,
) {
    m.insert(par, Arc::new(v));
}
