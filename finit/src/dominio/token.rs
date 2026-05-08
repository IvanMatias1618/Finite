use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // ID del usuario (sujeto)
    pub rol: String, // Rol del usuario
    pub exp: i64,    // Expiracion
}
