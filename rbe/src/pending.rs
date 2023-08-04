use std::collections::HashMap;
use std::hash::Hash;
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pending<V,R> 
where V: Hash + Eq
{
    pending: HashMap<V,Vec<R>>
}

impl <V,R> Pending<V,R>
where V: Hash + Eq {
    pub fn new() -> Pending<V,R> {
        Pending {
            pending: HashMap::new()
        }
    }
}