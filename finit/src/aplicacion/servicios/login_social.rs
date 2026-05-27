use crate::dominio::puertos::repositorio_usuario::RepositorioUsuario;
use crate::dominio::puertos::proveedor_social::ProveedorSocial;
use crate::dominio::usuario::Usuario;
use crate::dominio::token::Claims;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

pub struct CasoUsoLoginSocial {
    repositorio_usuario: Arc<dyn RepositorioUsuario>,
    proveedor_google: Arc<dyn ProveedorSocial>,
    proveedor_facebook: Arc<dyn ProveedorSocial>,
    jwt_secret: String,
}

impl CasoUsoLoginSocial {
    pub fn nuevo(
        repositorio_usuario: Arc<dyn RepositorioUsuario>,
        proveedor_google: Arc<dyn ProveedorSocial>,
        proveedor_facebook: Arc<dyn ProveedorSocial>,
        jwt_secret: String,
    ) -> Self {
        Self {
            repositorio_usuario,
            proveedor_google,
            proveedor_facebook,
            jwt_secret,
        }
    }

    pub async fn ejecutar(
        &self,
        proveedor: String,
        token_social: String,
        rol_solicitado: String,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let datos_social = match proveedor.to_lowercase().as_str() {
            "google" => self.proveedor_google.validar_token(&token_social).await?,
            "facebook" => self.proveedor_facebook.validar_token(&token_social).await?,
            _ => return Err("Proveedor social no soportado".into()),
        };

        // Buscar usuario por correo
        let usuario = match self.repositorio_usuario.buscar_por_correo(&datos_social.correo).await? {
            Some(u) => u,
            None => {
                // Registro automático
                let nueva_contrasenna = Uuid::new_v4().to_string(); // Contraseña aleatoria
                let hash_contrasenna = bcrypt::hash(nueva_contrasenna, bcrypt::DEFAULT_COST)?;
                
                let nuevo_usuario = Usuario {
                    id: None,
                    nombre: datos_social.nombre,
                    correo: datos_social.correo,
                    contrasenna: hash_contrasenna,
                    rol: rol_solicitado,
                };
                self.repositorio_usuario.guardar(nuevo_usuario).await?
            }
        };

        // Generar JWT
        let expiracion = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("Error al calcular expiracion")
            .timestamp();

        let claims = Claims {
            sub: usuario.id.unwrap().to_string(),
            rol: usuario.rol.clone(),
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
