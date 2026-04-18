use crate::dominio::usuario::Usuario;
use crate::dominio::colaborador::Colaborador;
use crate::dominio::servicio::{Servicio, PrecioServicioUrgencia};
use crate::dominio::solicitud::{SolicitudServicio, EstadoSolicitud};
use crate::dominio::urgencia::Urgencia;
use crate::dominio::puertos::repositorio_usuario::RepositorioUsuario;
use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use crate::dominio::puertos::repositorio_servicio::RepositorioServicio;
use crate::dominio::puertos::repositorio_solicitud::RepositorioSolicitud;
use std::error::Error;
use async_trait::async_trait;
use sqlx::{SqlitePool, Row, Sqlite};
use rust_decimal::Decimal;

pub struct RepositorioSQLite {
    pub pool: SqlitePool,
}

impl RepositorioSQLite {
    pub fn nuevo(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn inicializar_tablas(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query("CREATE TABLE IF NOT EXISTS usuario (id INTEGER PRIMARY KEY AUTOINCREMENT, nombre TEXT, correo TEXT UNIQUE)")
            .execute(&self.pool).await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS colaborador (id INTEGER PRIMARY KEY AUTOINCREMENT, usuario_id INTEGER, telefono TEXT, sitio_web TEXT)")
            .execute(&self.pool).await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS servicio (id INTEGER PRIMARY KEY AUTOINCREMENT, colaborador_id INTEGER, categoria_id INTEGER, descripcion TEXT, distancia_maxima_kilometros TEXT, precio_por_kilometro TEXT, latitud TEXT, longitud TEXT)")
            .execute(&self.pool).await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS precio_servicio_urgencia (id INTEGER PRIMARY KEY AUTOINCREMENT, servicio_id INTEGER, urgencia TEXT, precio TEXT)")
            .execute(&self.pool).await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS solicitud_servicio (id INTEGER PRIMARY KEY AUTOINCREMENT, usuario_id INTEGER, servicio_id INTEGER, urgencia TEXT, precio_final TEXT, estado TEXT, latitud_usuario TEXT, longitud_usuario TEXT, fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP)")
            .execute(&self.pool).await?;
        Ok(())
    }
}

#[async_trait]
impl RepositorioUsuario for RepositorioSQLite {
    async fn guardar(&self, usuario: Usuario) -> Result<Usuario, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO usuario (nombre, correo) VALUES (?, ?)")
            .bind(&usuario.nombre).bind(&usuario.correo).execute(&self.pool).await?;
        Ok(Usuario { id: Some(resultado.last_insert_rowid() as i32), ..usuario })
    }
    async fn buscar_por_id(&self, id: i32) -> Result<Option<Usuario>, Box<dyn Error + Send + Sync>> {
        Ok(sqlx::query_as::<_, Usuario>("SELECT id, nombre, correo FROM usuario WHERE id = ?").bind(id).fetch_optional(&self.pool).await?)
    }
    async fn buscar_por_correo(&self, correo: &str) -> Result<Option<Usuario>, Box<dyn Error + Send + Sync>> {
        Ok(sqlx::query_as::<_, Usuario>("SELECT id, nombre, correo FROM usuario WHERE correo = ?").bind(correo).fetch_optional(&self.pool).await?)
    }
}

#[async_trait]
impl RepositorioColaborador for RepositorioSQLite {
    async fn guardar(&self, colaborador: Colaborador) -> Result<Colaborador, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO colaborador (usuario_id, telefono, sitio_web) VALUES (?, ?, ?)")
            .bind(colaborador.usuario_id).bind(&colaborador.telefono).bind(&colaborador.sitio_web).execute(&self.pool).await?;
        Ok(Colaborador { id: Some(resultado.last_insert_rowid() as i32), ..colaborador })
    }
    async fn buscar_por_id(&self, id: i32) -> Result<Option<Colaborador>, Box<dyn Error + Send + Sync>> {
        Ok(sqlx::query_as::<_, Colaborador>("SELECT id, usuario_id, telefono, sitio_web FROM colaborador WHERE id = ?").bind(id).fetch_optional(&self.pool).await?)
    }
}

#[async_trait]
impl RepositorioServicio for RepositorioSQLite {
    async fn guardar(&self, servicio: Servicio) -> Result<Servicio, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO servicio (colaborador_id, categoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(servicio.colaborador_id).bind(servicio.categoria_id).bind(&servicio.descripcion)
            .bind(servicio.distancia_maxima_kilometros.to_string()).bind(servicio.precio_por_kilometro.to_string())
            .bind(servicio.latitud.to_string()).bind(servicio.longitud.to_string()).execute(&self.pool).await?;
        Ok(Servicio { id: Some(resultado.last_insert_rowid() as i32), ..servicio })
    }
    async fn guardar_precio_urgencia(&self, precio: PrecioServicioUrgencia) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query("INSERT INTO precio_servicio_urgencia (servicio_id, urgencia, precio) VALUES (?, ?, ?)")
            .bind(precio.servicio_id).bind(precio.urgencia.a_cadena()).bind(precio.precio.to_string()).execute(&self.pool).await?;
        Ok(())
    }
    async fn buscar_por_id(&self, id: i32) -> Result<Option<Servicio>, Box<dyn Error + Send + Sync>> {
        // Mapeo manual simple para SQLite por tipos Decimal
        let row = sqlx::query("SELECT id, colaborador_id, categoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud FROM servicio WHERE id = ?").bind(id).fetch_optional(&self.pool).await?;
        if let Some(r) = row {
            Ok(Some(Servicio {
                id: Some(r.get(0)), colaborador_id: r.get(1), categoria_id: r.get(2), descripcion: r.get(3),
                distancia_maxima_kilometros: r.get::<String, _>(4).parse().unwrap_or(Decimal::ZERO),
                precio_por_kilometro: r.get::<String, _>(5).parse().unwrap_or(Decimal::ZERO),
                latitud: r.get::<String, _>(6).parse().unwrap_or(Decimal::ZERO),
                longitud: r.get::<String, _>(7).parse().unwrap_or(Decimal::ZERO),
            }))
        } else { Ok(None) }
    }
    async fn buscar_por_categoria_y_cercania(&self, _categoria_id: i32, _latitud: f64, _longitud: f64) -> Result<Vec<Servicio>, Box<dyn Error + Send + Sync>> {
        // Para pruebas locales en SQLite, traemos todos (simulado)
        let rows = sqlx::query("SELECT id, colaborador_id, categoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud FROM servicio").fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|r| Servicio {
            id: Some(r.get(0)), colaborador_id: r.get(1), categoria_id: r.get(2), descripcion: r.get(3),
            distancia_maxima_kilometros: r.get::<String, _>(4).parse().unwrap_or(Decimal::ZERO),
            precio_por_kilometro: r.get::<String, _>(5).parse().unwrap_or(Decimal::ZERO),
            latitud: r.get::<String, _>(6).parse().unwrap_or(Decimal::ZERO),
            longitud: r.get::<String, _>(7).parse().unwrap_or(Decimal::ZERO),
        }).collect())
    }
    async fn buscar_precio_por_servicio_y_urgencia(&self, servicio_id: i32, urgencia: Urgencia) -> Result<Option<Decimal>, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query("SELECT precio FROM precio_servicio_urgencia WHERE servicio_id = ? AND urgencia = ?").bind(servicio_id).bind(urgencia.a_cadena()).fetch_optional(&self.pool).await?;
        Ok(row.map(|r| r.get::<String, _>(0).parse().unwrap_or(Decimal::ZERO)))
    }
}

#[async_trait]
impl RepositorioSolicitud for RepositorioSQLite {
    async fn crear(&self, solicitud: SolicitudServicio) -> Result<SolicitudServicio, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO solicitud_servicio (usuario_id, servicio_id, urgencia, precio_final, estado, latitud_usuario, longitud_usuario) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(solicitud.usuario_id).bind(solicitud.servicio_id).bind(solicitud.urgencia.a_cadena())
            .bind(solicitud.precio_final.to_string()).bind("en_espera_de_pago")
            .bind(solicitud.latitud_usuario.map(|l| l.to_string())).bind(solicitud.longitud_usuario.map(|l| l.to_string()))
            .execute(&self.pool).await?;
        Ok(SolicitudServicio { id: Some(resultado.last_insert_rowid() as i32), ..solicitud })
    }
    async fn buscar_por_id(&self, _id: i32) -> Result<Option<SolicitudServicio>, Box<dyn Error + Send + Sync>> { Ok(None) }
    async fn actualizar_estado(&self, _id: i32, _estado: EstadoSolicitud) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
}
