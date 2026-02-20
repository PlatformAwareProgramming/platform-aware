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
    use platform_aware_nvidia::{CUDA30, CUDA20, CUDA50, CUDA90, NVidiaH100};

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

    #[kernelversion(acc_model=NVidiaH100)]
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
