use crate::dominio::usuario::Usuario;
use crate::dominio::puertos::repositorio_usuario::RepositorioUsuario;
use std::error::Error;
use std::sync::Arc;

pub struct CasoUsoRegistroUsuario {
    repositorio_usuario: Arc<dyn RepositorioUsuario>,
}

impl CasoUsoRegistroUsuario {
    pub fn nuevo(repositorio_usuario: Arc<dyn RepositorioUsuario>) -> Self {
        Self { repositorio_usuario }
    }

    pub async fn ejecutar(
        &self,
        nombre: String,
        correo: String,
        contrasenna: String,
    ) -> Result<i32, Box<dyn Error + Send + Sync>> {
        // Verificar si el correo ya esta registrado
        if self.repositorio_usuario.buscar_por_correo(&correo).await?.is_some() {
            return Err("El correo ya se encuentra registrado".into());
        }

        // Hashear la contrasenna antes de guardar
        let hash = bcrypt::hash(contrasenna, bcrypt::DEFAULT_COST)
            .map_err(|_| "Error al cifrar la contrasenna")?;

        let usuario = Usuario {
            id: None,
            nombre,
            correo,
            contrasenna: hash,
        };

        let usuario_guardado = self.repositorio_usuario.guardar(usuario).await?;
        
        Ok(usuario_guardado.id.unwrap())
    }
}
