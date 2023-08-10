use core::hash::Hash;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use itertools::*;
use std::vec::IntoIter;

use crate::MatchCond;
use crate::Pending;
use crate::rbe0::Rbe;

pub struct RbeTable<K,V,R> 
where K: Hash + Eq + Display + Default,
      V: Hash + Eq + Default + PartialEq + Clone,
      R: Default + PartialEq + Clone,
{
    rbe: Rbe<K>,
    components: HashMap<K, Vec<MatchCond<K, V, R>>>
}

impl <K, V, R> RbeTable<K,V,R> 
where K: Hash + Eq + Debug + Display + Default,
      V: Hash + Eq + Default + Debug + Display + PartialEq + Clone,
      R: Default + PartialEq + Debug + Display + Clone,
{
    pub fn new() -> RbeTable<K,V,R> {
        RbeTable {
            rbe: Rbe::Empty,
            components: HashMap::new()
        }
    }

    pub fn matches(&self, values: Vec<(K,V)>) -> MatchTableIter<K,V,R> {
        let empty= Vec::new();
        let mut rs = Vec::new();
        for (key, value) in values {
            let conds = self.components.get(&key).unwrap_or_else(|| &empty);
            let mut pairs = Vec::new();
            for c in conds {
                pairs.push((value.clone(), (*c).clone()));
            }
            rs.push(pairs);
        }

        let mp: MultiProduct<IntoIter<(V, MatchCond<K, V, R>)>> = rs.into_iter().multi_cartesian_product(); 
        MatchTableIter {
            state: mp
        } 
    }
}

pub struct MatchTableIter<K,V,R> 
where K: Hash + Eq + Display + Default,
      V: Hash + Eq + Default + Display + Debug + PartialEq + Clone,
      R: Default + PartialEq + Display + Debug + Clone,
{
    state: MultiProduct<IntoIter<(V, MatchCond<K, V, R>)>>
}

impl <K, V, R> Iterator for MatchTableIter<K, V, R> 
where K: Hash + Eq + Display + Default,
      V: Hash + Eq + Default + Display + Debug + PartialEq + Clone,
      R: Default + PartialEq + Display + Debug + Clone,
{
    type Item = Pending<V, R>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_state = self.state.next();
        match next_state {
            None => None,
            Some(vs) => {
                for (v,cond) in vs {
                    print!("({v}, {cond})");
                }
                Some(Pending::new())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rbe_table_1() {
        let is_a: MatchCond<char, char, char> = 
          MatchCond::new().with_name("is_a".to_string());
        let ref_t: MatchCond<char, char, char> = 
          MatchCond::new().with_name("ref_t".to_string());
        let ref_u: MatchCond<char, char, char> = 
          MatchCond::new().with_name("ref_u".to_string());
    
        let vs = vec![('p', 'a'), ('q', 'y'), ('q', 'z')];
        let rbe_table:RbeTable<char, char,char> = RbeTable {
           rbe: Rbe::Empty,
           components: HashMap::from_iter(vec![('p', vec![is_a]), ('q', vec![ref_t, ref_u])].into_iter())
        };

        let mut iter = rbe_table.matches(vs);

        assert_eq!(iter.next(), 
          Some(Pending::from(vec![('y', vec!['t']), ('z', vec!['t'])].into_iter())));
        assert_eq!(iter.next(), 
          Some(Pending::from(vec![('y', vec!['t']), ('z', vec!['u'])].into_iter())));
        assert_eq!(iter.next(), 
          Some(Pending::from(vec![('y', vec!['u']), ('z', vec!['t'])].into_iter())));
        assert_eq!(iter.next(), 
          Some(Pending::from(vec![('y', vec!['u']), ('z', vec!['u'])].into_iter())));
        assert_eq!(iter.next(), 
          None);

    }

}