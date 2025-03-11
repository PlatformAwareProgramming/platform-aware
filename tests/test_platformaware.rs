
    mod mymodule {

        use platform_aware_features::{resolve, PlatformParameters};
        use std::collections::HashMap;
        use lazy_static::lazy_static;

        lazy_static! { static ref V:i32 = resolve(vec![HashMap::from(FEATURES1), HashMap::from(FEATURES2)]); }

        const FEATURES1:[(PlatformParameters, i32); 1] = [(PlatformParameters::simd,1)];
        const FEATURES2:[(PlatformParameters, i32); 2] = [(PlatformParameters::simd,1), (PlatformParameters::gpu_backend, 2)];
                
        pub fn f(x:i64, y:i64, z:i64) {    
              
            match *V{
                1 => f1(x,y,z),
                2 => f2(x,y,z),
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
            println!("second actions")        
        }
    
    }    

#[test]
fn test_macro(){

    mymodule::f(1,2,3);
   
}