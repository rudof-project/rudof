use prefixmap::{IriRef, PrefixMap, PrefixMapError};

use crate::Var;

pub struct TriplePattern {
    subj: Var,
    pred: IriRef,
    obj: Var,
}

impl TriplePattern {
    pub fn new(subj: &Var, pred: &IriRef, obj: &Var) -> TriplePattern {
        TriplePattern {
            subj: subj.clone(),
            pred: pred.clone(),
            obj: obj.clone(),
        }
    }

    pub fn show_qualified(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
        prefixmap: &PrefixMap,
    ) -> Result<(), PrefixMapError> {
        let pred_str = match &self.pred {
            IriRef::Iri(iri) => prefixmap.qualify(iri),
            IriRef::Prefixed { prefix, local } => {
                let iri = prefixmap.resolve_prefix_local(prefix, local)?;
                iri.to_string()
            }
        };
        write!(formatter, "{} {} {} .", self.subj, pred_str, self.obj)?;
        Ok(())
    }
}
