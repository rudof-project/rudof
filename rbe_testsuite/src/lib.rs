use rbe::{Rbe, Bag, RbeError};
use serde_derive::{Deserialize, Serialize};
use std::hash::Hash;
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct RbeTest<T> 
where T: Hash + Eq {
    rbe: Rbe<T>,
    bag: Bag<T>,
    matchResult: MatchResult
}

impl <T> RbeTest<T> 
where T: Hash + Eq + Clone + fmt::Debug {
    pub fn run(&self) -> Result<(), RbeError<T>> {
        self.rbe.match_bag(&self.bag, false)
    }
}

#[derive(Serialize, Deserialize)]
enum MatchResult {
    BooleanResult(bool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let str = r#"{
          "rbe": "Empty",
          "bag": [],
          "matchResult": { "BooleanResult": false }
        }"#;

        let rbe_test: RbeTest<i32> = serde_json::from_str(str).unwrap();
        assert_eq!(rbe_test.run(), Ok(()))
    }
}