use platform_aware_features::{CPUModel, create_feature_hierarchy};


// Xeon Family

create_feature_hierarchy! { register_features_model_family ; "cpu_model" : CPUModel :> IntelXeon;}


// Xeon Series

create_feature_hierarchy! { register_features_model_series ; "cpu_model" : 
    IntelXeon :> IntelXeon_W &
                 IntelXeon_D &
                 IntelXeon_E;
}

create_feature_hierarchy! { register_features_model_es ; "cpu_model" : 
     IntelXeon_E :> IntelXeon_E3 &
                    IntelXeon_E5 &
                    IntelXeon_E7;
}

create_feature_hierarchy! { register_features_model_e3v ; "cpu_model" : 
     IntelXeon_E3 :> IntelXeon_E3_v1 &
                     IntelXeon_E3_v2 &
                     IntelXeon_E3_v3 &
                     IntelXeon_E3_v4 &
                     IntelXeon_E3_v5 &
                     IntelXeon_E3_v6;
}

create_feature_hierarchy! { register_features_model_e5v ; "cpu_model" : 
     IntelXeon_E5 :> IntelXeon_E5_v1 &
                     IntelXeon_E5_v2 &
                     IntelXeon_E5_v3 &
                     IntelXeon_E5_v4 &
                     IntelXeon_E5_v5;
}

create_feature_hierarchy! { register_features_model_e7v ; "cpu_model" : 
     IntelXeon_E7 :> IntelXeon_E7_v1 &
                     IntelXeon_E7_v2 &
                     IntelXeon_E7_v3 &
                     IntelXeon_E7_v4;
}

create_feature_hierarchy! { register_features_model_s ; "cpu_model" : 
    IntelXeon :> IntelXeon_Scalable &
                 IntelXeon_Scalable_g2 & 
                 IntelXeon_Scalable_g3 & 
                 IntelXeon_Scalable_g4 & 
                 IntelXeon_Scalable_g5;
}


create_feature_hierarchy! { register_features_model_d ; "cpu_model" : 
 IntelXeon_D :> IntelXeon_D_1702 &
                IntelXeon_D_1712TR &
                IntelXeon_D_1713NT &
                IntelXeon_D_1713NTE &
                IntelXeon_D_1714 &
                IntelXeon_D_1715TER &
                IntelXeon_D_1718T &
                IntelXeon_D_1722NE &
                IntelXeon_D_1726 &
                IntelXeon_D_1732TE &
                IntelXeon_D_1733NT &
                IntelXeon_D_1734NT &
                IntelXeon_D_1735TR &
                IntelXeon_D_1736 &
                IntelXeon_D_1736NT &
                IntelXeon_D_1739 &
                IntelXeon_D_1746TER &
                IntelXeon_D_1747NTE &
                IntelXeon_D_1748TE &
                IntelXeon_D_1749NT &
                IntelXeon_D_2712T &
                IntelXeon_D_2733NT &
                IntelXeon_D_2738 &
                IntelXeon_D_2752NTE &
                IntelXeon_D_2752TER &
                IntelXeon_D_2753NT &
                IntelXeon_D_2766NT &
                IntelXeon_D_2775TE &
                IntelXeon_D_2776NT &
                IntelXeon_D_2779 &
                IntelXeon_D_2786NTE &
                IntelXeon_D_2795NT &
                IntelXeon_D_2796NT &
                IntelXeon_D_2796TE &
                IntelXeon_D_2798NT &
                IntelXeon_D_2799 &
                IntelXeon_D_1602 &
                IntelXeon_D_1622 &
                IntelXeon_D_1623N &
                IntelXeon_D_1627 &
                IntelXeon_D_1633N &
                IntelXeon_D_1637 &
                IntelXeon_D_1649N &
                IntelXeon_D_1653N &
                IntelXeon_D_2123IT &
                IntelXeon_D_2141I &
                IntelXeon_D_2142IT &
                IntelXeon_D_2143IT &
                IntelXeon_D_2145NT &
                IntelXeon_D_2161I &
                IntelXeon_D_2163IT &
                IntelXeon_D_2166NT &
                IntelXeon_D_2173IT &
                IntelXeon_D_2177NT &
                IntelXeon_D_2183IT &
                IntelXeon_D_2187NT &
                IntelXeon_D_1513N &
                IntelXeon_D_1523N &
                IntelXeon_D_1533N &
                IntelXeon_D_1543N &
                IntelXeon_D_1553N &
                IntelXeon_D_1529 &
                IntelXeon_D_1539 &
                IntelXeon_D_1559 &
                IntelXeon_D_1557 &
                IntelXeon_D_1567 &
                IntelXeon_D_1571 &
                IntelXeon_D_1577 &
                IntelXeon_D_1518 &
                IntelXeon_D_1521 &
                IntelXeon_D_1527 &
                IntelXeon_D_1528 &
                IntelXeon_D_1531 &
                IntelXeon_D_1537 &
                IntelXeon_D_1541 &
                IntelXeon_D_1548 &
                IntelXeon_D_1520 &
                IntelXeon_D_1540 ;
}

create_feature_hierarchy! { register_features_model_e ; "cpu_model" : 
IntelXeon_E :> IntelXeon_E_2286M &
                IntelXeon_E_2254ME &
                IntelXeon_E_2254ML &
                IntelXeon_E_2276ME &
                IntelXeon_E_2276ML &
                IntelXeon_E_2224G &
                IntelXeon_E_2226G &
                IntelXeon_E_2244G &
                IntelXeon_E_2246G &
                IntelXeon_E_2274G &
                IntelXeon_E_2276G &
                IntelXeon_E_2276M &
                IntelXeon_E_2278G &
                IntelXeon_E_2286G &
                IntelXeon_E_2288G &
                IntelXeon_E_2124G &
                IntelXeon_E_2126G &
                IntelXeon_E_2144G &
                IntelXeon_E_2146G &
                IntelXeon_E_2174G &
                IntelXeon_E_2176G &
                IntelXeon_E_2186G &
                IntelXeon_E_2176M &
                IntelXeon_E_2186M &
                IntelXeon_E_2226GE &
                IntelXeon_E_2278GE &
                IntelXeon_E_2278GEL &
                IntelXeon_E_2314 &
                IntelXeon_E_2324G &
                IntelXeon_E_2334 &
                IntelXeon_E_2336 &
                IntelXeon_E_2356G &
                IntelXeon_E_2374G &
                IntelXeon_E_2378 &
                IntelXeon_E_2378G &
                IntelXeon_E_2386G &
                IntelXeon_E_2388G &
                IntelXeon_E_2224 &
                IntelXeon_E_2234 &
                IntelXeon_E_2236 &
                IntelXeon_E_2124 &
                IntelXeon_E_2134 &
                IntelXeon_E_2136 ;
}

create_feature_hierarchy! { register_features_model_e3 ; "cpu_model" : 
    IntelXeon_E3 :> IntelXeon_E3_1260L &
                    IntelXeon_E3_1105C &
                    IntelXeon_E3_1125C &
                    IntelXeon_E3_1290 &
                    IntelXeon_E3_1220 &
                    IntelXeon_E3_1220L &
                    IntelXeon_E3_1225 &
                    IntelXeon_E3_1230 &
                    IntelXeon_E3_1235 &
                    IntelXeon_E3_1240 &
                    IntelXeon_E3_1245 &
                    IntelXeon_E3_1270 &
                    IntelXeon_E3_1275 &
                    IntelXeon_E3_1280 ;
}

create_feature_hierarchy! { register_features_model_e3v2 ; "cpu_model" : 
     IntelXeon_E3_v2 :> IntelXeon_E3_1265LV2 &
                        IntelXeon_E3_1105CV2 &
                        IntelXeon_E3_1125CV2 &
                        IntelXeon_E3_1220V2 &
                        IntelXeon_E3_1220LV2 &
                        IntelXeon_E3_1225V2 &
                        IntelXeon_E3_1230V2 &
                        IntelXeon_E3_1240V2 &
                        IntelXeon_E3_1245V2 &
                        IntelXeon_E3_1270V2 &
                        IntelXeon_E3_1275V2 &
                        IntelXeon_E3_1280V2 &
                        IntelXeon_E3_1290V2 ;
}

create_feature_hierarchy! { register_features_model_e3v3 ; "cpu_model" : 
     IntelXeon_E3_v3 :> IntelXeon_E3_1268LV3 &
                        IntelXeon_E3_1275LV3 &
                        IntelXeon_E3_1265Lv3 &
                        IntelXeon_E3_1226V3 &
                        IntelXeon_E3_1231V3 &
                        IntelXeon_E3_1240LV3 &
                        IntelXeon_E3_1241V3 &
                        IntelXeon_E3_1246V3 &
                        IntelXeon_E3_1271V3 &
                        IntelXeon_E3_1276V3 &
                        IntelXeon_E3_1281V3 &
                        IntelXeon_E3_1286V3 &
                        IntelXeon_E3_1286LV3 &
                        IntelXeon_E3_1220LV3 &
                        IntelXeon_E3_1220_v3 &
                        IntelXeon_E3_1225V3 &
                        IntelXeon_E3_1230_v3 &
                        IntelXeon_E3_1230Lv3 &
                        IntelXeon_E3_1240_v3 &
                        IntelXeon_E3_1245_v3 &
                        IntelXeon_E3_1270_v3 &
                        IntelXeon_E3_1275_v3 &
                        IntelXeon_E3_1280_v3 &
                        IntelXeon_E3_1285_v3 &
                        IntelXeon_E3_1285Lv3 ;
}

create_feature_hierarchy! { register_features_model_e3v4 ; "cpu_model" : 
     IntelXeon_E3_v4 :> IntelXeon_E3_1258LV4 &
                        IntelXeon_E3_1265LV4 &
                        IntelXeon_E3_1278LV4 &
                        IntelXeon_E3_1285V4 &
                        IntelXeon_E3_1285LV4 ;
}

create_feature_hierarchy! { register_features_model_e3v5 ; "cpu_model" : 
     IntelXeon_E3_v5 :> IntelXeon_E3_1225V5 &
                        IntelXeon_E3_1235LV5 &
                        IntelXeon_E3_1245V5 &
                        IntelXeon_E3_1268LV5 &
                        IntelXeon_E3_1275V5 &
                        IntelXeon_E3_1505LV5 &
                        IntelXeon_E3_1505MV5 &
                        IntelXeon_E3_1535MV5 &
                        IntelXeon_E3_1565LV5 &
                        IntelXeon_E3_1578LV5 &
                        IntelXeon_E3_1585V5 &
                        IntelXeon_E3_1585LV5 &
                        IntelXeon_E3_1515MV5 &
                        IntelXeon_E3_1545MV5 &
                        IntelXeon_E3_1575MV5 &
                        IntelXeon_E3_1558LV5 &
                        IntelXeon_E3_1220V5 &
                        IntelXeon_E3_1230V5 &
                        IntelXeon_E3_1240V5 &
                        IntelXeon_E3_1240LV5 &
                        IntelXeon_E3_1260LV5 &
                        IntelXeon_E3_1270V5 &
                        IntelXeon_E3_1280V5 ;
}

create_feature_hierarchy! { register_features_model_e3v6 ; "cpu_model" : 
     IntelXeon_E3_v6 :> IntelXeon_E3_1285V6 &
                        IntelXeon_E3_1501LV6 &
                        IntelXeon_E3_1501MV6 &
                        IntelXeon_E3_1225V6 &
                        IntelXeon_E3_1245V6 &
                        IntelXeon_E3_1275V6 &
                        IntelXeon_E3_1505LV6 &
                        IntelXeon_E3_1505MV6 &
                        IntelXeon_E3_1535MV6 &
                        IntelXeon_E3_1220V6 &
                        IntelXeon_E3_1230V6 &
                        IntelXeon_E3_1240V6 &
                        IntelXeon_E3_1270V6 &
                        IntelXeon_E3_1280V6 ;
}

create_feature_hierarchy! { register_features_model_e5 ; "cpu_model" : 
    IntelXeon_E5 :> IntelXeon_E5_1428L &
                    IntelXeon_E5_2403 &
                    IntelXeon_E5_2407 &
                    IntelXeon_E5_2418L &
                    IntelXeon_E5_2420 &
                    IntelXeon_E5_2428L &
                    IntelXeon_E5_2430 &
                    IntelXeon_E5_2430L &
                    IntelXeon_E5_2440 &
                    IntelXeon_E5_2448L &
                    IntelXeon_E5_2450 &
                    IntelXeon_E5_2450L &
                    IntelXeon_E5_2470 &
                    IntelXeon_E5_4603 &
                    IntelXeon_E5_4607 &
                    IntelXeon_E5_4610 &
                    IntelXeon_E5_4617 &
                    IntelXeon_E5_4620 &
                    IntelXeon_E5_4640 &
                    IntelXeon_E5_4650 &
                    IntelXeon_E5_4650L &
                    IntelXeon_E5_1620 &
                    IntelXeon_E5_1650 &
                    IntelXeon_E5_1660 &
                    IntelXeon_E5_2603 &
                    IntelXeon_E5_2609 &
                    IntelXeon_E5_2620 &
                    IntelXeon_E5_2630 &
                    IntelXeon_E5_2630L &
                    IntelXeon_E5_2637 &
                    IntelXeon_E5_2640 &
                    IntelXeon_E5_2643 &
                    IntelXeon_E5_2648L &
                    IntelXeon_E5_2650 &
                    IntelXeon_E5_2650L &
                    IntelXeon_E5_2658 &
                    IntelXeon_E5_2660 &
                    IntelXeon_E5_2665 &
                    IntelXeon_E5_2667 &
                    IntelXeon_E5_2670 &
                    IntelXeon_E5_2680 &
                    IntelXeon_E5_2687W &
                    IntelXeon_E5_2690 ;
}

create_feature_hierarchy! { register_features_model_e5v2 ; "cpu_model" : 
     IntelXeon_E5_v2 :> IntelXeon_E5_4603V2 &
                        IntelXeon_E5_4607V2 &
                        IntelXeon_E5_4610V2 &
                        IntelXeon_E5_4620V2 &
                        IntelXeon_E5_4624LV2 &
                        IntelXeon_E5_4627V2 &
                        IntelXeon_E5_4640V2 &
                        IntelXeon_E5_4650V2 &
                        IntelXeon_E5_4657LV2 &
                        IntelXeon_E5_1428LV2 &
                        IntelXeon_E5_2403V2 &
                        IntelXeon_E5_2407V2 &
                        IntelXeon_E5_2418LV2 &
                        IntelXeon_E5_2420V2 &
                        IntelXeon_E5_2428LV2 &
                        IntelXeon_E5_2430V2 &
                        IntelXeon_E5_2430LV2 &
                        IntelXeon_E5_2440V2 &
                        IntelXeon_E5_2448LV2 &
                        IntelXeon_E5_2450V2 &
                        IntelXeon_E5_2450LV2 &
                        IntelXeon_E5_2470V2 &
                        IntelXeon_E5_1620V2 &
                        IntelXeon_E5_1650V2 &
                        IntelXeon_E5_1660V2 &
                        IntelXeon_E5_2603V2 &
                        IntelXeon_E5_2609V2 &
                        IntelXeon_E5_2618LV2 &
                        IntelXeon_E5_2620V2 &
                        IntelXeon_E5_2628LV2 &
                        IntelXeon_E5_2630V2 &
                        IntelXeon_E5_2630LV2 &
                        IntelXeon_E5_2637V2 &
                        IntelXeon_E5_2640V2 &
                        IntelXeon_E5_2643V2 &
                        IntelXeon_E5_2648LV2 &
                        IntelXeon_E5_2650V2 &
                        IntelXeon_E5_2650LV2 &
                        IntelXeon_E5_2658V2 &
                        IntelXeon_E5_2660V2 &
                        IntelXeon_E5_2667V2 &
                        IntelXeon_E5_2670V2 &
                        IntelXeon_E5_2680V2 &
                        IntelXeon_E5_2687WV2 &
                        IntelXeon_E5_2690V2 &
                        IntelXeon_E5_2695V2 &
                        IntelXeon_E5_2697V2 ;
}

create_feature_hierarchy! { register_features_model_e5v3 ; "cpu_model" : 
     IntelXeon_E5_v3 :> IntelXeon_E5_4610V3 &
                        IntelXeon_E5_4620V3 &
                        IntelXeon_E5_4627V3 &
                        IntelXeon_E5_4640V3 &
                        IntelXeon_E5_4648V3 &
                        IntelXeon_E5_4650V3 &
                        IntelXeon_E5_4655V3 &
                        IntelXeon_E5_4660V3 &
                        IntelXeon_E5_4667V3 &
                        IntelXeon_E5_4669V3 &
                        IntelXeon_E5_2658AV3 &
                        IntelXeon_E5_1428LV3 &
                        IntelXeon_E5_2408LV3 &
                        IntelXeon_E5_2418LV3 &
                        IntelXeon_E5_2428LV3 &
                        IntelXeon_E5_2438LV3 &
                        IntelXeon_E5_1620V3 &
                        IntelXeon_E5_1630V3 &
                        IntelXeon_E5_1650V3 &
                        IntelXeon_E5_1660V3 &
                        IntelXeon_E5_1680V3 &
                        IntelXeon_E5_2603V3 &
                        IntelXeon_E5_2608LV3 &
                        IntelXeon_E5_2609V3 &
                        IntelXeon_E5_2618LV3 &
                        IntelXeon_E5_2620V3 &
                        IntelXeon_E5_2623V3 &
                        IntelXeon_E5_2628LV3 &
                        IntelXeon_E5_2630V3 &
                        IntelXeon_E5_2630LV3 &
                        IntelXeon_E5_2637V3 &
                        IntelXeon_E5_2640V3 &
                        IntelXeon_E5_2643V3 &
                        IntelXeon_E5_2648LV3 &
                        IntelXeon_E5_2650V3 &
                        IntelXeon_E5_2650LV3 &
                        IntelXeon_E5_2658V3 &
                        IntelXeon_E5_2660V3 &
                        IntelXeon_E5_2666V3 &
                        IntelXeon_E5_2667V3 &
                        IntelXeon_E5_2670V3 &
                        IntelXeon_E5_2676V3 &
                        IntelXeon_E5_2680V3 &
                        IntelXeon_E5_2683V3 &
                        IntelXeon_E5_2687WV3 &
                        IntelXeon_E5_2690V3 &
                        IntelXeon_E5_2695V3 &
                        IntelXeon_E5_2697V3 &
                        IntelXeon_E5_2698V3 &
                        IntelXeon_E5_2699V3 ;
}

create_feature_hierarchy! { register_features_model_e5v4 ; "cpu_model" : 
     IntelXeon_E5_v4 :> IntelXeon_E5_2699AV4 &
                        IntelXeon_E5_2699RV4 &
                        IntelXeon_E5_4610V4 &
                        IntelXeon_E5_4620V4 &
                        IntelXeon_E5_4627V4 &
                        IntelXeon_E5_4628LV4 &
                        IntelXeon_E5_4640V4 &
                        IntelXeon_E5_4650V4 &
                        IntelXeon_E5_4655V4 &
                        IntelXeon_E5_4660V4 &
                        IntelXeon_E5_4667V4 &
                        IntelXeon_E5_4669V4 &
                        IntelXeon_E5_1620V4 &
                        IntelXeon_E5_1630V4 &
                        IntelXeon_E5_1650V4 &
                        IntelXeon_E5_1660V4 &
                        IntelXeon_E5_1680V4 &
                        IntelXeon_E5_2603V4 &
                        IntelXeon_E5_2608LV4 &
                        IntelXeon_E5_2609V4 &
                        IntelXeon_E5_2618LV4 &
                        IntelXeon_E5_2620V4 &
                        IntelXeon_E5_2623V4 &
                        IntelXeon_E5_2628LV4 &
                        IntelXeon_E5_2630V4 &
                        IntelXeon_E5_2630LV4 &
                        IntelXeon_E5_2637V4 &
                        IntelXeon_E5_2640V4 &
                        IntelXeon_E5_2643V4 &
                        IntelXeon_E5_2648LV4 &
                        IntelXeon_E5_2650V4 &
                        IntelXeon_E5_2650LV4 &
                        IntelXeon_E5_2658V4 &
                        IntelXeon_E5_2660V4 &
                        IntelXeon_E5_2667V4 &
                        IntelXeon_E5_2680V4 &
                        IntelXeon_E5_2683V4 &
                        IntelXeon_E5_2686V4 &
                        IntelXeon_E5_2687WV4 &
                        IntelXeon_E5_2690V4 &
                        IntelXeon_E5_2695V4 &
                        IntelXeon_E5_2697V4 &
                        IntelXeon_E5_2697AV4 &
                        IntelXeon_E5_2698V4 &
                        IntelXeon_E5_2699V4 ;
}

create_feature_hierarchy! { register_features_model_e5v5 ; "cpu_model" : 
    IntelXeon_E5_v5 :> IntelXeon_E5_2686V5 ;
}

create_feature_hierarchy! { register_features_model_e7 ; "cpu_model" : 
    IntelXeon_E7 :> IntelXeon_E7_2803 &
                    IntelXeon_E7_2820 &
                    IntelXeon_E7_2830 &
                    IntelXeon_E7_2850 &
                    IntelXeon_E7_2860 &
                    IntelXeon_E7_2870 &
                    IntelXeon_E7_4807 &
                    IntelXeon_E7_4820 &
                    IntelXeon_E7_4830 &
                    IntelXeon_E7_4850 &
                    IntelXeon_E7_4860 &
                    IntelXeon_E7_4870 &
                    IntelXeon_E7_8830 &
                    IntelXeon_E7_8837 &
                    IntelXeon_E7_8850 &
                    IntelXeon_E7_8860 &
                    IntelXeon_E7_8867L &
                    IntelXeon_E7_8870 ;
}

create_feature_hierarchy! { register_features_model_e7v2 ; "cpu_model" : 
     IntelXeon_E7_v2 :> IntelXeon_E7_2850V2 &
                        IntelXeon_E7_2870V2 &
                        IntelXeon_E7_2880V2 &
                        IntelXeon_E7_2890V2 &
                        IntelXeon_E7_4809V2 &
                        IntelXeon_E7_4820V2 &
                        IntelXeon_E7_4830V2 &
                        IntelXeon_E7_4850V2 &
                        IntelXeon_E7_4860V2 &
                        IntelXeon_E7_4870V2 &
                        IntelXeon_E7_4880V2 &
                        IntelXeon_E7_4890V2 &
                        IntelXeon_E7_8850V2 &
                        IntelXeon_E7_8857V2 &
                        IntelXeon_E7_8870V2 &
                        IntelXeon_E7_8880V2 &
                        IntelXeon_E7_8880LV2 &
                        IntelXeon_E7_8890V2 &
                        IntelXeon_E7_8891V2 &
                        IntelXeon_E7_8893V2 ;
}

create_feature_hierarchy! { register_features_model_e7v3 ; "cpu_model" : 
     IntelXeon_E7_v3 :> IntelXeon_E7_4809V3 &
                        IntelXeon_E7_4820V3 &
                        IntelXeon_E7_4830V3 &
                        IntelXeon_E7_4850V3 &
                        IntelXeon_E7_8860V3 &
                        IntelXeon_E7_8867V3 &
                        IntelXeon_E7_8870V3 &
                        IntelXeon_E7_8880V3 &
                        IntelXeon_E7_8880LV3 &
                        IntelXeon_E7_8890V3 &
                        IntelXeon_E7_8891V3 &
                        IntelXeon_E7_8893V3 ;
}

create_feature_hierarchy! { register_features_model_e7v4 ; "cpu_model" : 
     IntelXeon_E7_v4 :> IntelXeon_E7_8894V4 &
                        IntelXeon_E7_4809V4 &
                        IntelXeon_E7_4820V4 &
                        IntelXeon_E7_4830V4 &
                        IntelXeon_E7_4850V4 &
                        IntelXeon_E7_8860V4 &
                        IntelXeon_E7_8867V4 &
                        IntelXeon_E7_8870V4 &
                        IntelXeon_E7_8880V4 &
                        IntelXeon_E7_8890V4 &
                        IntelXeon_E7_8891V4 &
                        IntelXeon_E7_8893V4 ;
}

create_feature_hierarchy! { register_features_model_sg1 ; "cpu_model" : 
    IntelXeon_Scalable :> IntelXeon_6138P &
                          IntelXeon_3104 &
                          IntelXeon_3106 &
                          IntelXeon_5115 &
                          IntelXeon_5118 &
                          IntelXeon_5119T &
                          IntelXeon_5120 &
                          IntelXeon_5120T &
                          IntelXeon_5122 &
                          IntelXeon_6126 &
                          IntelXeon_6126F &
                          IntelXeon_6126T &
                          IntelXeon_6128 &
                          IntelXeon_6130 &
                          IntelXeon_6130F &
                          IntelXeon_6130T &
                          IntelXeon_6132 &
                          IntelXeon_6134 &
                          IntelXeon_6136 &
                          IntelXeon_6138 &
                          IntelXeon_6138F &
                          IntelXeon_6138T &
                          IntelXeon_6140 &
                          IntelXeon_6142 &
                          IntelXeon_6142F &
                          IntelXeon_6144 &
                          IntelXeon_6146 &
                          IntelXeon_6148 &
                          IntelXeon_6148F &
                          IntelXeon_6150 &
                          IntelXeon_6152 &
                          IntelXeon_6154 &
                          IntelXeon_8124M &
                          IntelXeon_8151 &
                          IntelXeon_8153 &
                          IntelXeon_8156 &
                          IntelXeon_8158 &
                          IntelXeon_8160 &
                          IntelXeon_8160F &
                          IntelXeon_8160T &
                          IntelXeon_8164 &
                          IntelXeon_8168 &
                          IntelXeon_8170 &
                          IntelXeon_8175 &
                          IntelXeon_8176 &
                          IntelXeon_8176F &
                          IntelXeon_8180 &
                          IntelXeon_4108 &
                          IntelXeon_4109T &
                          IntelXeon_4110 &
                          IntelXeon_4112 &
                          IntelXeon_4114T &
                          IntelXeon_4116 &
                          IntelXeon_4116T ;
}

create_feature_hierarchy! { register_features_model_sg2 ; "cpu_model" : 
     IntelXeon_Scalable_g2 :> IntelXeon_3206R &
                              IntelXeon_5218R &
                              IntelXeon_5220R &
                              IntelXeon_6208U &
                              IntelXeon_6226R &
                              IntelXeon_6230R &
                              IntelXeon_6238R &
                              IntelXeon_6240R &
                              IntelXeon_6242R &
                              IntelXeon_6246R &
                              IntelXeon_6248R &
                              IntelXeon_6250 &
                              IntelXeon_6250L &
                              IntelXeon_6256 &
                              IntelXeon_6258R &
                              IntelXeon_4210R &
                              IntelXeon_4210T &
                              IntelXeon_4214R &
                              IntelXeon_4215R &
                              IntelXeon_9221 &
                              IntelXeon_9222 &
                              IntelXeon_3204 &
                              IntelXeon_5215 &
                              IntelXeon_5215L &
                              IntelXeon_5217 &
                              IntelXeon_5218 &
                              IntelXeon_5218B &
                              IntelXeon_5218N &
                              IntelXeon_5218T &
                              IntelXeon_5220 &
                              IntelXeon_5220S &
                              IntelXeon_5220T &
                              IntelXeon_5222 &
                              IntelXeon_6209U &
                              IntelXeon_6210U &
                              IntelXeon_6212U &
                              IntelXeon_6222V &
                              IntelXeon_6226 &
                              IntelXeon_6230 &
                              IntelXeon_6230N &
                              IntelXeon_6230T &
                              IntelXeon_6234 &
                              IntelXeon_6238 &
                              IntelXeon_6238L &
                              IntelXeon_6238T &
                              IntelXeon_6240 &
                              IntelXeon_6240L &
                              IntelXeon_6240Y &
                              IntelXeon_6242 &
                              IntelXeon_6244 &
                              IntelXeon_6246 &
                              IntelXeon_6248 &
                              IntelXeon_6252 &
                              IntelXeon_6252N &
                              IntelXeon_6254 &
                              IntelXeon_6262V &
                              IntelXeon_8252 &
                              IntelXeon_8253 &
                              IntelXeon_8256 &
                              IntelXeon_8259 &
                              IntelXeon_8259CL &
                              IntelXeon_8260 &
                              IntelXeon_8260L &
                              IntelXeon_8260Y &
                              IntelXeon_8268 &
                              IntelXeon_8270 &
                              IntelXeon_8275 &
                              IntelXeon_8275L &
                              IntelXeon_8275CL &
                              IntelXeon_8276 &
                              IntelXeon_8276L &
                              IntelXeon_8280 &
                              IntelXeon_8280L &
                              IntelXeon_9242 &
                              IntelXeon_9282 &
                              IntelXeon_4208 &
                              IntelXeon_4209T &
                              IntelXeon_4210 &
                              IntelXeon_4214 &
                              IntelXeon_4214Y &
                              IntelXeon_4215 &
                              IntelXeon_4216 ;

}

create_feature_hierarchy! { register_features_model_sg3 ; "cpu_model" : 
     IntelXeon_Scalable_g3 :> IntelXeon_5315Y &
                              IntelXeon_5317 &
                              IntelXeon_5318N &
                              IntelXeon_5318S &
                              IntelXeon_5318Y &
                              IntelXeon_5320 &
                              IntelXeon_5320T &
                              IntelXeon_6312U &
                              IntelXeon_6314U &
                              IntelXeon_6326 &
                              IntelXeon_6330 &
                              IntelXeon_6330N &
                              IntelXeon_6334 &
                              IntelXeon_6336Y &
                              IntelXeon_6338 &
                              IntelXeon_6338N &
                              IntelXeon_6338T &
                              IntelXeon_6342 &
                              IntelXeon_6346 &
                              IntelXeon_6348 &
                              IntelXeon_6354 &
                              IntelXeon_8351N &
                              IntelXeon_8352M &
                              IntelXeon_8352S &
                              IntelXeon_8352V &
                              IntelXeon_8352Y &
                              IntelXeon_8358 &
                              IntelXeon_8358P &
                              IntelXeon_8362 &
                              IntelXeon_8368 &
                              IntelXeon_8368Q &
                              IntelXeon_8380 &
                              IntelXeon_4309Y &
                              IntelXeon_4310 &
                              IntelXeon_4310T &
                              IntelXeon_4314 &
                              IntelXeon_4316 &
                              IntelXeon_6330H &
                              IntelXeon_8356H &
                              IntelXeon_8360H &
                              IntelXeon_8360HL &
                              IntelXeon_5318H &
                              IntelXeon_5320H &
                              IntelXeon_6328H &
                              IntelXeon_6328HL &
                              IntelXeon_6348H &
                              IntelXeon_8353H &
                              IntelXeon_8354H &
                              IntelXeon_8375 &
                              IntelXeon_8375C &
                              IntelXeon_8376H &
                              IntelXeon_8376HL &
                              IntelXeon_8380H ;
}


create_feature_hierarchy! { register_features_model_sg4 ; "cpu_model" : 
       IntelXeon_Scalable_g4 :> IntelXeon_8490H & 
                                IntelXeon_8480plus & 
                                IntelXeon_8470 & 
                                IntelXeon_8468 & 
                                IntelXeon_8462Yplus & 
                                IntelXeon_8460Yplus & 
                                IntelXeon_8458P & 
                                IntelXeon_8452Y & 
                                IntelXeon_8450H & 
                                IntelXeon_6448H & 
                                IntelXeon_6438Yplus & 
                                IntelXeon_6434 & 
                                IntelXeon_6430 & 
                                IntelXeon_6426Y & 
                                IntelXeon_6426Yplus & 
                                IntelXeon_6418H & 
                                IntelXeon_6416H & 
                                IntelXeon_5418Y & 
                                IntelXeon_5416S & 
                                IntelXeon_5415plus & 
                                IntelXeon_4416plys & 
                                IntelXeon_4410Y & 
                                IntelXeon_3408U; 
}


create_feature_hierarchy! { register_features_model_sg5 ; "cpu_model" : 
       IntelXeon_Scalable_g5 :> IntelXeon_8592plus & 
                                IntelXeon_8580 & 
                                IntelXeon_8570 & 
                                IntelXeon_8568Yplus & 
                                IntelXeon_8558 & 
                                IntelXeon_8550 & 
                                IntelXeon_8532Yplus & 
                                IntelXeon_8528 & 
                                IntelXeon_8510 & 
                                IntelXeon_6538Yplus & 
                                IntelXeon_6530 & 
                                IntelXeon_6526Y & 
                                IntelXeon_6526Yplus & 
                                IntelXeon_6515Y & 
                                IntelXeon_6510 & 
                                IntelXeon_6505 & 
                                IntelXeon_4516Y & 
                                IntelXeon_4514Y;  
}

create_feature_hierarchy! { register_features_model_w ; "cpu_model" : 
     IntelXeon_W :> IntelXeon_W_1250 &
                    IntelXeon_W_1250P &
                    IntelXeon_W_1270 &
                    IntelXeon_W_1270P &
                    IntelXeon_W_1290 &
                    IntelXeon_W_1290P &
                    IntelXeon_W_1290T &
                    IntelXeon_W_1250E &
                    IntelXeon_W_1250TE &
                    IntelXeon_W_1270E &
                    IntelXeon_W_1270TE &
                    IntelXeon_W_1290E &
                    IntelXeon_W_1290TE &
                    IntelXeon_W_11155MLE &
                    IntelXeon_W_11155MRE &
                    IntelXeon_W_11555MLE &
                    IntelXeon_W_11555MRE &
                    IntelXeon_W_11865MLE &
                    IntelXeon_W_11865MRE &
                    IntelXeon_W_11855M &
                    IntelXeon_W_11955M &
                    IntelXeon_W_10855M &
                    IntelXeon_W_10885M &
                    IntelXeon_W_3323 &
                    IntelXeon_W_3335 &
                    IntelXeon_W_3345 &
                    IntelXeon_W_3365 &
                    IntelXeon_W_3375 &
                    IntelXeon_W_1350 &
                    IntelXeon_W_1350P &
                    IntelXeon_W_1370 &
                    IntelXeon_W_1370P &
                    IntelXeon_W_1390 &
                    IntelXeon_W_1390P &
                    IntelXeon_W_1390T &
                    IntelXeon_W_2223 &
                    IntelXeon_W_2225 &
                    IntelXeon_W_2235 &
                    IntelXeon_W_2245 &
                    IntelXeon_W_2255 &
                    IntelXeon_W_2265 &
                    IntelXeon_W_2275 &
                    IntelXeon_W_2295 &
                    IntelXeon_W_3223 &
                    IntelXeon_W_3225 &
                    IntelXeon_W_3235 &
                    IntelXeon_W_3245 &
                    IntelXeon_W_3245M &
                    IntelXeon_W_3265 &
                    IntelXeon_W_3265M &
                    IntelXeon_W_3275 &
                    IntelXeon_W_3275M &
                    IntelXeon_W_3175X &
                    IntelXeon_W_2123 &
                    IntelXeon_W_2125 &
                    IntelXeon_W_2133 &
                    IntelXeon_W_2135 &
                    IntelXeon_W_2145 &
                    IntelXeon_W_2155 &
                    IntelXeon_W_2175 &
                    IntelXeon_W_2195 ;
}






// Xeon Models