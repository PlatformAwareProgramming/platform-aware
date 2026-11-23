mod accelerator;
mod processor;

pub use accelerator::*;
pub use processor::*;

#[cfg(test)]
mod tests {
    use super::*;
    use platform_aware_features::{AccModel, CPUMicroarchitecture, Feature};

    #[test]
    fn test_amd_microarchitecture_and_model_taxonomies() {
        assert!(Zen3.feature_obj().subtypeof(&AMDMicroarchitecture.feature_obj()));
        assert!(Zen3.feature_obj().subtypeof(&CPUMicroarchitecture.feature_obj()));
        assert!(Milan.feature_obj().subtypeof(&Zen3.feature_obj()));
        assert!(Milan.feature_obj().subtypeof(&CPUMicroarchitecture.feature_obj()));
        assert!(AMDInstinct_MI325X.feature_obj().subtypeof(&AMDAccelerator.feature_obj()));
        assert!(AMDInstinct_MI325X.feature_obj().subtypeof(&AccModel.feature_obj()));
    }
}
