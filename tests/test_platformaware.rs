#[allow(unused_imports)]
use platform_aware::{Feature, CPUSIMD, AVX512};

#[allow(unused)]
mod mymodule {
  
    use platform_aware::{resolve, AtMost, Feature, PlatformParameter,AVX512};
    use platform_aware_nvidia::CUDA;
    use std::{collections::HashMap, sync::Arc};
    use lazy_static::lazy_static;
            
    lazy_static! { static ref Vf:i32 = { 
                let features1:[(PlatformParameter, Arc<dyn Feature /* + Send +  Sync */>); 1] = [("cpu_simd".to_string(), Arc::new(AVX512))];
                let features2:[(PlatformParameter, Arc<dyn Feature /* + Send +  Sync */>); 2] = [("acc_backend".to_string(), Arc::new(CUDA)),
                                                                                          ("acc_count".to_string(), Arc::new(3))];
                let features3:[(PlatformParameter, Arc<dyn Feature /* + Send +  Sync */>); 1] = [("acc_backend".to_string(), Arc::new(CUDA))];
                let features4:[(PlatformParameter, Arc<dyn Feature /* + Send +  Sync */>); 2] = [("acc_backend".to_string(), Arc::new(CUDA)),
                                                                            ("acc_count".to_string(), Arc::new(AtMost{val:5}))];
                resolve(vec![HashMap::from(features1), 
                                HashMap::from(features2), 
                                HashMap::from(features3), 
                                HashMap::from(features4)])

        };
    }

    lazy_static! {
        static ref Vg: i32 = {
            let features1: [(PlatformParameter, Arc<dyn Feature /* + Send +  Sync */>); 1] =
                [("cpu_simd".to_string(), Arc::new(AVX512))];
            let features2: [(PlatformParameter, Arc<dyn Feature /* + Send +  Sync */>); 2] = [
                ("acc_backend".to_string(), Arc::new(CUDA)),
                ("acc_count".to_string(), Arc::new(3)),
            ];
            let features3: [(PlatformParameter, Arc<dyn Feature /* + Send +  Sync */>); 1] =
                [("acc_backend".to_string(), Arc::new(CUDA))];
            let features4: [(PlatformParameter, Arc<dyn Feature /* + Send +  Sync */>); 2] = [
                ("acc_backend".to_string(), Arc::new(CUDA)),
                ("acc_count".to_string(), Arc::new(AtMost { val: 5 })),
            ];
            resolve(vec![
                HashMap::from(features1),
                HashMap::from(features2),
                HashMap::from(features3),
                HashMap::from(features4),
            ])
        };
    }

    pub fn f(x: i64, y: i64, z: i64) -> i64 {
        match *Vf {
            1 => f1(x, y, z),
            2 => f2(x, y, z),
            3 => f3(x, y, z),
            4 => f4(x, y, z),
            _ => f0(x, y, z),
        }
    }

    // fall-back
    fn f0(x: i64, y: i64, z: i64) -> i64 {
        println!("fall-back actions");
        1i64
    }

    fn f1(x: i64, y: i64, z: i64) -> i64 {
        println!("first version actions");
        2i64
    }

    fn f2(x: i64, y: i64, z: i64) -> i64 {
        println!("second version actions");
        3i64
    }

    fn f3(x: i64, y: i64, z: i64) -> i64 {
        println!("third version actions");
        4i64
    }

    fn f4(x: i64, y: i64, z: i64) -> i64 {
        println!("fourth version actions");
        5i64
    }

    pub fn g(x: i64, y: i64, z: i64) -> i64 {
        match *Vg {
            1 => g1(x, y, z),
            2 => g2(x, y, z),
            3 => g3(x, y, z),
            4 => g4(x, y, z),
            _ => g0(x, y, z),
        }
    }

    // fall-back
    fn g0(x: i64, y: i64, z: i64) -> i64 {
        println!("fall-back actions");
        1i64
    }

    fn g1(x: i64, y: i64, z: i64) -> i64 {
        println!("first version actions");
        2i64
    }

    fn g2(x: i64, y: i64, z: i64) -> i64 {
        println!("second version actions");
        3i64
    }

    fn g3(x: i64, y: i64, z: i64) -> i64 {
        println!("third version actions");
        4i64
    }

    fn g4(x: i64, y: i64, z: i64) -> i64 {
        println!("fourth version actions");
        5i64
    }
}

//use platform_aware_macros::platformaware;

use platform_aware::platformaware;

#[platformaware]
mod mymodule2 {
   // use platform_aware::*;
   use platform_aware::{AtLeast, AVX512};
    use platform_aware_nvidia::{CUDA30, CUDA20, CUDA50, CUDA90, NVIDIA_H100};

    // default version (fallback)
    #[kernelversion]
    pub fn f(x: i64, y: i64, z: i64) -> i64 {
        println!("fall-back actions x={x} y={y} z={z}");
        1i64
    }

    #[kernelversion(cpu_simd=AVX512)]
    pub fn f(x: i64, y: i64, z: i64) -> i64 {
        println!("first version actions x={x} y={y} z={z}");
        2i64
    }

    #[kernelversion(acc_model=NVIDIA_H100)]
    pub fn f(x: i64, y: i64, z: i64) -> i64 {
        println!("second version actions x={x} y={y} z={z}");
        3i64
    }

    #[kernelversion(acc_count=AtLeast{val:4}, acc_backend=CUDA30 , acc_cudacc=AtLeast{val:50})]
    pub fn f(x: i64, y: i64, z: i64) -> i64 {
        println!("third version actions x={x} y={y} z={z}");
        4i64
    }

    #[kernelversion(acc_count=AtLeast{val:4})]
    pub fn f(x: i64, y: i64, z: i64) -> i64 {
        println!("fourth version actions x={x} y={y} z={z}");
        5i64
    }
}

#[test]
fn test_macro() {
    let r  = mymodule2::f(1, 2, 3);
    assert_eq!(r, 4i64)
}

#[platformaware]
mod taxonomy_dispatch_mod {
    use platform_aware::{CPUISA, ISA_x86_64_v3};
    use platform_aware_intel::{IntelCore, KabyLake};
    use platform_aware_nvidia::NVIDIA;

    #[kernelversion]
    pub fn check_platform() -> &'static str {
        "fallback"
    }

    #[kernelversion(cpu_isa = CPUISA)]
    pub fn check_platform() -> &'static str {
        "generic_isa"
    }

    #[kernelversion(
        cpu_model = IntelCore,
        cpu_microarchitecture = KabyLake,
        cpu_isa = ISA_x86_64_v3,
        acc_manufacturer = NVIDIA
    )]
    pub fn check_platform() -> &'static str {
        "intel_core_kabylake_nvidia"
    }
}

#[test]
fn test_platform_toml_resolves_vendor_taxonomies_without_ctor_race() {
    assert_eq!(
        taxonomy_dispatch_mod::check_platform(),
        "intel_core_kabylake_nvidia"
    );
}

pub trait ExternalVecKernel {
    fn sum_scaled(&self, factor: f64) -> f64;
}

pub trait TraitA {
    type Output;
    fn compute<const K: usize>(&self, x: i64) -> Self::Output;
}

pub trait TraitB {
    fn compute(&self, x: i64) -> i64;
}

pub struct DualTarget {
    pub base: i64,
}

#[platformaware]
mod trait_extensibility_mod {
    use super::{DualTarget, ExternalVecKernel, TraitA, TraitB};
    use platform_aware::AVX512;
    use platform_aware_nvidia::CUDA;

    // Implementing a k-method trait for an external std type (Vec<f64>) —
    // must compile without orphan-rule E0116 errors.
    impl ExternalVecKernel for Vec<f64> {
        #[kernelversion]
        fn sum_scaled(&self, factor: f64) -> f64 {
            self.iter().map(|v| v * factor).sum()
        }
    }

    #[platform]
    mod cpu_simd {
        impl ExternalVecKernel for Vec<f64> {
            #[kernelversion(cpu_simd = AVX512)]
            fn sum_scaled(&self, factor: f64) -> f64 {
                self.iter().map(|v| v * factor).sum::<f64>() + 100.0
            }
        }
    }

    // Two distinct traits with a homonymous k-method `compute` on the same type —
    // must compile without E0592 duplicate definition errors.
    impl TraitA for DualTarget {
        type Output = i64;

        #[kernelversion]
        fn compute<const K: usize>(&self, x: i64) -> Self::Output {
            self.base + x + K as i64
        }

        #[kernelversion(cpu_simd = AVX512)]
        fn compute<const K: usize>(&self, x: i64) -> Self::Output {
            (self.base + x) * K as i64
        }
    }

    impl TraitB for DualTarget {
        #[kernelversion]
        fn compute(&self, x: i64) -> i64 {
            self.base - x
        }

        #[kernelversion(acc_backend = CUDA)]
        fn compute(&self, x: i64) -> i64 {
            self.base + x * 10
        }
    }
}

#[test]
fn test_k_methods_support_external_types_and_avoid_cross_trait_collisions() {
    let data = vec![1.0_f64, 2.0, 3.0];
    // Platform.toml includes AVX512 -> selects the #[platform] cpu_simd variant (+ 100.0)
    assert_eq!(data.sum_scaled(2.0), 112.0);

    let target = DualTarget { base: 5 };
    // TraitA selects AVX512 variant: (5 + 3) * 4 = 32
    assert_eq!(TraitA::compute::<4>(&target, 3), 32);
    // TraitB selects CUDA variant (Platform.toml has CUDA50 <: CUDA): 5 + 3 * 10 = 35
    assert_eq!(TraitB::compute(&target, 3), 35);
}
