use platform_aware_features::{AccArchitecture, create_feature_hierarchy};


// Root
create_feature_hierarchy! {
    register_features_amdgpu_arch_1 ; "acc_architecture" :
    AccArchitecture :> AMDGPUArchitecture;
}

// Top-level architecture families under AMDGPUArchitecture
create_feature_hierarchy! {
    register_features_amdgpu_arch_2 ; "acc_architecture" :
    AMDGPUArchitecture :> CDNA & RDNA & GCN & TeraScale;
}

// ==================================================================
// CDNA (CDNA1, CDNA2, CDNA3, CDNA4)
// ==================================================================

create_feature_hierarchy! {
    register_features_amdgpu_arch_cdna_1 ; "acc_architecture" :
    CDNA :> CDNA_1_0 :> CDNA_2_0 :> CDNA_3_0 :> CDNA_4_0;
}

// Aliases (Julia `const`)
pub type CDNA1 = CDNA_1_0;
pub type CDNA2 = CDNA_2_0;
pub type CDNA3 = CDNA_3_0;
pub type CDNA4 = CDNA_4_0;

// ==================================================================
// RDNA (RDNA1, RDNA2, RDNA3, RDNA3.5, RDNA4)
// ==================================================================

create_feature_hierarchy! {
    register_features_amdgpu_arch_rdna_1 ; "acc_architecture" :
    RDNA :> RDNA_1_0 :> RDNA_2_0 :> RDNA_3_0  :> RDNA_3_5 :> RDNA_4_0;
}

// Aliases (Julia `const`)
pub type RDNA1 = RDNA_1_0;
pub type RDNA2 = RDNA_2_0;
pub type RDNA3 = RDNA_3_0;

// Common alias choices for “3.5” in Rust identifiers:
pub type RDNA35 = RDNA_3_5;
pub type RDNA3_5 = RDNA_3_5;
pub type RDNA4 = RDNA_4_0;

// ==================================================================
// GCN (unchanged)
// ==================================================================

create_feature_hierarchy! {
    register_features_amdgpu_arch_gcn_1 ; "acc_architecture" :
    GCN :> GCN_1_0 :> GCN_2_0 :> GCN_3_0 :> GCN_4_0 :> GCN_5_0 :> GCN_5_1;
}


// Aliases (Julia `const`)
// NOTE: the original Julia snippet has a typo: `const CGN1 = GCN_1_0`.
// We follow the exported intent (`GCN1`) here.
pub type GCN1 = GCN_1_0;
pub type GCN2 = GCN_2_0;
pub type GCN3 = GCN_3_0;
pub type GCN4 = GCN_4_0;
pub type GCN5 = GCN_5_0;

// Domain aliases
pub type Polaris = GCN4;
pub type Vega = GCN5;
pub type Vega20 = GCN_5_1;

// ==================================================================
// TeraScale (unchanged)
// ==================================================================

create_feature_hierarchy! {
    register_features_amdgpu_arch_terascale_1 ; "acc_architecture" :
    TeraScale :> TeraScale_1_0 :> TeraScale_2_0 :> TeraScale_3_0;
}

// Aliases (Julia `const`)
pub type TeraScale1 = TeraScale_1_0;
pub type TeraScale2 = TeraScale_2_0;
pub type TeraScale3 = TeraScale_3_0;