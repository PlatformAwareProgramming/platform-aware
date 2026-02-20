
use platform_aware_features::{CPUManufacturer, CPUMicroarchitecture, CPUModel, create_feature_hierarchy};

// ------------------------------------------------------------
// AMD (Julia -> Rust translation, analogous to the Intel Atom one)
// ------------------------------------------------------------

// AMD root (manufacturer-level marker; kept in the CPUModel hierarchy for consistency)
create_feature_hierarchy! { register_features_model_amd_1 ; "cpu_manufacturer" : CPUManufacturer :> AMD; }


// ------------------------------------------------------------
// AMD Microarchitectures
// (kept as CPUModel nodes to mirror the Atom translation style)
// ------------------------------------------------------------
create_feature_hierarchy! {
    register_features_model_amd_microarch_1 ; "cpu_microarchitecture" : CPUModel :> AMDMicroarchitecture; }

create_feature_hierarchy! {
    register_features_model_amd_microarch_2 ; "cpu_microarchitecture" :
    AMDMicroarchitecture :> K6 &
                           K7 &
                           K8 &
                           Hammer &
                           K10 &
                           Zen &
                           Zen2 &
                           Zen3 &
                           Zen4 &
                           Zen4c &
                           Zen5 &
                           Bobcat &
                           Bulldozer &
                           Piledriver &
                           Steamroller &
                           Excavator &
                           Jaguar &
                           Puma;
}


create_feature_hierarchy! {
    register_features_model_amd_families_0 ; "cpu_model" : CPUModel :> AMDProcessor;
}
// ------------------------------------------------------------
// Families (AMDProcessor children)
// ------------------------------------------------------------
create_feature_hierarchy! {
    register_features_model_amd_families_1 ; "cpu_model" :
    AMDProcessor :> AMD_ASeries &
                    AMD_ASeries_PRO &
                    AMDAthlon &
                    AMDAthlon_PRO &
                    AMD_ESeries &
                    AMDEPYC &
                    AMD_FX &
                    AMDOpteron &
                    AMDPhenom &
                    AMDRyzen &
                    AMDRyzen_PRO &
                    AMDSempron &
                    AMDTurion;
}

// ------------------------------------------------------------
// Lines (family -> line)
// ------------------------------------------------------------

// Generic "AMD 3000 Series Mobile Processors with Radeon Graphics"
create_feature_hierarchy! {
    register_features_model_amd_lines_3000_1 ; "cpu_model" :
    AMDProcessor :> AMD_3000;
}

// A-Series
create_feature_hierarchy! {
    register_features_model_amd_lines_a_series_1 ; "cpu_model" :
    AMD_ASeries :> AMD_A12 &
                  AMD_A10 &
                  AMD_A8 &
                  AMD_A6 &
                  AMD_A9 &
                  AMD_A4 &
                  AMD_A10_Business &
                  AMD_A8_Business &
                  AMD_A6_Business;
}

// PRO A-Series
create_feature_hierarchy! {
    register_features_model_amd_lines_a_series_pro_1 ; "cpu_model" :
    AMD_ASeries_PRO :> AMD_A4_PRO &
                      AMD_A6_PRO &
                      AMD_A8_PRO &
                      AMD_A10_PRO &
                      AMD_A12_PRO;
}

// Athlon
create_feature_hierarchy! {
    register_features_model_amd_lines_athlon_1 ; "cpu_model" :
    AMDAthlon :> AMDAthlon_Vega &
                AMDAthlon_3000G &
                AMDAthlonII_X4 &
                AMDAthlonII_X3 &
                AMDAthlonII_X2 &
                AMDAthlon_X4 &
                AMDAthlon_APU ; 
}

// Athlon PRO
create_feature_hierarchy! {
    register_features_model_amd_lines_athlon_pro_1 ; "cpu_model" :
    AMDAthlon_PRO :> AMDAthlon_PRO_3000 &
                    AMDAthlon_PRO_Vega;
}

// E-Series
create_feature_hierarchy! {
    register_features_model_amd_lines_e_series_1 ; "cpu_model" :
    AMD_ESeries :> AMD_E2 &
                  AMD_E1;
}

// EPYC
create_feature_hierarchy! {
    register_features_model_amd_lines_epyc_1 ; "cpu_model" :
    AMDEPYC :> AMDEPYC_7003 &
              AMDEPYC_7002 &
              AMDEPYC_7001 &
              AMDEPYC_9R14 &
              AMDEPYC_7B13 &
              AMDEPYC_7R13 &
              AMDEPYC_7R32;
}

create_feature_hierarchy! {
    register_features_model_amd_lines_epyc_2 ; "cpu_model" :
    AMDEPYC_7003 :> AMDEPYC_7003_VCache;
}

// FX
create_feature_hierarchy! {
    register_features_model_amd_lines_fx_1 ; "cpu_model" :
    AMD_FX :> AMD_FX_8_Black &
             AMD_FX_6_Black &
             AMD_FX_4_Black;
}

// Opteron
create_feature_hierarchy! {
    register_features_model_amd_lines_opteron_1 ; "cpu_model" :
    AMDOpteron :> AMDOpteron_X1100 &
                 AMDOpteron_6300 &
                 AMDOpteron_6200 &
                 AMDOpteron_6100 &
                 AMDOpteron_4300 &
                 AMDOpteron_4200 &
                 AMDOpteron_3300 &
                 AMDOpteron_3200 &
                 AMDOpteron_X2100;
}

// Phenom
create_feature_hierarchy! {
    register_features_model_amd_lines_phenom_1 ; "cpu_model" :
    AMDPhenom :> AMDPhenom_II_X6 &
                AMDPhenom_II_X4 &
                AMDPhenom_II_X4_Black &
                AMDPhenom_II_X2 &
                AMDPhenom_II_X2_Black &
                AMDPhenom_II_Black &
                AMDPhenom_II_QuadCore &
                AMDPhenom_II_TripleCore &
                AMDPhenom_II_DualCore &
                AMDPhenom_X4 &
                AMDPhenom_X3 &
                AMDPhenom_X2;
}

// Ryzen
create_feature_hierarchy! {
    register_features_model_amd_lines_ryzen_1 ; "cpu_model" :
    AMDRyzen :> AMDRyzen_9 &
               AMDRyzen_7 &
               AMDRyzen_5 &
               AMDRyzen_Threadripper &
               AMDRyzen_3 &
               AMDRyzen_7_5000G &
               AMDRyzen_5_5000G &
               AMDRyzen_3_5000G &
               AMDRyzen_7_4000G &
               AMDRyzen_5_4000G &
               AMDRyzen_3_4000G &
               AMDRyzen_5_Vega &
               AMDRyzen_7_Surface &
               AMDRyzen_5_Surface &
               AMDRyzen_7_RXVega11_Surface &
               AMDRyzen_3_Vega &
               AMDRyzen_7_RXVega &
               AMDRyzen_5_Vega_9_Surface;
}

// Ryzen PRO
create_feature_hierarchy! {
    register_features_model_amd_lines_ryzen_pro_1 ; "cpu_model" :
    AMDRyzen_PRO :> AMDRyzen_PRO_Threadripper_5000_WX &
                   AMDRyzen_PRO_Threadripper &
                   AMDRyzen_PRO_9 &
                   AMDRyzen_PRO_9_6000 &
                   AMDRyzen_PRO_7 &
                   AMDRyzen_PRO_7_Vega &
                   AMDRyzen_PRO_7_5000 &
                   AMDRyzen_PRO_7_4000 &
                   AMDRyzen_PRO_7_6000 &
                   AMDRyzen_PRO_5 &
                   AMDRyzen_PRO_5_Vega &
                   AMDRyzen_PRO_5_3000 &
                   AMDRyzen_PRO_5_4000 &
                   AMDRyzen_PRO_5_5000 &
                   AMDRyzen_PRO_5_6000 &
                   AMDRyzen_PRO_3 &
                   AMDRyzen_PRO_3_Vega &
                   AMDRyzen_PRO_3_4000 &
                   AMDRyzen_PRO_3_5000;
}

create_feature_hierarchy! {
    register_features_model_amd_lines_ryzen_pro_2 ; "cpu_model" :
    AMDRyzen_PRO_9 :> AMDRyzen_PRO_9_D;
}

create_feature_hierarchy! {
    register_features_model_amd_lines_ryzen_pro_3 ; "cpu_model" :
    AMDRyzen_PRO_7 :> AMDRyzen_PRO_7_D;
}

create_feature_hierarchy! {
    register_features_model_amd_lines_ryzen_pro_4 ; "cpu_model" :
    AMDRyzen_PRO_5 :> AMDRyzen_PRO_5_D;
}

create_feature_hierarchy! {
    register_features_model_amd_lines_ryzen_pro_5 ; "cpu_model" :
    AMDRyzen_PRO_7_Vega :> AMDRyzen_PRO_7_Vega_M;
}

create_feature_hierarchy! {
    register_features_model_amd_lines_ryzen_pro_6 ; "cpu_model" :
    AMDRyzen_PRO_7_5000 :> AMDRyzen_PRO_7_5000_D &
                          AMDRyzen_PRO_7_5000_M;
}

create_feature_hierarchy! {
    register_features_model_amd_lines_ryzen_pro_7 ; "cpu_model" :
    AMDRyzen_PRO_7_4000 :> AMDRyzen_PRO_7_4000_D &
                          AMDRyzen_PRO_7_4000_M;
}

create_feature_hierarchy! {
    register_features_model_amd_lines_ryzen_pro_8 ; "cpu_model" :
    AMDRyzen_PRO_7_6000 :> AMDRyzen_PRO_7_6000_M;
}

create_feature_hierarchy! {
    register_features_model_amd_lines_ryzen_pro_9 ; "cpu_model" :
    AMDRyzen_PRO_9_6000 :> AMDRyzen_PRO_9_6000_M;
}

// Sempron
create_feature_hierarchy! {
    register_features_model_amd_lines_sempron_1 ; "cpu_model" :
    AMDSempron :> AMDSempron_Quad_APU &
                 AMDSempron_Dual_APU;
}

// Turion
create_feature_hierarchy! {
    register_features_model_amd_lines_turion_1 ; "cpu_model" :
    AMDTurion :> AMDTurion_64_X2;
}

// ------------------------------------------------------------
// Models (line -> model)  [only those present in the snippet]
// ------------------------------------------------------------

// Athlon II X2
create_feature_hierarchy! {
    register_features_model_amd_models_athlon_iix2_1 ; "cpu_model" :
    AMDAthlonII_X2 :> AMDAthlon_II_X2_255e;
}

// Athlon II X3
create_feature_hierarchy! {
    register_features_model_amd_models_athlon_iix3_1 ; "cpu_model" :
    AMDAthlonII_X3 :> AMDAthlon_II_X3_460 &
                     AMDAthlon_II_X3_425e;
}

// Athlon II X4
create_feature_hierarchy! {
    register_features_model_amd_models_athlon_iix4_1 ; "cpu_model" :
    AMDAthlonII_X4 :> AMDAthlon_II_X4_631 &
                     AMDAthlon_II_X4_638 &
                     AMDAthlon_II_X4_641 &
                     AMDAthlon_II_X4_620e;
}

// Athlon X4
create_feature_hierarchy! {
    register_features_model_amd_models_athlon_x4_1 ; "cpu_model" :
    AMDAthlon_X4 :> AMDAthlon_X4_740 &
                   AMDAthlon_X4_750 &
                   AMDAthlon_X4_750K &
                   AMDAthlon_X4_760K &
                   AMDAthlon_X4_845 &
                   AMDAthlon_X4_860K &
                   AMDAthlon_X4_870K &
                   AMDAthlon_X4_880K &
                   AMDAthlon_7_X4_940 &
                   AMDAthlon_7_X4_950 &
                   AMDAthlon_7_X4_970;
}

// Athlon APU
create_feature_hierarchy! {
    register_features_model_amd_models_athlon_apu_1 ; "cpu_model" :
    AMDAthlon_APU :> AMDAthlon_5150_APU &
                    AMDAthlon_5350_APU &
                    AMDAthlon_5370_APU;
}

// Phenom X2/X3/X4 (Business Class lines)
create_feature_hierarchy! {
    register_features_model_amd_models_phenom_x2_1 ; "cpu_model" :
    AMDPhenom_X2 :> AMDPhenom_X2_B57 &
                   AMDPhenom_X2_B59 &
                   AMDPhenom_X2_B60;
}

create_feature_hierarchy! {
    register_features_model_amd_models_phenom_x3_1 ; "cpu_model" :
    AMDPhenom_X3 :> AMDPhenom_X3_B75 &
                   AMDPhenom_X3_B77;
}

create_feature_hierarchy! {
    register_features_model_amd_models_phenom_x4_1 ; "cpu_model" :
    AMDPhenom_X4 :> AMDPhenom_X4_B95 &
                   AMDPhenom_X4_B97 &
                   AMDPhenom_X4_B99;
}

// E1 / E2 models
create_feature_hierarchy! {
    register_features_model_amd_models_e1_1 ; "cpu_model" :
    AMD_E1 :> AMD_E1_7010 &
             AMD_E1_Micro_6200T &
             AMD_E1_2100 &
             AMD_E1_2200 &
             AMD_E1_2500 &
             AMD_E1_6010;
}

create_feature_hierarchy! {
    register_features_model_amd_models_e2_1 ; "cpu_model" :
    AMD_E2 :> AMD_E2_7110 &
             AMD_E2_3000 &
             AMD_E2_3800 &
             AMD_E2_6110 &
             AMD_E2_7_9010_APU;
}

// FX (Black Edition) desktop models
create_feature_hierarchy! {
    register_features_model_amd_models_fx4_black_1 ; "cpu_model" :
    AMD_FX_4_Black :> AMD_FX_4100 &
                     AMD_FX_4130 &
                     AMD_FX_4170 &
                     AMD_FX_4300 &
                     AMD_FX_4320 &
                     AMD_FX_4350;
}

create_feature_hierarchy! {
    register_features_model_amd_models_fx6_black_1 ; "cpu_model" :
    AMD_FX_6_Black :> AMD_FX_6100 &
                     AMD_FX_6200 &
                     AMD_FX_6300 &
                     AMD_FX_6350;
}

create_feature_hierarchy! {
    register_features_model_amd_models_fx8_black_1 ; "cpu_model" :
    AMD_FX_8_Black :> AMD_FX_8120 &
                     AMD_FX_8150 &
                     AMD_FX_8300 &
                     AMD_FX_8310 &
                     AMD_FX_8320 &
                     AMD_FX_8320E &
                     AMD_FX_8350 &
                     AMD_FX_8370 &
                     AMD_FX_8370E &
                     AMD_FX_9370 &
                     AMD_FX_9590;
}

// FX mobile-ish models (line AMD_FX)
create_feature_hierarchy! {
    register_features_model_amd_models_fx_misc_1 ; "cpu_model" :
    AMD_FX :> AMD_FX_8800P &
             AMD_FX_7500 &
             AMD_FX_7600P &
             AMD_FX_6_8800P_APU &
             AMD_FX_7_9800P_APU &
             AMD_FX_7_9830P_APU;
}

// Opteron 3200
create_feature_hierarchy! {
    register_features_model_amd_models_opteron_3200_1 ; "cpu_model" :
    AMDOpteron_3200 :> AMDOpteron_3280 &
                      AMDOpteron_3250 &
                      AMDOpteron_3260;
}

// Opteron 3300
create_feature_hierarchy! {
    register_features_model_amd_models_opteron_3300_1 ; "cpu_model" :
    AMDOpteron_3300 :> AMDOpteron_3365 &
                      AMDOpteron_3380 &
                      AMDOpteron_3320 &
                      AMDOpteron_3350;
}

// Opteron 4200
create_feature_hierarchy! {
    register_features_model_amd_models_opteron_4200_1 ; "cpu_model" :
    AMDOpteron_4200 :> AMDOpteron_4226 &
                      AMDOpteron_4234 &
                      AMDOpteron_4238 &
                      AMDOpteron_4240 &
                      AMDOpteron_4280 &
                      AMDOpteron_4284 &
                      AMDOpteron_4228 &
                      AMDOpteron_4230 &
                      AMDOpteron_4256 &
                      AMDOpteron_4274 &
                      AMDOpteron_4276;
}

// Opteron 4300
create_feature_hierarchy! {
    register_features_model_amd_models_opteron_4300_1 ; "cpu_model" :
    AMDOpteron_4300 :> AMDOpteron_4334 &
                      AMDOpteron_4340 &
                      AMDOpteron_4365 &
                      AMDOpteron_4386 &
                      AMDOpteron_4310 &
                      AMDOpteron_4332 &
                      AMDOpteron_4376;
}

// Opteron 6100
create_feature_hierarchy! {
    register_features_model_amd_models_opteron_6100_1 ; "cpu_model" :
    AMDOpteron_6100 :> AMDOpteron_6140 &
                      AMDOpteron_6176 &
                      AMDOpteron_6132 &
                      AMDOpteron_6166 &
                      AMDOpteron_6180;
}

// Opteron 6200
create_feature_hierarchy! {
    register_features_model_amd_models_opteron_6200_1 ; "cpu_model" :
    AMDOpteron_6200 :> AMDOpteron_6204 &
                      AMDOpteron_6212 &
                      AMDOpteron_6220 &
                      AMDOpteron_6234 &
                      AMDOpteron_6238 &
                      AMDOpteron_6272 &
                      AMDOpteron_6274 &
                      AMDOpteron_6276 &
                      AMDOpteron_6278 &
                      AMDOpteron_6262 &
                      AMDOpteron_6282 &
                      AMDOpteron_6284;
}

// Opteron 6300
create_feature_hierarchy! {
    register_features_model_amd_models_opteron_6300_1 ; "cpu_model" :
    AMDOpteron_6300 :> AMDOpteron_6308 &
                      AMDOpteron_6320 &
                      AMDOpteron_6328 &
                      AMDOpteron_6344 &
                      AMDOpteron_6348 &
                      AMDOpteron_6376 &
                      AMDOpteron_6378 &
                      AMDOpteron_6380 &
                      AMDOpteron_6338P &
                      AMDOpteron_6366 &
                      AMDOpteron_6370P &
                      AMDOpteron_6386;
}

// Opteron X1100
create_feature_hierarchy! {
    register_features_model_amd_models_opteron_x1100_1 ; "cpu_model" :
    AMDOpteron_X1100 :> AMDOpteron_X1150;
}

// Opteron X2100
create_feature_hierarchy! {
    register_features_model_amd_models_opteron_x2100_1 ; "cpu_model" :
    AMDOpteron_X2100 :> AMDOpteron_X2150_APU &
                       AMDOpteron_X2170;
}

// Phenom II mobile/desktop
create_feature_hierarchy! {
    register_features_model_amd_models_phenom_ii_black_1 ; "cpu_model" :
    AMDPhenom_II_Black :> AMDPhenom_II_X940;
}

create_feature_hierarchy! {
    register_features_model_amd_models_phenom_ii_dual_1 ; "cpu_model" :
    AMDPhenom_II_DualCore :> AMDPhenom_II_N640 &
                            AMDPhenom_II_N660 &
                            AMDPhenom_II_P650;
}

create_feature_hierarchy! {
    register_features_model_amd_models_phenom_ii_quad_1 ; "cpu_model" :
    AMDPhenom_II_QuadCore :> AMDPhenom_II_N960 &
                            AMDPhenom_II_N970;
}

create_feature_hierarchy! {
    register_features_model_amd_models_phenom_ii_triple_1 ; "cpu_model" :
    AMDPhenom_II_TripleCore :> AMDPhenom_II_N870 &
                              AMDPhenom_II_P860;
}

create_feature_hierarchy! {
    register_features_model_amd_models_phenom_ii_x2_1 ; "cpu_model" :
    AMDPhenom_II_X2 :> AMDPhenom_II_X2_565;
}

create_feature_hierarchy! {
    register_features_model_amd_models_phenom_ii_x2_black_1 ; "cpu_model" :
    AMDPhenom_II_X2_Black :> AMDPhenom_II_X2_Black_555 &
                            AMDPhenom_II_X2_Black_565 &
                            AMDPhenom_II_X2_Black_570;
}

create_feature_hierarchy! {
    register_features_model_amd_models_phenom_ii_x4_1 ; "cpu_model" :
    AMDPhenom_II_X4 :> AMDPhenom_II_840 &
                      AMDPhenom_II_850;
}

create_feature_hierarchy! {
    register_features_model_amd_models_phenom_ii_x4_black_1 ; "cpu_model" :
    AMDPhenom_II_X4_Black :> AMDPhenom_II_965 &
                            AMDPhenom_II_975 &
                            AMDPhenom_II_980 &
                            AMDPhenom_II_960T;
}

create_feature_hierarchy! {
    register_features_model_amd_models_phenom_ii_x6_1 ; "cpu_model" :
    AMDPhenom_II_X6 :> AMDPhenom_II_1045T &
                      AMDPhenom_II_1075T;
}

// Sempron
create_feature_hierarchy! {
    register_features_model_amd_models_sempron_1 ; "cpu_model" :
    AMDSempron :> AMDSempron_2800plus;
}

create_feature_hierarchy! {
    register_features_model_amd_models_sempron_dual_apu_1 ; "cpu_model" :
    AMDSempron_Dual_APU :> AMDSempron_2650_APU;
}

create_feature_hierarchy! {
    register_features_model_amd_models_sempron_quad_apu_1 ; "cpu_model" :
    AMDSempron_Quad_APU :> AMDSempron_3850_APU;
}

// Turion 64 X2
create_feature_hierarchy! {
    register_features_model_amd_models_turion_64_x2_1 ; "cpu_model" :
    AMDTurion_64_X2 :> AMDTurion_TL52 &
                      AMDTurion_TL56 &
                      AMDTurion_TL60 &
                      AMDTurion_TL64;
}

// AMD 3000 series mobile
create_feature_hierarchy! {
    register_features_model_amd_models_3000_series_1 ; "cpu_model" :
    AMD_3000 :> AMD_3015Ce &
               AMD_3015e &
               AMD_3020e;
}

// ------------------------------------------------------------
// A-Series and PRO A-Series models (as provided)
// ------------------------------------------------------------

// A10
create_feature_hierarchy! {
    register_features_model_amd_models_a10_1 ; "cpu_model" :
    AMD_A10 :> AMD_A10_6_8700P_APU &
              AMD_A10_6700 &
              AMD_A10_6700T &
              AMD_A10_6790K &
              AMD_A10_7_9600P_APU &
              AMD_A10_7_9630P_APU &
              AMD_A10_7_9700_APU &
              AMD_A10_7_9700E_APU &
              AMD_A10_7300 &
              AMD_A10_7400P &
              AMD_A10_7700K &
              AMD_A10_7800 &
              AMD_A10_7850K &
              AMD_A10_7860K &
              AMD_A10_7870K &
              AMD_A10_7890K &
              AMD_A10_8700P &
              AMD_A10_Micro_6700T &
              AMD_A10_PRO_7350B &
              AMD_A10_PRO_7800B &
              AMD_A10_PRO_7850B &
              AMD_A10_PRO_6_8700B_APU &
              AMD_A10_PRO_6_8730B_APU &
              AMD_A10_PRO_6_8750B_APU &
              AMD_A10_PRO_6_8770_APU &
              AMD_A10_PRO_6_8770E_APU &
              AMD_A10_PRO_6_8850B_APU &
              AMD_A10_PRO_7_9700_APU &
              AMD_A10_PRO_7_9700B_APU &
              AMD_A10_PRO_7_9700E_APU &
              AMD_A10_PRO_7_9730B_APU;
}

// A10 Business
create_feature_hierarchy! {
    register_features_model_amd_models_a10_business_1 ; "cpu_model" :
    AMD_A10_Business :> AMD_A10_6790B &
                       AMD_A10_6800B;
}

// A12
create_feature_hierarchy! {
    register_features_model_amd_models_a12_1 ; "cpu_model" :
    AMD_A12 :> AMD_A12_7_9700P_APU &
              AMD_A12_7_9730P_APU &
              AMD_A12_7_9800_APU &
              AMD_A12_7_9800E_APU;
}

// A12 PRO (line exists; models are named in snippet)
create_feature_hierarchy! {
    register_features_model_amd_models_a12_pro_1 ; "cpu_model" :
    AMD_A12_PRO :> AMD_A12_PRO_6_8800B_APU &
                  AMD_A12_PRO_6_8830B_APU &
                  AMD_A12_PRO_6_8870_APU &
                  AMD_A12_PRO_6_8870E_APU &
                  AMD_A12_PRO_7_9800_APU &
                  AMD_A12_PRO_7_9800B_APU &
                  AMD_A12_PRO_7_9800E_APU &
                  AMD_A12_PRO_7_9830B_APU;
}

// A4
create_feature_hierarchy! {
    register_features_model_amd_models_a4_1 ; "cpu_model" :
    AMD_A4 :> AMD_A4_5000 &
             AMD_A4_5100 &
             AMD_A4_6210 &
             AMD_A4_6300 &
             AMD_A4_6320 &
             AMD_A4_7_9120_APU &
             AMD_A4_7_9120C_APU &
             AMD_A4_7_9125_APU &
             AMD_A4_7210 &
             AMD_A4_7300 &
             AMD_A4_Micro_6400T &
             AMD_A4_PRO_3340B &
             AMD_A4_PRO_3350B &
             AMD_A4_PRO_7300B &
             AMD_A4_PRO_7350B &
             AMD_A4_PRO_6_8350B_APU &
             AMD_A4_PRO_7_4350B_APU &
             AMD_A4_PRO_7_5350B_APU;
}

// A6
create_feature_hierarchy! {
    register_features_model_amd_models_a6_1 ; "cpu_model" :
    AMD_A6 :> AMD_A6_5200 &
             AMD_A6_5200M &
             AMD_A6_5350M &
             AMD_A6_6310 &
             AMD_A6_6400K &
             AMD_A6_7_9210_APU &
             AMD_A6_7_9220_APU &
             AMD_A6_7_9220C_APU &
             AMD_A6_7_9225_APU &
             AMD_A6_7_9500_APU &
             AMD_A6_7_9500E_APU &
             AMD_A6_7_9550_APU &
             AMD_A6_7000 &
             AMD_A6_7310 &
             AMD_A6_7400K &
             AMD_A6_7470K &
             AMD_A6_8500P &
             AMD_A6_PRO_7050B &
             AMD_A6_PRO_7400B &
             AMD_A6_PRO_6_8500B_APU &
             AMD_A6_PRO_6_8530B_APU &
             AMD_A6_PRO_6_8550B_APU &
             AMD_A6_PRO_6_8570_APU &
             AMD_A6_PRO_6_8570E_APU &
             AMD_A6_PRO_7_7350B_APU &
             AMD_A6_PRO_7_8350B_APU &
             AMD_A6_PRO_7_9500_APU &
             AMD_A6_PRO_7_9500B_APU &
             AMD_A6_PRO_7_9500E_APU;
}

// A6 Business
create_feature_hierarchy! {
    register_features_model_amd_models_a6_business_1 ; "cpu_model" :
    AMD_A6_Business :> AMD_A6_6400B &
                      AMD_A6_6420B;
}

// A8
create_feature_hierarchy! {
    register_features_model_amd_models_a8_1 ; "cpu_model" :
    AMD_A8 :> AMD_A8_6_8600P_APU &
             AMD_A8_6410 &
             AMD_A8_6500 &
             AMD_A8_6500T &
             AMD_A8_6600K &
             AMD_A8_7_9600_APU &
             AMD_A8_7100 &
             AMD_A8_7200P &
             AMD_A8_7410 &
             AMD_A8_7600 &
             AMD_A8_7650K &
             AMD_A8_7670K &
             AMD_A8_8600P &
             AMD_A8_PRO_7150B &
             AMD_A8_PRO_7600B &
             AMD_A8_PRO_6_8600B_APU &
             AMD_A8_PRO_6_8650B_APU &
             AMD_A8_PRO_7_9600_APU &
             AMD_A8_PRO_7_9600B_APU &
             AMD_A8_PRO_7_9630B;
}

// A8 Business
create_feature_hierarchy! {
    register_features_model_amd_models_a8_business_1 ; "cpu_model" :
    AMD_A8_Business :> AMD_A8_6500B;
}

// A9
create_feature_hierarchy! {
    register_features_model_amd_models_a9_1 ; "cpu_model" :
    AMD_A9 :> AMD_A9_7_9410_APU &
             AMD_A9_7_9420_APU &
             AMD_A9_7_9425_APU;
}

// ------------------------------------------------------------
// Athlon Vega / 3000G / PRO lines models (as provided)
// ------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_amd_models_athlon_vega_1 ; "cpu_model" :
    AMDAthlon_Vega :> AMDAthlon_200GE &
                     AMDAthlon_220GE &
                     AMDAthlon_240GE &
                     AMDAthlon_300GE &
                     AMDAthlon_300U &
                     AMDAthlon_320GE;
}

create_feature_hierarchy! {
    register_features_model_amd_models_athlon_3000g_1 ; "cpu_model" :
    AMDAthlon_3000G :> AMDAthlon_Gold_3150G &
                      AMDAthlon_Gold_3150GE &
                      AMDAthlon_Silver_3050GE;
}

create_feature_hierarchy! {
    register_features_model_amd_models_athlon_misc_1 ; "cpu_model" :
    AMDAthlon :> AMDAthlon_Gold_3150C &
                AMDAthlon_Gold_3150U &
                AMDAthlon_Silver_3050C &
                AMDAthlon_Silver_3050e &
                AMDAthlon_Silver_3050U;
}

create_feature_hierarchy! {
    register_features_model_amd_models_athlon_pro_3000_1 ; "cpu_model" :
    AMDAthlon_PRO_3000 :> AMDAthlon_Gold_PRO_3150G &
                          AMDAthlon_Gold_PRO_3150GE &
                          AMDAthlon_Silver_PRO_3125GE;
}

create_feature_hierarchy! {
    register_features_model_amd_models_athlon_pro_vega_1 ; "cpu_model" :
    AMDAthlon_PRO_Vega :> AMDAthlon_PRO_200GE &
                          AMDAthlon_PRO_200U &
                          AMDAthlon_PRO_300GE &
                          AMDAthlon_PRO_300U;
}

create_feature_hierarchy! {
    register_features_model_amd_models_athlon_pro_misc_1 ; "cpu_model" :
    AMDAthlon_PRO :> AMDAthlon_PRO_3045B &
                    AMDAthlon_PRO_3145B;
}

// ------------------------------------------------------------
// EPYC models (as provided)
// ------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_amd_models_epyc_7001_1 ; "cpu_model" :
    AMDEPYC_7001 :> AMDEPYC_7251 &
                    AMDEPYC_7261 &
                    AMDEPYC_7281 &
                    AMDEPYC_7301 &
                    AMDEPYC_7351 &
                    AMDEPYC_7351P &
                    AMDEPYC_7371 &
                    AMDEPYC_7401 &
                    AMDEPYC_7401P &
                    AMDEPYC_7451 &
                    AMDEPYC_7501 &
                    AMDEPYC_7551 &
                    AMDEPYC_7551P &
                    AMDEPYC_7601;
}

create_feature_hierarchy! {
    register_features_model_amd_models_epyc_7002_1 ; "cpu_model" :
    AMDEPYC_7002 :> AMDEPYC_7232P &
                    AMDEPYC_7252 &
                    AMDEPYC_7262 &
                    AMDEPYC_7272 &
                    AMDEPYC_7282 &
                    AMDEPYC_7302 &
                    AMDEPYC_7302P &
                    AMDEPYC_7352 &
                    AMDEPYC_7402 &
                    AMDEPYC_7402P &
                    AMDEPYC_7452 &
                    AMDEPYC_7502 &
                    AMDEPYC_7502P &
                    AMDEPYC_7532 &
                    AMDEPYC_7542 &
                    AMDEPYC_7552 &
                    AMDEPYC_7642 &
                    AMDEPYC_7662 &
                    AMDEPYC_7702 &
                    AMDEPYC_7702P &
                    AMDEPYC_7742 &
                    AMDEPYC_7F32 &
                    AMDEPYC_7F52 &
                    AMDEPYC_7F72 &
                    AMDEPYC_7H12;
}

create_feature_hierarchy! {
    register_features_model_amd_models_epyc_7003_1 ; "cpu_model" :
    AMDEPYC_7003 :> AMDEPYC_72F3 &
                    AMDEPYC_7313 &
                    AMDEPYC_7313P &
                    AMDEPYC_7343 &
                    AMDEPYC_73F3 &
                    AMDEPYC_7413 &
                    AMDEPYC_7443 &
                    AMDEPYC_7443P &
                    AMDEPYC_7453 &
                    AMDEPYC_74F3 &
                    AMDEPYC_7513 &
                    AMDEPYC_7543 &
                    AMDEPYC_7543P &
                    AMDEPYC_75F3 &
                    AMDEPYC_7643 &
                    AMDEPYC_7663 &
                    AMDEPYC_7713 &
                    AMDEPYC_7713P &
                    AMDEPYC_7763;
}

create_feature_hierarchy! {
    register_features_model_amd_models_epyc_7003_vcache_1 ; "cpu_model" :
    AMDEPYC_7003_VCache :> AMDEPYC_7373X &
                          AMDEPYC_7473X &
                          AMDEPYC_7571 &
                          AMDEPYC_7573X &
                          AMDEPYC_7773X;
}
