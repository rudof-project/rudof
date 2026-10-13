use crate::{Rudof, api::data::staging::journal_snapshot};

pub fn reset_data(rudof: &mut Rudof) {
    journal_snapshot(rudof);
    rudof.data = None;
}
