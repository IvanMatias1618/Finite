use crate::dominio::colaborador::{Colaborador, TrabajoPortafolio, ResumenEstadisticasColaborador, EstadoVerificacion};
use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use crate::infraestructura::RepositorioMySQL;
use std::error::Error;
use async_trait::async_trait;
use sqlx::{MySql, Row};
use rust_decimal::Decimal;

#[async_trait]
impl RepositorioColaborador for RepositorioMySQL {
    async fn guardar(&self, colaborador: Colaborador) -> Result<Colaborador, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query(
            "INSERT INTO colaborador (usuario_id, telefono, sitio_web, foto_perfil, especialidad_resumen, es_verificado, estado_verificacion, ine_frontal, ine_trasera, comprobante_domicilio, foto_selfie_ine, medio_transporte, rating_promedio, total_servicios) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(colaborador.usuario_id)
        .bind(&colaborador.telefono)
        .bind(&colaborador.sitio_web)
        .bind(&colaborador.foto_perfil)
        .bind(&colaborador.especialidad_resumen)
        .bind(colaborador.es_verificado)
        .bind(colaborador.estado_verificacion.a_cadena_sqlite())
        .bind(&colaborador.ine_frontal)
        .bind(&colaborador.ine_trasera)
        .bind(&colaborador.comprobante_domicilio)
        .bind(&colaborador.foto_selfie_ine)
        .bind(&colaborador.medio_transporte)
        .bind(colaborador.rating_promedio)
        .bind(colaborador.total_servicios)
        .execute(&self.pool)
        .await?;

        let id = resultado.last_insert_id() as i32;
        Ok(Colaborador {
            id: Some(id),
            ..colaborador
        })
    }

    async fn actualizar(&self, colaborador: Colaborador) -> Result<Colaborador, Box<dyn Error + Send + Sync>> {
        sqlx::query(
            "UPDATE colaborador SET telefono = ?, sitio_web = ?, foto_perfil = ?, especialidad_resumen = ?, es_verificado = ?, estado_verificacion = ?, ine_frontal = ?, ine_trasera = ?, comprobante_domicilio = ?, foto_selfie_ine = ?, medio_transporte = ?, rating_promedio = ?, total_servicios = ? WHERE id = ?"
        )
        .bind(&colaborador.telefono)
        .bind(&colaborador.sitio_web)
        .bind(&colaborador.foto_perfil)
        .bind(&colaborador.especialidad_resumen)
        .bind(colaborador.es_verificado)
        .bind(colaborador.estado_verificacion.a_cadena_sqlite())
        .bind(&colaborador.ine_frontal)
        .bind(&colaborador.ine_trasera)
        .bind(&colaborador.comprobante_domicilio)
        .bind(&colaborador.foto_selfie_ine)
        .bind(&colaborador.medio_transporte)
        .bind(colaborador.rating_promedio)
        .bind(colaborador.total_servicios)
        .bind(colaborador.id)
        .execute(&self.pool)
        .await?;

        Ok(colaborador)
    }

    async fn buscar_por_id(&self, id: i32) -> Result<Option<Colaborador>, Box<dyn Error + Send + Sync>> {
        let registro = sqlx::query(
            "SELECT id, usuario_id, telefono, sitio_web, foto_perfil, especialidad_resumen, es_verificado, estado_verificacion, ine_frontal, ine_trasera, comprobante_domicilio, foto_selfie_ine, medio_transporte, rating_promedio, total_servicios FROM colaborador WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = registro {
            let estado_str: String = row.get("estado_verificacion");
            Ok(Some(Colaborador {
                id: Some(row.get::<i32, _>("id")),
                usuario_id: row.get::<i32, _>("usuario_id"),
                telefono: row.get::<String, _>("telefono"),
                sitio_web: row.get::<Option<String>, _>("sitio_web"),
                foto_perfil: row.get::<Option<String>, _>("foto_perfil"),
                especialidad_resumen: row.get::<Option<String>, _>("especialidad_resumen"),
                es_verificado: row.get::<i8, _>("es_verificado") != 0,
                estado_verificacion: EstadoVerificacion::desde_cadena_sqlite(&estado_str),
                ine_frontal: row.get::<Option<String>, _>("ine_frontal"),
                ine_trasera: row.get::<Option<String>, _>("ine_trasera"),
                comprobante_domicilio: row.get::<Option<String>, _>("comprobante_domicilio"),
                foto_selfie_ine: row.get::<Option<String>, _>("foto_selfie_ine"),
                medio_transporte: row.get::<Option<String>, _>("medio_transporte"),
                rating_promedio: row.get::<Decimal, _>("rating_promedio"),
                total_servicios: row.get::<i32, _>("total_servicios"),
            }))
        } else {
            Ok(None)
        }
    }

    async fn guardar_trabajo_portafolio(&self, trabajo: TrabajoPortafolio) -> Result<TrabajoPortafolio, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query(
            "INSERT INTO portafolio_colaborador (colaborador_id, foto_antes, foto_despues, descripcion) VALUES (?, ?, ?, ?)"
        )
        .bind(trabajo.colaborador_id)
        .bind(&trabajo.foto_antes)
        .bind(&trabajo.foto_despues)
        .bind(&trabajo.descripcion)
        .execute(&self.pool)
        .await?;

        let id = resultado.last_insert_id() as i32;
        Ok(TrabajoPortafolio {
            id: Some(id),
            ..trabajo
        })
    }

    async fn buscar_portafolio_por_colaborador(&self, colaborador_id: i32) -> Result<Vec<TrabajoPortafolio>, Box<dyn Error + Send + Sync>> {
        let registros = sqlx::query_as::<MySql, TrabajoPortafolio>(
            "SELECT id, colaborador_id, foto_antes, foto_despues, descripcion FROM portafolio_colaborador WHERE colaborador_id = ?"
        )
        .bind(colaborador_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(registros)
    }

    async fn eliminar_trabajo_portafolio(&self, trabajo_id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query("DELETE FROM portafolio_colaborador WHERE id = ?")
            .bind(trabajo_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn obtener_estadisticas(&self, colaborador_id: i32) -> Result<ResumenEstadisticasColaborador, Box<dyn Error + Send + Sync>> {
        // 1. Obtener datos basicos del colaborador
        let datos_base = sqlx::query(
            "SELECT total_servicios, rating_promedio FROM colaborador WHERE id = ?"
        )
        .bind(colaborador_id)
        .fetch_one(&self.pool)
        .await?;

        let total_servicios: i32 = datos_base.get("total_servicios");
        let rating_promedio: Decimal = datos_base.get("rating_promedio");

        // 2. Obtener ganancias totales
        let ganancias = sqlx::query(
            "SELECT SUM(precio_final) as total FROM solicitud_servicio WHERE colaborador_id = ? AND estado = 'terminado'"
        )
        .bind(colaborador_id)
        .fetch_one(&self.pool)
        .await?;

        let ganancias_totales: Decimal = ganancias.try_get("total").unwrap_or(Decimal::ZERO);

        // 3. Obtener servicios pendientes
        let pendientes = sqlx::query(
            "SELECT COUNT(*) as cuenta FROM solicitud_servicio WHERE colaborador_id = ? AND estado = 'pendiente'"
        )
        .bind(colaborador_id)
        .fetch_one(&self.pool)
        .await?;

        let servicios_pendientes: i64 = pendientes.get("cuenta");

        Ok(ResumenEstadisticasColaborador {
            total_servicios,
            rating_promedio,
            ganancias_totales,
            servicios_pendientes: servicios_pendientes as i32,
        })
    }

    async fn listar_todos(&self) -> Result<Vec<Colaborador>, Box<dyn Error + Send + Sync>> {
        let registros = sqlx::query(
            "SELECT id, usuario_id, telefono, sitio_web, foto_perfil, especialidad_resumen, es_verificado, estado_verificacion, ine_frontal, ine_trasera, comprobante_domicilio, foto_selfie_ine, medio_transporte, rating_promedio, total_servicios FROM colaborador"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut colaboradores = Vec::new();
        for row in registros {
            let estado_str: String = row.get("estado_verificacion");
            colaboradores.push(Colaborador {
                id: Some(row.get::<i32, _>("id")),
                usuario_id: row.get::<i32, _>("usuario_id"),
                telefono: row.get::<String, _>("telefono"),
                sitio_web: row.get::<Option<String>, _>("sitio_web"),
                foto_perfil: row.get::<Option<String>, _>("foto_perfil"),
                especialidad_resumen: row.get::<Option<String>, _>("especialidad_resumen"),
                es_verificado: row.get::<i8, _>("es_verificado") != 0,
                estado_verificacion: EstadoVerificacion::desde_cadena_sqlite(&estado_str),
                ine_frontal: row.get::<Option<String>, _>("ine_frontal"),
                ine_trasera: row.get::<Option<String>, _>("ine_trasera"),
                comprobante_domicilio: row.get::<Option<String>, _>("comprobante_domicilio"),
                foto_selfie_ine: row.get::<Option<String>, _>("foto_selfie_ine"),
                medio_transporte: row.get::<Option<String>, _>("medio_transporte"),
                rating_promedio: row.get::<Decimal, _>("rating_promedio"),
                total_servicios: row.get::<i32, _>("total_servicios"),
            });
        }
        Ok(colaboradores)
    }
}
