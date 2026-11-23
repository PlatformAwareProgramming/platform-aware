use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use toml::{Value, value::Array};

use configurable_features::{Feature, PlatformFeatures, PlatformParameter, QualifierFeature, lookup_feature};
pub use configurable_features::readplatform;

#[derive(Deserialize, Clone, Copy)]
#[serde(untagged)]
pub enum NumericQuantifier {
    Int(i64),
    Float(f64),
}

impl NumericQuantifier {
    pub fn to_i32_saturating(self) -> i32 {
        match self {
            NumericQuantifier::Int(v) => v.clamp(i32::MIN as i64, i32::MAX as i64) as i32,
            NumericQuantifier::Float(v) => {
                if !v.is_finite() {
                    return if v.is_sign_negative() { i32::MIN } else { i32::MAX };
                }
                v.round().clamp(i32::MIN as f64, i32::MAX as f64) as i32
            }
        }
    }
}

#[derive(Deserialize, Default)]
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
    pub acc_memorytype: Option<String>,
    pub acc_count: Option<i32>,
    pub acc_backend: Option<Array>,
    pub acc_architecture: Option<String>,
    pub acc_model: Option<String>,
    pub acc_processorcount: Option<i32>,
    pub acc_processor: Option<String>,
    pub acc_memorysize: Option<i64>,
    pub acc_tdp: Option<i32>,
    pub acc_type: Option<String>,
}

#[derive(Deserialize)]
pub struct PlatformProcessor {
    pub cpu_core_count: Option<i32>,
    pub cpu_count: Option<i32>,
    #[serde(alias = "cpu_threads_count")]
    pub cpu_core_threads_count: Option<i32>,
    pub cpu_simd: Option<Array>,
    pub cpu_L2_size: Option<i64>,
    pub cpu_L3_size: Option<i64>,
    pub cpu_L1_size: Option<i64>,
    pub cpu_core_clock: Option<i32>,
    pub cpu_manufacturer: Option<String>,
    pub cpu_isa: Option<String>,
    pub cpu_tdp: Option<i32>,
    pub cpu_model: Option<String>,
    pub cpu_microarchitecture: Option<String>,
}

#[derive(Deserialize)]
pub struct PlatformInterconnection {
    pub net_latency: Option<NumericQuantifier>,
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
    pub sto_latency: Option<NumericQuantifier>,
    pub sto_size: Option<i64>,
    pub sto_startuptime: Option<i32>,
    pub sto_networkbandwidth: Option<i32>,
    pub sto_interface: Option<String>,
}

#[derive(Deserialize)]
pub struct PlatformMemory {
    pub mem_frequency: Option<i32>,
    pub mem_size: Option<i64>,
    pub mem_latency: Option<NumericQuantifier>,
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
    let t: Platform = readplatform().unwrap_or_default();
    Mutex::new(t)
});

pub fn readqualifier(f:Option<String>, 
                         fstr:&str, 
                         m: &mut HashMap<PlatformParameter, Arc<dyn Feature /* + Send +  Sync */>>)
{
    match f {
        Some(v) => add_qualifier(m, fstr.to_string(), v.to_string()),
        None => {  }
    }
} 

pub fn readquantifier(f:Option<i32>, 
                         fstr:&str, 
                         m: &mut HashMap<PlatformParameter, Arc<dyn Feature /* + Send +  Sync */>>)
{
    match f {
        Some(v) => add_quantifier(m, fstr.to_string(), v),
        None => {  }
    }
}

fn readquantifier_i64(
    f: Option<i64>,
    fstr: &str,
    m: &mut HashMap<PlatformParameter, Arc<dyn Feature>>,
) {
    if let Some(v) = f {
        add_quantifier(
            m,
            fstr.to_string(),
            v.clamp(i32::MIN as i64, i32::MAX as i64) as i32,
        );
    }
}

fn readquantifier_num(
    f: Option<NumericQuantifier>,
    fstr: &str,
    m: &mut HashMap<PlatformParameter, Arc<dyn Feature>>,
) {
    if let Some(v) = f {
        add_quantifier(m, fstr.to_string(), v.to_i32_saturating());
    }
}


pub fn readqualifierlist(f:Option<Vec<Value>>, 
                         fstr:&str, 
                         m: &mut HashMap<PlatformParameter, Arc<dyn Feature /* + Send +  Sync */>>)
{
    if let Some(v) = f {
        let mut fvec: Vec<Arc<dyn QualifierFeature>> = Vec::new();
        for fv in v {
            if let Some(fs) = fv.as_str() {
                if let Some(feat) = lookup_feature(fs) {
                    fvec.push(feat);
                }
            }
        }
        m.insert(fstr.to_string(), Arc::new(fvec));
    }
}





pub fn readplatformfeatures() -> PlatformFeatures {
 
    let t = CURRENT_PLATFORM.lock().unwrap();

    let mut m: PlatformFeatures = ::std::collections::HashMap::new();

    match &t.accelerator {
        Some(u) => {
            readquantifier(u.acc_count, "acc_count", &mut m);
            readqualifier(u.acc_type.clone(), "acc_type", &mut m);
            readqualifierlist(u.acc_backend.clone(), "acc_backend", &mut m);
            readqualifier(u.acc_model.clone(), "acc_model", &mut m);
            readqualifier(u.acc_processor.clone(), "acc_processor", &mut m);
            readquantifier(u.acc_processorcount, "acc_processorcount", &mut m);
            readqualifier(u.acc_memorytype.clone(), "acc_memorytype", &mut m);
            readquantifier_i64(u.acc_memorysize, "acc_memorysize", &mut m);
            readqualifier(u.acc_interconnect.clone(), "acc_interconnect", &mut m);
            readquantifier(u.acc_tdp, "acc_tdp", &mut m);
            readqualifier(u.acc_architecture.clone(), "acc_architecture", &mut m);
            readqualifier(u.acc_manufacturer.clone(), "acc_manufacturer", &mut m);
        }
        None => {}
    }

    match &t.processor {
        Some(u) => {
            readqualifier(u.cpu_isa.clone(), "cpu_isa", &mut m);
            readqualifier(u.cpu_manufacturer.clone(), "cpu_manufacturer", &mut m);
            readqualifier(u.cpu_microarchitecture.clone(), "cpu_microarchitecture", &mut m);
            readqualifier(u.cpu_model.clone(), "cpu_model", &mut m);
            readquantifier_i64(u.cpu_L1_size, "cpu_L1_size", &mut m);
            readquantifier_i64(u.cpu_L2_size, "cpu_L2_size", &mut m);
            readquantifier_i64(u.cpu_L3_size, "cpu_L3_size", &mut m);
            readquantifier(u.cpu_core_clock, "cpu_core_clock", &mut m);
            readquantifier(u.cpu_core_count, "cpu_core_count", &mut m);
            readquantifier(u.cpu_core_threads_count, "cpu_core_threads_count", &mut m);
            readquantifier(u.cpu_count, "cpu_count", &mut m);
            readquantifier(u.cpu_tdp, "cpu_tdp", &mut m);
            readqualifierlist(u.cpu_simd.clone(), "cpu_simd", &mut m);
        }
        None => {}
    }

    match &t.memory {
        Some(u) => {
            readqualifier(u.mem_type.clone(), "mem_type", &mut m);
            readquantifier(u.mem_bandwidth, "mem_bandwidth", &mut m);
            readquantifier(u.mem_frequency, "mem_frequency", &mut m);
            readquantifier_num(u.mem_latency, "mem_latency", &mut m);
            readquantifier_i64(u.mem_size, "mem_size", &mut m);
        }
        None => {}
    }

    match &t.interconnection {
        Some(u) => {
            readqualifier(u.net_RDMA.clone(), "net_RDMA", &mut m);
            readqualifier(u.net_type.clone(), "net_type", &mut m);
            readqualifier(u.net_topology.clone(), "net_topology", &mut m);
            readquantifier(u.net_bandwidth, "net_bandwidth", &mut m);
            readquantifier_num(u.net_latency, "net_latency", &mut m);
            readquantifier(u.net_startuptime, "net_startuptime", &mut m);
        }
        None => {}
    }

    match &t.storage {
        Some(u) => {
            readqualifier(u.sto_interface.clone(), "sto_interface", &mut m);
            readqualifier(u.sto_type.clone(), "sto_type", &mut m);
            readquantifier(u.sto_bandwidth, "sto_bandwidth", &mut m);
            readquantifier_num(u.sto_latency, "sto_latency", &mut m);
            readquantifier(u.sto_startuptime, "sto_startuptime", &mut m);
            readquantifier(u.sto_networkbandwidth, "sto_networkbandwidth", &mut m);
            readquantifier_i64(u.sto_size, "sto_size", &mut m);
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
            readquantifier(u.vm_threads_count, "vm_threads_count", &mut m);
            readquantifier(u.vm_vcpus_count, "vm_vcpus_count", &mut m);
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
    m: &mut HashMap<PlatformParameter, Arc<dyn Feature /* + Send +  Sync */>>,
    par: PlatformParameter,
    v: String,
) {
    if let Some(f) = lookup_feature(&v) {
        m.insert(par, f);
    }
}

pub fn add_quantifier(
    m: &mut HashMap<PlatformParameter, Arc<dyn Feature /* + Send +  Sync */>>,
    par: PlatformParameter,
    v: i32,
) {
    m.insert(par, Arc::new(v));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AccMemoryType, DDR3, GDDR6, HBM2e, PlatformRegistry};

    #[test]
    fn test_listing_9_platform_toml_deserializes_without_error() {
        let listing_9 = r#"
[processor]
cpu_count = 1
cpu_core_count = 32
cpu_threads_count = 64
cpu_model = "AMD_EPYC_7513"
cpu_microarchitecture = "Milan"
cpu_isa = "ISA_x86_64_v3"
cpu_simd = ["MMX+", "SSE_4_2", "SSE_4a", "AVX2"]
cpu_L1_size = 65536
cpu_L2_size = 524288
cpu_L3_size = 134217728
cpu_core_clock = 2600
cpu_tdp = 200

[accelerator]
acc_count = 1
acc_type = "GPU"
acc_backend = ["CUDA_12_0", "Vulkan_1_3", "OpenGL_4_6", "OpenCL_3_0", "DirectX_12_0"]
acc_model = "NVIDIA_A100_80GB"
acc_processor = "GA100"
acc_processorcount = 6912
acc_memorytype = "HBM2e"
acc_memorysize = 80
acc_interconnect = "PCIe_4_0_x16"
acc_tdp = 300
acc_cudacc = 80
acc_cudatoolkit = 110
acc_cudadriver = 11

[memory]
mem_type = "DDR4"
mem_size = 527643940
mem_latency = 0.3
mem_bandwidth = 3200
mem_frequency = 3200

[storage]
sto_type = "SATA_SSD"
sto_interface = "PCIe_4_0_x16"
sto_size = 125522610176
sto_latency = 0
sto_bandwidth = 2133
sto_startuptime = 2400

[kernel]
problemclass = "Class_A"
"#;

        let parsed: Platform = toml::from_str(listing_9).expect("Listing 9 TOML must deserialize");
        let proc = parsed.processor.as_ref().unwrap();
        assert_eq!(proc.cpu_core_threads_count, Some(64));
        assert_eq!(proc.cpu_core_clock, Some(2600));
        assert_eq!(proc.cpu_tdp, Some(200));

        let mem = parsed.memory.as_ref().unwrap();
        assert_eq!(mem.mem_frequency, Some(3200));
        assert_eq!(mem.mem_latency.unwrap().to_i32_saturating(), 0);

        let sto = parsed.storage.as_ref().unwrap();
        assert_eq!(sto.sto_size, Some(125522610176));
        assert_eq!(sto.sto_startuptime, Some(2400));
    }

    #[test]
    fn test_acc_memory_type_subtyping_and_table_3_parameters() {
        assert!(HBM2e.feature_obj().subtypeof(&AccMemoryType.feature_obj()));
        assert!(GDDR6.feature_obj().subtypeof(&AccMemoryType.feature_obj()));
        assert!(DDR3.feature_obj().subtypeof(&AccMemoryType.feature_obj()));

        let tops = PlatformRegistry::top_features();
        assert_eq!(tops.get("cpu_L1_size").unwrap().string(), "atleast 0");
        assert_eq!(tops.get("cpu_L2_size").unwrap().string(), "atleast 0");
        assert_eq!(tops.get("cpu_L3_size").unwrap().string(), "atleast 0");
        assert_eq!(tops.get("cpu_core_clock").unwrap().string(), "atleast 0");
        assert_eq!(
            tops.get("cpu_tdp").unwrap().string(),
            format!("atmost {}", i32::MAX)
        );
        assert_eq!(
            tops.get("net_latency").unwrap().string(),
            format!("atmost {}", i32::MAX)
        );
        assert_eq!(
            tops.get("net_startuptime").unwrap().string(),
            format!("atmost {}", i32::MAX)
        );
        assert!(tops.contains_key("net_RDMA"));
        assert_eq!(
            tops.get("sto_latency").unwrap().string(),
            format!("atmost {}", i32::MAX)
        );
        assert_eq!(
            tops.get("sto_startuptime").unwrap().string(),
            format!("atmost {}", i32::MAX)
        );
        assert_eq!(
            tops.get("mem_latency").unwrap().string(),
            format!("atmost {}", i32::MAX)
        );
        assert_eq!(tops.get("mem_frequency").unwrap().string(), "atleast 0");
    }
}
