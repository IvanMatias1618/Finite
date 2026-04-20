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

    pub async fn emparejar_y_solicitar(
        &self,
        usuario_id: i32,
        subcategoria_id: i32,
        urgencia: Urgencia,
        latitud: Decimal,
        longitud: Decimal,
    ) -> Result<SolicitudServicio, Box<dyn Error + Send + Sync>> {
        // 1. Obtener servicios potenciales
        let latitud_flotante = latitud.to_f64().ok_or("Latitud invalida")?;
        let longitud_flotante = longitud.to_f64().ok_or("Longitud invalida")?;

        let servicios = self.repositorio_servicio
            .buscar_por_categoria_y_cercania(subcategoria_id, latitud_flotante, longitud_flotante)
            .await?;

        if servicios.is_empty() {
            return Err("No hay servicios disponibles para esta subcategoria en su zona".into());
        }

        // 2. Elegir el mejor (Matching) y calcular precio
        let mut mejor_opcion: Option<(Servicio, Decimal)> = None;

        for servicio in servicios {
            let precio_base_urgencia = match self.repositorio_servicio
                .buscar_precio_por_servicio_y_urgencia(servicio.id.unwrap(), urgencia)
                .await? {
                Some(p) => p,
                None => continue, // Si no tiene precio para esta urgencia, lo saltamos
            };

            // Distancia para el cálculo del precio
            let distancia = self.calcular_distancia_km(
                latitud_flotante, 
                longitud_flotante, 
                servicio.latitud.to_f64().unwrap_or(0.0), 
                servicio.longitud.to_f64().unwrap_or(0.0)
            );
            
            let precio_distancia = Decimal::from_f64_retain(distancia).unwrap_or(Decimal::ZERO) * servicio.precio_por_kilometro;
            let precio_final = precio_base_urgencia + precio_distancia;

            if mejor_opcion.is_none() || precio_final < mejor_opcion.as_ref().unwrap().1 {
                mejor_opcion = Some((servicio, precio_final));
            }
        }

        let (servicio_elegido, precio_final) = mejor_opcion.ok_or("No se encontro un servicio adecuado para la urgencia solicitada")?;

        // 4. Crear solicitud en estado de retención (EnEsperaDePago)
        let solicitud = SolicitudServicio {
            id: None,
            usuario_id,
            servicio_id: servicio_elegido.id.unwrap(),
            urgencia,
            precio_final,
            estado: EstadoSolicitud::EnEsperaDePago,
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
