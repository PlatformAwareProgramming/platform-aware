use platform_aware_features::{Feature, CPUSIMD, AVX512};


    #[allow(unused_variables)]
    mod mymodule {

        use platform_aware_features::{resolve, AtMost, Feature, PlatformParameters,AVX512};
        use platform_aware_nvidia::CUDA8;
        use std::collections::HashMap;
        use lazy_static::lazy_static;
                
        lazy_static! { static ref V:i32 = { 
                    let features1:[(PlatformParameters, &dyn Feature); 1] = [(PlatformParameters::cpu_simd, &AVX512)];
                    let features2:[(PlatformParameters, &dyn Feature); 2] = [(PlatformParameters::acc_backend, &CUDA8),
                                                                             (PlatformParameters::acc_count, &3)];
                    let features3:[(PlatformParameters, &dyn Feature); 1] = [(PlatformParameters::acc_backend, &CUDA8)];
                    let features4:[(PlatformParameters, &dyn Feature); 2] = [(PlatformParameters::acc_backend, &CUDA8),
                                                                             (PlatformParameters::acc_count, &AtMost{val:5})];
                    resolve(vec![HashMap::from(features1), 
                                 HashMap::from(features2), 
                                 HashMap::from(features3), 
                                 HashMap::from(features4)])
            };
        }

        pub fn f(x:i64, y:i64, z:i64) {
            match *V{
                1 => f1(x,y,z),
                2 => f2(x,y,z),
                3 => f3(x,y,z),
                4 => f4(x,y,z),
                _ => f0(x,y,z),
            }
        }
    
        // fall-back
        fn f0(x:i64, y:i64, z:i64) {
            println!("fall-back actions")
        }
    
        fn f1(x:i64, y:i64, z:i64) {
            println!("first version actions")
        }
    
        fn f2(x:i64, y:i64, z:i64) {
            println!("second version actions")        
        }
    
        fn f3(x:i64, y:i64, z:i64) {
            println!("third version actions")        
        }

        fn f4(x:i64, y:i64, z:i64) {
            println!("fourth version actions")        
        }
    }    

    use platform_aware_macros::platformaware;

//    #[allow(unused_imports)]
    #[platformaware(f)]
    mod mymodule2 {
    
       // default version (fallback)
       #[kernelversion]
       fn f(x:i64, y:i64, z:i64) {
         println!("fall-back actions x={x} y={y} z={z}") 
       }
    
       #[kernelversion(cpu_simd=AVX512)]
       fn f(x:i64, y:i64, z:i64) {
         println!("first version actions x={x} y={y} z={z}") 
       }
    
       #[kernelversion(acc_model=NVidiaH100)]
       fn f(x:i64, y:i64, z:i64) {
         println!("second version actions x={x} y={y} z={z}") 
       }
    
       #[kernelversion(acc_count=2, acc_backend=CUDA10)]
       fn f(x:i64, y:i64, z:i64) {
         println!("third version actions x={x} y={y} z={z}") 
       }
    
       #[kernelversion(acc_count=4)]
       fn f(x:i64, y:i64, z:i64) {
         println!("fourth version actions x={x} y={y} z={z}") 
       }
   
    }
    

#[test]
fn test_macro(){

    mymodule2::f(1,2,3);
   
}