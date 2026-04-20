use crate::dominio::categoria::{Categoria, Subcategoria};
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioCategoria: Send + Sync {
    async fn listar(&self) -> Result<Vec<Categoria>, Box<dyn Error + Send + Sync>>;
    async fn listar_subcategorias(&self, categoria_id: i32) -> Result<Vec<Subcategoria>, Box<dyn Error + Send + Sync>>;
}
