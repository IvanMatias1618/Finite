use crate::dominio::puertos::repositorio_servicio::RepositorioServicio;
use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use crate::dominio::puertos::repositorio_usuario::RepositorioUsuario;
use crate::dominio::puertos::repositorio_disponibilidad::RepositorioDisponibilidad;
use crate::dominio::urgencia::Urgencia;
use serde::Serialize;
use std::sync::Arc;
use std::error::Error;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use chrono::{Utc, Datelike, Timelike};

#[derive(Serialize)]
pub struct ColaboradorMarketplace {
    pub colaborador_id: i32,
    pub nombre: String,
    pub descripcion_servicio: String,
    pub precio_base: Decimal,
    pub distancia_km: Decimal,
}

pub struct CasoUsoListarColaboradoresMarketplace {
    repo_servicio: Arc<dyn RepositorioServicio>,
    repo_colaborador: Arc<dyn RepositorioColaborador>,
    repo_usuario: Arc<dyn RepositorioUsuario>,
    repo_disponibilidad: Arc<dyn RepositorioDisponibilidad>,
}

impl CasoUsoListarColaboradoresMarketplace {
    pub fn nuevo(
        repo_servicio: Arc<dyn RepositorioServicio>,
        repo_colaborador: Arc<dyn RepositorioColaborador>,
        repo_usuario: Arc<dyn RepositorioUsuario>,
        repo_disponibilidad: Arc<dyn RepositorioDisponibilidad>,
    ) -> Self {
        Self { repo_servicio, repo_colaborador, repo_usuario, repo_disponibilidad }
    }

    pub async fn ejecutar(
        &self,
        subcategoria_id: i32,
        latitud: Decimal,
        longitud: Decimal,
    ) -> Result<Vec<ColaboradorMarketplace>, Box<dyn Error + Send + Sync>> {
        let lat_f = latitud.to_f64().unwrap_or(0.0);
        let lon_f = longitud.to_f64().unwrap_or(0.0);

        let servicios = self.repo_servicio
            .buscar_por_categoria_y_cercania(subcategoria_id, lat_f, lon_f)
            .await?;

        let mut lista = Vec::new();
        let ahora = Utc::now();
        let dia_actual = ahora.weekday().num_days_from_sunday() as i8;
        let hora_actual = format!("{:02}:{:02}", ahora.hour(), ahora.minute());

        for s in servicios {
            // 1. Verificar disponibilidad real
            let horarios = self.repo_disponibilidad.buscar_por_colaborador(s.colaborador_id).await?;
            let esta_disponible = horarios.iter().any(|h| {
                h.dia_semana == dia_actual && 
                h.activo && 
                hora_actual >= h.hora_inicio && 
                hora_actual <= h.hora_fin
            });

            if !esta_disponible && !horarios.is_empty() {
                // Si tiene horarios definidos y ninguno coincide, lo saltamos.
                // Si NO tiene horarios definidos, por ahora lo mostramos (asumimos disponible 24/7 o falta de config)
                continue;
            }

            if let Some(colab) = self.repo_colaborador.buscar_por_id(s.colaborador_id).await? {
                if let Some(user) = self.repo_usuario.buscar_por_id(colab.usuario_id).await? {
                    let precio = self.repo_servicio
                        .buscar_precio_por_servicio_y_urgencia(s.id.unwrap(), Urgencia::Baja)
                        .await?
                        .unwrap_or(Decimal::ZERO);

                    let distancia = self.calcular_distancia_km(
                        lat_f, lon_f,
                        s.latitud.to_f64().unwrap_or(0.0),
                        s.longitud.to_f64().unwrap_or(0.0)
                    );
                    
                    lista.push(ColaboradorMarketplace {
                        colaborador_id: colab.id.unwrap(),
                        nombre: user.nombre,
                        descripcion_servicio: s.descripcion,
                        precio_base: precio,
                        distancia_km: Decimal::from_f64_retain(distancia).unwrap_or(Decimal::ZERO).round_dp(2),
                    });
                }
            }
        }

        // Ordenar por cercanía primero, luego por precio
        lista.sort_by(|a, b| {
            a.distancia_km.partial_cmp(&b.distancia_km).unwrap_or(std::cmp::Ordering::Equal)
                .then(a.precio_base.cmp(&b.precio_base))
        });

        Ok(lista)
    }

    fn calcular_distancia_km(&self, lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let radio_tierra = 6371.0;
        let d_lat = (lat2 - lat1).to_radians();
        let d_lon = (lon2 - lon1).to_radians();
        let a = (d_lat / 2.0).sin().powi(2) + 
                lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        radio_tierra * c
    }
}
