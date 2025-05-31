
use std::path::Path;
use std::sync::Arc;
use std::{env, fs};
use std::error::Error;

use serde::Deserialize;
use toml::value::Array;

#[derive(Deserialize)]
pub struct Platform {
    pub accelerator:Option<PlatformAccelerator>,
    pub processor:Option<PlatformProcessor>,
    pub interconnection:Option<PlatformInterconnection>,
    pub storage:Option<PlatformStorage>,
    pub memory:Option<PlatformMemory>,
    pub node:Option<PlatformNode>
}

#[derive(Deserialize)]
pub struct PlatformAccelerator {
    pub acc_manufacturer:Option<String>,
    pub acc_memory_type:Option<String>,
    pub acc_count:Option<i32>,
    pub acc_backend:Option<Array>,
    pub acc_architecture:Option<String>,
    pub acc_model:Option<String>,
    pub acc_processor_count:Option<i32>,
    pub acc_processor:Option<String>,
    pub acc_memory_size:Option<i32>,
    pub acc_tdp:Option<i32>,
    pub acc_type:Option<String>,
}

#[derive(Deserialize)]
pub struct PlatformProcessor {
    pub cpu_core_count:Option<i32>,
    pub cpu_count:Option<i32>,
    pub cpu_core_threads_count:Option<i32>,
    pub cpu_simd:Option<Array>,
    pub cpu_L2_size:Option<i32>,
    pub cpu_L3_size:Option<i32>,
    pub cpu_L1_size:Option<i32>,
    pub cpu_core_clock:Option<i32>,
    pub cpu_manufacturer:Option<String>,
    pub cpu_isa:Option<String>,
    pub cpu_tdp:Option<i32>,
    pub cpu_model:Option<String>,
    pub cpu_microarchitecture:Option<String>,
}

#[derive(Deserialize)]
pub struct PlatformInterconnection {
    pub net_latency:Option<i32>,
    pub net_type:Option<String>,
    pub net_bandwidth:Option<i32>,
    pub net_startuptime:Option<i32>,
    pub net_RDMA:Option<String>,
    pub net_topology:Option<String>,    
}

#[derive(Deserialize)]
pub struct PlatformStorage {
    pub sto_bandwidth:Option<i32>,
    pub sto_type:Option<String>,
    pub sto_latency:Option<i32>,
    pub sto_size:Option<i32>,
    pub sto_networkbandwidth:Option<i32>,
    pub sto_interface:Option<String>,
}

#[derive(Deserialize)]
pub struct PlatformMemory {
    pub mem_frequency:Option<i32>,
    pub mem_size:Option<i32>,
    pub mem_latency:Option<i32>,
    pub mem_type:Option<String>,
    pub mem_bandwidth:Option<i32>
}    

#[derive(Deserialize)]
pub struct PlatformNode {
    pub vm_dedicated:Option<bool>,
    pub vm_count:Option<i32>,
    pub vm_virtual:Option<bool>,
    pub vm_machinetype:Option<String>,
    pub vm_threads_count:Option<i32>,
    pub vm_machinefamily:Option<String>,
    pub vm_provider:Option<String>,
    pub vm_vcpus_count:Option<i32>,
}

pub fn readplatform() -> Result<Platform,Box<dyn Error>> {

    let platform_path = match env::var("PLATFORM_DESCRIPTION") {
        Ok(var) => var,
        Err(_) => env::var("PWD")?
    };

    let contents: String = fs::read_to_string(Path::new(&platform_path).join("Platform.toml"))?;

    Ok(toml::from_str(&contents).unwrap())
}
use std::collections::HashMap;


use crate::{lookup_feature, AtLeast, FeatureType, QuantifierType};

use super::{Feature, PlatformParameters};


// TODO: automatic read of the platform file at the beginning of the execution
//use lazy_static::lazy_static;
//use std::sync::Mutex;
// lazy_static! { static ref currentfeatures: Mutex<HashMap<PlatformParameters, &'static dyn Feature>> = Mutex::new(readplatformfeatures()); }

pub fn readplatformfeatures() -> HashMap<PlatformParameters, Arc<dyn Feature + Send + Sync>>
{
    let t = match readplatform() {
        Ok(t) => t,
        Err(_) => todo!(),
    };

   // let v0 = t.processor.unwrap();
   // let v1 = v0.cpu_simd.unwrap();
   // println!("ACCELERATOR {}", v1[0]);

    let mut m: HashMap<PlatformParameters, Arc<dyn Feature + Send + Sync>> = HashMap::new();

   // println!("readplatformfeatures at startup"); 

    match t.accelerator {
        Some(u) => {
            match u.acc_backend { Some(v ) => { 

                let mut fvec = Vec::new();
                for fv in v {
                    let fs = fv.as_str().unwrap();
                    let static_fs: &'static str = Box::leak(fs.to_string().into_boxed_str());
                    match  lookup_feature(static_fs) {
                        Some(f) =>  fvec.push(f),
                        None => { println!("qualifier {} not found and ignored", static_fs) },
                    }; 
                   
                }

              m.insert(PlatformParameters::acc_backend, Arc::new(fvec));

            }, None => {} }
            match u.acc_architecture { Some(v) => add_qualifier(&mut m, PlatformParameters::acc_architecture, v), None => {} }
            match u.acc_memory_type { Some(v) => add_qualifier(&mut m, PlatformParameters::acc_memory_type, v), None => {} }
            match u.acc_manufacturer { Some(v) => add_qualifier(&mut m, PlatformParameters::acc_manufacturer, v), None => {} }
            match u.acc_model { Some(v) =>  add_qualifier(&mut m, PlatformParameters::acc_model, v), None => {} }
            match u.acc_processor { Some(v) => add_qualifier(&mut m, PlatformParameters::acc_processor, v), None => {} }
            match u.acc_type { Some(v) => add_qualifier(&mut m, PlatformParameters::acc_type, v), None => {} }
            match u.acc_count { Some(v) => { add_quantifier(&mut m, PlatformParameters::acc_count, v); }, None => {} }
            match u.acc_memory_size { Some(v) => { add_quantifier(&mut m, PlatformParameters::acc_architecture, v); }, None => {} }
            match u.acc_processor_count { Some(v) => { add_quantifier(&mut m, PlatformParameters::acc_processor_count, v); }, None => {} }
            match u.acc_tdp { Some(v) => { add_quantifier(&mut m, PlatformParameters::acc_tdp, v); }, None => {} }
        },
        None => {},
    }

    match t.processor {
        Some(u) => {
            match u.cpu_isa { Some(v) => add_qualifier(&mut m, PlatformParameters::cpu_isa, v), None => {} }
            match u.cpu_manufacturer { Some(v) => add_qualifier(&mut m, PlatformParameters::cpu_manufacturer, v), None => {} }
            match u.cpu_microarchitecture { Some(v) => add_qualifier(&mut m, PlatformParameters::cpu_microarchitecture, v), None => {} }
            match u.cpu_model { Some(v) => add_qualifier(&mut m, PlatformParameters::cpu_model, v), None => {} }
            //match u.cpu_simd { Some(v) => add_qualifier(&mut m, PlatformParameters::cpu_simd, v), None => {} }
            match u.cpu_L1_size { Some(v) => { add_quantifier(&mut m, PlatformParameters::cpu_l1_size, v); }, None => {} }
            match u.cpu_L2_size { Some(v) => { add_quantifier(&mut m, PlatformParameters::cpu_l2_size, v); }, None => {} }
            match u.cpu_L3_size { Some(v) => { add_quantifier(&mut m, PlatformParameters::cpu_l3_size, v); }, None => {} }
            match u.cpu_core_clock { Some(v) => { add_quantifier(&mut m, PlatformParameters::cpu_clock, v); }, None => {} }
            match u.cpu_core_count { Some(v) => { add_quantifier(&mut m, PlatformParameters::cpu_core_count, v); }, None => {} }
            match u.cpu_core_threads_count { Some(v) => { add_quantifier(&mut m, PlatformParameters::cpu_core_threads_count, v); }, None => {} }
            match u.cpu_count { Some(v) => { add_quantifier(&mut m, PlatformParameters::cpu_count, v); }, None => {} }
            match u.cpu_tdp { Some(v) => { add_quantifier(&mut m, PlatformParameters::cpu_tdp, v); }, None => {} }
        },
        None => {},
    }

    match t.memory {
        Some(u) => {
            match u.mem_type { Some(v) => add_qualifier(&mut m, PlatformParameters::mem_type, v), None => {} }
            match u.mem_bandwidth { Some(v) => { add_quantifier(&mut m, PlatformParameters::mem_bandwidth, v); }, None => {} }
            match u.mem_frequency { Some(v) => { add_quantifier(&mut m, PlatformParameters::mem_frequency, v); }, None => {} }
            match u.mem_latency { Some(v) => { add_quantifier(&mut m, PlatformParameters::mem_latency, v); }, None => {} }
            match u.mem_size { Some(v) => { add_quantifier(&mut m, PlatformParameters::mem_size, v); }, None => {} }
        },
        None => {},
    }

    match t.interconnection {
        Some(u) => {
            match u.net_RDMA { Some(v) => add_qualifier(&mut m, PlatformParameters::net_rdma, v), None => {} }
            match u.net_type { Some(v) => add_qualifier(&mut m, PlatformParameters::net_type, v), None => {} }
            match u.net_topology { Some(v) => add_qualifier(&mut m, PlatformParameters::net_topology, v), None => {} }
            match u.net_bandwidth { Some(v) => { add_quantifier(&mut m, PlatformParameters::net_bandwidth, v); }, None => {} }
            match u.net_latency { Some(v) => { add_quantifier(&mut m, PlatformParameters::net_latency, v); }, None => {} }
            match u.net_startuptime { Some(v) => { add_quantifier(&mut m, PlatformParameters::net_startup_time, v); }, None => {} }
        },
        None => {},
    }

    match t.storage {
        Some(u) => {
            match u.sto_interface { Some(v) => add_qualifier(&mut m, PlatformParameters::sto_interface, v), None => {} }
            match u.sto_type { Some(v) => add_qualifier(&mut m, PlatformParameters::sto_type, v), None => {} }
            match u.sto_bandwidth { Some(v) => { add_quantifier(&mut m, PlatformParameters::sto_bandwidth, v); }, None => {} }
            match u.sto_latency { Some(v) => { add_quantifier(&mut m, PlatformParameters::sto_latency, v); }, None => {} }
            match u.sto_networkbandwidth { Some(v) => { add_quantifier(&mut m, PlatformParameters::sto_network_bandwidth, v); }, None => {} }
            match u.sto_size { Some(v) => { add_quantifier(&mut m, PlatformParameters::sto_size, v); }, None => {} }
        },
        None => {},
    }

    match t.node {
        Some(u) => {
            match u.vm_machinefamily { Some(v) => add_qualifier(&mut m, PlatformParameters::vm_machinefamily, v), None => {} }
            match u.vm_machinetype { Some(v) => add_qualifier(&mut m, PlatformParameters::vm_machinetype, v), None => {} }
            match u.vm_provider { Some(v) => add_qualifier(&mut m, PlatformParameters::vm_provider, v), None => {} }
            //match u.vm_count { Some(v) => { add_quantifier(&mut m, PlatformParameters::vm_count, v); }, None => {} }
            match u.vm_threads_count { Some(v) => { add_quantifier(&mut m, PlatformParameters::vm_threads_count, v); }, None => {} }
            match u.vm_vcpus_count { Some(v) => { add_quantifier(&mut m, PlatformParameters::vm_vcpus_count, v); }, None => {} }
            //match u.vm_dedicated { Some(v) => { add_quantifier(&mut m, PlatformParameters::vm_dedicated, v); }, None => {} }
            //match u.vm_virtual { Some(v) => { add_quantifier(&mut m, PlatformParameters::vm_virtual, v); }, None => {} }
        },
        None => {},
    }

   // for (k,v) in &m {
   //     println!("---> {:?}: {:?}", k, match v.feature_type() { FeatureType::Quantifier => v.string(), FeatureType::Qualifier => v.string() });
   // }


  //  m.insert(PlatformParameters::acc_backend, &CUDA8);
  //  m.insert(PlatformParameters::acc_count, Arc::new(AtLeast{val: 5} ));
  //  m.insert(PlatformParameters::cpu_core_count, Arc::new(12 ));

   // let f = lookup_feature("AVX512");
   // m.insert(PlatformParameters::cpu_simd, f);

    return m;
}

fn add_qualifier(m: &mut HashMap<PlatformParameters, Arc<dyn Feature + Send + Sync>>, par:PlatformParameters, v:String) { 
    let f = lookup_feature(Box::leak(v.into_boxed_str()));
    match f {
        Some(f) => { m.insert(par, f); },
        None => {},
    }
}

fn add_quantifier(m: &mut HashMap<PlatformParameters, Arc<dyn Feature + Send + Sync>>, par:PlatformParameters, v:i32) { 
    m.insert(par, Arc::new(v));
}