
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
    pub accelerator_manufacturer:Option<String>,
    pub accelerator_memory_type:Option<String>,
    pub accelerator_count:Option<i32>,
    pub accelerator_api:Option<Array>,
    pub accelerator_architecture:Option<String>,
    pub accelerator:Option<String>,
    pub accelerator_processor_count:Option<i32>,
    pub accelerator_processor:Option<String>,
    pub accelerator_memory_size:Option<i32>,
    pub accelerator_tdp:Option<i32>,
    pub accelerator_type:Option<String>,
}

#[derive(Deserialize)]
pub struct PlatformProcessor {
    pub processor_core_count:Option<i32>,
    pub processor_count:Option<i32>,
    pub processor_core_threads_count:Option<i32>,
    pub processor_simd:Option<Array>,
    pub processor_core_L2_size:Option<i32>,
    pub processor_L3_size:Option<i32>,
    pub processor_core_L1_size:Option<i32>,
    pub processor_core_clock:Option<i32>,
    pub processor_manufacturer:Option<String>,
    pub processor_isa:Option<String>,
    pub processor_tdp:Option<i32>,
    pub processor:Option<String>,
    pub processor_microarchitecture:Option<String>,
}

#[derive(Deserialize)]
pub struct PlatformInterconnection {
    pub interconnection_latency:Option<i32>,
    pub interconnection:Option<String>,
    pub interconnection_bandwidth:Option<i32>,
    pub interconnection_startuptime:Option<i32>,
    pub interconnection_RDMA:Option<String>,
    pub interconnection_topology:Option<String>,    
}

#[derive(Deserialize)]
pub struct PlatformStorage {
    pub storage_bandwidth:Option<i32>,
    pub storage_type:Option<String>,
    pub storage_latency:Option<i32>,
    pub storage_size:Option<i32>,
    pub storage_networkbandwidth:Option<i32>,
    pub storage_interface:Option<String>,
}

#[derive(Deserialize)]
pub struct PlatformMemory {
    pub node_memory_frequency:Option<i32>,
    pub node_memory_size:Option<i32>,
    pub node_memory_latency:Option<i32>,
    pub node_memory_type:Option<String>,
    pub node_memory_bandwidth:Option<i32>
}    

#[derive(Deserialize)]
pub struct PlatformNode {
    pub node_dedicated:Option<bool>,
    pub node_count:Option<i32>,
    pub node_virtual:Option<bool>,
    pub node_machinetype:Option<String>,
    pub node_threads_count:Option<i32>,
    pub node_machinefamily:Option<String>,
    pub node_provider:Option<String>,
    pub node_vcpus_count:Option<i32>,
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
   // let v1 = v0.processor_simd.unwrap();
   // println!("ACCELERATOR {}", v1[0]);

    let mut m: HashMap<PlatformParameters, &dyn Feature> = HashMap::new();

    println!("readplatformfeatures at startup"); 

  //  m.insert(PlatformParameters::acc_backend, &CUDA8);
    m.insert(PlatformParameters::acc_count, &4 );

    return m;
}