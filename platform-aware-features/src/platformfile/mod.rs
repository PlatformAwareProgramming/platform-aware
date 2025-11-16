use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use toml::{Value, value::Array};

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
    pub acc_interconnect: Option<String>,
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

pub fn readqualifier(f:Option<String>, 
                         fstr:&str, 
                         m: &mut HashMap<PlatformParameter, Arc<dyn Feature + Send + Sync>>)
{
    match f {
        Some(v) => add_qualifier(m, fstr.to_string(), v.to_string()),
        None => {}
    }
} 

pub fn readquantifier(f:Option<i32>, 
                         fstr:&str, 
                         m: &mut HashMap<PlatformParameter, Arc<dyn Feature + Send + Sync>>)
{
    match f {
        Some(v) => add_quantifier(m, fstr.to_string(), v),
        None => {}
    }
} 


pub fn readfeaturevector(f:Option<Vec<Value>>, 
                         fstr:&str, 
                         m: &mut HashMap<PlatformParameter, Arc<dyn Feature + Send + Sync>>)
{
               match f {
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

                    m.insert(fstr.to_string(), Arc::new(fvec));
                }
                None => {}
            }
}

pub fn readplatformfeatures() -> PlatformFeatures {
 
    let t = CURRENT_PLATFORM.lock().unwrap();

    let mut m: PlatformFeatures = ::std::collections::HashMap::new();

    match &t.accelerator {
        Some(u) => {
            readfeaturevector(u.acc_backend.clone(), "acc_backend", &mut m);
            readqualifier(u.acc_architecture.clone(), "acc_architecture", &mut m);
            readqualifier(u.acc_interconnect.clone(), "acc_interconnect", &mut m);
            readqualifier(u.acc_memory_type.clone(), "acc_memory_type", &mut m);
            readqualifier(u.acc_manufacturer.clone(), "acc_manufacturer", &mut m);
            readqualifier(u.acc_model.clone(), "acc_model", &mut m);
            readqualifier(u.acc_processor.clone(), "acc_processor", &mut m);
            readqualifier(u.acc_type.clone(), "acc_type", &mut m);
            readquantifier(u.acc_count.clone(), "acc_count", &mut m);
            readquantifier(u.acc_memorysize.clone(), "acc_memorysize", &mut m);
            readquantifier(u.acc_processor_count.clone(), "acc_processor_count", &mut m);
            readquantifier(u.acc_tdp.clone(), "acc_tdp", &mut m);
        }
        None => {}
    }

    match &t.processor {
        Some(u) => {
            readqualifier(u.cpu_isa.clone(), "cpu_isa", &mut m);
            readqualifier(u.cpu_manufacturer.clone(), "cpu_manufacturer", &mut m);
            readqualifier(u.cpu_microarchitecture.clone(), "cpu_microarchitecture", &mut m);
            readqualifier(u.cpu_model.clone(), "cpu_model", &mut m);
            readquantifier(u.cpu_L1_size.clone(), "cpu_L1_size", &mut m);
            readquantifier(u.cpu_L2_size.clone(), "cpu_L2_size", &mut m);
            readquantifier(u.cpu_L3_size.clone(), "cpu_L3_size", &mut m);
            readquantifier(u.cpu_core_clock.clone(), "cpu_core_clock", &mut m);
            readquantifier(u.cpu_core_count.clone(), "cpu_core_count", &mut m);
            readquantifier(u.cpu_core_threads_count.clone(), "cpu_core_threads_count", &mut m);
            readquantifier(u.cpu_count.clone(), "cpu_count", &mut m);
            readquantifier(u.cpu_tdp.clone(), "cpu_tdp", &mut m);
        }
        None => {}
    }

    match &t.memory {
        Some(u) => {
            readqualifier(u.mem_type.clone(), "mem_type", &mut m);
            readquantifier(u.mem_bandwidth.clone(), "mem_bandwidth", &mut m);
            readquantifier(u.mem_frequency.clone(), "cpmem_frequencyu_tdp", &mut m);
            readquantifier(u.mem_latency.clone(), "mem_latency", &mut m);
            readquantifier(u.mem_size.clone(), "mem_size", &mut m);
        }
        None => {}
    }

    match &t.interconnection {
        Some(u) => {
            readqualifier(u.net_RDMA.clone(), "net_rdma", &mut m);
            readqualifier(u.net_type.clone(), "net_type", &mut m);
            readqualifier(u.net_topology.clone(), "net_topology", &mut m);
            readquantifier(u.net_bandwidth.clone(), "net_bandwidth", &mut m);
            readquantifier(u.net_latency.clone(), "net_latency", &mut m);
            readquantifier(u.net_startuptime.clone(), "net_startuptime", &mut m);
        }
        None => {}
    }

    match &t.storage {
        Some(u) => {
            readqualifier(u.sto_interface.clone(), "sto_interface", &mut m);
            readqualifier(u.sto_type.clone(), "sto_type", &mut m);
            readquantifier(u.sto_bandwidth.clone(), "sto_bandwidth", &mut m);
            readquantifier(u.sto_latency.clone(), "sto_latency", &mut m);
            readquantifier(u.sto_networkbandwidth.clone(), "sto_networkbandwidth", &mut m);
            readquantifier(u.sto_size.clone(), "sto_size", &mut m);
        }
        None => {}
    }

    match &t.node {
        Some(u) => {
            readqualifier(u.vm_machinefamily.clone(), "vm_machinefamily", &mut m);
            readqualifier(u.vm_machinetype.clone(), "vm_machinetype", &mut m);
            readqualifier(u.vm_provider.clone(), "vm_provider", &mut m);
            //match u.vm_count { Some(v) => { add_quantifier(&mut m,
            // "vm_count".to_string(), v); }, None => {} }
            readquantifier(u.vm_threads_count.clone(), "vm_threads_count", &mut m);
            readquantifier(u.vm_vcpus_count.clone(), "vm_vcpus_count", &mut m);
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
