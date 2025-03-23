use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};


#[derive(Debug)]
pub enum PlatformParameters {
    SIMD,
    ACCModel,
    ACCBackend,
    ACCCount
}


const PLATFORM_PARAMETERS:&[PlatformParameters] = &[PlatformParameters::SIMD, 
                                                    PlatformParameters::ACCModel, 
                                                    PlatformParameters::ACCBackend, 
                                                    PlatformParameters::ACCCount];

impl PartialEq for PlatformParameters {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for PlatformParameters {}

impl std::hash::Hash for PlatformParameters {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

// QUANTIFIERS

pub enum QuantifierType {
    AtLeast,
    AtMost,
    Between,
    Value
}

pub struct AtLeast {
    pub val:i32
}

pub struct AtMost {
    pub val:i32
}

pub struct Between {
    pub val0:i32,
    pub val1:i32
}

pub trait Quantifier {
    fn quantifier_type() -> QuantifierType;
    fn val(self:&Self) -> i32;
}

impl Quantifier for i32 {
    fn quantifier_type() -> QuantifierType { QuantifierType::Value }
    fn val(self:&Self) -> i32 {*self}
}

impl Quantifier for AtLeast {
    fn quantifier_type() -> QuantifierType { QuantifierType::AtLeast }
    fn val(self:&Self) -> i32 {self.val}
}

impl Quantifier for AtMost {
    fn quantifier_type() -> QuantifierType { QuantifierType::AtMost }
    fn val(self:&Self) -> i32 {self.val}
}

impl Quantifier for Between {
    fn quantifier_type() -> QuantifierType { QuantifierType::Between }
    fn val(self:&Self) -> i32 {self.val0}
}


// FEATURES

pub enum FeatureType {
    Qualifier,
    QuantifierUp,
    QuantifierDown,
}

pub trait Feature {

    fn is_top(self:&Self) -> bool { false }

    fn string(self:&Self) -> String;

    fn hash_code(self:&Self) -> u64 {
        let mut s = DefaultHasher::new();
        self.string().hash(&mut s);
        s.finish()
    }

    fn feature_class(self:&Self) -> PlatformParameters;

    fn feature_type(self:&Self) -> FeatureType { FeatureType::Qualifier }

    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { 
        match self.feature_type() {
            FeatureType::Qualifier => None,
            FeatureType::QuantifierUp => {   
                let v = self.val().expect("unexpected error");        
                Some(Box::new(AccCount { val:v+1 }) as Box<dyn Feature>)                         
            },
            FeatureType::QuantifierDown => {   
                let v = self.val().expect("unexpected error");        
                Some(Box::new(AccCount { val:v-1 }) as Box<dyn Feature>)                         
            },
        }
    }

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
            FeatureType::QuantifierDown => {       

                let v1 = self.val().expect("unexpected error");
                let v2 = other.val().expect("unexpected error");

                let qt_self = self.quantifier_type().expect("quantifer expected");
                let qt_other = other.quantifier_type().expect("quantifier expected");
                match qt_self {
                    QuantifierType::AtLeast => match qt_other {
                        QuantifierType::AtLeast => v1 <= v2,
                        QuantifierType::AtMost => false,
                        QuantifierType::Between => false,
                        QuantifierType::Value => v1 <= v2,
                    },
                    QuantifierType::AtMost => match qt_other {
                        QuantifierType::AtLeast => false,
                        QuantifierType::AtMost => v1 >= v2,
                        QuantifierType::Between => false,
                        QuantifierType::Value => v1 >= v2,
                    },
                    QuantifierType::Between => match qt_other {
                        QuantifierType::AtLeast => false,
                        QuantifierType::AtMost => false,
                        QuantifierType::Between => {
                            let v1s = self.other_val().expect("interval upper limit expeted");
                            let v2s = other.other_val().expect("interval upper limit expeted");
                            v1 >= v2 && v1s <= v2s
                        },
                        QuantifierType::Value => {
                            let v1s = self.other_val().expect("interval upper limit expeted");
                            v2 >= v1 && v2<=v1s
                        },
                    },
                    QuantifierType::Value => match qt_other {
                        QuantifierType::AtLeast => v1 >= v2,
                        QuantifierType::AtMost => v1 <= v2,
                        QuantifierType::Between => {
                            let v2s = other.other_val().expect("interval upper limit expeted");
                            v1 >= v2 && v1<=v2s
                        },
                        QuantifierType::Value => v1 <= v2,
                    },
                }
            },
            FeatureType::QuantifierUp => {

                let v1 = self.val().expect("unexpected error");
                let v2 = other.val().expect("unexpected error");
                println!("v1 = {v1}");
                println!("v2 = {v2}");

                let qt_self = self.quantifier_type().expect("quantifer expected");
                let qt_other = other.quantifier_type().expect("quantifier expected");
                match qt_self {
                    QuantifierType::AtLeast => match qt_other {
                        QuantifierType::AtLeast => v1 <= v2,
                        QuantifierType::AtMost => false,
                        QuantifierType::Between => false,
                        QuantifierType::Value => v1 <= v2,
                    },
                    QuantifierType::AtMost => match qt_other {
                        QuantifierType::AtLeast => false,
                        QuantifierType::AtMost => v1 >= v2,
                        QuantifierType::Between => false,
                        QuantifierType::Value => v1 >= v2,
                    },
                    QuantifierType::Between => match qt_other {
                        QuantifierType::AtLeast => false,
                        QuantifierType::AtMost => false,
                        QuantifierType::Between => {
                            let v1s = self.other_val().expect("interval upper limit expeted");
                            let v2s = other.other_val().expect("interval upper limit expeted");
                            v1 >= v2 && v1s <= v2s
                        },
                        QuantifierType::Value => {
                            let v1s = self.other_val().expect("interval upper limit expeted");
                            v2 >= v1 && v2<=v1s
                        },
                    },
                    QuantifierType::Value => match qt_other {
                        QuantifierType::AtLeast => v1 >= v2,
                        QuantifierType::AtMost => v1 <= v2,
                        QuantifierType::Between => {
                            let v2s = other.other_val().expect("interval upper limit expeted");
                            v1 >= v2 && v1<=v2s
                        },
                        QuantifierType::Value => v1 >= v2,
                    },
                }
            },
        }
    }

}



// Feature types (for protoyping)

pub struct SIMD;

pub struct AVX512;

pub struct AccModel;

pub struct NVidiaH100;

pub struct AccBackend;

pub struct CUDA4;

pub struct CUDA6;

pub struct CUDA8;

pub struct CUDA10;

// AccCount may be used with at-least, at-most, at-least and exact value (i32)
// Other quantifier features may restrict their use according to their semantics
pub struct AccCount<T:Quantifier>{
    pub val:T
}

impl Feature for SIMD { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "simd".to_string() } 
    fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::SIMD } 
}

impl Feature for AVX512 { 
    fn string(self:&Self) -> String { "avx512".to_string() } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(SIMD)) } 
    fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::SIMD } 
}

impl Feature for AccModel { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "acc_model".to_string() } 
    fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::ACCModel } 
}
                            
impl Feature for NVidiaH100 { 
    fn string(self:&Self) -> String { "nvidia_H100".to_string() } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccModel)) } 
    fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::ACCModel } 
}

impl Feature for AccBackend { 
    fn is_top(self:&Self) -> bool { true }
    fn string(self:&Self) -> String { "acc_backend".to_string() } 
    fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::ACCBackend } 
}

impl Feature for CUDA4 { 
    fn string(self:&Self) -> String { "cuda4".to_string() } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(AccBackend)) } 
    fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::ACCBackend } 
}

impl Feature for CUDA6 { 
    fn string(self:&Self) -> String { "cuda6".to_string() } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CUDA4)) } 
    fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::ACCBackend } 
}

impl Feature for CUDA8 { 
    fn string(self:&Self) -> String { "cuda8".to_string() } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CUDA6)) } 
    fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::ACCBackend } 
}

impl Feature for CUDA10 { 
    fn string(self:&Self) -> String { "cuda10".to_string() } 
    fn supertype(self:&Self) -> Option<Box<dyn Feature>> { Some(Box::new(CUDA8)) } 
    fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::ACCBackend } 
}

impl Feature for AccCount<i32> {  
    fn is_top(self:&Self) -> bool { 
        let v = self.val().expect("a value is expected here");
        match self.feature_type() {
            FeatureType::Qualifier => false,
            FeatureType::QuantifierUp => v==std::i32::MAX,
            FeatureType::QuantifierDown => v==0,
        }                                
    }
    fn string(self:&Self) -> String { "acc_count".to_string() } 
    fn feature_type(self:&Self) -> FeatureType { FeatureType::QuantifierUp } 
    fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::ACCCount}
    fn val(self:&Self) -> Option<i32> { Some((*self).val) } 
    fn quantifier_type(self:&Self) -> Option<QuantifierType> { Some(QuantifierType::Value) }
}

impl Feature for AccCount<AtLeast> {  
    fn is_top(self:&Self) -> bool { 
        let v = self.val().expect("a value is expected here");
        match self.feature_type() {
            FeatureType::Qualifier => false,
            FeatureType::QuantifierUp => v==std::i32::MAX,
            FeatureType::QuantifierDown => v==0,
        }                                
    }
    fn string(self:&Self) -> String { "acc_count".to_string() } 
    fn feature_type(self:&Self) -> FeatureType { FeatureType::QuantifierUp } 
    fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::ACCCount}
    fn val(self:&Self) -> Option<i32> { Some((*self).val.val) } 
    fn quantifier_type(self:&Self) -> Option<QuantifierType> { Some(QuantifierType::AtLeast) }
}

impl Feature for AccCount<AtMost> {  
    fn is_top(self:&Self) -> bool { 
        let v = self.val().expect("a value is expected here");
        match self.feature_type() {
            FeatureType::Qualifier => false,
            FeatureType::QuantifierUp => v==std::i32::MAX,
            FeatureType::QuantifierDown => v==0,
        }                                
    }
    fn string(self:&Self) -> String { "acc_count".to_string() } 
    fn feature_type(self:&Self) -> FeatureType { FeatureType::QuantifierUp } 
    fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::ACCCount}
    fn val(self:&Self) -> Option<i32> { Some((*self).val.val) } 
    fn quantifier_type(self:&Self) -> Option<QuantifierType> { Some(QuantifierType::AtMost) }
}

impl Feature for AccCount<Between> {  
    fn is_top(self:&Self) -> bool { 
        let v = self.val().expect("a value is expected here");
        match self.feature_type() {
            FeatureType::Qualifier => false,
            FeatureType::QuantifierUp => v==std::i32::MAX,
            FeatureType::QuantifierDown => v==0,
        }                                
    }
    fn string(self:&Self) -> String { "acc_count".to_string() } 
    fn feature_type(self:&Self) -> FeatureType { FeatureType::QuantifierUp } 
    fn feature_class(self:&Self) -> PlatformParameters { PlatformParameters::ACCCount}
    fn val(self:&Self) -> Option<i32> { Some((*self).val.val0) } 
    fn other_val(self:&Self) -> Option<i32> { Some((*self).val.val1) } 
    fn quantifier_type(self:&Self) -> Option<QuantifierType> { Some(QuantifierType::Between) }
}




// TODO: automatic read of the platform file at the beginning of the execution
//use lazy_static::lazy_static;
//use std::sync::Mutex;
// lazy_static! { static ref currentfeatures: Mutex<HashMap<PlatformParameters, &'static dyn Feature>> = Mutex::new(readplatformfeatures()); }

// TODO: implement the read of the platform file
pub fn readplatformfeatures() -> HashMap<PlatformParameters, &'static dyn Feature>
{
    let mut m: HashMap<PlatformParameters, &dyn Feature> = HashMap::new();

    println!("readplatformfeatures at startup"); 

    m.insert(PlatformParameters::ACCBackend, &CUDA8);
    m.insert(PlatformParameters::ACCCount, &AccCount{ val:4 });

    return m;
}


// The glorious resolution algorithm
pub fn resolve(impls:Vec<HashMap<PlatformParameters, &dyn Feature>> ) -> i32
{
    let currentfeatures2: HashMap<PlatformParameters, &dyn Feature> = readplatformfeatures();

    let mut lastimpl: i32 = (impls.len()-1).try_into().unwrap();
    let mut current_choice: Option<&HashMap<PlatformParameters, &dyn Feature>> = None;
    let mut current_choice_index  = -1;

    loop {
        println!("loop {lastimpl}");
        while lastimpl >= 0 && !(issubtypeof(&currentfeatures2, &impls[lastimpl as usize]) 
                                   && (current_choice.is_none() || 
                                       issubtypeof(&impls[lastimpl as usize], &current_choice.expect("unexpected condition")))) { 
            lastimpl -= 1;
        }

        if lastimpl < 0 { break }

        current_choice = Some(&impls[lastimpl as usize]);
        current_choice_index = lastimpl;
        lastimpl -= 1;
    }


    println!("loop finish {lastimpl}");

    current_choice_index + 1
}
    

// check whether the left set of features is a subtype of the right set of features (compatibilty relation)
fn issubtypeof(lhs: &HashMap<PlatformParameters, &dyn Feature>, rhs: &HashMap<PlatformParameters, &dyn Feature>) -> bool {

    println!("subtype test"); // DEBUG

    for p in PLATFORM_PARAMETERS  {

        print!("subtype test - parameter {:?}", p); // DEBUG
        
        let vl = lhs.get(p);
        let vr = rhs.get(p);

        // begin DEBUG
        match vl {
            Some(s) => print!(" {}", s.string()),
            None => print!(" {}", "no"),
        };

        match vr {
            Some(s) => print!(" {}", s.string()),
            None => print!(" {}", "no"),
        };
        // end DEBUG

        let issubtype = match vl {
                                None => match vr {
                                    Some(vrt) => vrt.is_top(),
                                    None => true,
                                }
                                Some(vlt) => match vr {
                                    Some(vrt) => vlt.subtypeof(*vrt),
                                    None => true, 
                                }
                              };
        println!(" {}", issubtype); // DEBUG
        if !issubtype { return false; }
    } 

    true
}
