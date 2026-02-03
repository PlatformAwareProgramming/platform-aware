use configurable_features::{create_feature_hierarchy};

use crate::AccBackend;

create_feature_hierarchy! {  register_features ;
    "acc_backend" :
    AccBackend :>
    OpenGL :>
    OpenGL_4_6;
}
