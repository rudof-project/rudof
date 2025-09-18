use mie::Mie;
use sparql_service::ServiceDescription;

use crate::service_to_mie::Service2MieConfig;

#[derive(Clone, Debug)]
#[allow(dead_code)] // This is while we don't use config
pub struct Service2Mie {
    config: Service2MieConfig,
    current_mie: Mie,
}

impl Service2Mie {
    pub fn new(config: &Service2MieConfig) -> Self {
        Service2Mie {
            config: config.clone(),
            current_mie: Mie::default(),
        }
    }

    pub fn convert(&mut self, service: &ServiceDescription) -> Mie {
        let mie = service.service2mie();
        mie
    }
}
