use crate::dominio::cotizacion_especial::CotizacionEspecial;
use crate::dominio::puertos::repositorio_cotizacion_especial::RepositorioCotizacionEspecial;
use crate::dominio::urgencia::Urgencia;
use std::error::Error;
use std::sync::Arc;
use rust_decimal::Decimal;

pub struct CasoUsoCotizacionEspecial {
    repositorio: Arc<dyn RepositorioCotizacionEspecial>,
}

impl CasoUsoCotizacionEspecial {
    pub fn nuevo(repositorio: Arc<dyn RepositorioCotizacionEspecial>) -> Self {
        Self { repositorio }
    }

    pub async fn ejecutar(
        &self,
        usuario_id: i32,
        descripcion_trabajo: String,
        fotos_evidencia: Option<String>,
        presupuesto_estimado: Option<Decimal>,
        nivel_urgencia: Urgencia,
    ) -> Result<CotizacionEspecial, Box<dyn Error + Send + Sync>> {
        let cotizacion = CotizacionEspecial {
            id: None,
            usuario_id,
            descripcion_trabajo,
            fotos_evidencia,
            presupuesto_estimado,
            nivel_urgencia,
            fecha_creacion: None,
        };

        self.repositorio.guardar(cotizacion).await
    }
}
