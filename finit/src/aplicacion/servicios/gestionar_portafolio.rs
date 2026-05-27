use crate::dominio::colaborador::TrabajoPortafolio;
use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use std::error::Error;
use std::sync::Arc;

pub struct CasoUsoGestionarPortafolio {
    repositorio_colaborador: Arc<dyn RepositorioColaborador>,
}

impl CasoUsoGestionarPortafolio {
    pub fn nuevo(repositorio_colaborador: Arc<dyn RepositorioColaborador>) -> Self {
        Self { repositorio_colaborador }
    }

    pub async fn annadir_trabajo(
        &self,
        colaborador_id: i32,
        mut trabajo: TrabajoPortafolio,
    ) -> Result<TrabajoPortafolio, Box<dyn Error + Send + Sync>> {
        trabajo.colaborador_id = colaborador_id;
        self.repositorio_colaborador.guardar_trabajo_portafolio(trabajo).await
    }

    pub async fn eliminar_trabajo(
        &self,
        trabajo_id: i32,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.repositorio_colaborador.eliminar_trabajo_portafolio(trabajo_id).await
    }
}
