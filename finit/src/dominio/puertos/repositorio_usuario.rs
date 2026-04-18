use crate::dominio::usuario::Usuario;
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait RepositorioUsuario: Send + Sync {
    async fn guardar(&self, usuario: Usuario) -> Result<Usuario, Box<dyn Error + Send + Sync>>;
    async fn buscar_por_id(&self, id: i32) -> Result<Option<Usuario>, Box<dyn Error + Send + Sync>>;
    async fn buscar_por_correo(&self, correo: &str) -> Result<Option<Usuario>, Box<dyn Error + Send + Sync>>;
}
