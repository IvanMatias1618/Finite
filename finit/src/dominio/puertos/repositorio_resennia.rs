use crate::dominio::resennia::Resennia;
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioResennia: Send + Sync {
    async fn guardar(&self, resennia: Resennia) -> Result<Resennia, Box<dyn Error + Send + Sync>>;
    async fn buscar_por_solicitud(&self, solicitud_id: i32) -> Result<Option<Resennia>, Box<dyn Error + Send + Sync>>;
}
