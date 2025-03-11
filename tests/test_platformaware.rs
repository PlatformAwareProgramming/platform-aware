
    #[allow(unused_variables)]
    mod mymodule {

        use platform_aware_features::{resolve, PlatformParameters};
        use std::collections::HashMap;
        use lazy_static::lazy_static;

        const FEATURES1:[(PlatformParameters, i32); 1] = [(PlatformParameters::SIMD,1)];
        const FEATURES2:[(PlatformParameters, i32); 1] = [(PlatformParameters::ACCBackend, 2)];
        const FEATURES3:[(PlatformParameters, i32); 2] = [(PlatformParameters::ACCCount,1), (PlatformParameters::ACCBackend, 2)];
                
        lazy_static! { static ref V:i32 = resolve(vec![HashMap::from(FEATURES1), HashMap::from(FEATURES2), HashMap::from(FEATURES3)]); }

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