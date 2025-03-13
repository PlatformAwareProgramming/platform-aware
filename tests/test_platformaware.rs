
    #[allow(unused_variables)]
    mod mymodule {

        use platform_aware_features::{avx512, cuda10, resolve, Feature, PlatformParameters};
        use std::collections::HashMap;
        use lazy_static::lazy_static;
                
        lazy_static! { static ref V:i32 = { 
                    let features1:[(PlatformParameters, &dyn Feature); 1] = [(PlatformParameters::SIMD, &avx512)];
                    let features2:[(PlatformParameters, &dyn Feature); 1] = [(PlatformParameters::ACCBackend, &cuda10)];
                    let features3:[(PlatformParameters, &dyn Feature); 2] = [(PlatformParameters::ACCCount,&2), 
                                                                             (PlatformParameters::ACCBackend, &cuda10)];
                    resolve(vec![HashMap::from(features1), HashMap::from(features2), HashMap::from(features3)])
            };
        }

        pub fn f(x:i64, y:i64, z:i64) {
            match *V{
                1 => f1(x,y,z),
                2 => f2(x,y,z),
                3 => f3(x,y,z),
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
    }    

#[test]
fn test_macro(){

    mymodule::f(1,2,3);
   
}