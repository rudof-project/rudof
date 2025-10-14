use crate::rbe::Rbe;
use core::hash::Hash;
use pretty::RcDoc;
use std::fmt::Debug;
use std::fmt::Display;

pub struct RbePrettyPrinter {}

impl Default for RbePrettyPrinter {
    fn default() -> Self {
        Self::new()
    }
}

impl RbePrettyPrinter {
    pub fn new() -> Self {
        RbePrettyPrinter {}
    }

    pub fn print<A>(&self, rbe: &Rbe<A>, width: usize) -> String
    where
        A: Hash + Eq + Display + Debug,
    {
        let doc = pp_rbe(rbe);
        doc.pretty(width).to_string()
    }
}

pub fn pp_rbe<A>(rbe: &Rbe<A>) -> RcDoc<'_>
where
    A: Hash + Eq + Display + Debug,
{
    match rbe {
        Rbe::Fail { error } => RcDoc::text(format!("Fail {{{error:?}}}")),
        Rbe::Empty => RcDoc::text(""),
        Rbe::Symbol { value, card } => RcDoc::text(format!("{value}{card}")),
        Rbe::And { values } => values
            .iter()
            .fold(RcDoc::text("("), |acc, value| {
                acc.append(pp_rbe(value)).append(RcDoc::text(" ; "))
            })
            .append(RcDoc::text(")")),
        Rbe::Or { values } => values
            .iter()
            .fold(RcDoc::text("("), |acc, value| {
                acc.append(pp_rbe(value)).append(RcDoc::text(" | "))
            })
            .append(RcDoc::text(")")),
        Rbe::Star { value } => pp_rbe(value).append(RcDoc::text("*")),
        Rbe::Plus { value } => pp_rbe(value).append(RcDoc::text("+")),
        Rbe::Repeat { value, card } => pp_rbe(value).append(RcDoc::text(format!("{card}"))),
    }
}
