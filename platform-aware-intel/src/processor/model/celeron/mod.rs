use platform_aware_features::{CPUModel, create_feature_hierarchy};

// ------------------------------------------------------------------
// Celeron Family
// ------------------------------------------------------------------

create_feature_hierarchy! { register_features_model_celeron_1 ; "cpu_model" : CPUModel :> IntelCeleron; }

// ------------------------------------------------------------------
// Celeron Series
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_celeron_2 ; "cpu_model" :
    CPUModel :> IntelCeleron_G &
                IntelCeleron_J &
                IntelCeleron_N &
                IntelCeleron_7000 &
                IntelCeleron_6000 &
                IntelCeleron_5000 &
                IntelCeleron_4000 &
                IntelCeleron_3000 &
                IntelCeleron_2000 &
                IntelCeleron_1000;
}

// ------------------------------------------------------------------
// G Series
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_celeron_3 ; "cpu_model" :
    IntelCeleron_G :> IntelCeleron_G6900 &
                     IntelCeleron_G6900E &
                     IntelCeleron_G6900T &
                     IntelCeleron_G6900TE &
                     IntelCeleron_G5905 &
                     IntelCeleron_G5905T &
                     IntelCeleron_G5925 &
                     IntelCeleron_G5900 &
                     IntelCeleron_G5900E &
                     IntelCeleron_G5900T &
                     IntelCeleron_G5900TE &
                     IntelCeleron_G5920 &
                     IntelCeleron_G4930E &
                     IntelCeleron_G4932E &
                     IntelCeleron_G4930 &
                     IntelCeleron_G4930T &
                     IntelCeleron_G4950 &
                     IntelCeleron_G4900 &
                     IntelCeleron_G4900T &
                     IntelCeleron_G4920 &
                     IntelCeleron_G3930E &
                     IntelCeleron_G3930TE &
                     IntelCeleron_G3930 &
                     IntelCeleron_G3930T &
                     IntelCeleron_G3950 &
                     IntelCeleron_G3900E &
                     IntelCeleron_G3902E &
                     IntelCeleron_G3900 &
                     IntelCeleron_G3900T &
                     IntelCeleron_G3900TE &
                     IntelCeleron_G3920 &
                     IntelCeleron_G1840 &
                     IntelCeleron_G1840T &
                     IntelCeleron_G1850 &
                     IntelCeleron_G1820TE &
                     IntelCeleron_G1820 &
                     IntelCeleron_G1820T &
                     IntelCeleron_G1830 &
                     IntelCeleron_G1620T &
                     IntelCeleron_G1630 &
                     IntelCeleron_G1610 &
                     IntelCeleron_G1610T &
                     IntelCeleron_G1620;
}

// ------------------------------------------------------------------
// J Series
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_celeron_4 ; "cpu_model" :
    IntelCeleron_J :> IntelCeleron_J6412 &
                     IntelCeleron_J6413 &
                     IntelCeleron_J4025 &
                     IntelCeleron_J4125 &
                     IntelCeleron_J3355E &
                     IntelCeleron_J3455E &
                     IntelCeleron_J4005 &
                     IntelCeleron_J4105 &
                     IntelCeleron_J3355 &
                     IntelCeleron_J3455 &
                     IntelCeleron_J3060 &
                     IntelCeleron_J3160 &
                     IntelCeleron_J1800 &
                     IntelCeleron_J1900 &
                     IntelCeleron_J1750 &
                     IntelCeleron_J1850;
}

// ------------------------------------------------------------------
// N Series
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_celeron_5 ; "cpu_model" :
    IntelCeleron_N :> IntelCeleron_N6210 &
                     IntelCeleron_N4500 &
                     IntelCeleron_N4505 &
                     IntelCeleron_N5100 &
                     IntelCeleron_N5105 &
                     IntelCeleron_N6211 &
                     IntelCeleron_N4020 &
                     IntelCeleron_N4120 &
                     IntelCeleron_N3350E &
                     IntelCeleron_N4000 &
                     IntelCeleron_N4100 &
                     IntelCeleron_N3350 &
                     IntelCeleron_N3450 &
                     IntelCeleron_N3010 &
                     IntelCeleron_N3060 &
                     IntelCeleron_N3160 &
                     IntelCeleron_N3000 &
                     IntelCeleron_N3050 &
                     IntelCeleron_N3150 &
                     IntelCeleron_N2808 &
                     IntelCeleron_N2840 &
                     IntelCeleron_N2940 &
                     IntelCeleron_N2807 &
                     IntelCeleron_N2830 &
                     IntelCeleron_N2930 &
                     IntelCeleron_N2806 &
                     IntelCeleron_N2815 &
                     IntelCeleron_N2820 &
                     IntelCeleron_N2920 &
                     IntelCeleron_N2805 &
                     IntelCeleron_N2810 &
                     IntelCeleron_N2910;
}

// ------------------------------------------------------------------
// 7000 / 6000 / 5000 / 4000 / 3000 / 2000 / 1000 Series
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_celeron_6 ; "cpu_model" :
    IntelCeleron_7000 :> IntelCeleron_7305 &
                        IntelCeleron_7305E &
                        IntelCeleron_7300;
}

create_feature_hierarchy! {
    register_features_model_celeron_7 ; "cpu_model" :
    IntelCeleron_6000 :> IntelCeleron_6305 &
                        IntelCeleron_6600HE &
                        IntelCeleron_6305E;
}

create_feature_hierarchy! {
    register_features_model_celeron_8 ; "cpu_model" :
    IntelCeleron_5000 :> IntelCeleron_5305U &
                        IntelCeleron_5205U;
}

create_feature_hierarchy! {
    register_features_model_celeron_9 ; "cpu_model" :
    IntelCeleron_4000 :> IntelCeleron_4305U &
                        IntelCeleron_4305UE &
                        IntelCeleron_4205U;
}

create_feature_hierarchy! {
    register_features_model_celeron_10 ; "cpu_model" :
    IntelCeleron_3000 :> IntelCeleron_3867U &
                        IntelCeleron_3965Y &
                        IntelCeleron_3865U &
                        IntelCeleron_3965U &
                        IntelCeleron_3855U &
                        IntelCeleron_3955U &
                        IntelCeleron_3215U &
                        IntelCeleron_3765U &
                        IntelCeleron_3205U &
                        IntelCeleron_3755U;
}

create_feature_hierarchy! {
    register_features_model_celeron_11 ; "cpu_model" :
    IntelCeleron_2000 :> IntelCeleron_2970M &
                        IntelCeleron_2000E &
                        IntelCeleron_2002E &
                        IntelCeleron_2957U &
                        IntelCeleron_2961Y &
                        IntelCeleron_2981U &
                        IntelCeleron_2950M &
                        IntelCeleron_2955U &
                        IntelCeleron_2980U;
}

create_feature_hierarchy! {
    register_features_model_celeron_12 ; "cpu_model" :
    IntelCeleron_1000 :> IntelCeleron_1005M &
                        IntelCeleron_1017U &
                        IntelCeleron_1019Y &
                        IntelCeleron_1000M &
                        IntelCeleron_1007U &
                        IntelCeleron_1020E &
                        IntelCeleron_1020M &
                        IntelCeleron_1037U &
                        IntelCeleron_1047UE;
}
