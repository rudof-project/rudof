use rbe::Pending;
use std::hash::Hash;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display, Formatter},
};
#[derive(Debug, Clone)]
pub struct ResultMap<N, S>
where
    N: Hash + Eq,
    S: Hash + Eq,
{
    ok_map: HashMap<N, HashSet<S>>,
    fail_map: HashMap<N, HashSet<S>>,
    pending: Pending<N, S>,
}

impl<N, S> ResultMap<N, S>
where
    N: Hash + Eq + Clone + Debug,
    S: Hash + Eq + Clone + Debug,
{
    pub fn new() -> ResultMap<N, S> {
        ResultMap {
            ok_map: HashMap::new(),
            fail_map: HashMap::new(),
            pending: Pending::new(),
        }
    }

    /*pub fn iter(&self) -> ResultMapIterator {
        ResultMapIterator {
            ok_iter: self.ok_map.iter(),
            fail_iter: self.fail_map.iter(),
            pending_iter: self.pending.iter(),
        }
    }*/
}

impl<N, S> Display for ResultMap<N, S>
where
    N: Hash + Eq + Display + Clone + Debug,
    S: Hash + Eq + Display + Clone + Debug,
{
    fn fmt(&self, dest: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for (n, hs) in &self.ok_map {
            write!(dest, "{n}->+|")?;
            for s in hs {
                write!(dest, "{s}|")?;
            }
        }
        writeln!(dest);
        for (n, hs) in &self.fail_map {
            write!(dest, "{n}->NOT |")?;
            for s in hs {
                write!(dest, "{s}|")?;
            }
        }
        writeln!(dest);
        for (n, s) in self.pending.iter() {
            writeln!(dest, "{n}->?{s}")?;
        }
        Ok(())
    }
}

enum ResultValue {
    Pending,
    Ok,
    Failed,
}

/*struct ResultMapIterator<N, S> {
    ok_iter: Iterator<Item = (N, S)>,
    fail_iter: Iterator<Item = (N, S)>,
    pending_iter: Iterator<Item = (N, S)>,
}

impl<N, S> Iterator for ResultMapIterator<N, S> {
    type Item = (N, S, ResultValue);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((o, l)) = ok_iter.next() {
            Some((o, l, ResultValue::Ok))
        } else {
            if let Some((o, l)) = fail_iter.next() {
                Some((o, l, ResultValue::Failed))
            } else {
                if let Some((o, l)) = pending_iter.next() {
                    Some((o, l, ResultValue::Pending))
                } else {
                    None
                }
            }
        }
    }
}*/
