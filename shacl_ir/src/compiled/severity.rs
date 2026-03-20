use shacl_ast::severity::Severity;

// impl CompiledSeverity {
//     pub fn compile(severity: Option<Severity>) -> Result<Option<Self>, Box<CompiledShaclError>> {
//         let ans = match severity {
//             Some(severity) => {
//                 let severity = match severity {
//                     Severity::Trace => CompiledSeverity::Trace,
//                     Severity::Debug => CompiledSeverity::Debug,
//                     Severity::Violation => CompiledSeverity::Violation,
//                     Severity::Warning => CompiledSeverity::Warning,
//                     Severity::Info => CompiledSeverity::Info,
//                     Severity::Generic(iri_ref) => {
//                         let iri = iri_ref.get_iri().map_err(|e| CompiledShaclError::IriRefConversion {
//                             iri_ref: iri_ref.to_string(),
//                             err: e.to_string(),
//                         })?;
//                         CompiledSeverity::Generic(iri.clone())
//                     },
//                 };
//                 Some(severity)
//             },
//             None => None,
//         };
//
//         Ok(ans)
//     }
// }
