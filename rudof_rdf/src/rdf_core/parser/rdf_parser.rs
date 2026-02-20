use crate::rdf_core::{
    FocusRDF, RDFError,
    parser::rdf_node_parser::{
        RDFNodeParse,
        constructors::{
            HasTypeParser, InstancesParser, ListParser, SatisfyParser, SingleInstanceParser, SingleValuePropertyParser,
            TypeParser, ValuesPropertyParser,
        },
    },
};
use iri_s::IriS;
use prefixmap::PrefixMap;
use std::collections::HashSet;

/// Execution context for RDF parsing operations.
///
/// Holds the RDF graph and provides methods to execute parsers against it.
/// Automatically handles focus management and provides conveniences for
/// common RDF access patterns (properties, lists, type checking).
pub struct RDFParse<RDF>
where
    RDF: FocusRDF,
{
    /// The underlying RDF graph
    rdf: RDF,

    /// Optional: track original focus for restoration (if needed)
    initial_focus: Option<RDF::Term>,
}

impl<RDF> RDFParse<RDF>
where
    RDF: FocusRDF,
{
    /// Creates a new parsing context wrapping the RDF graph.
    pub fn new(rdf: RDF) -> Self {
        Self {
            rdf,
            initial_focus: None,
        }
    }

    /// Creates a context and immediately sets the focus to the given IRI.
    pub fn with_focus(rdf: RDF, focus_iri: &IriS) -> Self {
        let mut ctx = Self::new(rdf);
        ctx.set_focus_iri(focus_iri);
        ctx
    }

    // ============================================================================
    // Basic accessors
    // ============================================================================

    /// Returns the prefix map of the underlying graph.
    pub fn prefixmap(&self) -> Option<PrefixMap> {
        self.rdf.prefixmap()
    }

    /// Gets the current focus node, if set.
    pub fn current_focus(&self) -> Option<&RDF::Term> {
        self.rdf.get_focus()
    }

    /// Sets the focus node to a specific term.
    pub fn set_focus(&mut self, focus: &RDF::Term) {
        // Store initial focus on first mutation for potential restoration
        if self.initial_focus.is_none() {
            self.initial_focus = self.rdf.get_focus().cloned();
        }
        self.rdf.set_focus(focus);
    }

    /// Sets the focus node from an IRI.
    pub fn set_focus_iri(&mut self, iri: &IriS) {
        let term: RDF::Term = iri.clone().into();
        self.set_focus(&term);
    }

    /// Restores the focus to the initial focus when the context was created.
    pub fn restore_focus(&mut self) {
        if let Some(ref initial) = self.initial_focus {
            self.rdf.set_focus(initial);
        }
    }

    /// Returns a reference to the underlying RDF graph.
    pub fn rdf(&self) -> &RDF {
        &self.rdf
    }

    pub fn rdf_mut(&mut self) -> &mut RDF {
        &mut self.rdf
    }

    // ============================================================================
    // Generic execution (core)
    // ============================================================================

    /// Executes any parser against the current context.
    ///
    /// This is the universal entry point - accepts any parser implementing
    /// `RDFNodeParse`, from simple property extractors to complex compositions.
    ///
    /// # Type Parameters
    /// * `P` - The parser type
    /// * `T` - The output type of the parser
    pub fn run<P, T>(&mut self, parser: P) -> Result<T, RDFError>
    where
        P: RDFNodeParse<RDF, Output = T>,
    {
        parser.parse_focused(&mut self.rdf)
    }

    /// Executes a parser starting from a specific node (ignores current focus).
    ///
    /// Temporarily sets focus to `start_node`, executes the parser, then
    /// restores the previous focus.
    pub fn run_from<P, T>(&mut self, start_node: &IriS, parser: P) -> Result<T, RDFError>
    where
        P: RDFNodeParse<RDF, Output = T>,
    {
        let previous = self.rdf.get_focus().cloned();
        let term: RDF::Term = start_node.clone().into();
        self.rdf.set_focus(&term);

        let result = parser.parse_focused(&mut self.rdf);

        // Restore previous focus
        if let Some(prev) = previous {
            self.rdf.set_focus(&prev);
        }

        result
    }

    // ============================================================================
    // Convenience methods (wrappers around common constructors)
    // ============================================================================

    /// Gets all values of a property from the current focus node.
    ///
    /// Equivalent to `ctx.run(ValuesPropertyParser::new(pred))`.
    pub fn get_property_values(&mut self, pred: IriS) -> Result<HashSet<RDF::Term>, RDFError> {
        ValuesPropertyParser::new(pred).parse_focused(&mut self.rdf)
    }

    /// Gets a single value of a property.
    pub fn get_property(&mut self, pred: IriS) -> Result<RDF::Term, RDFError> {
        SingleValuePropertyParser::new(pred).parse_focused(&mut self.rdf)
    }

    /// Gets the `rdf:type` of the current focus node.
    pub fn get_type(&mut self) -> Result<RDF::Term, RDFError> {
        TypeParser::new().parse_focused(&mut self.rdf)
    }

    /// Checks if the current focus has the given type.
    pub fn has_type(&mut self, expected: IriS) -> Result<(), RDFError> {
        HasTypeParser::new(expected).parse_focused(&mut self.rdf)
    }

    /// Parses an RDF list starting at the current focus.
    pub fn parse_list(&mut self) -> Result<Vec<RDF::Term>, RDFError>
    where
        RDF: FocusRDF,
    {
        ListParser::new().parse_focused(&mut self.rdf)
    }

    /// Parses an RDF list pointed to by a property.
    pub fn get_list_property(&mut self, pred: IriS) -> Result<Vec<RDF::Term>, RDFError>
    where
        RDF: FocusRDF,
    {
        let head = self.get_property(pred)?;
        self.rdf.set_focus(&head);
        ListParser::new().parse_focused(&mut self.rdf)
    }

    // ============================================================================
    // Graph-wide queries
    // ============================================================================

    /// Finds all instances of a given type in the entire graph.
    ///
    /// This is a graph-wide query, not focus-dependent.
    pub fn find_instances_of(&mut self, type_iri: IriS) -> Result<Vec<RDF::Subject>, RDFError>
    where
        RDF: FocusRDF,
    {
        // Store current focus to restore later
        let saved_focus = self.rdf.get_focus().cloned();

        // Execute type-based finder
        let result = InstancesParser::new(type_iri).parse_focused(&mut self.rdf);

        // Restore focus
        if let Some(focus) = saved_focus {
            self.rdf.set_focus(&focus);
        }

        result
    }

    /// Finds exactly one instance of a type (fails if not exactly one).
    pub fn find_single_instance(&mut self, type_iri: IriS) -> Result<RDF::Subject, RDFError>
    where
        RDF: FocusRDF,
    {
        let saved_focus = self.rdf.get_focus().cloned();
        let result = SingleInstanceParser::new(type_iri).parse_focused(&mut self.rdf);

        if let Some(focus) = saved_focus {
            self.rdf.set_focus(&focus);
        }

        result
    }

    /// Validates current focus against a predicate.
    pub fn check<F>(&mut self, predicate: F, name: &str) -> Result<(), RDFError>
    where
        F: Fn(&RDF::Term) -> bool,
    {
        SatisfyParser::new(predicate, name).parse_focused(&mut self.rdf)
    }
}
