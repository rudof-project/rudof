use rdf_config::Mie;
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

    pub fn convert(&mut self, service: ServiceDescription) {
        if let Some(endpoint) = service.endpoint() {
            self.current_mie.add_endpoint(endpoint.as_str());
        }
    }
}
