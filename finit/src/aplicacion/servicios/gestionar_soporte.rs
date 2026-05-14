use crate::dominio::puertos::repositorio_soporte::RepositorioSoporte;
use std::error::Error;
use std::sync::Arc;

pub struct CasoUsoGestionarSoporte {
    repo_soporte: Arc<dyn RepositorioSoporte>,
}

impl CasoUsoGestionarSoporte {
    pub fn nuevo(repo_soporte: Arc<dyn RepositorioSoporte>) -> Self {
        Self { repo_soporte }
    }

    pub async fn crear_reporte(
        &self,
        usuario_id: i32,
        descripcion: String,
        fotos: Option<String>,
    ) -> Result<i32, Box<dyn Error + Send + Sync>> {
        self.repo_soporte.guardar_reporte(usuario_id, descripcion, fotos).await
    }
}
