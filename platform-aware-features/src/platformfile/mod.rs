// all unused imports are used inside the readplatformfeatures macro
use std::path::Path;
#[allow(unused_imports)]
use std::sync::Arc;
use std::{env, fs};
use std::error::Error;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use crate::{lookup_feature, AtLeast, FeatureType, QuantifierType};
#[allow(unused_imports)]
use super::{Feature, PlatformParameter};

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
    pub acc_cudacc:Option<i32>,
    pub acc_cudatoolkit:Option<i32>,
    pub acc_cudadriver:Option<i32>,
    pub acc_architecture:Option<String>,
    pub acc_model:Option<String>,
    pub acc_processor_count:Option<i32>,
    pub acc_processor:Option<String>,
    pub acc_memorysize:Option<i32>,
    pub acc_tdp:Option<i32>,
    pub acc_type:Option<String>,
}

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
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
        Err(_) => {
            eprintln!("PLATFORM_DESCRIPTION var not found");

            env::var("PWD")?
        }
    };

    // println!("{:?}", Path::new(&platform_path).join("Platform.toml"));

    let contents: String = fs::read_to_string(Path::new(&platform_path).join("Platform.toml"))?;

    Ok(toml::from_str(&contents).unwrap())
}

#[macro_export]
macro_rules! process_features {
    ($struct:expr, $map:expr, {
        $( $field:ident => ($param:path, $func:ident) ),*
        $(,)?
    }) => {
        let u = $struct;
        $(
            if let Some(v) = u.$field {
                $func($map, $param, v);
            }
        )*
    };
}

#[macro_export]
macro_rules! readplatformfeatures {
    ($platform:expr) => { 
        todo!("not implemented for extern platform crates yet")
    };
    // default impl
    () => {{
        type PlatformFeatures = HashMap<PlatformParameter, Arc<dyn Feature + Send + Sync>>;

        fn add_qualifier(m: &mut PlatformFeatures, par:PlatformParameter, v:String) { 
            let f = $crate::lookup_feature(Box::leak(v.into_boxed_str()));
            match f {
                Some(f) => { m.insert(par, f); },
                None => {},
            }
        }

        fn add_quantifier(m: &mut PlatformFeatures, par:PlatformParameter, v:i32) { 
            m.insert(par, Arc::new(v));
        }

        let platform =
            match $crate::platformfile::readplatform() {
                Ok(t) => t,
                Err(_) => todo!(
                    "error while trying to read Platform.toml or PWD var wasnt found"
                ),
            };


        // let v0 = t.processor.unwrap();
        // let v1 = v0.cpu_simd.unwrap();
        // println!("ACCELERATOR {}", v1[0]);

        let mut m: PlatformFeatures = ::std::collections::HashMap::new();

        // println!("readplatformfeatures at startup"); 

        if let Some(mut feature) = platform.accelerator {
            if let Some(v) = feature.acc_backend.take() {
                let mut fvec = Vec::new();

                for fv in v {
                    let fs = fv.as_str().unwrap();
                    let static_fs: &'static str = Box::leak(
                        fs.to_string().into_boxed_str()
                    );

                    match $crate::lookup_feature(static_fs) {
                        Some(f) =>  fvec.push(f),
                        None => { println!("qualifier {} not found and ignored", static_fs) },
                    }; 
                }

                m.insert($crate::PlatformParameter::acc_backend, std::sync::Arc::new(fvec));
            }

            $crate::process_features!(feature, &mut m, {
                acc_cudacc => ($crate::PlatformParameter::acc_cudacc, add_quantifier),
                acc_cudatoolkit => ($crate::PlatformParameter::acc_cudatoolkit, add_quantifier),
                acc_cudadriver => ($crate::PlatformParameter::acc_cudadriver, add_quantifier),
                acc_architecture => ($crate::PlatformParameter::acc_architecture, add_qualifier),
                acc_memory_type => ($crate::PlatformParameter::acc_memory_type, add_qualifier),
                acc_manufacturer => ($crate::PlatformParameter::acc_manufacturer, add_qualifier),
                acc_model => ($crate::PlatformParameter::acc_model, add_qualifier),
                acc_processor => ($crate::PlatformParameter::acc_processor, add_qualifier),
                acc_type => ($crate::PlatformParameter::acc_type, add_qualifier),
                acc_count => ($crate::PlatformParameter::acc_count, add_quantifier),
                acc_memorysize => ($crate::PlatformParameter::acc_memorysize, add_quantifier),
                acc_processor_count => ($crate::PlatformParameter::acc_processor_count, add_quantifier),
                acc_tdp => ($crate::PlatformParameter::acc_tdp, add_quantifier),
            });
        }

        if let Some(u) = platform.processor {
            $crate::process_features!(u, &mut m, {
                cpu_isa => ($crate::PlatformParameter::cpu_isa, add_qualifier),
                cpu_manufacturer => ($crate::PlatformParameter::cpu_manufacturer, add_qualifier),
                cpu_microarchitecture => ($crate::PlatformParameter::cpu_microarchitecture, add_qualifier),
                cpu_model => ($crate::PlatformParameter::cpu_model, add_qualifier),
                // cpu_simd => (PlatformParameter::cpu_simd, add_qualifier),
                cpu_L1_size => ($crate::PlatformParameter::cpu_l1_size, add_quantifier),
                cpu_L2_size => ($crate::PlatformParameter::cpu_l2_size, add_quantifier),
                cpu_L3_size => ($crate::PlatformParameter::cpu_l3_size, add_quantifier),
                cpu_core_clock => ($crate::PlatformParameter::cpu_clock, add_quantifier),
                cpu_core_count => ($crate::PlatformParameter::cpu_core_count, add_quantifier),
                cpu_core_threads_count => ($crate::PlatformParameter::cpu_core_threads_count, add_quantifier),
                cpu_count => ($crate::PlatformParameter::cpu_count, add_quantifier),
                cpu_tdp => ($crate::PlatformParameter::cpu_tdp, add_quantifier),
            });
        }

        if let Some(u) = platform.memory {
            $crate::process_features!(u, &mut m, {
                mem_type => ($crate::PlatformParameter::mem_type, add_qualifier),
                mem_bandwidth => ($crate::PlatformParameter::mem_bandwidth, add_quantifier),
                mem_frequency => ($crate::PlatformParameter::mem_frequency, add_quantifier),
                mem_latency  => ($crate::PlatformParameter::mem_latency, add_quantifier),
                mem_size  => ($crate::PlatformParameter::mem_size, add_quantifier),
            });
        }

        if let Some(u) = platform.interconnection {
            $crate::process_features!(u, &mut m, {
                net_RDMA => ($crate::PlatformParameter::net_rdma, add_qualifier),
                net_type=> ($crate::PlatformParameter::net_type, add_qualifier),
                net_topology => ($crate::PlatformParameter::net_topology, add_qualifier),
                net_bandwidth => ($crate::PlatformParameter::net_bandwidth, add_quantifier),
                net_latency => ($crate::PlatformParameter::net_latency, add_quantifier),
                net_startuptime=> ($crate::PlatformParameter::net_startup_time, add_quantifier),
            });
        }

        if let Some(u) = platform.storage {
            $crate::process_features!(u, &mut m, {
                sto_interface => ($crate::PlatformParameter::sto_interface, add_qualifier),
                sto_type => ($crate::PlatformParameter::sto_type, add_qualifier),
                sto_bandwidth => ($crate::PlatformParameter::sto_bandwidth, add_quantifier),
                sto_latency => ($crate::PlatformParameter::sto_latency, add_quantifier),
                sto_networkbandwidth => ($crate::PlatformParameter::sto_network_bandwidth, add_quantifier),
                sto_size=> ($crate::PlatformParameter::sto_size, add_quantifier),
            });
        }

        if let Some(u) = platform.node {
            $crate::process_features!(u, &mut m, {
                vm_machinefamily => ($crate::PlatformParameter::vm_machinefamily, add_qualifier),
                vm_machinetype => ($crate::PlatformParameter::vm_machinetype, add_qualifier),
                vm_provider => ($crate::PlatformParameter::vm_provider, add_qualifier),
                //vm_count => (PlatformParameter::vm_count, add_quantifier),
                vm_threads_count=> ($crate::PlatformParameter::vm_threads_count, add_quantifier),
                vm_vcpus_count => ($crate::PlatformParameter::vm_vcpus_count, add_quantifier),
                //vm_dedicated => Some(v) => { add_quantifier(&mut m, PlatformParameter::vm_dedicated, v); }, None => {} }
                //vm_virtual => Some(v) => { add_quantifier(&mut m, PlatformParameter::vm_virtual, v); }, None => {} }
            });
        }

        // for (k,v) in &m {
        //     println!("---> {:?}: {:?}", k, match v.feature_type() { FeatureType::Quantifier => v.string(), FeatureType::Qualifier => v.string() });
        // }


        //  m.insert(PlatformParameter::acc_backend, &CUDA8);
        //  m.insert(PlatformParameter::acc_count, Arc::new(AtLeast{val: 5} ));
        //  m.insert(PlatformParameter::cpu_core_count, Arc::new(12 ));

        // let f = lookup_feature("AVX512");
        // m.insert(PlatformParameter::cpu_simd, f);

        m
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readplatformfeatures_hashmap_shouldnt_be_empty() {
        // hardcoded for now but would be nice to use dotenvy for tests
        env::set_var("PLATFORM_DESCRIPTION", "D:/coding/platform-aware/");

        assert!(!readplatformfeatures!().is_empty());
    }

    #[ignore]
    #[test]
    fn readplatformfeatures_by_value() {
        todo!()
    }

    #[test]
    fn readplatformfeatures_print_params() {
        env::set_var(
            "PLATFORM_DESCRIPTION", "/home/mykael/coding/platformaware/platform-aware"
        );

        let plat = readplatformfeatures!();

        for i in plat.keys().into_iter() {
            println!("{:?}", i);
        }
    }
}