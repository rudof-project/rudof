use crate::{
    Component, Key, MatchCond, Pending, Rbe as Rbe1, RbeError, RbeTable, Ref, Value, Values, rbe::Rbe, rbe_error,
};

#[derive(Debug, Clone)]
pub struct EmptyIter<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    is_first: bool,
    rbe: Rbe1<K, V, R>,
    values: Values<K, V>,
}

impl<K, V, R> EmptyIter<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    pub fn new(rbe: &Rbe<Component>, table: &RbeTable<K, V, R>, values: &Values<K, V>) -> Self {
        let rbe1 = cnv_rbe(rbe, table);
        Self {
            is_first: true,
            rbe: rbe1,
            values: values.clone(),
        }
    }
}

impl<K, V, R> Iterator for EmptyIter<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    type Item = Result<Pending<V, R>, rbe_error::RbeError<K, V, R>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
            if self.rbe.nullable() {
                Some(Ok(Pending::new()))
            } else {
                Some(Err(RbeError::EmptyCandidates {
                    rbe: Box::new(self.rbe.clone()),
                    values: self.values.clone(),
                }))
            }
        } else {
            None
        }
    }
}

fn cnv_rbe<K, V, R>(rbe: &Rbe<Component>, table: &RbeTable<K, V, R>) -> Rbe1<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    match rbe {
        Rbe::Empty => Rbe1::Empty,
        Rbe::And { values } => {
            let values1 = values.iter().map(|c| cnv_rbe(c, table)).collect();
            Rbe1::And { exprs: values1 }
        },
        Rbe::Or { values } => {
            let values1 = values.iter().map(|c| cnv_rbe(c, table)).collect();
            Rbe1::Or { exprs: values1 }
        },
        Rbe::Symbol { value, card } => {
            let key = cnv_key(value, table);
            let cond = cnv_cond(value, table);
            Rbe1::Symbol {
                key,
                cond,
                card: (*card).clone(),
            }
        },
        _ => todo!(),
    }
}

fn cnv_cond<K, V, R>(c: &Component, table: &RbeTable<K, V, R>) -> MatchCond<K, V, R>
where
    K: Key,
    V: Value,
    R: Ref,
{
    table.get_condition(c).unwrap().clone()
}

fn cnv_key<K, V, R>(c: &Component, table: &RbeTable<K, V, R>) -> K
where
    K: Key,
    V: Value,
    R: Ref,
{
    table.get_key(c).unwrap().clone()
}
