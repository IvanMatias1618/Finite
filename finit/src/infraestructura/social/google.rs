use crate::dominio::puertos::proveedor_social::{ProveedorSocial, DatosUsuarioSocial};
use std::error::Error;
use async_trait::async_trait;
use serde::Deserialize;

pub struct GoogleProvider;

#[derive(Deserialize)]
struct GoogleUserResponse {
    sub: String,
    name: String,
    email: String,
}

#[async_trait]
impl ProveedorSocial for GoogleProvider {
    async fn validar_token(&self, token: &str) -> Result<DatosUsuarioSocial, Box<dyn Error + Send + Sync>> {
        let cliente = reqwest::Client::new();
        let url = format!("https://www.googleapis.com/oauth2/v3/tokeninfo?id_token={}", token);
        
        let respuesta = cliente.get(url)
            .send()
            .await?;

        if !respuesta.status().is_success() {
            return Err("Token de Google invalido".into());
        }

        let datos: GoogleUserResponse = respuesta.json().await?;

        Ok(DatosUsuarioSocial {
            id_social: datos.sub,
            nombre: datos.name,
            correo: datos.email,
        })
    }
}
