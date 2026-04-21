use crate::dominio::mensaje::MensajeSolicitud;
use crate::dominio::puertos::repositorio_mensaje::RepositorioMensaje;
use std::sync::Arc;
use std::error::Error;

pub struct CasoUsoGestionarMensajes {
    repo_mensaje: Arc<dyn RepositorioMensaje>,
}

impl CasoUsoGestionarMensajes {
    pub fn nuevo(repo_mensaje: Arc<dyn RepositorioMensaje>) -> Self {
        Self { repo_mensaje }
    }

    pub async fn enviar_mensaje(
        &self,
        solicitud_id: i32,
        emisor_id: i32,
        contenido: String,
    ) -> Result<MensajeSolicitud, Box<dyn Error + Send + Sync>> {
        let mensaje = MensajeSolicitud {
            id: None,
            solicitud_id,
            emisor_id,
            contenido,
            fecha_envio: None,
        };
        self.repo_mensaje.guardar(mensaje).await
    }

    pub async fn listar_mensajes(
        &self,
        solicitud_id: i32,
    ) -> Result<Vec<MensajeSolicitud>, Box<dyn Error + Send + Sync>> {
        self.repo_mensaje.listar_por_solicitud(solicitud_id).await
    }
}
