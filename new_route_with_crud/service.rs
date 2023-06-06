use super::dtos::health_response::HealthReponse;

pub struct #name#Service {}

impl #name#Service {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_one(&self) -> HealthReponse {
        let server_ok = true;

        HealthReponse { server_ok }
    }
}
