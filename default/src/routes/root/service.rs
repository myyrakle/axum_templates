use axum::Json;

use super::dtos::health_response::HealthReponse;

pub struct RootService {}

impl RootService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_health(&self) -> Json<HealthReponse> {
        let server_ok = true;

        Json(HealthReponse { server_ok })
    }
}
