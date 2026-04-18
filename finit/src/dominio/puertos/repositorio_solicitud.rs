use crate::dominio::solicitud::SolicitudServicio;
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioSolicitud: Send + Sync {
    async fn crear(&self, solicitud: SolicitudServicio) -> Result<SolicitudServicio, Box<dyn Error + Send + Sync>>;
    async fn buscar_por_id(&self, id: i32) -> Result<Option<SolicitudServicio>, Box<dyn Error + Send + Sync>>;
    async fn actualizar_estado(&self, id: i32, estado: crate::dominio::solicitud::EstadoSolicitud) -> Result<(), Box<dyn Error + Send + Sync>>;
}
