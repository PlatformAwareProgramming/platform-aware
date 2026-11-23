use platform_aware_features::{CPUModel, create_feature_hierarchy};

// ------------------------------------------------------------------
// Itanium Family
// ------------------------------------------------------------------

create_feature_hierarchy! { register_features_model_itanium_1 ; "cpu_model" : CPUModel :> IntelItanium; }

// ------------------------------------------------------------------
// Itanium Series
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_itanium_2 ; "cpu_model" :
    IntelItanium :> IntelItanium_9700 &
                    IntelItanium_9500 &
                    IntelItanium_9300 &
                    IntelItanium_9100 &
                    IntelItanium_9000 &
                    IntelItanium_FSB;
}

create_feature_hierarchy! {
    register_features_model_itanium_3 ; "cpu_model" :
    IntelItanium_FSB :> IntelItanium_FSB_400 &
                       IntelItanium_FSB_533 &
                       IntelItanium_FSB_677;
}

// ------------------------------------------------------------------
// Itanium Models
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_itanium_4 ; "cpu_model" :
    IntelItanium_9700 :> IntelItanium_9720 &
                        IntelItanium_9740 &
                        IntelItanium_9750 &
                        IntelItanium_9760;
}

create_feature_hierarchy! {
    register_features_model_itanium_5 ; "cpu_model" :
    IntelItanium_9500 :> IntelItanium_9520 &
                        IntelItanium_9540 &
                        IntelItanium_9550 &
                        IntelItanium_9560;
}

create_feature_hierarchy! {
    register_features_model_itanium_6 ; "cpu_model" :
    IntelItanium_9300 :> IntelItanium_9310 &
                        IntelItanium_9320 &
                        IntelItanium_9330 &
                        IntelItanium_9340 &
                        IntelItanium_9350;
}

create_feature_hierarchy! {
    register_features_model_itanium_7 ; "cpu_model" :
    IntelItanium_9100 :> IntelItanium_9110N &
                        IntelItanium_9120N &
                        IntelItanium_9130M &
                        IntelItanium_9140M &
                        IntelItanium_9140N &
                        IntelItanium_9150M &
                        IntelItanium_9150N &
                        IntelItanium_9152M;
}

create_feature_hierarchy! {
    register_features_model_itanium_8 ; "cpu_model" :
    IntelItanium_9000 :> IntelItanium_9015;
}
