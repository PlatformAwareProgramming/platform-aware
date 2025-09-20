mod base;
mod featuremap;
pub mod platformfile;

pub use base::*;
pub use featuremap::{insert_feature, lookup_feature};
pub use platformfile::*;
pub use resolve::*;

mod resolve;
