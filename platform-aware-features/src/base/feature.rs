//mod quantifier;


use crate::{PlatformParameters, QuantifierType};

use std::hash::{DefaultHasher, Hash, Hasher};

pub enum FeatureType {
    Qualifier,
    Quantifier,
}

pub trait Feature {

    fn is_top(self:&Self) -> bool { false }

    fn string(self:&Self) -> String;

    fn hash_code(self:&Self) -> u64 {
        let mut s = DefaultHasher::new();
        self.string().hash(&mut s);
        s.finish()
    }

    fn feature_class(self:&Self) -> Option<PlatformParameters>;

    fn feature_type(self:&Self) -> FeatureType { FeatureType::Qualifier }

    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { None } 

    fn quantifier_type(self:&Self) -> Option<QuantifierType> { None } // only for quantifiers
    fn val(self:&Self) -> Option<i32> { None }                        // only for quantifiers
    fn other_val(self:&Self) -> Option<i32> { None }                  // only for quantifiers

    // Subtype relation betwenn qualifier and quantifier features
    fn subtypeof(self:&Self, other:&dyn Feature) -> bool {
        if self.feature_class() != other.feature_class() { return false };
        match self.feature_type() {
            FeatureType::Qualifier => {
                if self.hash_code() == other.hash_code() { return true }
                let mut s = self.supertype();
                loop {
                    match s {
                        Some(ref f) => { 
                            if f.hash_code() == other.hash_code() { break } 
                            else { s = s.expect("unexpected error").supertype() }
                        },
                        None => break
                    };                    
                };
                s.is_some()
            },
            FeatureType::Quantifier => {       

                let v1 = self.val().expect("unexpected error");
                let v2 = other.val().expect("unexpected error");

                let qt_self = self.quantifier_type().expect("quantifer expected");
                let qt_other = other.quantifier_type().expect("quantifier expected");
               
                match qt_self {
                    QuantifierType::AtLeast => match qt_other {
                        QuantifierType::AtLeast => v1 <= v2,
                        QuantifierType::AtMost => false,
                        QuantifierType::Value => { v1 <= v2 },
                    },
                    QuantifierType::AtMost => match qt_other {
                        QuantifierType::AtLeast => false,
                        QuantifierType::AtMost => v1 >= v2,
                        QuantifierType::Value => v1 >= v2,
                    },
                    QuantifierType::Value => match qt_other {
                        QuantifierType::AtLeast => { v1 >= v2 },
                        QuantifierType::AtMost => v1 <= v2,
                        QuantifierType::Value => v1 == v2,
                    },
                }
            },
        }
    }

}



