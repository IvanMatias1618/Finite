use crate::dominio::mensaje::MensajeSolicitud;
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioMensaje: Send + Sync {
    async fn guardar(&self, mensaje: MensajeSolicitud) -> Result<MensajeSolicitud, Box<dyn Error + Send + Sync>>;
    async fn listar_por_solicitud(&self, solicitud_id: i32) -> Result<Vec<MensajeSolicitud>, Box<dyn Error + Send + Sync>>;
}
