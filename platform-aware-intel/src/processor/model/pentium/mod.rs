use platform_aware_features::{CPUModel, create_feature_hierarchy};

// ------------------------------------------------------------------
// Pentium Family
// ------------------------------------------------------------------

create_feature_hierarchy! { register_features_model_pentium_1 ; "cpu_model" : CPUModel :> IntelPentium; }

// ------------------------------------------------------------------
// Pentium Series
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_pentium_2 ; "cpu_model" :
    CPUModel :> IntelPentium_Gold &
                IntelPentium_Silver &
                IntelPentium_D &
                IntelPentium_G &
                IntelPentium_J &
                IntelPentium_N &
                IntelPentium_6800 &
                IntelPentium_4000 &
                IntelPentium_3000 &
                IntelPentium_2000 &
                IntelPentium_1000;
}

// ------------------------------------------------------------------
// Pentium Models
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_pentium_3 ; "cpu_model" :
    IntelPentium_N :> IntelPentium_N3700 &
                     IntelPentium_N4200E &
                     IntelPentium_N4200 &
                     IntelPentium_N3540 &
                     IntelPentium_N3530 &
                     IntelPentium_N3520 &
                     IntelPentium_N3510 &
                     IntelPentium_N6415 &
                     IntelPentium_N3710;
}

create_feature_hierarchy! {
    register_features_model_pentium_4 ; "cpu_model" :
    IntelPentium_G :> IntelPentium_G4600 &
                     IntelPentium_G4600T &
                     IntelPentium_G4620 &
                     IntelPentium_G4560 &
                     IntelPentium_G4560T &
                     IntelPentium_G4500 &
                     IntelPentium_G4500T &
                     IntelPentium_G4520 &
                     IntelPentium_G4400TE &
                     IntelPentium_G4400 &
                     IntelPentium_G4400T &
                     IntelPentium_G3260 &
                     IntelPentium_G3260T &
                     IntelPentium_G3460T &
                     IntelPentium_G3470 &
                     IntelPentium_G3250 &
                     IntelPentium_G3250T &
                     IntelPentium_G3450T &
                     IntelPentium_G3460 &
                     IntelPentium_G3258 &
                     IntelPentium_G3240 &
                     IntelPentium_G3240T &
                     IntelPentium_G3440 &
                     IntelPentium_G3440T &
                     IntelPentium_G3450 &
                     IntelPentium_G3220 &
                     IntelPentium_G3220T &
                     IntelPentium_G3320TE &
                     IntelPentium_G3420 &
                     IntelPentium_G3420T &
                     IntelPentium_G3430 &
                     IntelPentium_G2030 &
                     IntelPentium_G2030T &
                     IntelPentium_G2120T &
                     IntelPentium_G2140 &
                     IntelPentium_G2010 &
                     IntelPentium_G2020 &
                     IntelPentium_G2020T &
                     IntelPentium_G2130 &
                     IntelPentium_G2100T &
                     IntelPentium_G2120;
}

create_feature_hierarchy! {
    register_features_model_pentium_5 ; "cpu_model" :
    IntelPentium_Gold :> IntelPentium_4415Y &
                        IntelPentium_4410Y &
                        IntelPentium_4417U &
                        IntelPentium_4415U &
                        IntelPentium_8500 &
                        IntelPentium_8505 &
                        IntelPentium_G7400 &
                        IntelPentium_G7400E &
                        IntelPentium_G7400T &
                        IntelPentium_G7400TE &
                        IntelPentium_G6405 &
                        IntelPentium_G6405T &
                        IntelPentium_G6505 &
                        IntelPentium_G6505T &
                        IntelPentium_G6605 &
                        IntelPentium_6500Y &
                        IntelPentium_7505 &
                        IntelPentium_G6400 &
                        IntelPentium_G6400E &
                        IntelPentium_G6400T &
                        IntelPentium_G6400TE &
                        IntelPentium_G6500 &
                        IntelPentium_G6500T &
                        IntelPentium_G6600 &
                        IntelPentium_6405U &
                        IntelPentium_G5420 &
                        IntelPentium_G5420T &
                        IntelPentium_G5600T &
                        IntelPentium_G5620 &
                        IntelPentium_4425Y &
                        IntelPentium_G5400 &
                        IntelPentium_G5400T &
                        IntelPentium_G5500 &
                        IntelPentium_G5500T &
                        IntelPentium_5405U;
}

create_feature_hierarchy! {
    register_features_model_pentium_6 ; "cpu_model" :
    IntelPentium_Silver :> IntelPentium_N6000 &
                          IntelPentium_N6005 &
                          IntelPentium_J5040 &
                          IntelPentium_N5030 &
                          IntelPentium_J5005 &
                          IntelPentium_N5000;
}

create_feature_hierarchy! {
    register_features_model_pentium_7 ; "cpu_model" :
    IntelPentium_D :> IntelPentium_D1519 &
                     IntelPentium_D1507 &
                     IntelPentium_D1508 &
                     IntelPentium_D1509 &
                     IntelPentium_D1517;
}

create_feature_hierarchy! {
    register_features_model_pentium_8 ; "cpu_model" :
    IntelPentium_J :> IntelPentium_J4205 &
                     IntelPentium_J2900 &
                     IntelPentium_J2850 &
                     IntelPentium_J6426 &
                     IntelPentium_J3710;
}

create_feature_hierarchy! {
    register_features_model_pentium_9 ; "cpu_model" :
    IntelPentium_6800 :> IntelPentium_6805;
}

create_feature_hierarchy! {
    register_features_model_pentium_10 ; "cpu_model" :
    IntelPentium_4000 :> IntelPentium_4405Y &
                        IntelPentium_4405U;
}

create_feature_hierarchy! {
    register_features_model_pentium_11 ; "cpu_model" :
    IntelPentium_3000 :> IntelPentium_3825U &
                        IntelPentium_3805U &
                        IntelPentium_3560M &
                        IntelPentium_3558U &
                        IntelPentium_3561Y &
                        IntelPentium_3550M &
                        IntelPentium_3556U &
                        IntelPentium_3560Y;
}

create_feature_hierarchy! {
    register_features_model_pentium_12 ; "cpu_model" :
    IntelPentium_2000 :> IntelPentium_2127U &
                        IntelPentium_2030M &
                        IntelPentium_2129Y &
                        IntelPentium_2020M &
                        IntelPentium_2117U;
}

create_feature_hierarchy! {
    register_features_model_pentium_13 ; "cpu_model" :
    IntelPentium_1000 :> IntelPentium_A1018 &
                        IntelPentium_A1020 &
                        IntelPentium_1405V2 &
                        IntelPentium_1405;
}
