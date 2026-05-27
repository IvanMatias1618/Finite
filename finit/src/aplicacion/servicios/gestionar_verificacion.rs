use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use crate::dominio::colaborador::EstadoVerificacion;
use std::error::Error;
use std::sync::Arc;

pub struct CasoUsoGestionarVerificacion {
    repo_colaborador: Arc<dyn RepositorioColaborador>,
}

impl CasoUsoGestionarVerificacion {
    pub fn nuevo(repo_colaborador: Arc<dyn RepositorioColaborador>) -> Self {
        Self { repo_colaborador }
    }

    pub async fn listar_pendientes(&self) -> Result<Vec<crate::dominio::colaborador::Colaborador>, Box<dyn Error + Send + Sync>> {
        // En un entorno ideal, el repo tendria un buscar_por_estado. 
        // Por ahora, listamos todos y filtramos, o podriamos añadir el metodo al repo.
        // Vamos a asumir que podemos listar todos por ahora.
        let todos = self.repo_colaborador.listar_todos().await?;
        Ok(todos.into_iter().filter(|c| c.estado_verificacion == EstadoVerificacion::Pendiente).collect())
    }

    pub async fn procesar_verificacion(
        &self,
        colaborador_id: i32,
        nuevo_estado: EstadoVerificacion,
        _comentario: Option<String>, // Para feedback al colaborador
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut colaborador = self.repo_colaborador.buscar_por_id(colaborador_id).await?
            .ok_or("Colaborador no encontrado")?;

        colaborador.estado_verificacion = nuevo_estado;
        colaborador.es_verificado = nuevo_estado == EstadoVerificacion::Verificado;

        self.repo_colaborador.actualizar(colaborador).await?;
        Ok(())
    }
}
