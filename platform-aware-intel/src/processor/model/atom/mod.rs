use platform_aware_features::{CPUModel, create_feature_hierarchy};

// Atom Family

create_feature_hierarchy! { register_features_model_atom_1 ; "cpu_model" : CPUModel :> IntelAtom;}

// Atom Series

create_feature_hierarchy! { 
    register_features_model_atom_2 ; "cpu_model" : 
    CPUModel :> IntelAtom_C & 
                IntelAtom_D & 
                IntelAtom_E & 
                IntelAtom_N & 
                IntelAtom_P & 
                IntelAtom_S & 
                IntelAtom_X & 
                IntelAtom_Z;
}

// Atom Models

create_feature_hierarchy! { 
    register_features_model_atom_3 ; "cpu_model" : 
    IntelAtom_C :> IntelAtom_C5115 &
                    IntelAtom_C5125 &
                    IntelAtom_C5310 &
                    IntelAtom_C5315 &
                    IntelAtom_C5320 &
                    IntelAtom_C5325 &
                    IntelAtom_C3338R &
                    IntelAtom_C3436L &
                    IntelAtom_C3558R &
                    IntelAtom_C3758R &
                    IntelAtom_C3336 &
                    IntelAtom_C3308 &
                    IntelAtom_C3508 &
                    IntelAtom_C3538 &
                    IntelAtom_C3558 &
                    IntelAtom_C3708 &
                    IntelAtom_C3750 &
                    IntelAtom_C3758 &
                    IntelAtom_C3808 &
                    IntelAtom_C3830 &
                    IntelAtom_C3850 &
                    IntelAtom_C3858 &
                    IntelAtom_C3950 &
                    IntelAtom_C3955 &
                    IntelAtom_C3958 &
                    IntelAtom_C2316 &
                    IntelAtom_C2516 &
                    IntelAtom_C3338 &
                    IntelAtom_C2308 &
                    IntelAtom_C2508 &
                    IntelAtom_C2338 &
                    IntelAtom_C2350 &
                    IntelAtom_C2358 &
                    IntelAtom_C2518 &
                    IntelAtom_C2530 &
                    IntelAtom_C2538 &
                    IntelAtom_C2550 &
                    IntelAtom_C2558 &
                    IntelAtom_C2718 &
                    IntelAtom_C2730 &
                    IntelAtom_C2738 &
                    IntelAtom_C2750 &
                    IntelAtom_C2758; 
}


create_feature_hierarchy! { 
    register_features_model_atom_4 ; "cpu_model" : 
    IntelAtom_D :> IntelAtom_D510 &
                    IntelAtom_D525 &
                    IntelAtom_D2500 &
                    IntelAtom_D2550 &
                    IntelAtom_D2560 &
                    IntelAtom_D2700 &
                    IntelAtom_D2701 &
                    IntelAtom_D2702 &
                    IntelAtom_D2703 &
                    IntelAtom_D2705 &
                    IntelAtom_D2706 &
                    IntelAtom_D2707 &
                    IntelAtom_D2708;
}

create_feature_hierarchy! { 
    register_features_model_atom_5 ; "cpu_model" : 
    IntelAtom_E :> IntelAtom_E3800 &
                    IntelAtom_E3805 &
                    IntelAtom_E3815 &
                    IntelAtom_E3825 &
                    IntelAtom_E3826 &
                    IntelAtom_E3827 &
                    IntelAtom_E3845 & 
                    IntelAtom_E3900 &
                    IntelAtom_E3930 &
                    IntelAtom_E3940 &
                    IntelAtom_E3950 &
                    IntelAtom_E3960 &
                    IntelAtom_E3970 &
                    IntelAtom_E3980; 
}

create_feature_hierarchy! { 
    register_features_model_atom_6 ; "cpu_model" : 
    IntelAtom_N :> IntelAtom_N2600 &
                    IntelAtom_N2800 &
                    IntelAtom_N2900 &
                    IntelAtom_N2700 &
                    IntelAtom_N2805 &
                    IntelAtom_N2810 &
                    IntelAtom_N2815 &
                    IntelAtom_N2820 &
                    IntelAtom_N2825 &
                    IntelAtom_N2830 &
                    IntelAtom_N2840 &
                    IntelAtom_N2845; 
}

create_feature_hierarchy! { 
    register_features_model_atom_7 ; "cpu_model" : 
    IntelAtom_P :> IntelAtom_P5322 &
                    IntelAtom_P5332 &
                    IntelAtom_P5342 &
                    IntelAtom_P5352 &
                    IntelAtom_P5362 &
                    IntelAtom_P5721 &
                    IntelAtom_P5731 &
                    IntelAtom_P5742 &
                    IntelAtom_P5752 &
                    IntelAtom_P5921B &
                    IntelAtom_P5931B &
                    IntelAtom_P5942B &
                    IntelAtom_P5962B;
}

create_feature_hierarchy! { 
    register_features_model_atom_8 ; "cpu_model" : 
    IntelAtom_S :> IntelAtom_S1220 &
                   IntelAtom_S1240 &
                   IntelAtom_S1260; 
}

create_feature_hierarchy! { 
    register_features_model_atom_9 ; "cpu_model" : 
    IntelAtom_X :> IntelAtom_6200FE &
                    IntelAtom_6211E &
                    IntelAtom_6212RE &
                    IntelAtom_6413E &
                    IntelAtom_6414RE &
                    IntelAtom_6425E &
                    IntelAtom_6425RE &
                    IntelAtom_6427FE &
                    IntelAtom_x3_3205RK &
                    IntelAtom_x3_C3235RK &
                    IntelAtom_x3_C3265RK &
                    IntelAtom_x3_C3295RK &
    //                IntelAtom_E3930 &
    //                IntelAtom_E3940 &
    //                IntelAtom_E3950 &
                    IntelAtom_x5_Z8550 &
                    IntelAtom_x7_Z8750 &
                    IntelAtom_x5_Z8330 &
                    IntelAtom_x5_Z8350 &
                    IntelAtom_E8000 &
                    IntelAtom_x3_C3200RK &
                    IntelAtom_x3_C3405 &
                    IntelAtom_x3_C3230RK &
                    IntelAtom_x3_C3445 &
                    IntelAtom_x5_Z8300 &
                    IntelAtom_x5_Z8500 &
                    IntelAtom_x7_Z8700; 
}


create_feature_hierarchy! { 
    register_features_model_atom_10 ; "cpu_model" : 
    IntelAtom_Z :> IntelAtom_Z3590 &
                    IntelAtom_Z3570 &
                    IntelAtom_Z3736F &
                    IntelAtom_Z3736G &
                    IntelAtom_Z3530 &
                    IntelAtom_Z3785 &
                    IntelAtom_Z3560 &
                    IntelAtom_Z3580 &
                    IntelAtom_Z3735F &
                    IntelAtom_Z3735G &
                    IntelAtom_Z3460 &
                    IntelAtom_Z3480 &
                    IntelAtom_Z3735D &
                    IntelAtom_Z3735E &
                    IntelAtom_Z3775D &
                    IntelAtom_Z3795 &
                    IntelAtom_Z3745 &
                    IntelAtom_Z3745D &
                    IntelAtom_Z3775 &
                    IntelAtom_Z3740 &
                    IntelAtom_Z3740D &
                    IntelAtom_Z3770 &
                    IntelAtom_Z3770D &
                    IntelAtom_Z2520 &
                    IntelAtom_Z2560 &
                    IntelAtom_Z2580 &
                    IntelAtom_Z2420 &
                    IntelAtom_Z2480 &
                    IntelAtom_Z2760 &
                    IntelAtom_Z2460;
}