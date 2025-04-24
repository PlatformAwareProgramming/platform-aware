
use std::path::Path;
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
    pub acc_api:Option<Array>,
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
    pub net_model:Option<String>,
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


use super::{Feature, PlatformParameters};


// TODO: automatic read of the platform file at the beginning of the execution
//use lazy_static::lazy_static;
//use std::sync::Mutex;
// lazy_static! { static ref currentfeatures: Mutex<HashMap<PlatformParameters, &'static dyn Feature>> = Mutex::new(readplatformfeatures()); }

// TODO: implement the read of the platform file
pub fn readplatformfeatures() -> HashMap<PlatformParameters, &'static dyn Feature>
{
    let t = match readplatform() {
        Ok(t) => t,
        Err(_) => todo!(),
    };

   // let v0 = t.processor.unwrap();
   // let v1 = v0.cpu_simd.unwrap();
   // println!("ACCELERATOR {}", v1[0]);

    let mut m: HashMap<PlatformParameters, &dyn Feature> = HashMap::new();

    println!("readplatformfeatures at startup"); 

  //  m.insert(PlatformParameters::acc_backend, &CUDA8);
    m.insert(PlatformParameters::acc_count, &4 );

    return m;
}