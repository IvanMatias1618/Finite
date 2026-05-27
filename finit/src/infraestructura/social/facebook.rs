use crate::dominio::puertos::proveedor_social::{ProveedorSocial, DatosUsuarioSocial};
use std::error::Error;
use async_trait::async_trait;
use serde::Deserialize;

pub struct FacebookProvider;

#[derive(Deserialize)]
struct FacebookUserResponse {
    id: String,
    name: String,
    email: String,
}

#[async_trait]
impl ProveedorSocial for FacebookProvider {
    async fn validar_token(&self, token: &str) -> Result<DatosUsuarioSocial, Box<dyn Error + Send + Sync>> {
        let cliente = reqwest::Client::new();
        let url = format!("https://graph.facebook.com/me?fields=id,name,email&access_token={}", token);
        
        let respuesta = cliente.get(url)
            .send()
            .await?;

        if !respuesta.status().is_success() {
            return Err("Token de Facebook invalido".into());
        }

        let datos: FacebookUserResponse = respuesta.json().await?;

        Ok(DatosUsuarioSocial {
            id_social: datos.id,
            nombre: datos.name,
            correo: datos.email,
        })
    }
}
