use crate::dominio::disponibilidad::Disponibilidad;
use crate::dominio::puertos::repositorio_disponibilidad::RepositorioDisponibilidad;
use std::error::Error;
use std::sync::Arc;

pub struct CasoUsoConfigurarHorarios {
    repositorio_disponibilidad: Arc<dyn RepositorioDisponibilidad>,
}

impl CasoUsoConfigurarHorarios {
    pub fn nuevo(repositorio_disponibilidad: Arc<dyn RepositorioDisponibilidad>) -> Self {
        Self { repositorio_disponibilidad }
    }

    pub async fn ejecutar(
        &self,
        colaborador_id: i32,
        horarios: Vec<Disponibilidad>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Limpiar horarios anteriores para este colaborador
        self.repositorio_disponibilidad.eliminar_por_colaborador(colaborador_id).await?;

        // Guardar los nuevos horarios
        for mut disp in horarios {
            disp.colaborador_id = colaborador_id;
            self.repositorio_disponibilidad.guardar_disponibilidad(disp).await?;
        }

        Ok(())
    }
}
