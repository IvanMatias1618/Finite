use std::error::Error;
use async_trait::async_trait;

#[derive(Debug)]
pub struct DatosUsuarioSocial {
    pub id_social: String,
    pub nombre: String,
    pub correo: String,
}

#[async_trait]
pub trait ProveedorSocial: Send + Sync {
    async fn validar_token(&self, token: &str) -> Result<DatosUsuarioSocial, Box<dyn Error + Send + Sync>>;
}
