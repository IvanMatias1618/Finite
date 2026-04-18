use crate::dominio::servicio::{Servicio, PrecioServicioUrgencia};
use crate::dominio::urgencia::Urgencia;
use crate::dominio::puertos::repositorio_servicio::RepositorioServicio;
use crate::infraestructura::RepositorioMySQL;
use std::error::Error;
use rust_decimal::Decimal;
use async_trait::async_trait;
use sqlx::{Row, MySql};

#[async_trait]
impl RepositorioServicio for RepositorioMySQL {
    async fn guardar(&self, servicio: Servicio) -> Result<Servicio, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query(
            "INSERT INTO servicio (colaborador_id, categoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(servicio.colaborador_id)
        .bind(servicio.categoria_id)
        .bind(&servicio.descripcion)
        .bind(servicio.distancia_maxima_kilometros)
        .bind(servicio.precio_por_kilometro)
        .bind(servicio.latitud)
        .bind(servicio.longitud)
        .execute(&self.pool)
        .await?;

        let id = resultado.last_insert_id() as i32;
        Ok(Servicio {
            id: Some(id),
            ..servicio
        })
    }

    async fn guardar_precio_urgencia(&self, precio: PrecioServicioUrgencia) -> Result<(), Box<dyn Error + Send + Sync>> {
        let urgencia_cadena = precio.urgencia.a_cadena();
        sqlx::query(
            "INSERT INTO precio_servicio_urgencia (servicio_id, urgencia, precio) VALUES (?, ?, ?)"
        )
        .bind(precio.servicio_id)
        .bind(urgencia_cadena)
        .bind(precio.precio)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn buscar_por_id(&self, id: i32) -> Result<Option<Servicio>, Box<dyn Error + Send + Sync>> {
        let registro = sqlx::query_as::<MySql, Servicio>(
            "SELECT id, colaborador_id, categoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud FROM servicio WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(registro)
    }

    async fn buscar_por_categoria_y_cercania(&self, categoria_id: i32, latitud: f64, longitud: f64) -> Result<Vec<Servicio>, Box<dyn Error + Send + Sync>> {
        let registros = sqlx::query_as::<MySql, Servicio>(
            "SELECT id, colaborador_id, categoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud 
             FROM servicio 
             WHERE categoria_id = ? 
             AND (ST_Distance_Sphere(point(longitud, latitud), point(?, ?)) / 1000) <= CAST(distancia_maxima_kilometros AS DOUBLE)"
        )
        .bind(categoria_id)
        .bind(longitud)
        .bind(latitud)
        .fetch_all(&self.pool)
        .await?;

        Ok(registros)
    }

    async fn buscar_precio_por_servicio_y_urgencia(&self, servicio_id: i32, urgencia: Urgencia) -> Result<Option<Decimal>, Box<dyn Error + Send + Sync>> {
        let urgencia_cadena = urgencia.a_cadena();
        let registro = sqlx::query(
            "SELECT precio FROM precio_servicio_urgencia WHERE servicio_id = ? AND urgencia = ?"
        )
        .bind(servicio_id)
        .bind(urgencia_cadena)
        .fetch_optional(&self.pool)
        .await?;

        Ok(registro.map(|r| r.get("precio")))
    }
}
