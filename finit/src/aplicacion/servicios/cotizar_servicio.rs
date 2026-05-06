use crate::dominio::puertos::repositorio_servicio::RepositorioServicio;
use crate::dominio::puertos::repositorio_configuracion_precio::RepositorioConfiguracionPrecio;
use crate::dominio::urgencia::Urgencia;
use std::sync::Arc;
use std::error::Error;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use chrono::{Utc, Datelike, Timelike};

pub struct CasoUsoCotizarServicio {
    repo_servicio: Arc<dyn RepositorioServicio>,
    repo_config: Arc<dyn RepositorioConfiguracionPrecio>,
}

use serde::Serialize;

#[derive(Serialize)]
pub struct Cotizacion {
    pub precio_base: Decimal,
    pub precio_distancia: Decimal,
    pub recargo_nocturno: Decimal,
    pub recargo_domingo: Decimal,
    pub total: Decimal,
}

impl CasoUsoCotizarServicio {
    pub fn nuevo(
        repo_servicio: Arc<dyn RepositorioServicio>,
        repo_config: Arc<dyn RepositorioConfiguracionPrecio>,
    ) -> Self {
        Self { repo_servicio, repo_config }
    }

    pub async fn ejecutar(
        &self,
        colaborador_id: i32,
        subcategoria_id: i32,
        urgencia: Urgencia,
        latitud_usuario: Decimal,
        longitud_usuario: Decimal,
    ) -> Result<Cotizacion, Box<dyn Error + Send + Sync>> {
        // 1. Buscar el servicio
        let servicios = self.repo_servicio.buscar_por_colaborador(colaborador_id).await?;
        let servicio = servicios.into_iter()
            .find(|s| s.subcategoria_id == subcategoria_id)
            .ok_or("El colaborador no ofrece este servicio")?;

        // 2. Obtener precio base segun urgencia
        let precio_base = self.repo_servicio
            .buscar_precio_por_servicio_y_urgencia(servicio.id.unwrap(), urgencia)
            .await?
            .ok_or("No hay precio definido para esta urgencia")?;

        // 3. Obtener configuracion de precios del colaborador
        let config = self.repo_config.buscar_por_colaborador(colaborador_id).await?
            .ok_or("Configuracion de precios no encontrada para el colaborador")?;

        // 4. Calcular distancia y precio por km
        let distancia = self.calcular_distancia_km(
            latitud_usuario.to_f64().unwrap_or(0.0),
            longitud_usuario.to_f64().unwrap_or(0.0),
            servicio.latitud.to_f64().unwrap_or(0.0),
            servicio.longitud.to_f64().unwrap_or(0.0)
        );

        let precio_distancia = Decimal::from_f64_retain(distancia).unwrap_or(Decimal::ZERO) * config.precio_por_kilometro;

        // 5. Calcular recargos (Nocturno y Domingo)
        let ahora = Utc::now(); // Deberia ser la hora local del servicio, pero usamos UTC por ahora
        let mut recargo_nocturno = Decimal::ZERO;
        let mut recargo_domingo = Decimal::ZERO;

        // Noche: 20:00 a 06:00
        if ahora.hour() >= 20 || ahora.hour() < 6 {
            recargo_nocturno = config.recargo_nocturno;
        }

        // Domingo: weekday 0 es lunes en chrono? No, 0 es lunes, 6 es domingo.
        // Wait, Chrono Datelike::weekday() returns Weekday enum.
        if ahora.weekday().number_from_monday() == 7 { // Domingo
            recargo_domingo = config.recargo_domingo;
        }

        let total = precio_base + precio_distancia + recargo_nocturno + recargo_domingo;

        Ok(Cotizacion {
            precio_base,
            precio_distancia,
            recargo_nocturno,
            recargo_domingo,
            total,
        })
    }

    fn calcular_distancia_km(&self, latitud_1: f64, longitud_1: f64, latitud_2: f64, longitud_2: f64) -> f64 {
        let radio_tierra_km = 6371.0;
        let d_latitud = (latitud_2 - latitud_1).to_radians();
        let d_longitud = (longitud_2 - longitud_1).to_radians();
        let a = (d_latitud / 2.0).sin().powi(2)
            + latitud_1.to_radians().cos() * latitud_2.to_radians().cos() * (d_longitud / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        radio_tierra_km * c
    }
}
