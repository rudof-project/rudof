use iri_s::IriS;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::{RdfVocab, RdfsVocab, ShaclVocab};
use rudof_rdf::rdf_core::BuildRDF;
use std::fmt::{Display, Formatter};

/// Represents target declarations
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Target {
    Node(Object), // TODO - Replace with node expr
    Class(Object),
    SubjectsOf(IriS),
    ObjectsOf(IriS),
    ImplicitClass(Object),

    // The following target declaration are not well-formed, but we keep them
    // to generate violation errors for them
    WrongNode(Object),
    WrongClass(Object),
    WrongSubjectsOf(Object),
    WrongObjectsOf(Object),
    WrongImplicitClass(Object),
}

impl Target {
    pub fn register<RDF: BuildRDF>(&self, id: &Object, graph: &mut RDF) -> Result<(), RDF::Err> {
        let node: RDF::Subject = id.clone().try_into().map_err(|_| unreachable!())?;

        match self {
            Target::Node(n) => graph.add_triple(node, ShaclVocab::sh_target_node().clone(), n.clone()),
            Target::Class(c) => graph.add_triple(node, ShaclVocab::sh_target_class().clone(), c.clone()),
            Target::SubjectsOf(s) => graph.add_triple(node, ShaclVocab::sh_target_subjects_of().clone(), s.clone()),
            Target::ObjectsOf(o) => graph.add_triple(node, ShaclVocab::sh_target_objects_of().clone(), o.clone()),
            // TODO - Review this code and in SHACL 1.2, add sh_shape_class ?
            Target::ImplicitClass(_) => graph.add_triple(node, RdfVocab::rdf_type().clone(), RdfsVocab::rdfs_class().clone()),
            Target::WrongNode(_) => todo!(),
            Target::WrongClass(_) => todo!(),
            Target::WrongSubjectsOf(_) => todo!(),
            Target::WrongObjectsOf(_) => todo!(),
            Target::WrongImplicitClass(_) => todo!(),
        }
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Node(o) => write!(f, "targetNode({o})"),
            Target::Class(o) => write!(f, "targetClass({o})"),
            Target::SubjectsOf(iri) => write!(f, "targetSubjectsOf({iri})"),
            Target::ObjectsOf(iri) => write!(f, "targetObjectsOf({iri})"),
            Target::ImplicitClass(o) => write!(f, "targetImplicitClass({o})"),
            Target::WrongNode(o) => write!(f, "targetNode({o})"),
            Target::WrongClass(o) => write!(f, "targetClass({o})"),
            Target::WrongSubjectsOf(iri) => write!(f, "targetSubjectsOf({iri})"),
            Target::WrongObjectsOf(iri) => write!(f, "targetObjectsOf({iri})"),
            Target::WrongImplicitClass(o) => write!(f, "targetImplicitClass({o})"),
        }
    }
}
