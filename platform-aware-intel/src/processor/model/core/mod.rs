

use platform_aware_features::{CPUModel, create_feature_hierarchy};

// ------------------------------------------------------------------
// Core Family
// ------------------------------------------------------------------

create_feature_hierarchy! { register_features_model_core_1 ; "cpu_model" : CPUModel :> IntelCore; }

// ------------------------------------------------------------------
// Core Series
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_2 ; "cpu_model" :
    IntelCore :> IntelCore_X &
                 IntelCore_i9 &
                 IntelCore_i7 &
                 IntelCore_i5 &
                 IntelCore_i3 &
                 IntelCore_M;
}

// ------------------------------------------------------------------
// Generations (sub-series)
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_3 ; "cpu_model" :
    IntelCore_i9 :> IntelCore_i9_g8 &
                   IntelCore_i9_g9 &
                   IntelCore_i9_g10 &
                   IntelCore_i9_g11 &
                   IntelCore_i9_g12;
}

create_feature_hierarchy! {
    register_features_model_core_4 ; "cpu_model" :
    IntelCore_i7 :> IntelCore_i7_g4 &
                   IntelCore_i7_g5 &
                   IntelCore_i7_g6 &
                   IntelCore_i7_g7 &
                   IntelCore_i7_g8 &
                   IntelCore_i7_g9 &
                   IntelCore_i7_g10 &
                   IntelCore_i7_g11 &
                   IntelCore_i7_g12;
}

create_feature_hierarchy! {
    register_features_model_core_5 ; "cpu_model" :
    IntelCore_i5 :> IntelCore_i5_g4 &
                   IntelCore_i5_g5 &
                   IntelCore_i5_g6 &
                   IntelCore_i5_g7 &
                   IntelCore_i5_g8 &
                   IntelCore_i5_g9 &
                   IntelCore_i5_g10 &
                   IntelCore_i5_g11 &
                   IntelCore_i5_g12;
}

create_feature_hierarchy! {
    register_features_model_core_6 ; "cpu_model" :
    IntelCore_i3 :> IntelCore_i3_g4 &
                   IntelCore_i3_g5 &
                   IntelCore_i3_g6 &
                   IntelCore_i3_g7 &
                   IntelCore_i3_g8 &
                   IntelCore_i3_g9 &
                   IntelCore_i3_g10 &
                   IntelCore_i3_g11 &
                   IntelCore_i3_g12;
}

create_feature_hierarchy! {
    register_features_model_core_7 ; "cpu_model" :
    IntelCore_M :> IntelCore_M_g5 &
                  IntelCore_M_g6 &
                  IntelCore_M_g7 &
                  IntelCore_M_g8;
}

// ------------------------------------------------------------------
// Models: IntelCore_X
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_x_1 ; "cpu_model" :
    IntelCore_X :> IntelCore_i7_3940XM &
                  IntelCore_i7_3920XM &
                  IntelCore_i7_4930MX &
                  IntelCore_i7_4940MX &
                  IntelCore_i7_3970X &
                  IntelCore_i7_3820 &
                  IntelCore_i7_3930K &
                  IntelCore_i7_3960X &
                  IntelCore_i7_4820K &
                  IntelCore_i7_4930K &
                  IntelCore_i7_4960X &
                  IntelCore_i7_5820K &
                  IntelCore_i7_5930K &
                  IntelCore_i7_5960X &
                  IntelCore_i7_6800K &
                  IntelCore_i7_6850K &
                  IntelCore_i7_6900K &
                  IntelCore_i7_6950X &
                  IntelCore_i5_7640X &
                  IntelCore_i7_7740X &
                  IntelCore_i7_7800X &
                  IntelCore_i7_7820X &
                  IntelCore_i9_7900X &
                  IntelCore_i9_7920X &
                  IntelCore_i9_7940X &
                  IntelCore_i9_7960X &
                  IntelCore_i9_7980XE &
                  IntelCore_i7_9800X &
                  IntelCore_i9_9820X &
                  IntelCore_i9_9900X &
                  IntelCore_i9_9920X &
                  IntelCore_i9_9940X &
                  IntelCore_i9_9960X &
                  IntelCore_i9_9980XE &
                  IntelCore_i9_10900X &
                  IntelCore_i9_10920X &
                  IntelCore_i9_10940X &
                  IntelCore_i9_10980XE;
}

// ------------------------------------------------------------------
// Models: IntelCore_i9_g12
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i9_g12_1 ; "cpu_model" :
    IntelCore_i9_g12 :> IntelCore_i9_12900 &
                       IntelCore_i9_12900HX &
                       IntelCore_i9_12950HX &
                       IntelCore_i9_12900KS &
                       IntelCore_i9_12900E &
                       IntelCore_i9_12900F &
                       IntelCore_i9_12900H &
                       IntelCore_i9_12900HK &
                       IntelCore_i9_12900T &
                       IntelCore_i9_12900TE &
                       IntelCore_i9_12900K &
                       IntelCore_i9_12900KF;
}

// ------------------------------------------------------------------
// Models: IntelCore_i9_g11
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i9_g11_1 ; "cpu_model" :
    IntelCore_i9_g11 :> IntelCore_i9_11900KB &
                       IntelCore_i9_11900H &
                       IntelCore_i9_11950H &
                       IntelCore_i9_11980HK &
                       IntelCore_i9_11900 &
                       IntelCore_i9_11900F &
                       IntelCore_i9_11900K &
                       IntelCore_i9_11900KF &
                       IntelCore_i9_11900T;
}

// ------------------------------------------------------------------
// Models: IntelCore_i9_g10
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i9_g10_1 ; "cpu_model" :
    IntelCore_i9_g10 :> IntelCore_i9_10850K &
                       IntelCore_i9_10900 &
                       IntelCore_i9_10900E &
                       IntelCore_i9_10900K &
                       IntelCore_i9_10900T &
                       IntelCore_i9_10900TE &
                       IntelCore_i9_10885H &
                       IntelCore_i9_10980HK &
                       IntelCore_i9_10900F &
                       IntelCore_i9_10900KF;
}

// ------------------------------------------------------------------
// Models: IntelCore_i9_g9
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i9_g9_1 ; "cpu_model" :
    IntelCore_i9_g9 :> IntelCore_i9_9980HK &
                      IntelCore_i9_9880H &
                      IntelCore_i9_9900 &
                      IntelCore_i9_9900T &
                      IntelCore_i9_9900K &
                      IntelCore_i9_9900KS &
                      IntelCore_i9_9900KF;
}

// ------------------------------------------------------------------
// Models: IntelCore_i9_g8
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i9_g8_1 ; "cpu_model" :
    IntelCore_i9_g8 :> IntelCore_i9_8950HK;
}

// ------------------------------------------------------------------
// Models: IntelCore_i7_g12
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i7_g12_1 ; "cpu_model" :
    IntelCore_i7_g12 :> IntelCore_i7_1255U &
                       IntelCore_i7_1265U &
                       IntelCore_i7_12700 &
                       IntelCore_i7_1260P &
                       IntelCore_i7_12650HX &
                       IntelCore_i7_12800HX &
                       IntelCore_i7_12850HX &
                       IntelCore_i7_1265UE &
                       IntelCore_i7_1270PE &
                       IntelCore_i7_1250U &
                       IntelCore_i7_1260U &
                       IntelCore_i7_1270P &
                       IntelCore_i7_1280P &
                       IntelCore_i7_12650H &
                       IntelCore_i7_12700E &
                       IntelCore_i7_12700F &
                       IntelCore_i7_12700H &
                       IntelCore_i7_12700T &
                       IntelCore_i7_12700TE &
                       IntelCore_i7_12800H &
                       IntelCore_i7_12800HE &
                       IntelCore_i7_12700K &
                       IntelCore_i7_12700KF;
}

// ------------------------------------------------------------------
// Models: IntelCore_i7_g11
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i7_g11_1 ; "cpu_model" :
    IntelCore_i7_g11 :> IntelCore_i7_11370H &
                       IntelCore_i7_11375H &
                       IntelCore_i7_1180G7 &
                       IntelCore_i7_1185G7E &
                       IntelCore_i7_1185GRE &
                       IntelCore_i7_1160G7 &
                       IntelCore_i7_1165G7 &
                       IntelCore_i7_1185G7 &
                       IntelCore_i7_11700B &
                       IntelCore_i7_11800H &
                       IntelCore_i7_11850HE &
                       IntelCore_i7_11600H &
                       IntelCore_i7_11850H &
                       IntelCore_i7_11390H &
                       IntelCore_i7_1195G7 &
                       IntelCore_i7_11700 &
                       IntelCore_i7_11700F &
                       IntelCore_i7_11700K &
                       IntelCore_i7_11700KF &
                       IntelCore_i7_11700T;
}

// ------------------------------------------------------------------
// Models: IntelCore_i7_g10
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i7_g10_1 ; "cpu_model" :
    IntelCore_i7_g10 :> IntelCore_i7_10710U &
                       IntelCore_i7_1068NG7 &
                       IntelCore_i7_1060G7 &
                       IntelCore_i7_1065G7 &
                       IntelCore_i7_10700 &
                       IntelCore_i7_10700E &
                       IntelCore_i7_10700K &
                       IntelCore_i7_10700T &
                       IntelCore_i7_10700TE &
                       IntelCore_i7_10870H &
                       IntelCore_i7_10610U &
                       IntelCore_i7_10810U &
                       IntelCore_i7_10750H &
                       IntelCore_i7_10850H &
                       IntelCore_i7_10875H &
                       IntelCore_i7_10700F &
                       IntelCore_i7_10700KF &
                       IntelCore_i7_10510U &
                       IntelCore_i7_10510Y;
}

// ------------------------------------------------------------------
// Models: IntelCore_i7_g9
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i7_g9_1 ; "cpu_model" :
    IntelCore_i7_g9 :> IntelCore_i7_9750H &
                      IntelCore_i7_9850H &
                      IntelCore_i7_9700E &
                      IntelCore_i7_9700TE &
                      IntelCore_i7_9850HE &
                      IntelCore_i7_9850HL &
                      IntelCore_i7_9700 &
                      IntelCore_i7_9700T &
                      IntelCore_i7_9700K &
                      IntelCore_i7_9700F &
                      IntelCore_i7_9750HF &
                      IntelCore_i7_9700KF;
}

// ------------------------------------------------------------------
// Models: IntelCore_i7_g8
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i7_g8_1 ; "cpu_model" :
    IntelCore_i7_g8 :> IntelCore_i7_8565U &
                      IntelCore_i7_8665U &
                      IntelCore_i7_8559U &
                      IntelCore_i7_8705G &
                      IntelCore_i7_8809G &
                      IntelCore_i7_8569U &
                      IntelCore_i7_8557U &
                      IntelCore_i7_8706G &
                      IntelCore_i7_8709G &
                      IntelCore_i7_8086K &
                      IntelCore_i7_8700 &
                      IntelCore_i7_8700B &
                      IntelCore_i7_8700T &
                      IntelCore_i7_8750H &
                      IntelCore_i7_8850H &
                      IntelCore_i7_8665UE &
                      IntelCore_i7_8550U &
                      IntelCore_i7_8650U &
                      IntelCore_i7_8500Y;
}

// ------------------------------------------------------------------
// Models: IntelCore_i7_g7
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i7_g7_1 ; "cpu_model" :
    IntelCore_i7_g7 :> IntelCore_i7_7567U &
                      IntelCore_i7_7700 &
                      IntelCore_i7_7700HQ &
                      IntelCore_i7_7700K &
                      IntelCore_i7_7700T &
                      IntelCore_i7_7820EQ &
                      IntelCore_i7_7820HK &
                      IntelCore_i7_7820HQ &
                      IntelCore_i7_7920HQ &
                      IntelCore_i7_7600U &
                      IntelCore_i7_7500U &
                      IntelCore_i7_7560U &
                      IntelCore_i7_7660U &
                      IntelCore_i7_7Y75;
}

// ------------------------------------------------------------------
// Models: IntelCore_i7_g6
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i7_g6_1 ; "cpu_model" :
    IntelCore_i7_g6 :> IntelCore_i7_6567U &
                      IntelCore_i7_6660U &
                      IntelCore_i7_6560U &
                      IntelCore_i7_6650U &
                      IntelCore_i7_6700TE &
                      IntelCore_i7_6820EQ &
                      IntelCore_i7_6822EQ &
                      IntelCore_i7_6700 &
                      IntelCore_i7_6700HQ &
                      IntelCore_i7_6700T &
                      IntelCore_i7_6820HK &
                      IntelCore_i7_6820HQ &
                      IntelCore_i7_6920HQ &
                      IntelCore_i7_6700K &
                      IntelCore_i7_6500U &
                      IntelCore_i7_6600U &
                      IntelCore_i7_6785R &
                      IntelCore_i7_6770HQ &
                      IntelCore_i7_6870HQ &
                      IntelCore_i7_6970HQ;
}

// ------------------------------------------------------------------
// Models: IntelCore_i7_g5
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i7_g5_1 ; "cpu_model" :
    IntelCore_i7_g5 :> IntelCore_i7_5557U &
                      IntelCore_i7_5550U &
                      IntelCore_i7_5650U &
                      IntelCore_i7_5500U &
                      IntelCore_i7_5600U &
                      IntelCore_i7_5700HQ &
                      IntelCore_i7_5750HQ &
                      IntelCore_i7_5775C &
                      IntelCore_i7_5775R &
                      IntelCore_i7_5850EQ &
                      IntelCore_i7_5850HQ &
                      IntelCore_i7_5950HQ &
                      IntelCore_i7_5700EQ;
}

// ------------------------------------------------------------------
// Models: IntelCore_i7_g4
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i7_g4_1 ; "cpu_model" :
    IntelCore_i7_g4 :> IntelCore_i7_4578U &
                      IntelCore_i7_4558U &
                      IntelCore_i7_4550U &
                      IntelCore_i7_4650U &
                      IntelCore_i7_4720HQ &
                      IntelCore_i7_4722HQ &
                      IntelCore_i7_4790K &
                      IntelCore_i7_4785T &
                      IntelCore_i7_4790 &
                      IntelCore_i7_4790S &
                      IntelCore_i7_4790T &
                      IntelCore_i7_4710HQ &
                      IntelCore_i7_4710MQ &
                      IntelCore_i7_4712HQ &
                      IntelCore_i7_4712MQ &
                      IntelCore_i7_4610M &
                      IntelCore_i7_4810MQ &
                      IntelCore_i7_4910MQ &
                      IntelCore_i7_4600M &
                      IntelCore_i7_4771 &
                      IntelCore_i7_4700EQ &
                      IntelCore_i7_4700HQ &
                      IntelCore_i7_4700MQ &
                      IntelCore_i7_4702HQ &
                      IntelCore_i7_4702MQ &
                      IntelCore_i7_4765T &
                      IntelCore_i7_4770 &
                      IntelCore_i7_4770K &
                      IntelCore_i7_4770S &
                      IntelCore_i7_4770T &
                      IntelCore_i7_4770TE &
                      IntelCore_i7_4800MQ &
                      IntelCore_i7_4900MQ &
                      IntelCore_i7_4170 &
                      IntelCore_i7_4170T &
                      IntelCore_i7_4510U &
                      IntelCore_i7_4600U &
                      IntelCore_i7_4500U &
                      IntelCore_i7_4610Y &
                      IntelCore_i7_4770HQ &
                      IntelCore_i7_4870HQ &
                      IntelCore_i7_4980HQ &
                      IntelCore_i7_4760HQ &
                      IntelCore_i7_4860HQ &
                      IntelCore_i7_4960HQ &
                      IntelCore_i7_4750HQ &
                      IntelCore_i7_4850HQ &
                      IntelCore_i7_4950HQ &
                      IntelCore_i7_4770R &
                      IntelCore_i7_4700EC &
                      IntelCore_i7_4702EC;
}

// ------------------------------------------------------------------
// Models: IntelCore_i5_g12
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i5_g12_1 ; "cpu_model" :
    IntelCore_i5_g12 :> IntelCore_i5_1235U &
                       IntelCore_i5_1245U &
                       IntelCore_i5_1240P &
                       IntelCore_i5_12450HX &
                       IntelCore_i5_12600HX &
                       IntelCore_i5_1245UE &
                       IntelCore_i5_1250PE &
                       IntelCore_i5_1230U &
                       IntelCore_i5_1240U &
                       IntelCore_i5_1250P &
                       IntelCore_i5_12400 &
                       IntelCore_i5_12400F &
                       IntelCore_i5_12400T &
                       IntelCore_i5_12450H &
                       IntelCore_i5_12500 &
                       IntelCore_i5_12500E &
                       IntelCore_i5_12500H &
                       IntelCore_i5_12500T &
                       IntelCore_i5_12500TE &
                       IntelCore_i5_12600 &
                       IntelCore_i5_12600H &
                       IntelCore_i5_12600HE &
                       IntelCore_i5_12600T &
                       IntelCore_i5_12600K &
                       IntelCore_i5_12600KF;
}

// ------------------------------------------------------------------
// Models: IntelCore_i5_g11
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i5_g11_1 ; "cpu_model" :
    IntelCore_i5_g11 :> IntelCore_i5_11300H &
                       IntelCore_i5_1140G7 &
                       IntelCore_i5_1145G7 &
                       IntelCore_i5_1145G7E &
                       IntelCore_i5_1145GRE &
                       IntelCore_i5_1130G7 &
                       IntelCore_i5_1135G7 &
                       IntelCore_i5_11400H &
                       IntelCore_i5_11260H &
                       IntelCore_i5_11500HE &
                       IntelCore_i5_11500H &
                       IntelCore_i5_11320H &
                       IntelCore_i5_1155G7 &
                       IntelCore_i5_11400 &
                       IntelCore_i5_11400F &
                       IntelCore_i5_11400T &
                       IntelCore_i5_11500 &
                       IntelCore_i5_11500T &
                       IntelCore_i5_11600 &
                       IntelCore_i5_11600K &
                       IntelCore_i5_11600KF &
                       IntelCore_i5_11600T;
}

// ------------------------------------------------------------------
// Models: IntelCore_i5_g10
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i5_g10_1 ; "cpu_model" :
    IntelCore_i5_g10 :> IntelCore_i5_10210U &
                       IntelCore_i5_1038NG7 &
                       IntelCore_i5_1030G4 &
                       IntelCore_i5_1030G7 &
                       IntelCore_i5_1035G4 &
                       IntelCore_i5_1035G7 &
                       IntelCore_i5_10505 &
                       IntelCore_i5_10400 &
                       IntelCore_i5_10400T &
                       IntelCore_i5_10500 &
                       IntelCore_i5_10500E &
                       IntelCore_i5_10500T &
                       IntelCore_i5_10500TE &
                       IntelCore_i5_10600 &
                       IntelCore_i5_10600K &
                       IntelCore_i5_10600T &
                       IntelCore_i5_10500H &
                       IntelCore_i5_10200H &
                       IntelCore_i5_10310U &
                       IntelCore_i5_10300H &
                       IntelCore_i5_10400H &
                       IntelCore_i5_10210Y &
                       IntelCore_i5_10310Y &
                       IntelCore_i5_1035G1 &
                       IntelCore_i5_10400F &
                       IntelCore_i5_10600KF;
}

// ------------------------------------------------------------------
// Models: IntelCore_i5_g9
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i5_g9_1 ; "cpu_model" :
    IntelCore_i5_g9 :> IntelCore_i5_9300H &
                      IntelCore_i5_9500E &
                      IntelCore_i5_9500TE &
                      IntelCore_i5_9400H &
                      IntelCore_i5_9400T &
                      IntelCore_i5_9500 &
                      IntelCore_i5_9500T &
                      IntelCore_i5_9600 &
                      IntelCore_i5_9600T &
                      IntelCore_i5_9400 &
                      IntelCore_i5_9600K &
                      IntelCore_i5_9500F &
                      IntelCore_i5_9300HF &
                      IntelCore_i5_9400F &
                      IntelCore_i5_9600KF;
}

// ------------------------------------------------------------------
// Models: IntelCore_i5_g8
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i5_g8_1 ; "cpu_model" :
    IntelCore_i5_g8 :> IntelCore_i5_8265U &
                      IntelCore_i5_8365U &
                      IntelCore_i5_8259U &
                      IntelCore_i5_8279U &
                      IntelCore_i5_8269U &
                      IntelCore_i5_8257U &
                      IntelCore_i5_8305G &
                      IntelCore_i5_8400 &
                      IntelCore_i5_8400B &
                      IntelCore_i5_8400H &
                      IntelCore_i5_8400T &
                      IntelCore_i5_8500 &
                      IntelCore_i5_8500B &
                      IntelCore_i5_8500T &
                      IntelCore_i5_8600 &
                      IntelCore_i5_8600T &
                      IntelCore_i5_8300H &
                      IntelCore_i5_8600K &
                      IntelCore_i5_8260U &
                      IntelCore_i5_8250U &
                      IntelCore_i5_8350U &
                      IntelCore_i5_8310Y &
                      IntelCore_i5_8210Y &
                      IntelCore_i5_8200Y;
}

// ------------------------------------------------------------------
// Models: IntelCore_i5_g7
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i5_g7_1 ; "cpu_model" :
    IntelCore_i5_g7 :> IntelCore_i5_7300U &
                      IntelCore_i5_7260U &
                      IntelCore_i5_7300HQ &
                      IntelCore_i5_7400 &
                      IntelCore_i5_7400T &
                      IntelCore_i5_7440EQ &
                      IntelCore_i5_7440HQ &
                      IntelCore_i5_7442EQ &
                      IntelCore_i5_7500 &
                      IntelCore_i5_7500T &
                      IntelCore_i5_7600 &
                      IntelCore_i5_7600K &
                      IntelCore_i5_7600T &
                      IntelCore_i5_7200U &
                      IntelCore_i5_7Y57 &
                      IntelCore_i5_7Y54 &
                      IntelCore_i5_7267U &
                      IntelCore_i5_7287U &
                      IntelCore_i5_7360U;
}

// ------------------------------------------------------------------
// Models: IntelCore_i5_g5 (and i5 models listed under g5)
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i5_g5_1 ; "cpu_model" :
    IntelCore_i5_g5 :> IntelCore_i5_6267U &
                      IntelCore_i5_6287U &
                      IntelCore_i5_6260U &
                      IntelCore_i5_6360U &
                      IntelCore_i5_6440EQ &
                      IntelCore_i5_6442EQ &
                      IntelCore_i5_6500TE &
                      IntelCore_i5_6300HQ &
                      IntelCore_i5_6400 &
                      IntelCore_i5_6400T &
                      IntelCore_i5_6440HQ &
                      IntelCore_i5_6500 &
                      IntelCore_i5_6500T &
                      IntelCore_i5_6600 &
                      IntelCore_i5_6600T &
                      IntelCore_i5_6600K &
                      IntelCore_i5_6200U &
                      IntelCore_i5_6300U &
                      IntelCore_i5_6402P &
                      IntelCore_i5_5250U &
                      IntelCore_i5_5350U &
                      IntelCore_i5_5015U &
                      IntelCore_i5_5020U &
                      IntelCore_i5_5005U &
                      IntelCore_i5_5010U &
                      IntelCore_i5_5200U &
                      IntelCore_i5_5300U &
                      IntelCore_i5_5350H &
                      IntelCore_i5_5575R &
                      IntelCore_i5_5675C &
                      IntelCore_i5_5675R &
                      IntelCore_i5_6350HQ &
                      IntelCore_i5_6585R &
                      IntelCore_i5_6685R;
}

// ------------------------------------------------------------------
// Models: IntelCore_i5_g4
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i5_g4_1 ; "cpu_model" :
    IntelCore_i5_g4 :> IntelCore_i5_4278U &
                      IntelCore_i5_4308U &
                      IntelCore_i5_4258U &
                      IntelCore_i5_4288U &
                      IntelCore_i5_4260U &
                      IntelCore_i5_4360U &
                      IntelCore_i5_4250U &
                      IntelCore_i5_4350U &
                      IntelCore_i5_4210H &
                      IntelCore_i5_4690K &
                      IntelCore_i5_4460 &
                      IntelCore_i5_4460S &
                      IntelCore_i5_4460T &
                      IntelCore_i5_4590 &
                      IntelCore_i5_4590S &
                      IntelCore_i5_4590T &
                      IntelCore_i5_4690 &
                      IntelCore_i5_4690S &
                      IntelCore_i5_4690T &
                      IntelCore_i5_4210M &
                      IntelCore_i5_4410E &
                      IntelCore_i5_4422E &
                      IntelCore_i5_4310M &
                      IntelCore_i5_4340M &
                      IntelCore_i5_4200H &
                      IntelCore_i5_4200M &
                      IntelCore_i5_4300M &
                      IntelCore_i5_4330M &
                      IntelCore_i5_4400E &
                      IntelCore_i5_4402E &
                      IntelCore_i5_4440 &
                      IntelCore_i5_4440S &
                      IntelCore_i5_4430 &
                      IntelCore_i5_4430S &
                      IntelCore_i5_4570 &
                      IntelCore_i5_4570S &
                      IntelCore_i5_4570T &
                      IntelCore_i5_4570TE &
                      IntelCore_i5_4670 &
                      IntelCore_i5_4670K &
                      IntelCore_i5_4670S &
                      IntelCore_i5_4670T &
                      IntelCore_i5_4210U &
                      IntelCore_i5_4310U &
                      IntelCore_i5_4300U &
                      IntelCore_i5_4200U &
                      IntelCore_i5_4220Y &
                      IntelCore_i5_4202Y &
                      IntelCore_i5_4210Y &
                      IntelCore_i5_4300Y &
                      IntelCore_i5_4302Y &
                      IntelCore_i5_4200Y &
                      IntelCore_i5_4570R &
                      IntelCore_i5_4670R &
                      IntelCore_i5_4402EC;
}

// ------------------------------------------------------------------
// Models: IntelCore_i3_g12
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i3_g12_1 ; "cpu_model" :
    IntelCore_i3_g12 :> IntelCore_i3_1215U &
                       IntelCore_i3_12300HE &
                       IntelCore_i3_1215UE &
                       IntelCore_i3_1220PE &
                       IntelCore_i3_1210U &
                       IntelCore_i3_1220P &
                       IntelCore_i3_12100 &
                       IntelCore_i3_12100E &
                       IntelCore_i3_12100F &
                       IntelCore_i3_12100T &
                       IntelCore_i3_12100TE &
                       IntelCore_i3_12300 &
                       IntelCore_i3_12300T;
}

// ------------------------------------------------------------------
// Models: IntelCore_i3_g11
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i3_g11_1 ; "cpu_model" :
    IntelCore_i3_g11 :> IntelCore_i3_1115G4 &
                       IntelCore_i3_11100HE &
                       IntelCore_i3_1115G4E &
                       IntelCore_i3_1115GRE &
                       IntelCore_i3_1120G4 &
                       IntelCore_i3_1125G4 &
                       IntelCore_i3_1110G4;
}

// ------------------------------------------------------------------
// Models: IntelCore_i3_g10
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i3_g10_1 ; "cpu_model" :
    IntelCore_i3_g10 :> IntelCore_i3_10110U &
                       IntelCore_i3_10105 &
                       IntelCore_i3_10105T &
                       IntelCore_i3_10305 &
                       IntelCore_i3_10305T &
                       IntelCore_i3_10325 &
                       IntelCore_i3_10100 &
                       IntelCore_i3_10100E &
                       IntelCore_i3_10100T &
                       IntelCore_i3_10100TE &
                       IntelCore_i3_10300 &
                       IntelCore_i3_10300T &
                       IntelCore_i3_10320 &
                       IntelCore_i3_1000G4 &
                       IntelCore_i3_1000G1 &
                       IntelCore_i3_1005G1 &
                       IntelCore_i3_10110Y &
                       IntelCore_i3_10100Y &
                       IntelCore_i3_10105F &
                       IntelCore_i3_10100F;
}

// ------------------------------------------------------------------
// Models: IntelCore_i3_g9
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i3_g9_1 ; "cpu_model" :
    IntelCore_i3_g9 :> IntelCore_i3_9100E &
                      IntelCore_i3_9100HL &
                      IntelCore_i3_9100TE &
                      IntelCore_i3_9100 &
                      IntelCore_i3_9100T &
                      IntelCore_i3_9300 &
                      IntelCore_i3_9300T &
                      IntelCore_i3_9320 &
                      IntelCore_i3_9350K &
                      IntelCore_i3_9100F &
                      IntelCore_i3_9350KF;
}

// ------------------------------------------------------------------
// Models: IntelCore_i3_g8
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i3_g8_1 ; "cpu_model" :
    IntelCore_i3_g8 :> IntelCore_i3_8145U &
                      IntelCore_i3_8109U &
                      IntelCore_i3_8140U &
                      IntelCore_i3_8145UE &
                      IntelCore_i3_8130U &
                      IntelCore_i3_8100B &
                      IntelCore_i3_8100H &
                      IntelCore_i3_8100T &
                      IntelCore_i3_8300 &
                      IntelCore_i3_8300T &
                      IntelCore_i3_8100 &
                      IntelCore_i3_8350K;
}

// ------------------------------------------------------------------
// Models: IntelCore_i3_g7
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i3_g7_1 ; "cpu_model" :
    IntelCore_i3_g7 :> IntelCore_i3_7100U &
                      IntelCore_i3_7167U &
                      IntelCore_i3_7300 &
                      IntelCore_i3_7100 &
                      IntelCore_i3_7100E &
                      IntelCore_i3_7100H &
                      IntelCore_i3_7100T &
                      IntelCore_i3_7101E &
                      IntelCore_i3_7101TE &
                      IntelCore_i3_7102E &
                      IntelCore_i3_7300T &
                      IntelCore_i3_7320 &
                      IntelCore_i3_7350K &
                      IntelCore_i3_7020U &
                      IntelCore_i3_7130U;
}

// ------------------------------------------------------------------
// Models: IntelCore_i3_g6
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i3_g6_1 ; "cpu_model" :
    IntelCore_i3_g6 :> IntelCore_i3_6157U &
                      IntelCore_i3_6167U &
                      IntelCore_i3_6100E &
                      IntelCore_i3_6100TE &
                      IntelCore_i3_6102E &
                      IntelCore_i3_6100 &
                      IntelCore_i3_6100H &
                      IntelCore_i3_6100T &
                      IntelCore_i3_6300 &
                      IntelCore_i3_6300T &
                      IntelCore_i3_6320 &
                      IntelCore_i3_6006U &
                      IntelCore_i3_6100U &
                      IntelCore_i3_6098P;
}

// ------------------------------------------------------------------
// Models: IntelCore_i3_g5
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i3_g5_1 ; "cpu_model" :
    IntelCore_i3_g5 :> IntelCore_i3_5157U &
                      IntelCore_i3_5015U &
                      IntelCore_i3_5020U &
                      IntelCore_i3_5005U &
                      IntelCore_i3_5010U;
}

// ------------------------------------------------------------------
// Models: IntelCore_i3_g4
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i3_g4_1 ; "cpu_model" :
    IntelCore_i3_g4 :> IntelCore_i3_4158U &
                      IntelCore_i3_4370T &
                      IntelCore_i3_4360T &
                      IntelCore_i3_4370 &
                      IntelCore_i3_4340TE &
                      IntelCore_i3_4350 &
                      IntelCore_i3_4350T &
                      IntelCore_i3_4360 &
                      IntelCore_i3_4110E &
                      IntelCore_i3_4110M &
                      IntelCore_i3_4112E &
                      IntelCore_i3_4000M &
                      IntelCore_i3_4100E &
                      IntelCore_i3_4100M &
                      IntelCore_i3_4102E &
                      IntelCore_i3_4330 &
                      IntelCore_i3_4330T &
                      IntelCore_i3_4330TE &
                      IntelCore_i3_4340 &
                      IntelCore_i3_4160 &
                      IntelCore_i3_4160T &
                      IntelCore_i3_4150 &
                      IntelCore_i3_4150T &
                      IntelCore_i3_4025U &
                      IntelCore_i3_4030U &
                      IntelCore_i3_4120U &
                      IntelCore_i3_4005U &
                      IntelCore_i3_4130 &
                      IntelCore_i3_4130T &
                      IntelCore_i3_4010U &
                      IntelCore_i3_4100U &
                      IntelCore_i3_4030Y &
                      IntelCore_i3_4012Y &
                      IntelCore_i3_4020Y;
}

// ------------------------------------------------------------------
// Models: IntelCore_M_g8 / g7 / g6 / g5
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_m_g8_1 ; "cpu_model" :
    IntelCore_M_g8 :> IntelCore_M3_8100Y;
}

create_feature_hierarchy! {
    register_features_model_core_m_g7_1 ; "cpu_model" :
    IntelCore_M_g7 :> IntelCore_M3_7Y32 &
                     IntelCore_M3_7Y30;
}

create_feature_hierarchy! {
    register_features_model_core_m_g6_1 ; "cpu_model" :
    IntelCore_M_g6 :> IntelCore_M3_6Y30 &
                     IntelCore_M5_6Y54 &
                     IntelCore_M5_6Y57 &
                     IntelCore_M7_6Y75;
}

create_feature_hierarchy! {
    register_features_model_core_m_g5_1 ; "cpu_model" :
    IntelCore_M_g5 :> IntelCore_5Y10c &
                     IntelCore_5Y31 &
                     IntelCore_5Y51 &
                     IntelCore_5Y71 &
                     IntelCore_5Y10 &
                     IntelCore_5Y10a &
                     IntelCore_5Y70;
}

// ------------------------------------------------------------------
// Generations (sub-series) — additions
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_3_g13_g14 ; "cpu_model" :
    IntelCore_i9 :> IntelCore_i9_g13 &
                   IntelCore_i9_g14;
}

create_feature_hierarchy! {
    register_features_model_core_4_g13_g14 ; "cpu_model" :
    IntelCore_i7 :> IntelCore_i7_g13 &
                   IntelCore_i7_g14;
}

create_feature_hierarchy! {
    register_features_model_core_5_g13_g14 ; "cpu_model" :
    IntelCore_i5 :> IntelCore_i5_g13 &
                   IntelCore_i5_g14;
}

create_feature_hierarchy! {
    register_features_model_core_6_g13_g14 ; "cpu_model" :
    IntelCore_i3 :> IntelCore_i3_g13 &
                   IntelCore_i3_g14;
}

// ------------------------------------------------------------------
// Models: IntelCore_i9_g13 — additions
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i9_g13_1 ; "cpu_model" :
    IntelCore_i9_g13 :> IntelCore_i9_13900K &
                       IntelCore_i9_13900KF &
                       IntelCore_i9_13900KS &
                       IntelCore_i9_13900 &
                       IntelCore_i9_13900F &
                       IntelCore_i9_13900T &
                       IntelCore_i9_13900HX &
                       IntelCore_i9_13950HX &
                       IntelCore_i9_13980HX &
                       IntelCore_i9_13900HK &
                       IntelCore_i9_13900H &
                       IntelCore_i9_13905H;
}

// ------------------------------------------------------------------
// Models: IntelCore_i9_g14 — additions
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i9_g14_1 ; "cpu_model" :
    IntelCore_i9_g14 :> IntelCore_i9_14900K &
                       IntelCore_i9_14900KF &
                       IntelCore_i9_14900KS &
                       IntelCore_i9_14900 &
                       IntelCore_i9_14900F &
                       IntelCore_i9_14900T &
                       IntelCore_i9_14900HX;
}

// ------------------------------------------------------------------
// Models: IntelCore_i7_g13 — additions
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i7_g13_1 ; "cpu_model" :
    IntelCore_i7_g13 :> IntelCore_i7_13700K &
                       IntelCore_i7_13700KF &
                       IntelCore_i7_13700 &
                       IntelCore_i7_13700F &
                       IntelCore_i7_13700T &
                       IntelCore_i7_13700TE &
                       IntelCore_i7_13850HX &
                       IntelCore_i7_13700HX &
                       IntelCore_i7_13650HX &
                       IntelCore_i7_13800H &
                       IntelCore_i7_13700H &
                       IntelCore_i7_13620H &
                       IntelCore_i7_13705H &
                       IntelCore_i7_1370P &
                       IntelCore_i7_1360P &
                       IntelCore_i7_1365U;
}

// ------------------------------------------------------------------
// Models: IntelCore_i7_g14 — additions
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i7_g14_1 ; "cpu_model" :
    IntelCore_i7_g14 :> IntelCore_i7_14700K &
                       IntelCore_i7_14700KF &
                       IntelCore_i7_14700 &
                       IntelCore_i7_14700F &
                       IntelCore_i7_14700HX;
}

// ------------------------------------------------------------------
// Models: IntelCore_i5_g13 — additions
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i5_g13_1 ; "cpu_model" :
    IntelCore_i5_g13 :> IntelCore_i5_13600K &
                       IntelCore_i5_13600KF &
                       IntelCore_i5_13600 &
                       IntelCore_i5_13500 &
                       IntelCore_i5_13500T &
                       IntelCore_i5_13490F &
                       IntelCore_i5_13400 &
                       IntelCore_i5_13400F &
                       IntelCore_i5_13400T &
                       IntelCore_i5_13600HX &
                       IntelCore_i5_13500HX &
                       IntelCore_i5_13600H &
                       IntelCore_i5_13500H &
                       IntelCore_i5_13420H &
                       IntelCore_i5_13505H &
                       IntelCore_i5_1350P &
                       IntelCore_i5_1340P &
                       IntelCore_i5_1345U &
                       IntelCore_i5_1335U &
                       IntelCore_i5_1334U;
}

// ------------------------------------------------------------------
// Models: IntelCore_i5_g14 — additions
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i5_g14_1 ; "cpu_model" :
    IntelCore_i5_g14 :> IntelCore_i5_14600K &
                       IntelCore_i5_14600KF &
                       IntelCore_i5_14600 &
                       IntelCore_i5_14500 &
                       IntelCore_i5_14500T &
                       IntelCore_i5_14400 &
                       IntelCore_i5_14400F &
                       IntelCore_i5_14400T;
}

// ------------------------------------------------------------------
// Models: IntelCore_i3_g13 — additions
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i3_g13_1 ; "cpu_model" :
    IntelCore_i3_g13 :> IntelCore_i3_13100 &
                       IntelCore_i3_13100F &
                       IntelCore_i3_13100T &
                       IntelCore_i3_1315U &
                       IntelCore_i3_1305U;
}

// ------------------------------------------------------------------
// Models: IntelCore_i3_g14 — additions
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_i3_g14_1 ; "cpu_model" :
    IntelCore_i3_g14 :> IntelCore_i3_14100 &
                       IntelCore_i3_14100F &
                       IntelCore_i3_14100T;
}

// ------------------------------------------------------------------
// Optional: Intel Core Ultra (Meteor Lake / Arrow Lake) — additions
// (only if your crate models Core Ultra under IntelCore as well)
// ------------------------------------------------------------------

create_feature_hierarchy! {
    register_features_model_core_ultra_1 ; "cpu_model" :
    IntelCore :> IntelCore_Ultra;
}

create_feature_hierarchy! {
    register_features_model_core_ultra_2 ; "cpu_model" :
    IntelCore_Ultra :> IntelCore_Ultra_g1 &
                      IntelCore_Ultra_g2;
}

create_feature_hierarchy! {
    register_features_model_core_ultra_g1_1 ; "cpu_model" :
    IntelCore_Ultra_g1 :> IntelCore_Ultra9_185H &
                          IntelCore_Ultra7_165H &
                          IntelCore_Ultra7_155H &
                          IntelCore_Ultra7_165U &
                          IntelCore_Ultra7_155U &
                          IntelCore_Ultra5_135H &
                          IntelCore_Ultra5_125H &
                          IntelCore_Ultra5_135U &
                          IntelCore_Ultra5_125U;
}

create_feature_hierarchy! {
    register_features_model_core_ultra_g2_1 ; "cpu_model" :
    IntelCore_Ultra_g2 :> IntelCore_Ultra9_285K &
                          IntelCore_Ultra7_265K &
                          IntelCore_Ultra5_245K;
}
