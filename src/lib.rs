use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex},
};

use platform_aware_features::{readplatformfeatures, Feature, PlatformParameter};

// reexport the macro
pub use platform_aware_macros::platformaware;

//#[cfg(feature = "amd")]
pub use platform_aware_amd;
//#[cfg(feature = "intel")]
pub use platform_aware_intel;
//#[cfg(feature = "nvidia")]
pub use platform_aware_nvidia;

/*

type PlatformFeatures = HashMap<PlatformParameter, Arc<dyn Feature + Send + Sync>>;

pub static PLATFORMFEATURES: LazyLock<Mutex<PlatformFeatures>> = LazyLock::new(
    || readplatformfeatures().into()
);

#[cfg(test)]
mod tests {
    #[test]
    fn load_platformfeatures() {
        assert!(!super::PLATFORMFEATURES.lock().unwrap().is_empty());
    }

    #[test]
    fn mutate_platformfeatures() {
        let mut platformfeatures = super::PLATFORMFEATURES.lock().unwrap();

        platformfeatures.clear();
        assert!(platformfeatures.is_empty());
    }
}
*/
