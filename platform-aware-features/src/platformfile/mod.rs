
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
        Err(_) => {
            eprintln!("PLATFORM_DESCRIPTION var not found");

            env::var("PWD")?
        }
    };

    println!("{:?}", Path::new(&platform_path).join("Platform.toml"));

    let contents: String = fs::read_to_string(Path::new(&platform_path).join("Platform.toml"))?;

    Ok(toml::from_str(&contents).unwrap())
}
use std::collections::HashMap;


use crate::{lookup_feature, AtLeast, FeatureType, QuantifierType};

use super::{Feature, PlatformParameters};

pub mod __private {
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

    pub(crate) use process_features;
}

// TODO: automatic read of the platform file at the beginning of the execution
//use lazy_static::lazy_static;
//use std::sync::Mutex;
// lazy_static! { static ref currentfeatures: Mutex<HashMap<PlatformParameters, &'static dyn Feature>> = Mutex::new(readplatformfeatures()); }

#[macro_export]
macro_rules! readplatformfeatures {
    ($platform:expr) => { 
        todo!("not implemented for extern platform crates yet")
    };
    // default impl
    () => {{
        type PlatformFeatures = HashMap<PlatformParameters, Arc<dyn Feature + Send + Sync>>;

        fn add_qualifier(m: &mut PlatformFeatures, par:PlatformParameters, v:String) { 
            let f = $crate::lookup_feature(Box::leak(v.into_boxed_str()));
            match f {
                Some(f) => { m.insert(par, f); },
                None => {},
            }
        }

        fn add_quantifier(m: &mut PlatformFeatures, par:PlatformParameters, v:i32) { 
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

                m.insert($crate::PlatformParameters::acc_backend, std::sync::Arc::new(fvec));
            }

            $crate::__private::process_features!(feature, &mut m, {
                acc_cudacc => ($crate::PlatformParameters::acc_cudacc, add_quantifier),
                acc_cudatoolkit => ($crate::PlatformParameters::acc_cudatoolkit, add_quantifier),
                acc_cudadriver => ($crate::PlatformParameters::acc_cudadriver, add_quantifier),
                acc_architecture => ($crate::PlatformParameters::acc_architecture, add_qualifier),
                acc_memory_type => ($crate::PlatformParameters::acc_memory_type, add_qualifier),
                acc_manufacturer => ($crate::PlatformParameters::acc_manufacturer, add_qualifier),
                acc_model => ($crate::PlatformParameters::acc_model, add_qualifier),
                acc_processor => ($crate::PlatformParameters::acc_processor, add_qualifier),
                acc_type => ($crate::PlatformParameters::acc_type, add_qualifier),
                acc_count => ($crate::PlatformParameters::acc_count, add_quantifier),
                acc_memorysize => ($crate::PlatformParameters::acc_memorysize, add_quantifier),
                acc_processor_count => ($crate::PlatformParameters::acc_processor_count, add_quantifier),
                acc_tdp => ($crate::PlatformParameters::acc_tdp, add_quantifier),
            });
        }

        if let Some(u) = platform.processor {
            $crate::__private::process_features!(u, &mut m, {
                cpu_isa => ($crate::PlatformParameters::cpu_isa, add_qualifier),
                cpu_manufacturer => ($crate::PlatformParameters::cpu_manufacturer, add_qualifier),
                cpu_microarchitecture => ($crate::PlatformParameters::cpu_microarchitecture, add_qualifier),
                cpu_model => ($crate::PlatformParameters::cpu_model, add_qualifier),
                // cpu_simd => (PlatformParameters::cpu_simd, add_qualifier),
                cpu_L1_size => ($crate::PlatformParameters::cpu_l1_size, add_quantifier),
                cpu_L2_size => ($crate::PlatformParameters::cpu_l2_size, add_quantifier),
                cpu_L3_size => ($crate::PlatformParameters::cpu_l3_size, add_quantifier),
                cpu_core_clock => ($crate::PlatformParameters::cpu_clock, add_quantifier),
                cpu_core_count => ($crate::PlatformParameters::cpu_core_count, add_quantifier),
                cpu_core_threads_count => ($crate::PlatformParameters::cpu_core_threads_count, add_quantifier),
                cpu_count => ($crate::PlatformParameters::cpu_count, add_quantifier),
                cpu_tdp => ($crate::PlatformParameters::cpu_tdp, add_quantifier),
            });
        }

        if let Some(u) = platform.memory {
            $crate::__private::process_features!(u, &mut m, {
                mem_type => ($crate::PlatformParameters::mem_type, add_qualifier),
                mem_bandwidth => ($crate::PlatformParameters::mem_bandwidth, add_quantifier),
                mem_frequency => ($crate::PlatformParameters::mem_frequency, add_quantifier),
                mem_latency  => ($crate::PlatformParameters::mem_latency, add_quantifier),
                mem_size  => ($crate::PlatformParameters::mem_size, add_quantifier),
            });
        }

        if let Some(u) = platform.interconnection {
            $crate::__private::process_features!(u, &mut m, {
                net_RDMA => ($crate::PlatformParameters::net_rdma, add_qualifier),
                net_type=> ($crate::PlatformParameters::net_type, add_qualifier),
                net_topology => ($crate::PlatformParameters::net_topology, add_qualifier),
                net_bandwidth => ($crate::PlatformParameters::net_bandwidth, add_quantifier),
                net_latency => ($crate::PlatformParameters::net_latency, add_quantifier),
                net_startuptime=> ($crate::PlatformParameters::net_startup_time, add_quantifier),
            });
        }

        if let Some(u) = platform.storage {
            $crate::__private::process_features!(u, &mut m, {
                sto_interface => ($crate::PlatformParameters::sto_interface, add_qualifier),
                sto_type => ($crate::PlatformParameters::sto_type, add_qualifier),
                sto_bandwidth => ($crate::PlatformParameters::sto_bandwidth, add_quantifier),
                sto_latency => ($crate::PlatformParameters::sto_latency, add_quantifier),
                sto_networkbandwidth => ($crate::PlatformParameters::sto_network_bandwidth, add_quantifier),
                sto_size=> ($crate::PlatformParameters::sto_size, add_quantifier),
            });
        }

        if let Some(u) = platform.node {
            $crate::__private::process_features!(u, &mut m, {
                vm_machinefamily => ($crate::PlatformParameters::vm_machinefamily, add_qualifier),
                vm_machinetype => ($crate::PlatformParameters::vm_machinetype, add_qualifier),
                vm_provider => ($crate::PlatformParameters::vm_provider, add_qualifier),
                //vm_count => (PlatformParameters::vm_count, add_quantifier),
                vm_threads_count=> ($crate::PlatformParameters::vm_threads_count, add_quantifier),
                vm_vcpus_count => ($crate::PlatformParameters::vm_vcpus_count, add_quantifier),
                //vm_dedicated => Some(v) => { add_quantifier(&mut m, PlatformParameters::vm_dedicated, v); }, None => {} }
                //vm_virtual => Some(v) => { add_quantifier(&mut m, PlatformParameters::vm_virtual, v); }, None => {} }
            });
        }

        // for (k,v) in &m {
        //     println!("---> {:?}: {:?}", k, match v.feature_type() { FeatureType::Quantifier => v.string(), FeatureType::Qualifier => v.string() });
        // }


        //  m.insert(PlatformParameters::acc_backend, &CUDA8);
        //  m.insert(PlatformParameters::acc_count, Arc::new(AtLeast{val: 5} ));
        //  m.insert(PlatformParameters::cpu_core_count, Arc::new(12 ));

        // let f = lookup_feature("AVX512");
        // m.insert(PlatformParameters::cpu_simd, f);

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
        env::set_var("PLATFORM_DESCRIPTION", "D:/coding/platform-aware/");

        let plat = readplatformfeatures!();

        for i in plat.keys().into_iter() {
            println!("{:?}", i);
        }
    }
}