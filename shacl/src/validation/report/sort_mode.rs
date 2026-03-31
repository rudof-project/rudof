
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub(crate) enum SortModeReport {
    #[default]
    Node,
    Severity,
    Shape,
    Component,
    Source,
    Path,
    Value,
    Details
}