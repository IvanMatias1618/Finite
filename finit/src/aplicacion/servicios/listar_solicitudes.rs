use crate::dominio::solicitud::SolicitudServicio;
use crate::dominio::puertos::repositorio_solicitud::RepositorioSolicitud;
use std::sync::Arc;
use std::error::Error;

pub struct CasoUsoListarSolicitudes {
    repo_solicitud: Arc<dyn RepositorioSolicitud>,
}

impl CasoUsoListarSolicitudes {
    pub fn nuevo(repo_solicitud: Arc<dyn RepositorioSolicitud>) -> Self {
        Self { repo_solicitud }
    }

    pub async fn ejecutar(&self, usuario_id: Option<i32>) -> Result<Vec<SolicitudServicio>, Box<dyn Error + Send + Sync>> {
        match usuario_id {
            Some(id) => self.repo_solicitud.listar_por_usuario(id).await,
            None => self.repo_solicitud.listar_todas().await,
        }
    }
}
