use crate::dominio::categoria::Categoria;
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioCategoria: Send + Sync {
    async fn listar(&self) -> Result<Vec<Categoria>, Box<dyn Error + Send + Sync>>;
}
