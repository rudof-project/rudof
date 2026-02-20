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
        Rbe::Fail { error } => Rbe1::Fail {
            error: RbeError::MsgError {
                msg: format!("{error}"),
            },
        },
        Rbe::Star { value } => Rbe1::Star {
            expr: Box::new(cnv_rbe(value, table)),
        },
        Rbe::Plus { value } => Rbe1::Plus {
            expr: Box::new(cnv_rbe(value, table)),
        },
        Rbe::Repeat { value, card } => Rbe1::Repeat {
            expr: Box::new(cnv_rbe(value, table)),
            card: (*card).clone(),
        },
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cardinality, Max, Min, SingleCond};

    impl Key for u8 {}
    impl Value for u16 {}
    impl Ref for u32 {}

    type K = u8;
    type V = u16;
    type R = u32;

    fn make_table_with_symbol() -> (RbeTable<K, V, R>, Component) {
        let mut table = RbeTable::new();
        let cond: MatchCond<K, V, R> =
            MatchCond::single(SingleCond::new().with_name("any").with_cond(|_v| Ok(Pending::new())));
        let c = table.add_component(1, &cond);
        (table, c)
    }

    #[test]
    fn cnv_rbe_empty() {
        let table: RbeTable<K, V, R> = RbeTable::new();
        let rbe = Rbe::Empty;
        let result = cnv_rbe(&rbe, &table);
        assert_eq!(result, Rbe1::Empty);
    }

    #[test]
    fn cnv_rbe_symbol() {
        let (table, c) = make_table_with_symbol();
        let rbe = Rbe::symbol(c, 1, Max::IntMax(3));
        let result = cnv_rbe(&rbe, &table);
        match result {
            Rbe1::Symbol { key, card, .. } => {
                assert_eq!(key, 1);
                assert_eq!(card, Cardinality::from(Min::from(1), Max::IntMax(3)));
            },
            other => panic!("Expected Symbol, got {:?}", other),
        }
    }

    #[test]
    fn cnv_rbe_and() {
        let (table, c) = make_table_with_symbol();
        let rbe = Rbe::and(vec![Rbe::Empty, Rbe::symbol(c, 1, Max::IntMax(1))]);
        let result = cnv_rbe(&rbe, &table);
        match result {
            Rbe1::And { exprs } => {
                assert_eq!(exprs.len(), 2);
                assert_eq!(exprs[0], Rbe1::Empty);
                assert!(matches!(exprs[1], Rbe1::Symbol { .. }));
            },
            other => panic!("Expected And, got {:?}", other),
        }
    }

    #[test]
    fn cnv_rbe_or() {
        let (table, c) = make_table_with_symbol();
        let rbe = Rbe::or(vec![Rbe::Empty, Rbe::symbol(c, 0, Max::IntMax(1))]);
        let result = cnv_rbe(&rbe, &table);
        match result {
            Rbe1::Or { exprs } => {
                assert_eq!(exprs.len(), 2);
                assert_eq!(exprs[0], Rbe1::Empty);
                assert!(matches!(exprs[1], Rbe1::Symbol { .. }));
            },
            other => panic!("Expected Or, got {:?}", other),
        }
    }

    #[test]
    fn cnv_rbe_star() {
        let (table, c) = make_table_with_symbol();
        let rbe = Rbe::star(Rbe::symbol(c, 1, Max::IntMax(1)));
        let result = cnv_rbe(&rbe, &table);
        match result {
            Rbe1::Star { expr } => {
                assert!(matches!(*expr, Rbe1::Symbol { .. }));
            },
            other => panic!("Expected Star, got {:?}", other),
        }
    }

    #[test]
    fn cnv_rbe_plus() {
        let (table, c) = make_table_with_symbol();
        let rbe = Rbe::plus(Rbe::symbol(c, 1, Max::IntMax(1)));
        let result = cnv_rbe(&rbe, &table);
        match result {
            Rbe1::Plus { expr } => {
                assert!(matches!(*expr, Rbe1::Symbol { .. }));
            },
            other => panic!("Expected Plus, got {:?}", other),
        }
    }

    #[test]
    fn cnv_rbe_repeat() {
        let (table, c) = make_table_with_symbol();
        let rbe = Rbe::repeat(Rbe::symbol(c, 1, Max::IntMax(1)), 2, Max::IntMax(5));
        let result = cnv_rbe(&rbe, &table);
        match result {
            Rbe1::Repeat { expr, card } => {
                assert!(matches!(*expr, Rbe1::Symbol { .. }));
                assert_eq!(card, Cardinality::from(Min::from(2), Max::IntMax(5)));
            },
            other => panic!("Expected Repeat, got {:?}", other),
        }
    }

    #[test]
    fn cnv_rbe_fail() {
        let table: RbeTable<K, V, R> = RbeTable::new();
        let rbe = Rbe::Fail {
            error: crate::deriv_error::DerivError::MkOrValuesFail,
        };
        let result = cnv_rbe(&rbe, &table);
        assert!(matches!(result, Rbe1::Fail { .. }));
    }

    #[test]
    fn cnv_rbe_nested_and_or() {
        let (table, c) = make_table_with_symbol();
        let rbe = Rbe::and(vec![
            Rbe::or(vec![Rbe::Empty, Rbe::symbol(c, 1, Max::IntMax(1))]),
            Rbe::Empty,
        ]);
        let result = cnv_rbe(&rbe, &table);
        match result {
            Rbe1::And { exprs } => {
                assert_eq!(exprs.len(), 2);
                assert!(matches!(exprs[0], Rbe1::Or { .. }));
                assert_eq!(exprs[1], Rbe1::Empty);
            },
            other => panic!("Expected And, got {:?}", other),
        }
    }
}
