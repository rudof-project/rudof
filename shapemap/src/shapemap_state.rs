#[derive(PartialEq, Eq, Debug)]
pub enum ShapeMapState {
    Conforms,
    Fails,
    Pending,
    Unknown,
    Inconsistent,
}
