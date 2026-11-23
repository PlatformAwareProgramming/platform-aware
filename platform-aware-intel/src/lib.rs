mod accelerator;
mod processor;

pub use accelerator::*;
pub use processor::*;

#[cfg(test)]
mod tests {
    use super::*;
    use platform_aware_features::{AccModel, CPUMicroarchitecture, CPUModel, Feature};

    #[test]
    fn test_intel_taxonomies_preserve_family_subtyping() {
        // Xeon Phi under IntelXeonPhi <: IntelAccelerator <: AccModel
        assert!(IntelXeonPhi_7210.feature_obj().subtypeof(&IntelXeonPhi.feature_obj()));
        assert!(IntelXeonPhi_7210.feature_obj().subtypeof(&IntelAccelerator.feature_obj()));
        assert!(IntelXeonPhi_7210.feature_obj().subtypeof(&AccModel.feature_obj()));

        // CPU model families connect series transitively to family and CPUModel
        assert!(IntelAtom_C5115.feature_obj().subtypeof(&IntelAtom.feature_obj()));
        assert!(IntelAtom_C5115.feature_obj().subtypeof(&CPUModel.feature_obj()));

        assert!(IntelCeleron_G6900.feature_obj().subtypeof(&IntelCeleron.feature_obj()));
        assert!(IntelCore_i7_7500U.feature_obj().subtypeof(&IntelCore.feature_obj()));
        assert!(IntelCore_Ultra9_285K.feature_obj().subtypeof(&IntelCore_Ultra.feature_obj()));
        assert!(IntelCore_Ultra9_285K.feature_obj().subtypeof(&IntelCore.feature_obj()));
        assert!(IntelItanium_9700.feature_obj().subtypeof(&IntelItanium.feature_obj()));
        assert!(IntelPentium_Gold.feature_obj().subtypeof(&IntelPentium.feature_obj()));

        // Microarchitecture includes KabyLake under IntelMicroarchitecture <: CPUMicroarchitecture
        assert!(KabyLake.feature_obj().subtypeof(&IntelMicroarchitecture.feature_obj()));
        assert!(KabyLake.feature_obj().subtypeof(&CPUMicroarchitecture.feature_obj()));
    }
}
