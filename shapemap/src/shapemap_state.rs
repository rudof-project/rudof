/// Value of an node-shape association, which can be that it conforms,
///  fails, is pending, is unknown or is inconsistent.
#[derive(PartialEq, Eq, Debug)]
pub enum ShapeMapState {
    Conforms,
    Fails,
    Pending,
    Unknown,
    Inconsistent,
}
