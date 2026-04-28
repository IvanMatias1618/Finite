use crate::dominio::puertos::repositorio_usuario::RepositorioUsuario;
use crate::dominio::token::Claims;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use std::error::Error;
use std::sync::Arc;

pub struct CasoUsoLoginUsuario {
    repositorio_usuario: Arc<dyn RepositorioUsuario>,
    jwt_secret: String,
}

impl CasoUsoLoginUsuario {
    pub fn nuevo(
        repositorio_usuario: Arc<dyn RepositorioUsuario>,
        jwt_secret: String,
    ) -> Self {
        Self { 
            repositorio_usuario,
            jwt_secret,
        }
    }

    pub async fn ejecutar(
        &self,
        correo: String,
        contrasenna: String,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        // Buscar usuario por correo
        let usuario = self.repositorio_usuario.buscar_por_correo(&correo).await?
            .ok_or("Credenciales invalidas")?;

        // Validar contrasenna comparando el hash
        if !bcrypt::verify(&contrasenna, &usuario.contrasenna)? {
            return Err("Credenciales invalidas".into());
        }

        // Generar JWT
        let expiracion = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("Error al calcular expiracion")
            .timestamp();

        let claims = Claims {
            sub: usuario.id.unwrap().to_string(),
            exp: expiracion,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;
        
        Ok(token)
    }
}
