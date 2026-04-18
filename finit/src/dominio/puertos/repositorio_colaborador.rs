use crate::dominio::colaborador::Colaborador;
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioColaborador: Send + Sync {
    async fn guardar(&self, colaborador: Colaborador) -> Result<Colaborador, Box<dyn Error + Send + Sync>>;
    async fn buscar_por_id(&self, id: i32) -> Result<Option<Colaborador>, Box<dyn Error + Send + Sync>>;
}
