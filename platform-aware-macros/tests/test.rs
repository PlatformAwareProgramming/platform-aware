

mod imports {
    #[test]
    fn import_platformaware_macro(){
        #[allow(unused_imports)]
        use platform_aware_macros::platformaware;
    }
}

mod uses {
    use platform_aware_macros::platformaware;

    #[should_panic(expected = "not yet implemented")]
    #[test]
    fn basic() {
        use platform_aware_macros::platformaware;

        #[platformaware(foo, bar)]
        mod mymodule {
            //fallback
            #[kernelversion]
            pub fn foo(x: i32, y: i32, z: i32) {
                println!("fallback foo");
            }

            #[kernelversion(CPUSIMD=AVX512, CPUSIMD=AVX512)]
            pub fn foo(x: i32, y: i32, z: i32) {
                println!("1 foo");
            }

            #[kernelversion(CPUSIMD=AVX512, ACCBackend=CUDA8)]
            pub fn foo(x: i32, y: i32, z: i32) {
                println!("2 foo");
            }

            #[kernelversion(CPUSIMD=AVX512, ACCBackend=CUDA10)]
            pub fn foo(x: i32, y: i32, z: i32) {
                println!("3 foo");
            }

            #[kernelversion]
            pub fn bar() {
                println!("fallback bar");
            }
        }

        mymodule::foo(1,2,3);
        mymodule::bar();
    }

    #[should_panic(expected = "not yet implemented")]
    #[test]
    fn basic_types() {
        #[platformaware(foo, bar, foobar)]
        mod mymodule {
            use std::print;

            use platform_aware_macros::kernelversion;
        
            //fallback
            #[kernelversion]
            pub fn foo(x: i32, y: i32, z: i32) {
                println!("fallback foo");
            }
        
            #[kernelversion]
            pub fn bar(s: &str) {
                println!("fallback bar");
            }

            #[kernelversion]
            pub fn foobar() {
                println!("fallback foobar");
            }

            #[kernelversion(CPUSIMD=AVX512)]
            pub fn foo(x: i32, y: i32, z: i32) {
               println!("foo 1");
            }

            #[kernelversion(ACCBackend=CUDA10, ACCCount=2)]
            pub fn bar(s: &str) {
                println!("fallback bar");
            }

            #[kernelversion(CPUSIMD=AVX512)]
            pub fn foobar() {
                println!("foobar 1");
            }

            #[kernelversion(ACCBackend=CUDA10)]
            pub fn buzz(a: &mut u8) {
                println!("fallback buzz");
            }
        }

        mymodule::foo(1,2,3);
        mymodule::bar("ok");
        mymodule::foobar();
    }

    #[should_panic(expected = "not yet implemented")]
    #[test]
    fn only_fallback() {
        #[platformaware(foo, bar, foobar)]
        mod mymodule {
            use platform_aware_macros::kernelversion;
        
            //fallback
            #[kernelversion]
            pub fn foo(x: i32, y: i32, z: i32) {
                println!("fallback foo");
            }
        
            #[kernelversion]
            pub fn bar(s: &str) {
                println!("fallback bar");
            }

            #[kernelversion]
            pub fn foobar() {
                println!("fallback foobar");
            }
        }

        mymodule::foo(1,2,3);
        mymodule::bar("ok");
        mymodule::foobar();
    }
}