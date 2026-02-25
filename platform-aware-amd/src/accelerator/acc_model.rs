
use platform_aware_features::{AccModel, create_feature_hierarchy};

// ------------------------------------------------------------------
// Licensed under the MIT License. See LICENCE in the project root.
// ------------------------------------------------------------------

// ==================================================================
// AMD Accelerators
// ==================================================================

// Family (root)

create_feature_hierarchy! {
    register_features_model_amd_accelerator_1 ; "acc_model" :
    AccModel :> AMDAccelerator;
}

// ------------------------------------------------------------------
// Families 1 (direct children of AMDAccelerator)
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_2 ; "acc_model" :
    AMDAccelerator :> AMDRadeon &
                     AMDInstinct &
                     AMDFirePro;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_3 ; "acc_model" :
    AMDRadeon :> AMDRadeon_Vega &
                AMDRadeon_PRO &
                AMDRadeon_AI_Pro;
}

// ------------------------------------------------------------------
// Families 2 (Radeon subfamilies)
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_4 ; "acc_model" :
    AMDRadeon :> AMDRadeon_Vega2 &
                AMDRadeon_RX_6000 &
                AMDRadeon_R9 &
                AMDRadeon_R7 &
                AMDRadeon_R5 &
                AMDRadeon_HD &
                AMDRadeon_600 &
                AMDRadeon_5700 &
                AMDRadeon_5600 &
                AMDRadeon_5500 &
                AMDRadeon_5300 &
                AMDRadeon_5000M &
                AMDRadeon_500 &
                AMDRadeon_400 & 
                AMDRadeon_RX_7000;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_5 ; "acc_model" :
    AMDRadeon_Vega :> AMDRadeon_Vega_RX;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_6 ; "acc_model" :
    AMDRadeon_RX_6000 :> AMDRadeon_RX_6000S &
                        AMDRadeon_RX_6000M;
}

// ------------------------------------------------------------------
// Families 3 (deeper Radeon subfamilies)
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_7 ; "acc_model" :
    AMDRadeon_500 :> AMDRadeon_500X &
                    AMDRadeon_RX_500 &
                    AMDRadeon_RX_500X;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_8 ; "acc_model" :
    AMDRadeon_HD :> ATIRadeon_HD_5000 &
                   AMDRadeon_HD_6000 &
                   AMDRadeon_HD_7000 &
                   AMDRadeon_HD_8000M;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_9 ; "acc_model" :
    AMDRadeon_R5 :> AMDRadeon_R5_200 &
                    AMDRadeon_R5_300;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_10 ; "acc_model" :
    AMDRadeon_R7 :> AMDRadeon_R7_200 &
                    AMDRadeon_R7_300;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_11 ; "acc_model" :
    AMDRadeon_R9 :> AMDRadeon_R9_200 &
                    AMDRadeon_R9_300 &
                    AMDRadeon_R9_Fury;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_12 ; "acc_model" :
    AMDRadeon_400 :> AMDRadeon_RX_400;
}


create_feature_hierarchy! {
    register_features_model_amd_accelerator_add_2021_2 ; "acc_model" :
    AMDRadeon_RX_7000 :> AMDRadeon_RX_7000S &
                        AMDRadeon_RX_7000M &  
                        AMDRadeon_RX_7600 &
                        AMDRadeon_RX_7600_XT &
                        AMDRadeon_RX_7700 &
                        AMDRadeon_RX_7800 &
                        AMDRadeon_RX_7900 &
                        AMDRadeon_RX_7900M &
                        AMDRadeon_RX_7800M &
                        AMDRadeon_RX_7600M_XT &
                        AMDRadeon_RX_7700S &
                        AMDRadeon_RX_7600S;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_add_2021_rx7000_1 ; "acc_model" :
    AMDRadeon_RX_7900 :> AMDRadeon_RX_7900_XTX &
                        AMDRadeon_RX_7900_XT &
                        AMDRadeon_RX_7900_GRE;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_add_2021_rx7000_2 ; "acc_model" :
    AMDRadeon_RX_7800 :> AMDRadeon_RX_7800_XT;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_add_2021_rx7000_3 ; "acc_model" :
    AMDRadeon_RX_7700 :> AMDRadeon_RX_7700_XT;
}



// RX 5000M branch exists as a family too.
create_feature_hierarchy! {
    register_features_model_amd_accelerator_13 ; "acc_model" :
    AMDRadeon_5000M :> AMDRadeon_RX_5000M;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_14 ; "acc_model" :
    AMDRadeon_5300 :> AMDRadeon_RX_5300;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_15 ; "acc_model" :
    AMDRadeon_5500 :> AMDRadeon_RX_5500;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_16 ; "acc_model" :
    AMDRadeon_5600 :> AMDRadeon_RX_5600;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_17 ; "acc_model" :
    AMDRadeon_5700 :> AMDRadeon_RX_5700;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_18 ; "acc_model" :
    AMDRadeon_RX_6000 :> AMDRadeon_RX_6400 &
                        AMDRadeon_RX_6500 &
                        AMDRadeon_RX_6600 &
                        AMDRadeon_RX_6700 &
                        AMDRadeon_RX_6800 &
                        AMDRadeon_RX_6900;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_19 ; "acc_model" :
    AMDRadeon_Vega_RX :> AMDRadeon_RX_Vega;
}

// ------------------------------------------------------------------
// Radeon PRO / Instinct / FirePro families
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_20 ; "acc_model" :
    AMDRadeon_PRO :> AMDRadeon_PRO_V &
                     AMDRadeon_PRO_W6000 &
                     AMDRadeon_PRO_W6000_Mobile &
                     AMDRadeon_PRO_VII &
                     AMDRadeon_PRO_W5000 &
                     AMDRadeon_PRO_W5000_Mobile &
                     AMDRadeon_PRO_WX_x200 &
                     AMDRadeon_PRO_WX_x100 & 
                     AMDRadeon_PRO_W7000 &
                     AMDRadeon_PRO_W7900 &
                     AMDRadeon_PRO_W7800 &
                     AMDRadeon_PRO_W7600 &
                     AMDRadeon_PRO_W7500;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_21 ; "acc_model" :
    AMDInstinct :> AMDInstinct_MI;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_22 ; "acc_model" :
    AMDFirePro :> AMDFirePro_Wx100 &
                  AMDFirePro_Wx000 &
                  AMDFirePro_S &
                  AMDFirePro_R &
                  AMDFirePro_Mobility &
                  AMDFirePro_MultiView;
}

// ==================================================================
// Models
// ==================================================================

// --------------------------
// AMD Instinct (MI)
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_1 ; "acc_model" :
    AMDInstinct_MI :> AMDInstinct_MI250X &
                     AMDInstinct_MI250 &
                     AMDInstinct_MI210 &
                     AMDInstinct_MI100 &
                     AMDInstinct_MI60 &
                     AMDInstinct_MI50_32GB &
                     AMDInstinct_MI50_16GB &
                     AMDInstinct_MI25 &
                     AMDInstinct_MI8 &
                     AMDInstinct_MI6 &
                     AMDInstinct_MI300 &
                     AMDInstinct_MI350 & 
                     AMDInstinct_MI300A &
                     AMDInstinct_MI300X &
                     AMDInstinct_MI325X &
                     AMDInstinct_MI350X &
                     AMDInstinct_MI355X;
}

// --------------------------
// Radeon PRO (V / W / WX / misc)
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_2 ; "acc_model" :
    AMDRadeon_PRO_V :> AMDRadeonPRO_V_250 &
                      AMDRadeon_PRO_V620 &
                      AMDRadeon_PRO_V520;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_3 ; "acc_model" :
    AMDRadeon_PRO_W6000 :> AMDRadeon_PRO_W6800 &
                          AMDRadeon_PRO_W6600 &
                          AMDRadeon_PRO_W6400;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_4 ; "acc_model" :
    AMDRadeon_PRO_W6000_Mobile :> AMDRadeon_PRO_W6600M &
                                 AMDRadeon_PRO_W6500M &
                                 AMDRadeon_PRO_W6300M;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_5 ; "acc_model" :
    AMDRadeon_PRO_W5000 :> AMDRadeon_PRO_W5700 &
                          AMDRadeon_PRO_W5500;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_6 ; "acc_model" :
    AMDRadeon_PRO_W5000_Mobile :> AMDRadeon_PRO_W5500M;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_7 ; "acc_model" :
    AMDRadeon_PRO_WX_x200 :> AMDRadeon_PRO_WX_8200 &
                            AMDRadeon_PRO_WX_3200;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_8 ; "acc_model" :
    AMDRadeon_PRO_WX_x100 :> AMDRadeon_PRO_WX_9100 &
                            AMDRadeon_PRO_WX_7100 &
                            AMDRadeon_PRO_WX_5100 &
                            AMDRadeon_PRO_WX_4100 &
                            AMDRadeon_PRO_WX_3100 &
                            AMDRadeon_PRO_WX_2100;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_add_2021_5 ; "acc_model" :
    AMDRadeon_PRO_W7000 :> AMDRadeon_PRO_W7000_Mobile;
}

// PRO misc models that attach directly to AMDRadeon_PRO
create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_9 ; "acc_model" :
    AMDRadeon_PRO :> AMDRadeon_PRO_SSG &
                     AMDRadeon_Vega_Frontier &
                     AMDRadeon_PRO_Duo
                     ;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_add_2021_ai_pro_2 ; "acc_model":
    AMDRadeon_AI_Pro :> AMDRadeon_AI_Pro_R9700;
}

// --------------------------
// FirePro (Wx100)
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_10 ; "acc_model" :
    AMDFirePro_Wx100 :> AMDFirePro_W9100_32GB &
                       AMDFirePro_W9100 &
                       AMDFirePro_W8100 &
                       AMDFirePro_W7100 &
                       AMDFirePro_W5100 &
                       AMDFirePro_W4300 &
                       AMDFirePro_W4100 &
                       AMDFirePro_W2100;
}

// --------------------------
// FirePro (Wx000)
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_11 ; "acc_model" :
    AMDFirePro_Wx000 :> AMDFirePro_W9000 &
                       AMDFirePro_W8000 &
                       AMDFirePro_W7000 &
                       AMDFirePro_W5000 &
                       AMDFirePro_W5000_DVI &
                       AMDFirePro_W600;
}

// --------------------------
// FirePro (S)
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_12 ; "acc_model" :
    AMDFirePro_S :> AMDFirePro_S10000 &
                   AMDFirePro_S10000_12GB &
                   AMDFirePro_S9300_x2 &
                   AMDFirePro_S9170 &
                   AMDFirePro_S9150 &
                   AMDFirePro_S9100 &
                   AMDFirePro_S9050 &
                   AMDFirePro_S9000 &
                   AMDFirePro_S7150_x2 &
                   AMDFirePro_S7150 &
                   AMDFirePro_S7100X &
                   AMDFirePro_S7000 &
                   AMDFirePro_S4000X;
}

// --------------------------
// FirePro (R / Mobility / MultiView)
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_13 ; "acc_model" :
    AMDFirePro_R :> AMDFirePro_R5000;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_14 ; "acc_model" :
    AMDFirePro_Mobility :> AMDFirePro_W7170M &
                          AMDFirePro_W6150M &
                          AMDFirePro_W5170M &
                          AMDFirePro_W5130M &
                          AMDFirePro_W4190M &
                          AMDFirePro_W4170M;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_15 ; "acc_model" :
    AMDFirePro_MultiView :> AMDFirePro_2460 &
                           AMDFirePro_2270_x1 &
                           AMDFirePro_2270_1GB &
                           AMDFirePro_2270;
}

// --------------------------
// Radeon RX 6000 desktop (RX_6400/6500/6600/6700/6800/6900)
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_16 ; "acc_model" :
    AMDRadeon_RX_6900 :> AMDRadeon_RX_6950_XT &
                        AMDRadeon_RX_6900_XT;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_17 ; "acc_model" :
    AMDRadeon_RX_6800 :> AMDRadeon_RX_6800_XT;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_18 ; "acc_model" :
    AMDRadeon_RX_6700 :> AMDRadeon_RX_6750_XT &
                        AMDRadeon_RX_6700_XT;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_19 ; "acc_model" :
    AMDRadeon_RX_6600 :> AMDRadeon_RX_6650_XT &
                        AMDRadeon_RX_6600_XT;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_20 ; "acc_model" :
    AMDRadeon_RX_6500 :> AMDRadeon_RX_6500_XT &
                        AMDRadeon_RX_6500M;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_21 ; "acc_model" :
    AMDRadeon_RX_6000M :> AMDRadeon_RX_6850M_XT &
                         AMDRadeon_RX_6800M &
                         AMDRadeon_RX_6700M &
                         AMDRadeon_RX_6650M_XT &
                         AMDRadeon_RX_6650M &
                         AMDRadeon_RX_6600M &
                         AMDRadeon_RX_6300M;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_22 ; "acc_model" :
    AMDRadeon_RX_6000S :> AMDRadeon_RX_6800S &
                         AMDRadeon_RX_6700S &
                         AMDRadeon_RX_6600S;
}

// --------------------------
// Radeon RX 5000M
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_23 ; "acc_model" :
    AMDRadeon_RX_5000M :> AMDRadeon_RX_5700M &
                         AMDRadeon_RX_5600M &
                         AMDRadeon_RX_5500M &
                         AMDRadeon_RX_5300M;
}

// --------------------------
// Radeon RX 5700 / 5600 / 5500
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_24 ; "acc_model" :
    AMDRadeon_RX_5700 :> AMDRadeon_RX_5700_XT;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_25 ; "acc_model" :
    AMDRadeon_RX_5600 :> AMDRadeon_RX_5600_XT;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_26 ; "acc_model" :
    AMDRadeon_RX_5500 :> AMDRadeon_RX_5500_XT;
}

// --------------------------
// Radeon RX Vega
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_27 ; "acc_model" :
    AMDRadeon_RX_Vega :> AMDRadeon_RX_Vega_64_L &
                        AMDRadeon_RX_Vega_64 &
                        AMDRadeon_RX_Vega_56;
}

// --------------------------
// Radeon VII (Vega2)
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_28 ; "acc_model" :
    AMDRadeon_Vega2 :> AMDRadeon_VII;
}

// --------------------------
// Radeon RX 500/500X and Radeon 500/500X
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_29 ; "acc_model" :
    AMDRadeon_RX_500 :> AMDRadeon_RX_590 &
                       AMDRadeon_RX_580 &
                       AMDRadeon_RX_570 &
                       AMDRadeon_RX_560 &
                       AMDRadeon_RX_550 &
                       AMDRadeon_RX_540;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_30 ; "acc_model" :
    AMDRadeon_RX_500X :> AMDRadeon_RX_580X &
                        AMDRadeon_RX_570X &
                        AMDRadeon_RX_560X &
                        AMDRadeon_RX_550X &
                        AMDRadeon_RX_540X;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_31 ; "acc_model" :
    AMDRadeon_600 :> AMDRadeon_RX_640 &
                    AMDRadeon_630 &
                    AMDRadeon_625 &
                    AMDRadeon_620 &
                    AMDRadeon_610;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_32 ; "acc_model" :
    AMDRadeon_500X :> AMDRadeon_550X &
                     AMDRadeon_540X;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_33 ; "acc_model" :
    AMDRadeon_500 :> AMDRadeon_540 &
                    AMDRadeon_535 &
                    AMDRadeon_530 &
                    AMDRadeon_520;
}

// --------------------------
// Radeon RX 400
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_34 ; "acc_model" :
    AMDRadeon_RX_400 :> AMDRadeon_RX_480 &
                       AMDRadeon_RX_470 &
                       AMDRadeon_RX_460;
}

// --------------------------
// Radeon R9 Fury / R9 300 / R9 200
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_35 ; "acc_model" :
    AMDRadeon_R9_Fury :> AMDRadeon_R9_Fury_X &
                        AMDRadeon_R9_Nano;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_36 ; "acc_model" :
    AMDRadeon_R9_300 :> AMDRadeon_R9_390X &
                       AMDRadeon_R9_390 &
                       AMDRadeon_R9_380X &
                       AMDRadeon_R9_380 &
                       AMDRadeon_R9_M395X &
                       AMDRadeon_R9_M395 &
                       AMDRadeon_R9_M390X &
                       AMDRadeon_R9_M390 &
                       AMDRadeon_R9_M385X &
                       AMDRadeon_R9_M385 &
                       AMDRadeon_R9_M380 &
                       AMDRadeon_R9_M375X &
                       AMDRadeon_R9_M375 &
                       AMDRadeon_R9_M365X &
                       AMDRadeon_R9_M360;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_37 ; "acc_model" :
    AMDRadeon_R9_200 :> AMDRadeon_R9_295X2 &
                       AMDRadeon_R9_290X &
                       AMDRadeon_R9_290 &
                       AMDRadeon_R9_285 &
                       AMDRadeon_R9_280X &
                       AMDRadeon_R9_280 &
                       AMDRadeon_R9_270X &
                       AMDRadeon_R9_270 &
                       AMDRadeon_R9_M295X &
                       AMDRadeon_R9_M290X &
                       AMDRadeon_R9_M285X &
                       AMDRadeon_R9_M280X &
                       AMDRadeon_R9_M280 &
                       AMDRadeon_R9_M275X &
                       AMDRadeon_R9_M270X &
                       AMDRadeon_R9_M265X;
}

// --------------------------
// Radeon R7 300 / R7 200
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_38 ; "acc_model" :
    AMDRadeon_R7_300 :> AMDRadeon_R7_370 &
                       AMDRadeon_R7_360 &
                       AMDRadeon_R7_M380 &
                       AMDRadeon_R7_M375 &
                       AMDRadeon_R7_M370 &
                       AMDRadeon_R7_M365X &
                       AMDRadeon_R7_M365 &
                       AMDRadeon_R7_M360 &
                       AMDRadeon_R7_M350 &
                       AMDRadeon_R7_M340;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_39 ; "acc_model" :
    AMDRadeon_R7_200 :> AMDRadeon_R7_265 &
                       AMDRadeon_R7_260X &
                       AMDRadeon_R7_260 &
                       AMDRadeon_R7_250X &
                       AMDRadeon_R7_250 &
                       AMDRadeon_R7_240 &
                       AMDRadeon_R7_M270 &
                       AMDRadeon_R7_M265X &
                       AMDRadeon_R7_M265AE &
                       AMDRadeon_R7_M265 &
                       AMDRadeon_R7_M260X &
                       AMDRadeon_R7_M260;
}

// --------------------------
// Radeon R5 300 / R5 200
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_40 ; "acc_model" :
    AMDRadeon_R5_300 :> AMDRadeon_R5_M335X &
                       AMDRadeon_R5_M335 &
                       AMDRadeon_R5_M330 &
                       AMDRadeon_R5_M320 &
                       AMDRadeon_R5_M315;
}

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_41 ; "acc_model" :
    AMDRadeon_R5_200 :> AMDRadeon_R5_235 &
                       AMDRadeon_R5_230 &
                       AMDRadeon_R5_M255X &
                       AMDRadeon_R5_M255 &
                       AMDRadeon_R5_M240X &
                       AMDRadeon_R5_M240 &
                       AMDRadeon_R5_M230;
}

// --------------------------
// Radeon HD 8000M
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_42 ; "acc_model" :
    AMDRadeon_HD_8000M :> AMDRadeon_HD_8970M &
                         AMDRadeon_HD_8870M &
                         AMDRadeon_HD_8850M &
                         AMDRadeon_HD_8830M &
                         AMDRadeon_HD_8790M &
                         AMDRadeon_HD_8770M &
                         AMDRadeon_HD_8750M &
                         AMDRadeon_HD_8730M &
                         AMDRadeon_HD_8690M &
                         AMDRadeon_HD_8670M &
                         AMDRadeon_HD_8590M &
                         AMDRadeon_HD_8570M;
}

// --------------------------
// Radeon HD 7000
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_43 ; "acc_model" :
    AMDRadeon_HD_7000 :> AMDRadeon_HD_7990 &
                        AMDRadeon_HD_7970_GE &
                        AMDRadeon_HD_7970 &
                        AMDRadeon_HD_7950 &
                        AMDRadeon_HD_7870_GE &
                        AMDRadeon_HD_7850 &
                        AMDRadeon_HD_7790 &
                        AMDRadeon_HD_7770_GE &
                        AMDRadeon_HD_7750 &
                        AMDRadeon_HD_7730;
}

// --------------------------
// Radeon HD 6000
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_44 ; "acc_model" :
    AMDRadeon_HD_6000 :> AMDRadeon_HD_6970 &
                        AMDRadeon_HD_6950 &
                        AMDRadeon_HD_6870 &
                        AMDRadeon_HD_6850 &
                        AMDRadeon_HD_6770 &
                        AMDRadeon_HD_6750 &
                        AMDRadeon_HD_6670 &
                        AMDRadeon_HD_6570 &
                        AMDRadeon_HD_6450;
}

// --------------------------
// ATI Radeon HD 5000
// --------------------------

create_feature_hierarchy! {
    register_features_model_amd_accelerator_models_45 ; "acc_model" :
    ATIRadeon_HD_5000 :> ATIRadeon_HD_5970 &
                        ATIRadeon_HD_5870 &
                        ATIRadeon_HD_5850 &
                        ATIRadeon_HD_5830 &
                        ATIRadeon_HD_5770 &
                        ATIRadeon_HD_5750 &
                        ATIRadeon_HD_5670 &
                        ATIRadeon_HD_5570 &
                        ATIRadeon_HD_5450;
}