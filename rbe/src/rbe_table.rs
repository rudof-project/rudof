use core::hash::Hash;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use itertools::*;
use std::vec::IntoIter;

use crate::Bag;
use crate::MatchCond;
use crate::Pending;
use crate::rbe::Rbe;
use crate::{rbe_error, deriv_error};
use crate::Component;

pub struct RbeTable<K,V,R> 
where K: Hash + Eq + Display + Default,
      V: Hash + Eq + Default + PartialEq + Clone,
      R: Default + PartialEq + Clone,
{
    rbe: Rbe<Component>,
    key_components: HashMap<K, Vec<Component>>,
    component_cond: HashMap<Component, MatchCond<K,V,R>>,
    open: bool, 
    component_counter: usize
}

impl <K, V, R> RbeTable<K,V,R> 
where K: Hash + Eq + Debug + Display + Default + Clone,
      V: Hash + Eq + Default + Debug + Display + PartialEq + Clone,
      R: Default + PartialEq + Debug + Display + Clone,
{
    pub fn new() -> RbeTable<K,V,R> {
        RbeTable {
            rbe: Rbe::Empty,
            key_components: HashMap::new(),
            component_cond: HashMap::new(),
            open: false,
            component_counter: 0
        }
    }

    pub fn add_component(&mut self, k: K, component_cond: MatchCond<K,V,R>) {
        // self.key_components.entry(k)
        todo!();
        // self.component_counter += 1;
    }

    pub fn matches(&self, values: Vec<(K,V)>) -> MatchTableIter<K,V,R> {
        let empty= Vec::new();
        let mut rs = Vec::new();
        for (key, value) in values {
            let conds = self.key_components.get(&key).unwrap_or_else(|| &empty);
            let mut pairs = Vec::new();
            for c in conds {
                // TODO: Add some better error control (this should mark an internal error anyway)
                let cond = self.component_cond.get(c).unwrap();
                pairs.push((key.clone(), value.clone(), (*c).clone(), (*cond).clone()));
            }
            rs.push(pairs);
        }

        let mp: MultiProduct<IntoIter<(K, V, Component, MatchCond<K,V,R>)>> = rs.into_iter().multi_cartesian_product(); 
        MatchTableIter {
            state: mp,
            rbe: self.rbe.clone(),
            open: self.open,
            // controlled: self.controlled.clone()
        } 
    }
}

pub struct MatchTableIter<K, V, R> 
where K: Hash + Eq + Display + Default + Clone,
      V: Hash + Eq + Default + Display + Debug + PartialEq + Clone,
      R: Default + PartialEq + Display + Debug + Clone,
{
    state: MultiProduct<IntoIter<(K, V, Component, MatchCond<K,V,R>)>>,
    rbe: Rbe<Component>,
    open: bool,
    // controlled: HashSet<K>
}

impl <K, V, R> Iterator for MatchTableIter<K, V, R> 
where K: Hash + Eq + Display + Default + Clone,
      V: Hash + Eq + Default + Display + Debug + PartialEq + Clone,
      R: Default + PartialEq + Display + Debug + Clone,
{
    type Item = Result<Pending<V, R>, rbe_error::RbeError<K, V, R>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_state = self.state.next();
        match next_state {
            None => None,
            Some(vs) => {
                for (k, v, c, cond) in &vs {
                    print!("(({k},{v}), {c},{cond})");
                }
                let mut pending: Pending<V,R> = Pending::new();
                for (k,v,_,cond) in &vs {
                    match cond.matches(k, v) {
                        Ok(new_pending) => { 
                            pending.merge(new_pending)
                        },
                        Err(err) => { 
                            return Some(Err(err));
                        }
                    }
                }
                println!("Pending after checking conditions: {pending:?}");
                let bag = Bag::from_iter(vs.into_iter().map(|(_, _, c, _)| { c} ));
                match self.rbe.match_bag(&bag, self.open) {
                    Ok(()) => { Some(Ok(pending)) },
                    Err(err) => { 
                        println!("Error: {err}\nSkipped");
                        self.next()
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Max;

    use super::*;

    #[test]
    fn test_rbe_table_1() {
        
        let is_a: MatchCond<char, char, char> = 
          MatchCond::new().with_name("is_a".to_string()).with_cond(move |k,v| {
            if *v == 'a' { Ok(Pending::new()) } else { 
                Err(rbe_error::RbeError::MsgError {
                  msg: format!("Value {v}!='a'")
               })}
          });
        
        let ref_t: MatchCond<char, char, char> = 
          MatchCond::new().with_name("ref_t".to_string()).with_cond(move |k, v| {
            let mut pending = Pending::new();
            pending.insert(*v, 't');
            Ok(pending)
          });

        let ref_u: MatchCond<char, char, char> = 
          MatchCond::new().with_name("ref_u".to_string()).with_cond(move |k, v| {
            let mut pending = Pending::new();
            pending.insert(*v, 'u');
            Ok(pending)
          });

        let c1 = Component::from(1);  
        let c2 = Component::from(2);  
        let c3 = Component::from(3);  
    
        let vs = vec![('p', 'a'), ('q', 'y'), ('q', 'z')];
        let rbe_table:RbeTable<char, char, char> = RbeTable {
           rbe: Rbe::and(vec![
            Rbe::symbol(c1,1,Max::IntMax(1)),
            Rbe::symbol(c2,1,Max::IntMax(1)),
            Rbe::symbol(c3,1,Max::Unbounded)
           ]),
           key_components: HashMap::from_iter(vec![('p', vec![c1]), ('q', vec![c2, c3])].into_iter()),
           component_cond: HashMap::from_iter(vec![(c1, is_a), (c2, ref_t), (c3, ref_u)]),
           open: false,
           component_counter: 0
           // controlled: HashSet::from_iter(vec!['p','q'])
        };

        let mut iter = rbe_table.matches(vs);

        assert_eq!(iter.next(), 
          Some(Ok(Pending::from(vec![('y', vec!['t']), ('z', vec!['u'])].into_iter()))));
        assert_eq!(iter.next(), 
          Some(Ok(Pending::from(vec![('y', vec!['u']), ('z', vec!['t'])].into_iter()))));
        assert_eq!(iter.next(), 
          None);

    }

}