use crate::{Rudof, RudofConfig};

pub fn update_config(rudof: &mut Rudof, config: RudofConfig) {
    rudof.config = config;
}
