use csv::Reader;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]
pub struct TapReader {
    reader: csv::Reader,
}

impl TapReader {
    fn from_reader(reader: csv::Reader) -> TapReader {
        TapReader { reader }
    }
}
