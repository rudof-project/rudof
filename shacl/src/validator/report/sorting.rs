
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ValidationReportSorting {
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