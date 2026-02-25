use platform_aware_features::{AccModel, create_feature_hierarchy};

// ------------------------------------------------------------
// Intel GPU Models (analogous notation to NVIDIA file)
// ------------------------------------------------------------

// GPU models root
create_feature_hierarchy! {
    register_features_model_intel_1 ; "acc_model" :
    AccModel :> IntelAccelerator;
}

// ------------------------------------------------------------
// GPU model series (high-level families)
// ------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_intel_series_1 ; "acc_model" :
    IntelAccelerator :> IntelArc &
                       IntelArcPro &
                       IntelDataCenterGPU &
                       IntelIris &
                       IntelUHD &
                       IntelHD;
}

// ------------------------------------------------------------
// Intel Arc (consumer) + subfamilies
// ------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_intel_arc_series_1 ; "acc_model" :
    IntelArc :> IntelArc_A_Series &
               IntelArc_B_Series;
}

// Arc A-Series (desktop + mobile)
create_feature_hierarchy! {
    register_features_model_intel_arc_a_series_1 ; "acc_model" :
    IntelArc_A_Series :> IntelArc_A_Desktop &
                        IntelArc_A_Mobile;
}

create_feature_hierarchy! {
    register_features_model_intel_arc_a_desktop_1 ; "acc_model" :
    IntelArc_A_Desktop :> IntelArc_A770 &
                         IntelArc_A750 &
                         IntelArc_A580 &
                         IntelArc_A380 &
                         IntelArc_A310 &
                         IntelArc_A380E;
}

// Official Intel support/ARK lists these mobile parts; includes A530M/A570M additions.
create_feature_hierarchy! {
    register_features_model_intel_arc_a_mobile_1 ; "acc_model" :
    IntelArc_A_Mobile :> IntelArc_A770M &
                        IntelArc_A730M &
                        IntelArc_A570M &
                        IntelArc_A550M &
                        IntelArc_A530M &
                        IntelArc_A380M &
                        IntelArc_A370M &
                        IntelArc_A350M;
}

// Arc B-Series (Battlemage) – launched models
create_feature_hierarchy! {
    register_features_model_intel_arc_b_series_1 ; "acc_model" :
    IntelArc_B_Series :> IntelArc_B580 &
                        IntelArc_B570;
}

// ------------------------------------------------------------
// Intel Arc Pro (workstation) + subfamilies
// ------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_intel_arc_pro_series_1 ; "acc_model" :
    IntelArcPro :> IntelArcPro_A_Series &
                  IntelArcPro_B_Series;
}

// Arc Pro A-Series (workstation)
create_feature_hierarchy! {
    register_features_model_intel_arc_pro_a_series_1 ; "acc_model" :
    IntelArcPro_A_Series :> IntelArcPro_A40 &
                           IntelArcPro_A50 &
                           IntelArcPro_A60 &
                           IntelArcPro_A60M;
}

// Arc Pro B-Series (workstation, Battlemage)
create_feature_hierarchy! {
    register_features_model_intel_arc_pro_b_series_1 ; "acc_model" :
    IntelArcPro_B_Series :> IntelArcPro_B50 &
                           IntelArcPro_B60;
}

// ------------------------------------------------------------
// Intel Data Center GPUs + subfamilies
// ------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_intel_dcgpu_series_1 ; "acc_model" :
    IntelDataCenterGPU :> IntelDataCenterGPU_Max &
                         IntelDataCenterGPU_Flex;
}

// Data Center GPU Max Series (Ponte Vecchio / Xe-HPC)
create_feature_hierarchy! {
    register_features_model_intel_dcgpu_max_1 ; "acc_model" :
    IntelDataCenterGPU_Max :> IntelDataCenterGPU_Max_1100 &
                             IntelDataCenterGPU_Max_1350 &
                             IntelDataCenterGPU_Max_1550;
}

// Data Center GPU Flex Series (media / VDI / inference)
create_feature_hierarchy! {
    register_features_model_intel_dcgpu_flex_1 ; "acc_model" :
    IntelDataCenterGPU_Flex :> IntelDataCenterGPU_Flex_140 &
                              IntelDataCenterGPU_Flex_170;
}

// ------------------------------------------------------------
// Intel Iris / UHD / HD (integrated families; keep coarse-grained)
// ------------------------------------------------------------

// Iris (umbrella; you can extend with more specific subfamilies if needed)
create_feature_hierarchy! {
    register_features_model_intel_iris_1 ; "acc_model" :
    IntelIris :> IntelIrisXe &
                IntelIrisPlus &
                IntelIrisPro;
}

// UHD Graphics (umbrella)
create_feature_hierarchy! {
    register_features_model_intel_uhd_1 ; "acc_model" :
    IntelUHD :> IntelUHD_600_Series &
               IntelUHD_700_Series &
               IntelUHD_7000_Series;
}

// HD Graphics (umbrella)
create_feature_hierarchy! {
    register_features_model_intel_hd_1 ; "acc_model" :
    IntelHD :> IntelHD_2000_Series &
              IntelHD_3000_Series &
              IntelHD_4000_Series &
              IntelHD_5000_Series;
}

// ------------------------------------------------------------
// Optional: expand Iris Xe / UHD / HD with a few representative models
// ------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_intel_iris_xe_models_1 ; "acc_model" :
    IntelIrisXe :> IntelIrisXe_MAX;
}

// UHD examples
create_feature_hierarchy! {
    register_features_model_intel_uhd_models_1 ; "acc_model" :
    IntelUHD_600_Series :> IntelUHD_620 &
                          IntelUHD_630;
}

create_feature_hierarchy! {
    register_features_model_intel_uhd_models_2 ; "acc_model" :
    IntelUHD_700_Series :> IntelUHD_730 &
                          IntelUHD_750 &
                          IntelUHD_770;
}

// HD examples
create_feature_hierarchy! {
    register_features_model_intel_hd_models_1 ; "acc_model" :
    IntelHD_4000_Series :> IntelHD_4400 &
                          IntelHD_4600;
}

create_feature_hierarchy! {
    register_features_model_intel_hd_models_2 ; "acc_model" :
    IntelHD_5000_Series :> IntelHD_520 &
                          IntelHD_530;
}