use platform_aware_features::{create_feature_hierarchy, AccBackend};

// AMD's native GPU programming environment. Registering this assumption lets
// platform-aware kernel versions select a ROCm implementation through the
// same `acc_backend` parameter used by CUDA and the common GPU backends.
create_feature_hierarchy! {
    register_features_backend_amd_accelerator ; "acc_backend" :
    AccBackend :> ROCM;
}
