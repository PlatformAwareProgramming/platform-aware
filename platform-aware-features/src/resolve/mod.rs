use std::collections::HashMap;

use crate::{readplatformfeatures, Feature, PlatformParameters, PLATFORM_PARAMETERS};




// The glorious resolution algorithm
pub fn resolve(impls:Vec<HashMap<PlatformParameters, &dyn Feature>> ) -> i32
{
    let currentfeatures2: HashMap<PlatformParameters, &dyn Feature> = readplatformfeatures();

    let mut lastimpl: i32 = (impls.len()-1).try_into().unwrap();
    let mut current_choice: Option<&HashMap<PlatformParameters, &dyn Feature>> = None;
    let mut current_choice_index  = -1;

    loop {
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
pub fn issubtypeof(lhs: &HashMap<PlatformParameters, &dyn Feature>, rhs: &HashMap<PlatformParameters, &dyn Feature>) -> bool {

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