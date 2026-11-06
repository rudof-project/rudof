/// What went through a sink.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SinkReport {
    /// Triples **pushed** into the sink, duplicates included: a streaming sink
    /// cannot tell a duplicate from a new triple.
    pub triples: u64,
}
