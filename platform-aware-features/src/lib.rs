mod base; 
pub mod platformfile;
mod featuremap;

pub use base::*;
pub use platformfile::*;
pub use resolve::*;
pub use featuremap::{insert_feature, lookup_feature};

mod resolve;

