# The platform-aware crate
Structured platform-aware programming for Rust.

This is an experimental crate that supports the authors' research on structured platform-aware programming, in which Rust is one of the target languages.

It is the Rust equivalent of the [PlatformAware.jl](https://github.com/PlatformAwareProgramming/PlatformAware.jl) package for Julia.

## How it works

In Rust, platform-aware programming can be useful for both system and high-performance computing (HPC) programming. 

In the context of HPC, let `my_kernel_function` be a function with high computational requirements, i.e., a significant portion of the application's overall execution time is spent executing this function. 

Functions like `my_kernel_function` are called _kernel functions_.

We want to offer multiple versions of `my_kernel_function` based on assumptions about the execution platform's features, to maximize its performance.

This is a possible implementation using the features of the **platform-aware** crate:

```
#[platformaware]
mod my_kernel_module {  

   #[kernelversion] // fallback version (no platform arguments)
   fn my_kernel_function( ... ) { ... } // v0

   #[kernelversion(cpu_simd=AVX512, cpu_core_count=AtMost{val=4})]
   fn my_kernel_function( ... ) { ... } // v1

   #[kernelversion(cpu_simd=AVX512, cpu_core_count=AtLeast{val=8})]
   fn my_kernel_function( ... ) { ... } // v2

   #[kernelversion(acc_model=NVIDIA_H200)]
   fn my_kernel_function( ... ) { ... } // v3

   #[kernelversion(acc_count=AtLeast{val=2}, acc_backend=CUDA, acc_cudacc=AtLeast{val=90})]
   fn my_kernel_function( ... ) { ... } // v4

   #[kernelversion(acc_backend=CUDA, acc_cudacc=AtLeast{val=80})]
   fn my_kernel_function( ... ) { ... } // v5

   #[kernelversion(acc_count=2, acc_model=AMD_MI325X)]
   fn my_kernel_function( ... ) { ... } // v6
}
```

The `platformaware` macro introduces a _platform module_. It processes a module in which kernel functions with multiple versions are declared. 

Each kernel version has a _platform type_, defined by the macro-like annotation `kernelversion`, which includes parameters, each encoding an assumption. In the examples, we call the kernel versions ***v0***, ***v1***, ***v2***, ***v3***, ***v4***, ***v5***, and ***v6***. 

An assumption is an association of an _assumption type_ (e.g., `CUDA`, `AVX512`, and `AtLeast{val=8}`) with a parameter name (e.g., `acc_backend`, `cpu_simd`, and `cpu_core_count`).

The first version of `my_kernel_function` makes no assumptions. It is the fallback (or default) version, selected when the platform type of any kernel version matches the features of the underlying execution platform. 

It is recommended that the fallback version be declared first. Indeed, it is recommended to order the versions from most generic to most specialized, as the resolution algorithm will examine the kernel version in reverse order. This imposes a priority rule across kernel versions, resolving ambiguity when the platform type of multiple kernel functions matches the actual platform type, i.e., the assumptions of multiple kernel versions are satisfied by the execution platform's features.

### How the resolution algorithm works

The description of the execution platform's features is stored in a _platform description file_, called _Platform.toml_. 

Now, assume that the following entries are declared in a hypothetical _Platform.toml_:

* `acc_model = "NVIDIA H100"`
* `acc_backend = "CUDA"`
* `acc_computecapability = 100`  #10.0
* `acc_count = 4` 
* `cpu_core_count = 8` 
* `cpu_simd = "AVX-512"`

For an execution platform with four NVIDIA H100 GPUs and an 8-core CPU with AVX-512 support, the platform types ***v0***, ***v2***, ***v4***, and ***v5*** are compatible. They are all candidates for selection. The algorithm will first select ***v5***. Note that this is possible because the platform's four H100 GPUs support CUDA compute capability 10.0. Therefore, due to backward compatibility, it supports code written for 8.0, assuming ***v5*** for `acc_computecapability`. However, ***v4*** makes a stronger assumption than ***v5***, i.e., its platform type is a subtype of the platform type of ***v5***, since compute capability 9.0 is closer to the compute capability of H100 GPUs than 8.0 (`AtLeast{val=100}` <: `AtLeast{val=90}` <: `AtLeast{val=80}`). Additionally, regarding the assumption about `acc_count`, ***v4*** can use two of the four GPUs available on the platform, whereas ***v5*** can use only one. Then, ***v2*** is discarded because it is not a subtype of ***v4***, as it makes no assumptions about GPUs. Finally, since ***v0***'s platform type cannot be a subtype of any other platform type, ***v4*** is the selected k-function version.

### Kernel versions in trait implementations

In a platform module, it is also possible to declare kernel functions within trait implementations: 

```
#[platformaware]
mod my_kernel_module {  
   ...
   impl MyTrait for MyDataType {  

      #[kernelversion] // fallback version (no platform arguments)
      fn my_kernel_function(self:Self, ... ) { ... } // v0

      #[kernelversion(cpu_simd=AVX512, cpu_core_count=AtMost{val=4})]
      fn my_kernel_function(self:Self, ... ) { ... } // v1

      #[kernelversion(cpu_simd=AVX512, cpu_core_count=AtLeast{val=4})]
      fn my_kernel_function(self:Self, ... ) { ... } // v2

      #[kernelversion(acc_model=NVIDIA_H200)]
      fn my_kernel_function(self:Self, ... ) { ... } // v3

      #[kernelversion(acc_count=AtLeast{val=2}, acc_backend=CUDA, acc_cudacc=AtLeast{val=90})]
      fn my_kernel_function(self:Self, ... ) { ... } // v4

      #[kernelversion(acc_backend=CUDA, acc_cudacc=AtLeast{val=80})]
      fn my_kernel_function(self:Self, ... ) { ... } // v5

      #[kernelversion(acc_count=2, acc_model=AMD_MI325X)]
      fn my_kernel_function(self:Self, ... ) { ... } // v6
   }
}
```

## Publications

Francisco Heron de Carvalho-Junior, José Mykael Nogueira, and João Marcelo Uchôa de Alencar. 2025. Structured platform-aware programming for Rust. In Proceedings of the XXIX Simpósio Brasileiro de Linguagens de Programação (SBLP'2025), september 22, 2025, Recife/PE, Brazil. SBC, Porto Alegre, Brazil, 51-58. DOI: https://doi.org/10.5753/sblp.2025.11166.






