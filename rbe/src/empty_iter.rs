use crate::{
    Component, Context, Key, MatchCond, Pending, RbeError, RbeStruct, RbeTable, Ref, Value, Values, rbe::Rbe,
    rbe_cond::RbeCond, rbe_error,
};

#[derive(Debug, Clone)]
pub struct EmptyIter<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    is_first: bool,
    rbe: RbeCond<K, V, R, Ctx>,
    values: Values<K, V, Ctx>,
}

impl<K, V, R, Ctx> EmptyIter<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    pub fn new(rbe: &RbeStruct<Component>, table: &RbeTable<K, V, R, Ctx>, values: &Values<K, V, Ctx>) -> Self {
        let rbe1 = cnv_rbe(rbe.inner_rbe(), table);
        Self {
            is_first: true,
            rbe: rbe1,
            values: values.clone(),
        }
    }
}

impl<K, V, R, Ctx> Iterator for EmptyIter<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    type Item = Result<Pending<V, R>, rbe_error::RbeError<K, V, R, Ctx>>;

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

fn cnv_rbe<K, V, R, Ctx>(rbe: &Rbe<Component>, table: &RbeTable<K, V, R, Ctx>) -> RbeCond<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    match rbe {
        Rbe::Empty => RbeCond::Empty,
        Rbe::And { values } => {
            let values1 = values.iter().map(|c| cnv_rbe(c, table)).collect();
            RbeCond::And { exprs: values1 }
        },
        Rbe::Or { values } => {
            let values1 = values.iter().map(|c| cnv_rbe(c, table)).collect();
            RbeCond::Or { exprs: values1 }
        },
        Rbe::Symbol { value, card } => {
            let key = cnv_key(value, table);
            let cond = cnv_cond(value, table);
            RbeCond::Symbol {
                key,
                cond,
                card: (*card).clone(),
            }
        },
        Rbe::Fail { error } => RbeCond::Fail {
            error: RbeError::MsgError {
                msg: format!("{error}"),
            },
        },
        Rbe::Star { value } => RbeCond::Star {
            expr: Box::new(cnv_rbe(value, table)),
        },
        Rbe::Plus { value } => RbeCond::Plus {
            expr: Box::new(cnv_rbe(value, table)),
        },
        Rbe::Repeat { value, card } => RbeCond::Repeat {
            expr: Box::new(cnv_rbe(value, table)),
            card: (*card).clone(),
        },
    }
}

fn cnv_cond<K, V, R, Ctx>(c: &Component, table: &RbeTable<K, V, R, Ctx>) -> MatchCond<K, V, R, Ctx>
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    table.get_condition(c).unwrap().clone()
}

fn cnv_key<K, V, R, Ctx>(c: &Component, table: &RbeTable<K, V, R, Ctx>) -> K
where
    K: Key,
    V: Value,
    R: Ref,
    Ctx: Context,
{
    table.get_key(c).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Cardinality, Max, Min, RbeStruct, SingleCond};

    impl Key for u8 {}
    impl Value for u16 {}
    impl Ref for u32 {}

    type K = u8;
    type V = u16;
    type R = u32;
    type Ctx = char;

    fn make_table_with_symbol() -> (RbeTable<K, V, R, Ctx>, Component) {
        let mut table = RbeTable::new();
        let cond: MatchCond<K, V, R, Ctx> = MatchCond::single(
            SingleCond::new()
                .with_name("any")
                .with_cond(|_v, _c| Ok(Pending::new())),
        );
        let c = table.add_component(1, &cond);
        (table, c)
    }

    #[test]
    fn cnv_rbe_empty() {
        let table: RbeTable<K, V, R, Ctx> = RbeTable::new();
        let rbe = Rbe::Empty;
        let result = cnv_rbe(&rbe, &table);
        assert_eq!(result, RbeCond::Empty);
    }

    #[test]
    fn cnv_rbe_symbol() {
        let (table, c) = make_table_with_symbol();
        let rbe = RbeStruct::symbol(c, 1, Max::IntMax(3));
        let result = cnv_rbe(rbe.inner_rbe(), &table);
        match result {
            RbeCond::Symbol { key, card, .. } => {
                assert_eq!(key, 1);
                assert_eq!(card, Cardinality::from(Min::from(1), Max::IntMax(3)));
            },
            other => panic!("Expected Symbol, got {:?}", other),
        }
    }

    #[test]
    fn cnv_rbe_and() {
        let (table, c) = make_table_with_symbol();
        let rbe = RbeStruct::and(vec![RbeStruct::empty(), RbeStruct::symbol(c, 1, Max::IntMax(1))]);
        let result = cnv_rbe(rbe.inner_rbe(), &table);
        match result {
            RbeCond::And { exprs } => {
                assert_eq!(exprs.len(), 2);
                assert_eq!(exprs[0], RbeCond::Empty);
                assert!(matches!(exprs[1], RbeCond::Symbol { .. }));
            },
            other => panic!("Expected And, got {:?}", other),
        }
    }

    #[test]
    fn cnv_rbe_or() {
        let (table, c) = make_table_with_symbol();
        let rbe = RbeStruct::or(vec![RbeStruct::empty(), RbeStruct::symbol(c, 0, Max::IntMax(1))]);
        let result = cnv_rbe(rbe.inner_rbe(), &table);
        match result {
            RbeCond::Or { exprs } => {
                assert_eq!(exprs.len(), 2);
                assert_eq!(exprs[0], RbeCond::Empty);
                assert!(matches!(exprs[1], RbeCond::Symbol { .. }));
            },
            other => panic!("Expected Or, got {:?}", other),
        }
    }

    #[test]
    fn cnv_rbe_star() {
        let (table, c) = make_table_with_symbol();
        let rbe = RbeStruct::star(RbeStruct::symbol(c, 1, Max::IntMax(1)));
        let result = cnv_rbe(rbe.inner_rbe(), &table);
        match result {
            RbeCond::Star { expr } => {
                assert!(matches!(*expr, RbeCond::Symbol { .. }));
            },
            other => panic!("Expected Star, got {:?}", other),
        }
    }

    #[test]
    fn cnv_rbe_plus() {
        let (table, c) = make_table_with_symbol();
        let rbe = RbeStruct::plus(RbeStruct::symbol(c, 1, Max::IntMax(1)));
        let result = cnv_rbe(rbe.inner_rbe(), &table);
        match result {
            RbeCond::Plus { expr } => {
                assert!(matches!(*expr, RbeCond::Symbol { .. }));
            },
            other => panic!("Expected Plus, got {:?}", other),
        }
    }

    #[test]
    fn cnv_rbe_repeat() {
        let (table, c) = make_table_with_symbol();
        let rbe = RbeStruct::repeat(RbeStruct::symbol(c, 1, Max::IntMax(1)), 2, Max::IntMax(5));
        let result = cnv_rbe(rbe.inner_rbe(), &table);
        match result {
            RbeCond::Repeat { expr, card } => {
                assert!(matches!(*expr, RbeCond::Symbol { .. }));
                assert_eq!(card, Cardinality::from(Min::from(2), Max::IntMax(5)));
            },
            other => panic!("Expected Repeat, got {:?}", other),
        }
    }

    #[test]
    fn cnv_rbe_fail() {
        let table: RbeTable<K, V, R, Ctx> = RbeTable::new();
        let rbe = Rbe::Fail {
            error: crate::deriv_error::DerivError::MkOrValuesFail,
        };
        let result = cnv_rbe(&rbe, &table);
        assert!(matches!(result, RbeCond::Fail { .. }));
    }

    #[test]
    fn cnv_rbe_nested_and_or() {
        let (table, c) = make_table_with_symbol();
        let rbe = RbeStruct::and(vec![
            RbeStruct::or(vec![RbeStruct::empty(), RbeStruct::symbol(c, 1, Max::IntMax(1))]),
            RbeStruct::empty(),
        ]);
        let result = cnv_rbe(rbe.inner_rbe(), &table);
        match result {
            RbeCond::And { exprs } => {
                assert_eq!(exprs.len(), 2);
                assert!(matches!(exprs[0], RbeCond::Or { .. }));
                assert_eq!(exprs[1], RbeCond::Empty);
            },
            other => panic!("Expected And, got {:?}", other),
        }
    }
}
