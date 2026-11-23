use platform_aware_features::{CPUMicroarchitecture, create_feature_hierarchy};

create_feature_hierarchy! {
    register_features_microarchitecture_1 ; "cpu_microarchitecture" : 
    CPUMicroarchitecture :>  IntelMicroarchitecture;
}

create_feature_hierarchy! {
    register_features_microarchitecture_2 ; "cpu_microarchitecture" : 
    IntelMicroarchitecture :> Bonnell &
                            Nehalem &
                            NehalemEP &
                            NehalemEX &
                            Westmere &
                            WestmereEP &
                            WestmereEX &
                            Clarkdale &
                            Arrandale &
                            Tukwila &
                            SandyBridge &
                            SandyBridgeEP &
                            Saltwell &
                            IvyBridge &
                            IvyBridgeEP &
                            Cedarview &
                            Haswell &
                            HaswellEP &
                            CrystalWell &
                            Silvermont &
                            Avoton &
                            Rangeley &
                            Broadwell &
                            BroadwellEP &
                            KnightsFerry &
                            KnightsCorner &
                            Braswell &
                            Airmont &
                            Montvale &
                            Montecito &
                            Poulson &
                            Kittson &
                            Skylake &
                            SkylakeSP &
                            KabyLake &
                            Goldmont &
                            ApolloLake &
                            Denverton &
                            CannonLake &
                            CoffeeLake &
                            GeminiLake &
                            WhiskeyLake &
                            CascadeLake &
                            CascadeLakeSP &
                            AmberLake &
                            CometLake &
                            KnightsLanding &
                            KnightsMill &
                            CooperLake &
                            CooperLakeSP &
                            SnowRidge &
                            IceLake &
                            IceLakeSP &
                            TigerLake &
                            WillowCove &
                            RocketLake &
                            CypressCove &
                            Lakefield &
                            Tremont &
                            ElkhartLake &
                            JasperLake &
                            GeminiLakeRefresh &
                            AlderLake &
                            GoldenCove &
                            Gracemont &
                            RaptorLake &
                            RaptorLakeRefresh &
                            SapphireRapids &
                            MeteorLake &
                            RedwoodCove &
                            Crestmont &
                            EmeraldRapids &
                            GraniteRapids &
                            SierraForest &
                            LunarLake &
                            LionCove &
                            Skymont &
                            ArrowLake &
                            PantherLake &
                            HewittLake &
                            ParkerRidge;
}