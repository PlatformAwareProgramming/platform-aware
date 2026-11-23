use platform_aware_features::{AccModel, create_feature_hierarchy};

// ------------------------------------------------------------
// NVIDIA GPU Models (Julia -> Rust translation, analogous notation)
// ------------------------------------------------------------

// GPU models root
create_feature_hierarchy! { register_features_model_nvidia_1 ; "acc_model" : AccModel :> NVIDIAAccelerator; }

// ------------------------------------------------------------
// GPU model series
// ------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_nvidia_tensorcore_1 ; "acc_model" :
    NVIDIAAccelerator :> NVIDIATensorCore & 
                         NVIDIATesla & 
                         NVIDIA_RTX & 
                         NVIDIA_NVS & 
                         NVIDIA_Switch & 
                         NVIDIA_P & 
                         NVIDIA_T & 
                         NVIDIATitan & 
                         NVIDIAGrid & 
                         NVIDIAQuadro & 
                         NVIDIAQuadro_Plex & 
                         NVIDIAJetson & 
                         NVIDIACmp & 
                         NVIDIAGeForce;
}

// ------------------------------------------------------------
// GPU models (Tensor Core)
// ------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_nvidia_tensorcore_2 ; "acc_model" :
    NVIDIATensorCore :> NVIDIA_L4 &
                       NVIDIA_A10 &
                       NVIDIA_A100 &
                       NVIDIA_A10G &
                       NVIDIA_A16 &
                       NVIDIA_A2 &
                       NVIDIA_A30 &
                       NVIDIA_A40 &
                       NVIDIA_H100 &
                       NVIDIA_H200 &
                       NVIDIA_H800 &
                       NVIDIA_H20 &
                       NVIDIA_L40 &
                       NVIDIA_L40S &
                       NVIDIA_GH200 &
                       NVIDIA_GH200_GraceHopper &
                       NVIDIA_B100 &
                       NVIDIA_B200 &
                       NVIDIA_GB200;
}

create_feature_hierarchy! {
    register_features_model_nvidia_a100_variants ; "acc_model" :
    NVIDIA_A100 :> NVIDIA_A100_40GB &
                   NVIDIA_A100_80GB;
}

// ------------------------------------------------------------
// GPU models (Tesla)
// ------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_nvidia_tesla_2 ; "acc_model" :
    NVIDIATesla :> NVIDIATesla_C870 &
                  NVIDIATesla_D870 &
                  NVIDIATesla_S870 &
                  NVIDIATesla_S1070 &
                  NVIDIATesla_S1075 &
                  NVIDIATesla_C1060 &
                  NVIDIATesla_C2050 &
                  NVIDIATesla_M2050 &
                  NVIDIATesla_C2070 &
                  NVIDIATesla_C2075 &
                  NVIDIATesla_M2070 &
                  NVIDIATesla_M2070Q &
                  NVIDIATesla_M2090 &
                  NVIDIATesla_S2050 &
                  NVIDIATesla_S2070 &
                  NVIDIATesla_K10 &
                  NVIDIATesla_K20 &
                  NVIDIATesla_K20X &
                  NVIDIATesla_K40 &
                  NVIDIATesla_K80 &
                  NVIDIATesla_M6 &
                  NVIDIATesla_M60 &
                  NVIDIATesla_M4 &
                  NVIDIATesla_M40 &
                  NVIDIATesla_M10 &
                  NVIDIATesla_P100 &
                  NVIDIATesla_P4 &
                  NVIDIATesla_P40 &
                  NVIDIATesla_P6 &
                  NVIDIATesla_V100 &
                  NVIDIATesla_T4 &
                  NVIDIATesla_A100 &
                  NVIDIATesla_A40 &
                  NVIDIATesla_A10 &
                  NVIDIATesla_A16 &
                  NVIDIATesla_A30 &
                  NVIDIATesla_A2 &
                  NVIDIATesla_H100 &
                  NVIDIATesla_P10 &
                  NVIDIATesla_PG500_216 &
                  NVIDIATesla_PG503_216 &
                  NVIDIATesla_V100S;
}

// ------------------------------------------------------------
// GPU models (RTX)
// ------------------------------------------------------------


create_feature_hierarchy! {
    register_features_model_nvidia_rtx_2 ; "acc_model" :
    NVIDIA_RTX :> NVIDIA_RTX_A1000 &
                 NVIDIA_RTX_A2000 &
                 NVIDIA_RTX_A3000 &
                 NVIDIA_RTX_A4 &
                 NVIDIA_RTX_A4000 &
                 NVIDIA_RTX_A4500 &
                 NVIDIA_RTX_A500 &
                 NVIDIA_RTX_A5000 &
                 NVIDIA_RTX_A5500 &
                 NVIDIA_RTX_A6000 &
                 NVIDIA_RTX_4000_SFF_Ada &
                 NVIDIA_RTX_4000_Ada &
                 NVIDIA_RTX_4500_Ada &
                 NVIDIA_RTX_5000_Ada &
                 NVIDIA_RTX_5880_Ada &
                 NVIDIA_RTX_6000_Ada;
}

// ------------------------------------------------------------
// GPU models (NVS)
// ------------------------------------------------------------


create_feature_hierarchy! {
    register_features_model_nvidia_nvs_2 ; "acc_model" :
    NVIDIA_NVS :> NVIDIA_NVS_810;
}


// ------------------------------------------------------------
// GPU models (P)
// ------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_nvidia_p_2 ; "acc_model" :
    NVIDIA_P :> NVIDIA_P102_100 &
               NVIDIA_P102_101 &
               NVIDIA_P104_100 &
               NVIDIA_P104_101 &
               NVIDIA_P106_090 &
               NVIDIA_P106_100 &
               NVIDIA_P106M &
               NVIDIA_PG506_232 &
               NVIDIA_PG506_242;
}

// ------------------------------------------------------------
// GPU models (T)
// ------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_nvidia_t_2 ; "acc_model" :
    NVIDIA_T :> NVIDIA_T1000 &
               NVIDIA_T400 &
               NVIDIA_T500 &
               NVIDIA_T550 &
               NVIDIA_T600;
}

// ------------------------------------------------------------
// GPU models (Titan)
// ------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_nvidia_titan_2 ; "acc_model" :
    NVIDIATitan :> NVIDIATitan_RTX &
                  NVIDIATitan_V &
                  NVIDIATitan_X &
                  NVIDIATitan_Xp;
}

// ------------------------------------------------------------
// GPU models (Grid)
// ------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_nvidia_grid_2 ; "acc_model" :
    NVIDIAGrid :> NVIDIAGrid_K520 &
                 NVIDIAGrid_A100A &
                 NVIDIAGrid_A100B &
                 NVIDIAGrid_M10_8Q &
                 NVIDIAGrid_M3_3020 &
                 NVIDIAGrid_M40 &
                 NVIDIAGrid_M6_8Q &
                 NVIDIAGrid_M60_1Q &
                 NVIDIAGrid_M60_2Q &
                 NVIDIAGrid_M60_4A &
                 NVIDIAGrid_M60_8Q &
                 NVIDIAGrid_RTX_T10_16 &
                 NVIDIAGrid_RTX_T10_2 &
                 NVIDIAGrid_RTX_T10_4 &
                 NVIDIAGrid_RTX_T10_8;
}

// ------------------------------------------------------------
// GPU models (Quadro) + Quadro Plex
// ------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_nvidia_quadro_plex_1 ; "acc_model" :
    NVIDIAQuadro_Plex :> NVIDIAQuadro_2200_D2 &
                        NVIDIAQuadro_2200_S4;
}

create_feature_hierarchy! {
    register_features_model_nvidia_quadro_2 ; "acc_model" :
    NVIDIAQuadro :> NVIDIAQuadro_GP100 &
                   NVIDIAQuadro_GV100 &
                   NVIDIAQuadro_K1200 &
                   NVIDIAQuadro_K620M &
                   NVIDIAQuadro_M1000M &
                   NVIDIAQuadro_M1200 &
                   NVIDIAQuadro_M2000 &
                   NVIDIAQuadro_M2000M &
                   NVIDIAQuadro_M2200 &
                   NVIDIAQuadro_M3000 &
                   NVIDIAQuadro_M3000M &
                   NVIDIAQuadro_M4000 &
                   NVIDIAQuadro_M4000M &
                   NVIDIAQuadro_M5000 &
                   NVIDIAQuadro_M5000M &
                   NVIDIAQuadro_M500M &
                   NVIDIAQuadro_M520 &
                   NVIDIAQuadro_M5500 &
                   NVIDIAQuadro_M6000 &
                   NVIDIAQuadro_M600M &
                   NVIDIAQuadro_M620 &
                   NVIDIAQuadro_P1000 &
                   NVIDIAQuadro_P2000 &
                   NVIDIAQuadro_P2200 &
                   NVIDIAQuadro_P3000 &
                   NVIDIAQuadro_P3200 &
                   NVIDIAQuadro_P400 &
                   NVIDIAQuadro_P4000 &
                   NVIDIAQuadro_P4200 &
                   NVIDIAQuadro_P500 &
                   NVIDIAQuadro_P5000 &
                   NVIDIAQuadro_P520 &
                   NVIDIAQuadro_P5200 &
                   NVIDIAQuadro_P600 &
                   NVIDIAQuadro_P6000 &
                   NVIDIAQuadro_P620 &
                   NVIDIAQuadro_RTX_3000 &
                   NVIDIAQuadro_RTX_4000 &
                   NVIDIAQuadro_RTX_5000 &
                   NVIDIAQuadro_RTX_6000 &
                   NVIDIAQuadro_RTX_8000 &
                   NVIDIAQuadro_T1000 &
                   NVIDIAQuadro_T1200 &
                   NVIDIAQuadro_T2000;
}

// ------------------------------------------------------------
// GPU models (Jetson)
// ------------------------------------------------------------


create_feature_hierarchy! {
    register_features_model_nvidia_jetson_2 ; "acc_model" :
    NVIDIAJetson :> NVIDIAJetson_Nano &
                   NVIDIAJetson_TX1 &
                   NVIDIAJetson_TX2 &
                   NVIDIAJetson_Xavier & 
                   NVIDIAJetson_Orin_Nano &
                   NVIDIAJetson_Orin_NX &
                   NVIDIAJetson_AGX_Orin;
}

// ------------------------------------------------------------
// GPU models (Cmp)
// ------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_nvidia_cmp_2 ; "acc_model" :
    NVIDIACmp :> NVIDIACmp_170HX &
                NVIDIACmp_30HX &
                NVIDIACmp_40HX &
                NVIDIACmp_50HX &
                NVIDIACmp_70HX &
                NVIDIACmp_90HX;
}

// ------------------------------------------------------------
// GPU models (GeForce) + subfamilies
// ------------------------------------------------------------


create_feature_hierarchy! {
    register_features_model_nvidia_geforce_2 ; "acc_model" :
    NVIDIAGeForce :> NVIDIAGeForce7 &
                    NVIDIAGeForce8 &
                    NVIDIAGeForce9 &
                    NVIDIAGeForce_GT &
                    NVIDIAGeForce_MX &
                    NVIDIAGeForce_GTX &
                    NVIDIAGeForce_RTX;
}

create_feature_hierarchy! {
    register_features_model_nvidia_geforce_series_1 ; "acc_model" :
    NVIDIAGeForce_GTX :> NVIDIAGeForce_GTX7Series &
                        NVIDIAGeForce_GTX8Series &
                        NVIDIAGeForce_GTX9Series &
                        NVIDIAGeForce_GTX10Series &
                        NVIDIAGeForce_GTX16Series;
}

create_feature_hierarchy! {
    register_features_model_nvidia_geforce_series_2 ; "acc_model" :
    NVIDIAGeForce_RTX :> NVIDIAGeForce_RTX20Series &
                        NVIDIAGeForce_RTX30Series &
                        NVIDIAGeForce_RTX40Series;
}

// Note: the Julia snippet had an export typo for RTX40Series; we keep the type name as written.
create_feature_hierarchy! {
    register_features_model_nvidia_geforce_models_7_8_9_1 ; "acc_model" :
    NVIDIAGeForce7 :> NVIDIAGeForce_710A;
}

create_feature_hierarchy! {
    register_features_model_nvidia_geforce_models_8_1 ; "acc_model" :
    NVIDIAGeForce8 :> NVIDIAGeForce_810M &
                     NVIDIAGeForce_820M &
                     NVIDIAGeForce_845M;
}

create_feature_hierarchy! {
    register_features_model_nvidia_geforce_models_9_1 ; "acc_model" :
    NVIDIAGeForce9 :> NVIDIAGeForce_910M &
                     NVIDIAGeForce_920A &
                     NVIDIAGeForce_920M &
                     NVIDIAGeForce_920MX &
                     NVIDIAGeForce_930A &
                     NVIDIAGeForce_930M &
                     NVIDIAGeForce_930MX &
                     NVIDIAGeForce_940A &
                     NVIDIAGeForce_940M &
                     NVIDIAGeForce_940MX &
                     NVIDIAGeForce_945A &
                     NVIDIAGeForce_945M;
}

create_feature_hierarchy! {
    register_features_model_nvidia_geforce_models_gt_1 ; "acc_model" :
    NVIDIAGeForce_GT :> NVIDIAGeForce_GT_1010 &
                       NVIDIAGeForce_GT_1030 &
                       NVIDIAGeForce_GT_610 &
                       NVIDIAGeForce_GT_710 &
                       NVIDIAGeForce_GT_720 &
                       NVIDIAGeForce_GT_730 &
                       NVIDIAGeForce_GT_740;
}

create_feature_hierarchy! {
    register_features_model_nvidia_geforce_models_gtx7_1 ; "acc_model" :
    NVIDIAGeForce_GTX7Series :> NVIDIAGeForce_GTX_750 &
                               NVIDIAGeForce_GTX_760;
}

create_feature_hierarchy! {
    register_features_model_nvidia_geforce_models_gtx8_1 ; "acc_model" :
    NVIDIAGeForce_GTX8Series :> NVIDIAGeForce_GTX_860M;
}

create_feature_hierarchy! {
    register_features_model_nvidia_geforce_models_gtx9_1 ; "acc_model" :
    NVIDIAGeForce_GTX9Series :> NVIDIAGeForce_GTX_950 &
                               NVIDIAGeForce_GTX_950A &
                               NVIDIAGeForce_GTX_950M &
                               NVIDIAGeForce_GTX_960 &
                               NVIDIAGeForce_GTX_960A &
                               NVIDIAGeForce_GTX_960M &
                               NVIDIAGeForce_GTX_965M &
                               NVIDIAGeForce_GTX_980 &
                               NVIDIAGeForce_GTX_980MX;
}

create_feature_hierarchy! {
    register_features_model_nvidia_geforce_models_gtx10_1 ; "acc_model" :
    NVIDIAGeForce_GTX10Series :> NVIDIAGeForce_GTX_1050 &
                                NVIDIAGeForce_GTX_1060 &
                                NVIDIAGeForce_GTX_1070 &
                                NVIDIAGeForce_GTX_1080;
}

create_feature_hierarchy! {
    register_features_model_nvidia_geforce_models_gtx16_1 ; "acc_model" :
    NVIDIAGeForce_GTX16Series :> NVIDIAGeForce_GTX_1630 &
                                NVIDIAGeForce_GTX_1650 &
                                NVIDIAGeForce_GTX_1660;
}

create_feature_hierarchy! {
    register_features_model_nvidia_geforce_models_gfx_misc_1 ; "acc_model" :
    NVIDIAGeForce_GTX :> NVIDIAGeForce_GTX_TITAN_X;
}

create_feature_hierarchy! {
    register_features_model_nvidia_geforce_models_mx_1 ; "acc_model" :
    NVIDIAGeForce_MX :> NVIDIAGeForce_MX110 &
                       NVIDIAGeForce_MX130 &
                       NVIDIAGeForce_MX150 &
                       NVIDIAGeForce_MX230 &
                       NVIDIAGeForce_MX250 &
                       NVIDIAGeForce_MX330 &
                       NVIDIAGeForce_MX350 &
                       NVIDIAGeForce_MX450 &
                       NVIDIAGeForce_MX550 &
                       NVIDIAGeForce_MX570;
}

create_feature_hierarchy! {
    register_features_model_nvidia_geforce_models_mx570_1 ; "acc_model" :
    NVIDIAGeForce_MX570 :> NVIDIAGeForce_MX570A;
}

create_feature_hierarchy! {
    register_features_model_nvidia_geforce_models_rtx20_1 ; "acc_model" :
    NVIDIAGeForce_RTX20Series :> NVIDIAGeForce_RTX_2050 &
                                NVIDIAGeForce_RTX_2060 & NVIDIAGeForce_RTX_2060_12GB &
                                NVIDIAGeForce_RTX_2070 &
                                NVIDIAGeForce_RTX_2080;
}

create_feature_hierarchy! {
    register_features_model_nvidia_geforce_models_rtx30_1 ; "acc_model" :
    NVIDIAGeForce_RTX30Series :> NVIDIAGeForce_RTX_3050 &
                                NVIDIAGeForce_RTX_3060 &
                                NVIDIAGeForce_RTX_3070 & 
                                NVIDIAGeForce_RTX_3070Ti &
                                NVIDIAGeForce_RTX_3080 & 
                                NVIDIAGeForce_RTX_3080Ti &
                                NVIDIAGeForce_RTX_3090 & 
                                NVIDIAGeForce_RTX_3050Ti &
                                NVIDIAGeForce_RTX_3080_12GB;
}

create_feature_hierarchy! {
    register_features_model_nvidia_geforce_models_rtx40_1 ; "acc_model" :
    NVIDIAGeForce_RTX40Series :> NVIDIAGeForce_RTX_4060 &
                                NVIDIAGeForce_RTX_4070 &
                                NVIDIAGeForce_RTX_4080 &
                                NVIDIAGeForce_RTX_4090 & 
                                NVIDIAGeForce_RTX_4050 &
                                NVIDIAGeForce_RTX_4060Ti &
                                NVIDIAGeForce_RTX_4070Ti &
                                NVIDIAGeForce_RTX_4070Ti_SUPER &
                                NVIDIAGeForce_RTX_4080_SUPER &
                                NVIDIAGeForce_RTX_4090D;
}