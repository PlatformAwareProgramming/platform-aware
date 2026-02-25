use platform_aware_features::{AccArchitecture, create_feature_hierarchy};

// ------------------------------------------------------------
// NVIDIA GPU Architectures (Julia -> Rust translation, analogous notation)
// ------------------------------------------------------------

// Architecture root
create_feature_hierarchy! { register_features_arch_nvidia_arch ; "acc_architecture" : AccArchitecture :> NVIDIAArchitecture; }

// Linear architecture progression (as in the Julia hierarchy)
create_feature_hierarchy! {
    register_features_arch_nvidia_2 ; "acc_architecture" :
    NVIDIAArchitecture :> Farenheit :> 
                          Celsius :> 
                          Kelvin :> 
                          Rankine :> 
                          Curie :> 
                          Tesla :> 
                          Tesla2 :> 
                          Fermi :> 
                          Kepler :> 
                          Kepler2 :> 
                          Maxwell :> 
                          Maxwell2 :> 
                          Pascal :> 
                          Volta :> 
                          Turing :> 
                          Ampere :> 
                          Ada :> 
                          Hopper :>
                          Blackwell; 
}

