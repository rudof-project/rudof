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
use crate::RbeError;
use crate::rbe::Rbe;
use crate::{rbe_error, deriv_error};
use crate::Component;


#[derive(Debug, PartialEq)]
pub struct RbeTable<K,V,R> 
where K: Hash + Eq + Display + Default + Clone,
      V: Hash + Eq + Default + Display + PartialEq + Clone,
      R: Default + PartialEq + Display + Clone,
{
    rbe: Rbe<Component>,
    key_components: HashMap<K, HashSet<Component>>,
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

    pub fn add_component(&mut self, k: K, cond: MatchCond<K,V,R>) -> Component {
        let c = Component::from(self.component_counter);
        self.key_components.entry(k).and_modify(|vs| {
            (*vs).insert(c);
        }).or_insert_with(|| { 
            let mut hs = HashSet::new();
            hs.insert(c);
            hs
        });
        self.component_cond.insert(c,cond);
        self.component_counter += 1;
        c
    }

    pub fn with_rbe(&mut self, rbe: Rbe<Component>) {
        self.rbe = rbe;
    }

/*    pub fn from_rbe1(rbe1: rbe1::Rbe<K, V,R>) -> Result<RbeTable<K, V, R>, RbeError<K,V,R>> {
        let mut rbe_table = RbeTable::new();    
        Self::rbe1_to_table(&rbe1, &mut rbe_table)?;
        Ok(rbe_table)
    }

    fn rbe1_to_table(rbe1: &rbe1::Rbe<K,V,R>, current: &mut RbeTable<K,V,R>) -> Result<Rbe<Component>, RbeError<K,V,R>> {
       match *rbe1 {
         rbe1::Rbe::Fail { error } => Err(error),
         rbe1::Rbe::Empty => { 
            *current = RbeTable::new(); 
            Ok(Rbe::Empty) 
        },
         rbe1::Rbe::Symbol { key, cond, card } => { 
            let comp = Component::from(current.component_counter);
            current.key_components
            .entry(key)
            .and_modify(|vs| { vs.push(comp); })
            .or_insert(vec![comp]);
            current.component_cond.insert(comp, cond);
            current.component_counter += 1;
            let rbe = Rbe::symbol(comp, card.min.value, card.max);
            current.rbe = rbe;
            Ok(rbe)
        },
        rbe1::Rbe::And { exprs } => {
            let mut es = Vec::new();
            let new_table = exprs.into_iter()
              .try_fold(&current, move |c, e| {
                let new_rbe = Self::rbe1_to_table(&e, &mut c)?;
                es.push(new_rbe);
                Ok(c)
            })?;
            let new_rbe = Rbe::and(es);
            current.rbe = new_rbe;
            Ok(new_rbe)
        },
        _ => {
            todo!()
        }
       }
    }
*/
    pub fn matches(&self, values: Vec<(K,V)>) -> MatchTableIter<K,V,R> {
        let empty= HashSet::new();
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

    
        let vs = vec![('p', 'a'), ('q', 'y'), ('q', 'z')];
        let mut rbe_table = RbeTable::new() ;
        let c1 = rbe_table.add_component('p', is_a);
        let c2 = rbe_table.add_component('q', ref_t);
        let c3 = rbe_table.add_component('q', ref_u);
        rbe_table.with_rbe(Rbe::and(vec![
            Rbe::symbol(c1,1,Max::IntMax(1)),
            Rbe::symbol(c2,1,Max::IntMax(1)),
            Rbe::symbol(c3,1,Max::Unbounded)
           ]));

        let mut iter = rbe_table.matches(vs);

        assert_eq!(iter.next(), 
          Some(Ok(Pending::from(vec![('y', vec!['t']), ('z', vec!['u'])].into_iter()))));
        assert_eq!(iter.next(), 
          Some(Ok(Pending::from(vec![('y', vec!['u']), ('z', vec!['t'])].into_iter()))));
        assert_eq!(iter.next(), 
          None);

    }

}