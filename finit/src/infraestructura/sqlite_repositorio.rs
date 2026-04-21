use crate::dominio::usuario::Usuario;
use crate::dominio::colaborador::{Colaborador, TrabajoPortafolio};
use crate::dominio::servicio::{Servicio, PrecioServicioUrgencia};
use crate::dominio::solicitud::{SolicitudServicio, EstadoSolicitud};
use crate::dominio::urgencia::Urgencia;
use crate::dominio::categoria::{Categoria, Subcategoria};
use crate::dominio::puertos::repositorio_categoria::RepositorioCategoria;
use crate::dominio::puertos::repositorio_usuario::RepositorioUsuario;
use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use crate::dominio::puertos::repositorio_servicio::RepositorioServicio;
use crate::dominio::puertos::repositorio_solicitud::RepositorioSolicitud;
use std::error::Error;
use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use rust_decimal::Decimal;

pub struct RepositorioSQLite {
    pub pool: SqlitePool,
}

impl RepositorioSQLite {
    pub fn nuevo(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn inicializar_tablas(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query("CREATE TABLE IF NOT EXISTS usuario (id INTEGER PRIMARY KEY AUTOINCREMENT, nombre TEXT, correo TEXT UNIQUE, contrasenna TEXT)")
            .execute(&self.pool).await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS colaborador (id INTEGER PRIMARY KEY AUTOINCREMENT, usuario_id INTEGER, telefono TEXT, sitio_web TEXT, foto_perfil TEXT, especialidad_resumen TEXT, es_verificado INTEGER DEFAULT 0, medio_transporte TEXT, rating_promedio TEXT DEFAULT '0.0', total_servicios INTEGER DEFAULT 0)")
            .execute(&self.pool).await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS portafolio_colaborador (id INTEGER PRIMARY KEY AUTOINCREMENT, colaborador_id INTEGER, foto_antes TEXT, foto_despues TEXT, descripcion TEXT, FOREIGN KEY (colaborador_id) REFERENCES colaborador(id))")
            .execute(&self.pool).await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS categoria (id INTEGER PRIMARY KEY AUTOINCREMENT, nombre TEXT UNIQUE)")
            .execute(&self.pool).await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS subcategoria (id INTEGER PRIMARY KEY AUTOINCREMENT, categoria_id INTEGER, nombre TEXT, descripcion TEXT, FOREIGN KEY (categoria_id) REFERENCES categoria(id))")
            .execute(&self.pool).await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS servicio (id INTEGER PRIMARY KEY AUTOINCREMENT, colaborador_id INTEGER, subcategoria_id INTEGER, descripcion TEXT, distancia_maxima_kilometros TEXT, precio_por_kilometro TEXT, latitud TEXT, longitud TEXT, FOREIGN KEY (colaborador_id) REFERENCES colaborador(id), FOREIGN KEY (subcategoria_id) REFERENCES subcategoria(id))")
            .execute(&self.pool).await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS precio_servicio_urgencia (id INTEGER PRIMARY KEY AUTOINCREMENT, servicio_id INTEGER, urgencia TEXT, precio TEXT)")
            .execute(&self.pool).await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS solicitud_servicio (id INTEGER PRIMARY KEY AUTOINCREMENT, usuario_id INTEGER, colaborador_id INTEGER, subcategoria_id INTEGER, servicio_id INTEGER, urgencia TEXT, precio_final TEXT, estado TEXT, descripcion_detallada TEXT, fotos_evidencia_inicial TEXT, latitud_usuario TEXT, longitud_usuario TEXT, fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP)")
            .execute(&self.pool).await?;
        Ok(())
    }
}

#[async_trait]
impl RepositorioCategoria for RepositorioSQLite {
    async fn listar(&self) -> Result<Vec<Categoria>, Box<dyn Error + Send + Sync>> {
        let categorias = sqlx::query_as::<_, Categoria>("SELECT id, nombre FROM categoria")
            .fetch_all(&self.pool).await?;
        Ok(categorias)
    }
    async fn listar_subcategorias(&self, categoria_id: i32) -> Result<Vec<Subcategoria>, Box<dyn Error + Send + Sync>> {
        let subcategorias = sqlx::query_as::<_, Subcategoria>("SELECT id, categoria_id, nombre, descripcion FROM subcategoria WHERE categoria_id = ?")
            .bind(categoria_id)
            .fetch_all(&self.pool).await?;
        Ok(subcategorias)
    }
}

#[async_trait]
impl RepositorioUsuario for RepositorioSQLite {
    async fn guardar(&self, usuario: Usuario) -> Result<Usuario, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO usuario (nombre, correo, contrasenna) VALUES (?, ?, ?)")
            .bind(&usuario.nombre).bind(&usuario.correo).bind(&usuario.contrasenna).execute(&self.pool).await?;
        Ok(Usuario { id: Some(resultado.last_insert_rowid() as i32), ..usuario })
    }
    async fn buscar_por_id(&self, id: i32) -> Result<Option<Usuario>, Box<dyn Error + Send + Sync>> {
        Ok(sqlx::query_as::<_, Usuario>("SELECT id, nombre, correo, contrasenna FROM usuario WHERE id = ?").bind(id).fetch_optional(&self.pool).await?)
    }
    async fn buscar_por_correo(&self, correo: &str) -> Result<Option<Usuario>, Box<dyn Error + Send + Sync>> {
        Ok(sqlx::query_as::<_, Usuario>("SELECT id, nombre, correo, contrasenna FROM usuario WHERE correo = ?").bind(correo).fetch_optional(&self.pool).await?)
    }
}

#[async_trait]
impl RepositorioColaborador for RepositorioSQLite {
    async fn guardar(&self, colaborador: Colaborador) -> Result<Colaborador, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO colaborador (usuario_id, telefono, sitio_web, foto_perfil, especialidad_resumen, es_verificado, medio_transporte, rating_promedio, total_servicios) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(colaborador.usuario_id).bind(&colaborador.telefono).bind(&colaborador.sitio_web)
            .bind(&colaborador.foto_perfil).bind(&colaborador.especialidad_resumen).bind(colaborador.es_verificado)
            .bind(&colaborador.medio_transporte).bind(colaborador.rating_promedio.to_string()).bind(colaborador.total_servicios)
            .execute(&self.pool).await?;
        Ok(Colaborador { id: Some(resultado.last_insert_rowid() as i32), ..colaborador })
    }
    async fn buscar_por_id(&self, id: i32) -> Result<Option<Colaborador>, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query("SELECT id, usuario_id, telefono, sitio_web, foto_perfil, especialidad_resumen, es_verificado, medio_transporte, rating_promedio, total_servicios FROM colaborador WHERE id = ?").bind(id).fetch_optional(&self.pool).await?;
        if let Some(r) = row {
            Ok(Some(Colaborador {
                id: Some(r.get(0)), usuario_id: r.get(1), telefono: r.get(2), sitio_web: r.get(3),
                foto_perfil: r.get(4), especialidad_resumen: r.get(5),
                es_verificado: r.get::<i32, _>(6) != 0,
                medio_transporte: r.get(7),
                rating_promedio: r.get::<String, _>(8).parse().unwrap_or(Decimal::ZERO),
                total_servicios: r.get(9),
            }))
        } else { Ok(None) }
    }
    async fn guardar_trabajo_portafolio(&self, trabajo: TrabajoPortafolio) -> Result<TrabajoPortafolio, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO portafolio_colaborador (colaborador_id, foto_antes, foto_despues, descripcion) VALUES (?, ?, ?, ?)")
            .bind(trabajo.colaborador_id).bind(&trabajo.foto_antes).bind(&trabajo.foto_despues).bind(&trabajo.descripcion)
            .execute(&self.pool).await?;
        Ok(TrabajoPortafolio { id: Some(resultado.last_insert_rowid() as i32), ..trabajo })
    }
    async fn buscar_portafolio_por_colaborador(&self, colaborador_id: i32) -> Result<Vec<TrabajoPortafolio>, Box<dyn Error + Send + Sync>> {
        let trabajos = sqlx::query_as::<_, TrabajoPortafolio>("SELECT id, colaborador_id, foto_antes, foto_despues, descripcion FROM portafolio_colaborador WHERE colaborador_id = ?")
            .bind(colaborador_id).fetch_all(&self.pool).await?;
        Ok(trabajos)
    }
}

#[async_trait]
impl RepositorioServicio for RepositorioSQLite {
    async fn guardar(&self, servicio: Servicio) -> Result<Servicio, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO servicio (colaborador_id, subcategoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(servicio.colaborador_id).bind(servicio.subcategoria_id).bind(&servicio.descripcion)
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
        let row = sqlx::query("SELECT id, colaborador_id, subcategoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud FROM servicio WHERE id = ?").bind(id).fetch_optional(&self.pool).await?;
        if let Some(r) = row {
            Ok(Some(Servicio {
                id: Some(r.get(0)), colaborador_id: r.get(1), subcategoria_id: r.get(2), descripcion: r.get(3),
                distancia_maxima_kilometros: r.get::<String, _>(4).parse().unwrap_or(Decimal::ZERO),
                precio_por_kilometro: r.get::<String, _>(5).parse().unwrap_or(Decimal::ZERO),
                latitud: r.get::<String, _>(6).parse().unwrap_or(Decimal::ZERO),
                longitud: r.get::<String, _>(7).parse().unwrap_or(Decimal::ZERO),
            }))
        } else { Ok(None) }
    }
    async fn buscar_por_colaborador(&self, colaborador_id: i32) -> Result<Vec<Servicio>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query("SELECT id, colaborador_id, subcategoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud FROM servicio WHERE colaborador_id = ?").bind(colaborador_id).fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|r| Servicio {
            id: Some(r.get(0)), colaborador_id: r.get(1), subcategoria_id: r.get(2), descripcion: r.get(3),
            distancia_maxima_kilometros: r.get::<String, _>(4).parse().unwrap_or(Decimal::ZERO),
            precio_por_kilometro: r.get::<String, _>(5).parse().unwrap_or(Decimal::ZERO),
            latitud: r.get::<String, _>(6).parse().unwrap_or(Decimal::ZERO),
            longitud: r.get::<String, _>(7).parse().unwrap_or(Decimal::ZERO),
        }).collect())
    }
    async fn buscar_por_categoria_y_cercania(&self, subcategoria_id: i32, _latitud: f64, _longitud: f64) -> Result<Vec<Servicio>, Box<dyn Error + Send + Sync>> {
        // Para pruebas locales en SQLite, traemos todos los de la subcategoria (simulado)
        let rows = sqlx::query("SELECT id, colaborador_id, subcategoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud FROM servicio WHERE subcategoria_id = ?").bind(subcategoria_id).fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|r| Servicio {
            id: Some(r.get(0)), colaborador_id: r.get(1), subcategoria_id: r.get(2), descripcion: r.get(3),
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
        let resultado = sqlx::query("INSERT INTO solicitud_servicio (usuario_id, colaborador_id, subcategoria_id, servicio_id, urgencia, precio_final, estado, descripcion_detallada, fotos_evidencia_inicial, latitud_usuario, longitud_usuario) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(solicitud.usuario_id).bind(solicitud.colaborador_id).bind(solicitud.subcategoria_id)
            .bind(solicitud.servicio_id).bind(solicitud.urgencia.a_cadena())
            .bind(solicitud.precio_final.to_string()).bind("pendiente_de_revision")
            .bind(&solicitud.descripcion_detallada).bind(&solicitud.fotos_evidencia_inicial)
            .bind(solicitud.latitud_usuario.map(|l| l.to_string())).bind(solicitud.longitud_usuario.map(|l| l.to_string()))
            .execute(&self.pool).await?;
        Ok(SolicitudServicio { id: Some(resultado.last_insert_rowid() as i32), ..solicitud })
    }
    async fn buscar_por_id(&self, _id: i32) -> Result<Option<SolicitudServicio>, Box<dyn Error + Send + Sync>> { Ok(None) }
    
    async fn listar_por_usuario(&self, usuario_id: i32) -> Result<Vec<SolicitudServicio>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query("SELECT id, usuario_id, colaborador_id, subcategoria_id, servicio_id, urgencia, precio_final, estado, descripcion_detallada, fotos_evidencia_inicial, latitud_usuario, longitud_usuario, fecha_creacion FROM solicitud_servicio WHERE usuario_id = ?")
            .bind(usuario_id).fetch_all(&self.pool).await?;
        
        let mut solicitudes = Vec::new();
        for r in rows {
            solicitudes.push(self.mapear_solicitud(r)?);
        }
        Ok(solicitudes)
    }

    async fn listar_todas(&self) -> Result<Vec<SolicitudServicio>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query("SELECT id, usuario_id, colaborador_id, subcategoria_id, servicio_id, urgencia, precio_final, estado, descripcion_detallada, fotos_evidencia_inicial, latitud_usuario, longitud_usuario, fecha_creacion FROM solicitud_servicio")
            .fetch_all(&self.pool).await?;
        
        let mut solicitudes = Vec::new();
        for r in rows {
            solicitudes.push(self.mapear_solicitud(r)?);
        }
        Ok(solicitudes)
    }

    async fn actualizar_estado(&self, id: i32, estado: EstadoSolicitud) -> Result<(), Box<dyn Error + Send + Sync>> {
        let estado_str = match estado {
            EstadoSolicitud::PendienteDeRevision => "pendiente_de_revision",
            EstadoSolicitud::AceptadoPorColaborador => "aceptado_por_colaborador",
            EstadoSolicitud::CitaProgramada => "cita_programada",
            EstadoSolicitud::Terminado => "terminado",
            EstadoSolicitud::Cancelado => "cancelado",
            EstadoSolicitud::EnEsperaDePago => "en_espera_de_pago",
        };
        sqlx::query("UPDATE solicitud_servicio SET estado = ? WHERE id = ?")
            .bind(estado_str).bind(id).execute(&self.pool).await?;
        Ok(())
    }
}

impl RepositorioSQLite {
    fn mapear_solicitud(&self, r: sqlx::sqlite::SqliteRow) -> Result<SolicitudServicio, Box<dyn Error + Send + Sync>> {
        use crate::dominio::urgencia::Urgencia;
        use crate::dominio::solicitud::EstadoSolicitud;
        use chrono::{DateTime, Utc};

        let urgencia_str: String = r.get(5);
        let estado_str: String = r.get(7);
        let fecha_str: String = r.get(12);

        Ok(SolicitudServicio {
            id: Some(r.get(0)),
            usuario_id: r.get(1),
            colaborador_id: r.get(2),
            subcategoria_id: r.get(3),
            servicio_id: r.get(4),
            urgencia: Urgencia::desde_cadena(&urgencia_str).unwrap_or(Urgencia::Baja),
            precio_final: r.get::<String, _>(6).parse().unwrap_or(Decimal::ZERO),
            estado: EstadoSolicitud::desde_cadena(&estado_str).unwrap_or(EstadoSolicitud::EnEsperaDePago),
            descripcion_detallada: r.get(8),
            fotos_evidencia_inicial: r.get(9),
            latitud_usuario: r.get::<Option<String>, _>(10).and_then(|s| s.parse().ok()),
            longitud_usuario: r.get::<Option<String>, _>(11).and_then(|s| s.parse().ok()),
            fecha_creacion: Some(DateTime::parse_from_str(&format!("{} +0000", fecha_str), "%Y-%m-%d %H:%M:%S %z")?.with_timezone(&Utc)),
        })
    }
}
