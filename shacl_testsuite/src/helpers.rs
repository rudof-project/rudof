// pub fn get_triple_with_predicate(graph: &SRDFGraph, predicate: &IriS) -> Triple<SRDFGraph> {
//     match graph.triples_with_predicate(predicate.as_named_node()) {
//         Ok(triples) => match triples.into_iter().nth(0) {
//             Some(triple) => triple,
//             None => todo!(),
//         },
//         Err(_) => todo!(),
//     }
// }

// pub fn get_triples_with_predicate(graph: &SRDFGraph, predicate: &IriS) -> Vec<Triple<SRDFGraph>> {
//     match graph.triples_with_predicate(predicate.as_named_node()) {
//         Ok(triples) => triples,
//         Err(_) => todo!(),
//     }
// }

// TODO: use the subject
pub fn object(graph: &SRDFGraph, _subject: &Subject, predicate: &IriS) -> Option<Term> {
    match graph.triples_with_predicate(predicate.as_named_node()) {
        Ok(triples) => match triples.into_iter().nth(0) {
            Some(triple) => Some(triple.obj()),
            None => None,
        },
        Err(_) => None,
    }
}

// TODO: use the subject
pub fn objects(graph: &SRDFGraph, _subject: &Subject, predicate: &IriS) -> Vec<Term> {
    match graph.triples_with_predicate(predicate.as_named_node()) {
        Ok(triples) => triples.iter().map(|triple| triple.obj()).collect(),
        Err(_) => todo!(),
    }
}
