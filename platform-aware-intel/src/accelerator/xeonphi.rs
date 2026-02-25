
use platform_aware_features::{AccModel, create_feature_hierarchy};

// ------------------------------------------------------------------
// Xeon Phi family
// ------------------------------------------------------------------

create_feature_hierarchy! { register_features_model_xeonphi_1 ; "acc_model" : AccModel :> IntelXeonPhi; }

// ------------------------------------------------------------------
// Xeon Phi series
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_xeonphi_2 ; "acc_model" :
    AccModel :> IntelXeonPhi_72x5 &
                IntelXeonPhi_x100 &
                IntelXeonPhi_x200;
}

// ------------------------------------------------------------------
// Xeon Phi models
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_xeonphi_3 ; "acc_model" :
    IntelXeonPhi_x100 :> IntelXeonPhi_7120A &
                        IntelXeonPhi_7120D &
                        IntelXeonPhi_3120A &
                        IntelXeonPhi_3120P &
                        IntelXeonPhi_5120D &
                        IntelXeonPhi_7120P &
                        IntelXeonPhi_7120X &
                        IntelXeonPhi_5110P;
}

create_feature_hierarchy! {
    register_features_model_xeonphi_4 ; "acc_model" :
    IntelXeonPhi_x200 :> IntelXeonPhi_7210 &
                        IntelXeonPhi_7210F &
                        IntelXeonPhi_7230 &
                        IntelXeonPhi_7230F &
                        IntelXeonPhi_7250 &
                        IntelXeonPhi_7250F &
                        IntelXeonPhi_7290 &
                        IntelXeonPhi_7290F;
}

create_feature_hierarchy! {
    register_features_model_xeonphi_5 ; "acc_model" :
    IntelXeonPhi_72x5 :> IntelXeonPhi_7235 &
                        IntelXeonPhi_7285 &
                        IntelXeonPhi_7295;
}
