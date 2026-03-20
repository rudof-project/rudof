use shacl_ast::target::Target;

// impl CompiledTarget {
//     pub fn compile<S: Rdf>(target: Target<S>) -> Result<Self, Box<CompiledShaclError>> {
//         let ans = match target {
//             Target::Node(object) => CompiledTarget::Node(object),
//             Target::Class(object) => CompiledTarget::Class(object),
//             Target::SubjectsOf(iri_ref) => CompiledTarget::SubjectsOf(iri_ref.into()),
//             Target::ObjectsOf(iri_ref) => CompiledTarget::ObjectsOf(iri_ref.into()),
//             Target::ImplicitClass(object) => CompiledTarget::ImplicitClass(object),
//             Target::WrongNode(_) => todo!(),
//             Target::WrongClass(_) => todo!(),
//             Target::WrongSubjectsOf(_) => todo!(),
//             Target::WrongObjectsOf(_) => todo!(),
//             Target::WrongImplicitClass(_) => todo!(),
//         };
//
//         Ok(ans)
//     }
// }
