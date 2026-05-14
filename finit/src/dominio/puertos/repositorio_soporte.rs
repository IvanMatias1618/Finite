use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioSoporte: Send + Sync {
    async fn guardar_reporte(&self, usuario_id: i32, descripcion: String, fotos: Option<String>) -> Result<i32, Box<dyn Error + Send + Sync>>;
}
