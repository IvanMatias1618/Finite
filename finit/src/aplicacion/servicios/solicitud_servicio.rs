use crate::dominio::solicitud::{SolicitudServicio, EstadoSolicitud};
use crate::dominio::urgencia::Urgencia;
use crate::dominio::puertos::repositorio_solicitud::RepositorioSolicitud;
use crate::dominio::puertos::repositorio_servicio::RepositorioServicio;
use crate::dominio::servicio::Servicio;
use std::error::Error;
use std::sync::Arc;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

pub struct CasoUsoSolicitudServicio {
    repositorio_solicitud: Arc<dyn RepositorioSolicitud>,
    repositorio_servicio: Arc<dyn RepositorioServicio>,
}

impl CasoUsoSolicitudServicio {
    pub fn nuevo(
        repositorio_solicitud: Arc<dyn RepositorioSolicitud>,
        repositorio_servicio: Arc<dyn RepositorioServicio>,
    ) -> Self {
        Self {
            repositorio_solicitud,
            repositorio_servicio,
        }
    }

    pub async fn crear_solicitud_directa(
        &self,
        usuario_id: i32,
        colaborador_id: i32,
        subcategoria_id: i32,
        urgencia: Urgencia,
        descripcion_detallada: String,
        fotos_evidencia_inicial: Option<String>,
        latitud: Decimal,
        longitud: Decimal,
    ) -> Result<SolicitudServicio, Box<dyn Error + Send + Sync>> {
        // 1. Obtener el servicio del colaborador para esa subcategoria
        let servicios = self.repositorio_servicio.buscar_por_colaborador(colaborador_id).await?;
        let servicio = servicios.into_iter()
            .find(|s| s.subcategoria_id == subcategoria_id)
            .ok_or("El colaborador no ofrece este servicio")?;

        // 2. Calcular precio (mismo algoritmo que emparejar)
        let precio_base = self.repositorio_servicio
            .buscar_precio_por_servicio_y_urgencia(servicio.id.unwrap(), urgencia)
            .await?
            .ok_or("No hay precio definido para esta urgencia")?;

        let distancia = self.calcular_distancia_km(
            latitud.to_f64().unwrap_or(0.0),
            longitud.to_f64().unwrap_or(0.0),
            servicio.latitud.to_f64().unwrap_or(0.0),
            servicio.longitud.to_f64().unwrap_or(0.0)
        );

        let precio_distancia = Decimal::from_f64_retain(distancia).unwrap_or(Decimal::ZERO) * servicio.precio_por_kilometro;
        let precio_final = precio_base + precio_distancia;

        // 3. Crear solicitud
        let solicitud = SolicitudServicio {
            id: None,
            usuario_id,
            colaborador_id,
            subcategoria_id,
            servicio_id: servicio.id.unwrap(),
            urgencia,
            precio_final,
            estado: EstadoSolicitud::PendienteDeRevision,
            descripcion_detallada,
            fotos_evidencia_inicial,
            latitud_usuario: Some(latitud),
            longitud_usuario: Some(longitud),
            fecha_creacion: None,
        };

        self.repositorio_solicitud.crear(solicitud).await
    }

    fn calcular_distancia_km(&self, latitud_1: f64, longitud_1: f64, latitud_2: f64, longitud_2: f64) -> f64 {
        let radio_tierra_km = 6371.0; // Radio de la Tierra en km
        let d_latitud = (latitud_2 - latitud_1).to_radians();
        let d_longitud = (longitud_2 - longitud_1).to_radians();
        let a = (d_latitud / 2.0).sin().powi(2)
            + latitud_1.to_radians().cos() * latitud_2.to_radians().cos() * (d_longitud / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        radio_tierra_km * c
    }
}
