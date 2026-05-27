use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use std::error::Error;
use std::sync::Arc;

pub struct CasoUsoActualizarDocumentacion {
    repositorio_colaborador: Arc<dyn RepositorioColaborador>,
}

impl CasoUsoActualizarDocumentacion {
    pub fn nuevo(repositorio_colaborador: Arc<dyn RepositorioColaborador>) -> Self {
        Self { repositorio_colaborador }
    }

    pub async fn ejecutar(
        &self,
        colaborador_id: i32,
        ine_frontal: String,
        ine_trasera: String,
        comprobante_domicilio: String,
        foto_selfie_ine: String,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut colaborador = self.repositorio_colaborador.buscar_por_id(colaborador_id).await?
            .ok_or("Colaborador no encontrado")?;

        colaborador.ine_frontal = Some(ine_frontal);
        colaborador.ine_trasera = Some(ine_trasera);
        colaborador.comprobante_domicilio = Some(comprobante_domicilio);
        colaborador.foto_selfie_ine = Some(foto_selfie_ine);
        // Al actualizar documentos, el estado vuelve a pendiente para revision
        colaborador.estado_verificacion = crate::dominio::colaborador::EstadoVerificacion::Pendiente;

        self.repositorio_colaborador.actualizar(colaborador).await?;
        Ok(())
    }
}
