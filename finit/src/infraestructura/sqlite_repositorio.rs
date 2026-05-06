use crate::dominio::usuario::Usuario;
use crate::dominio::colaborador::{Colaborador, TrabajoPortafolio, EstadoVerificacion, ResumenEstadisticasColaborador};
use crate::dominio::servicio::{Servicio, PrecioServicioUrgencia};
use crate::dominio::solicitud::{SolicitudServicio, EstadoSolicitud};
use crate::dominio::urgencia::Urgencia;
use crate::dominio::mensaje::MensajeSolicitud;
use crate::dominio::categoria::{Categoria, Subcategoria};
use crate::dominio::disponibilidad::Disponibilidad;
use crate::dominio::configuracion_precio::ConfiguracionPrecio;
use crate::dominio::puertos::repositorio_categoria::RepositorioCategoria;
use crate::dominio::puertos::repositorio_usuario::RepositorioUsuario;
use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use crate::dominio::puertos::repositorio_servicio::RepositorioServicio;
use crate::dominio::puertos::repositorio_solicitud::RepositorioSolicitud;
use crate::dominio::puertos::repositorio_mensaje::RepositorioMensaje;
use crate::dominio::puertos::repositorio_disponibilidad::RepositorioDisponibilidad;
use crate::dominio::puertos::repositorio_configuracion_precio::RepositorioConfiguracionPrecio;
use crate::dominio::puertos::repositorio_resennia::RepositorioResennia;
use crate::dominio::resennia::Resennia;
use std::error::Error;
use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use rust_decimal::prelude::*;

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
        
        sqlx::query("CREATE TABLE IF NOT EXISTS colaborador (id INTEGER PRIMARY KEY AUTOINCREMENT, usuario_id INTEGER, telefono TEXT, sitio_web TEXT, foto_perfil TEXT, especialidad_resumen TEXT, es_verificado INTEGER DEFAULT 0, estado_verificacion TEXT DEFAULT 'pendiente', ine_frontal TEXT, ine_trasera TEXT, comprobante_domicilio TEXT, foto_selfie_ine TEXT, medio_transporte TEXT, rating_promedio TEXT DEFAULT '0.0', total_servicios INTEGER DEFAULT 0)")
            .execute(&self.pool).await?;

        // Migraciones manuales para 'colaborador' (por si ya existe la tabla)
        let _ = sqlx::query("ALTER TABLE colaborador ADD COLUMN estado_verificacion TEXT DEFAULT 'pendiente'").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE colaborador ADD COLUMN ine_frontal TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE colaborador ADD COLUMN ine_trasera TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE colaborador ADD COLUMN comprobante_domicilio TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE colaborador ADD COLUMN foto_selfie_ine TEXT").execute(&self.pool).await;

        sqlx::query("CREATE TABLE IF NOT EXISTS portafolio_colaborador (id INTEGER PRIMARY KEY AUTOINCREMENT, colaborador_id INTEGER, foto_antes TEXT, foto_despues TEXT, descripcion TEXT, FOREIGN KEY (colaborador_id) REFERENCES colaborador(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS disponibilidad_colaborador (id INTEGER PRIMARY KEY AUTOINCREMENT, colaborador_id INTEGER, dia_semana INTEGER, hora_inicio TEXT, hora_fin TEXT, activo INTEGER DEFAULT 1, FOREIGN KEY (colaborador_id) REFERENCES colaborador(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS configuracion_precio_colaborador (id INTEGER PRIMARY KEY AUTOINCREMENT, colaborador_id INTEGER, precio_por_kilometro TEXT, recargo_lluvia TEXT, recargo_domingo TEXT, recargo_nocturno TEXT, FOREIGN KEY (colaborador_id) REFERENCES colaborador(id))")
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
        
        sqlx::query("CREATE TABLE IF NOT EXISTS mensaje_solicitud (id INTEGER PRIMARY KEY AUTOINCREMENT, solicitud_id INTEGER, emisor_id INTEGER, contenido TEXT, fecha_envio DATETIME DEFAULT CURRENT_TIMESTAMP, FOREIGN KEY (solicitud_id) REFERENCES solicitud_servicio(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS resennia (id INTEGER PRIMARY KEY AUTOINCREMENT, solicitud_id INTEGER, calificacion INTEGER, comentario TEXT, fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP, FOREIGN KEY (solicitud_id) REFERENCES solicitud_servicio(id))")
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
    async fn buscar_subcategoria_por_id(&self, id: i32) -> Result<Option<Subcategoria>, Box<dyn Error + Send + Sync>> {
        let subcategoria = sqlx::query_as::<_, Subcategoria>("SELECT id, categoria_id, nombre, descripcion FROM subcategoria WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool).await?;
        Ok(subcategoria)
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
        let resultado = sqlx::query("INSERT INTO colaborador (usuario_id, telefono, sitio_web, foto_perfil, especialidad_resumen, es_verificado, estado_verificacion, ine_frontal, ine_trasera, comprobante_domicilio, foto_selfie_ine, medio_transporte, rating_promedio, total_servicios) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(colaborador.usuario_id).bind(&colaborador.telefono).bind(&colaborador.sitio_web)
            .bind(&colaborador.foto_perfil).bind(&colaborador.especialidad_resumen).bind(colaborador.es_verificado)
            .bind(colaborador.estado_verificacion.a_cadena_sqlite())
            .bind(&colaborador.ine_frontal).bind(&colaborador.ine_trasera).bind(&colaborador.comprobante_domicilio).bind(&colaborador.foto_selfie_ine)
            .bind(&colaborador.medio_transporte).bind(colaborador.rating_promedio.to_string()).bind(colaborador.total_servicios)
            .execute(&self.pool).await?;
        Ok(Colaborador { id: Some(resultado.last_insert_rowid() as i32), ..colaborador })
    }

    async fn actualizar(&self, colaborador: Colaborador) -> Result<Colaborador, Box<dyn Error + Send + Sync>> {
        sqlx::query("UPDATE colaborador SET telefono = ?, sitio_web = ?, foto_perfil = ?, especialidad_resumen = ?, es_verificado = ?, estado_verificacion = ?, ine_frontal = ?, ine_trasera = ?, comprobante_domicilio = ?, foto_selfie_ine = ?, medio_transporte = ?, rating_promedio = ?, total_servicios = ? WHERE id = ?")
            .bind(&colaborador.telefono).bind(&colaborador.sitio_web).bind(&colaborador.foto_perfil).bind(&colaborador.especialidad_resumen).bind(colaborador.es_verificado)
            .bind(colaborador.estado_verificacion.a_cadena_sqlite())
            .bind(&colaborador.ine_frontal).bind(&colaborador.ine_trasera).bind(&colaborador.comprobante_domicilio).bind(&colaborador.foto_selfie_ine)
            .bind(&colaborador.medio_transporte).bind(colaborador.rating_promedio.to_string()).bind(colaborador.total_servicios)
            .bind(colaborador.id).execute(&self.pool).await?;
        Ok(colaborador)
    }

    async fn buscar_por_id(&self, id: i32) -> Result<Option<Colaborador>, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query("SELECT id, usuario_id, telefono, sitio_web, foto_perfil, especialidad_resumen, es_verificado, estado_verificacion, ine_frontal, ine_trasera, comprobante_domicilio, foto_selfie_ine, medio_transporte, rating_promedio, total_servicios FROM colaborador WHERE id = ?").bind(id).fetch_optional(&self.pool).await?;
        if let Some(r) = row {
            Ok(Some(Colaborador {
                id: Some(r.get(0)), usuario_id: r.get(1), telefono: r.get(2), sitio_web: r.get(3),
                foto_perfil: r.get(4), especialidad_resumen: r.get(5),
                es_verificado: r.get::<i32, _>(6) != 0,
                estado_verificacion: EstadoVerificacion::desde_cadena_sqlite(&r.get::<String, _>(7)),
                ine_frontal: r.get(8), ine_trasera: r.get(9), comprobante_domicilio: r.get(10), foto_selfie_ine: r.get(11),
                medio_transporte: r.get(12),
                rating_promedio: r.get::<String, _>(13).parse().unwrap_or(Decimal::ZERO),
                total_servicios: r.get(14),
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

    async fn eliminar_trabajo_portafolio(&self, trabajo_id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query("DELETE FROM portafolio_colaborador WHERE id = ?")
            .bind(trabajo_id).execute(&self.pool).await?;
        Ok(())
    }

    async fn obtener_estadisticas(&self, colaborador_id: i32) -> Result<ResumenEstadisticasColaborador, Box<dyn Error + Send + Sync>> {
        use crate::dominio::colaborador::ResumenEstadisticasColaborador;

        let r = sqlx::query("SELECT total_servicios, rating_promedio FROM colaborador WHERE id = ?")
            .bind(colaborador_id)
            .fetch_one(&self.pool).await?;
        
        let total_servicios: i32 = r.get(0);
        let rating_promedio: Decimal = r.get::<String, _>(1).parse().unwrap_or(Decimal::ZERO);

        let ganancias = sqlx::query("SELECT SUM(CAST(precio_final AS REAL)) FROM solicitud_servicio WHERE colaborador_id = ? AND estado = 'terminado'")
            .bind(colaborador_id)
            .fetch_one(&self.pool).await?;
        
        let ganancias_totales = Decimal::from_f64_retain(ganancias.get::<Option<f64>, _>(0).unwrap_or(0.0)).unwrap_or(Decimal::ZERO);

        let pendientes = sqlx::query("SELECT COUNT(*) FROM solicitud_servicio WHERE colaborador_id = ? AND estado = 'pendiente_de_revision'")
            .bind(colaborador_id)
            .fetch_one(&self.pool).await?;
        
        let servicios_pendientes: i32 = pendientes.get(0);

        Ok(ResumenEstadisticasColaborador {
            total_servicios,
            rating_promedio,
            ganancias_totales,
            servicios_pendientes,
        })
    }

    async fn listar_todos(&self) -> Result<Vec<Colaborador>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query("SELECT id, usuario_id, telefono, sitio_web, foto_perfil, especialidad_resumen, es_verificado, estado_verificacion, ine_frontal, ine_trasera, comprobante_domicilio, foto_selfie_ine, medio_transporte, rating_promedio, total_servicios FROM colaborador").fetch_all(&self.pool).await?;
        
        let mut colaboradores = Vec::new();
        for r in rows {
            colaboradores.push(Colaborador {
                id: Some(r.get(0)), usuario_id: r.get(1), telefono: r.get(2), sitio_web: r.get(3),
                foto_perfil: r.get(4), especialidad_resumen: r.get(5),
                es_verificado: r.get::<i32, _>(6) != 0,
                estado_verificacion: EstadoVerificacion::desde_cadena_sqlite(&r.get::<String, _>(7)),
                ine_frontal: r.get(8), ine_trasera: r.get(9), comprobante_domicilio: r.get(10), foto_selfie_ine: r.get(11),
                medio_transporte: r.get(12),
                rating_promedio: r.get::<String, _>(13).parse().unwrap_or(Decimal::ZERO),
                total_servicios: r.get(14),
            });
        }
        Ok(colaboradores)
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
impl RepositorioDisponibilidad for RepositorioSQLite {
    async fn guardar_disponibilidad(&self, disp: Disponibilidad) -> Result<Disponibilidad, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO disponibilidad_colaborador (colaborador_id, dia_semana, hora_inicio, hora_fin, activo) VALUES (?, ?, ?, ?, ?)")
            .bind(disp.colaborador_id).bind(disp.dia_semana).bind(&disp.hora_inicio).bind(&disp.hora_fin).bind(disp.activo)
            .execute(&self.pool).await?;
        Ok(Disponibilidad { id: Some(resultado.last_insert_rowid() as i32), ..disp })
    }
    async fn buscar_por_colaborador(&self, colaborador_id: i32) -> Result<Vec<Disponibilidad>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query_as::<_, Disponibilidad>("SELECT id, colaborador_id, dia_semana, hora_inicio, hora_fin, activo FROM disponibilidad_colaborador WHERE colaborador_id = ?")
            .bind(colaborador_id).fetch_all(&self.pool).await?;
        Ok(rows)
    }
    async fn eliminar_por_colaborador(&self, colaborador_id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query("DELETE FROM disponibilidad_colaborador WHERE colaborador_id = ?").bind(colaborador_id).execute(&self.pool).await?;
        Ok(())
    }
}

#[async_trait]
impl RepositorioConfiguracionPrecio for RepositorioSQLite {
    async fn guardar(&self, conf: ConfiguracionPrecio) -> Result<ConfiguracionPrecio, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO configuracion_precio_colaborador (colaborador_id, precio_por_kilometro, recargo_lluvia, recargo_domingo, recargo_nocturno) VALUES (?, ?, ?, ?, ?)")
            .bind(conf.colaborador_id).bind(conf.precio_por_kilometro.to_string()).bind(conf.recargo_lluvia.to_string())
            .bind(conf.recargo_domingo.to_string()).bind(conf.recargo_nocturno.to_string())
            .execute(&self.pool).await?;
        Ok(ConfiguracionPrecio { id: Some(resultado.last_insert_rowid() as i32), ..conf })
    }
    async fn buscar_por_colaborador(&self, colaborador_id: i32) -> Result<Option<ConfiguracionPrecio>, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query("SELECT id, colaborador_id, precio_por_kilometro, recargo_lluvia, recargo_domingo, recargo_nocturno FROM configuracion_precio_colaborador WHERE colaborador_id = ?").bind(colaborador_id).fetch_optional(&self.pool).await?;
        if let Some(r) = row {
            Ok(Some(ConfiguracionPrecio {
                id: Some(r.get(0)), colaborador_id: r.get(1),
                precio_por_kilometro: r.get::<String, _>(2).parse().unwrap_or(Decimal::ZERO),
                recargo_lluvia: r.get::<String, _>(3).parse().unwrap_or(Decimal::ZERO),
                recargo_domingo: r.get::<String, _>(4).parse().unwrap_or(Decimal::ZERO),
                recargo_nocturno: r.get::<String, _>(5).parse().unwrap_or(Decimal::ZERO),
            }))
        } else { Ok(None) }
    }
    async fn actualizar(&self, conf: ConfiguracionPrecio) -> Result<ConfiguracionPrecio, Box<dyn Error + Send + Sync>> {
        sqlx::query("UPDATE configuracion_precio_colaborador SET precio_por_kilometro = ?, recargo_lluvia = ?, recargo_domingo = ?, recargo_nocturno = ? WHERE id = ?")
            .bind(conf.precio_por_kilometro.to_string()).bind(conf.recargo_lluvia.to_string()).bind(conf.recargo_domingo.to_string())
            .bind(conf.recargo_nocturno.to_string()).bind(conf.id).execute(&self.pool).await?;
        Ok(conf)
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
    async fn buscar_por_id(&self, id: i32) -> Result<Option<SolicitudServicio>, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query("SELECT id, usuario_id, colaborador_id, subcategoria_id, servicio_id, urgencia, precio_final, estado, descripcion_detallada, fotos_evidencia_inicial, latitud_usuario, longitud_usuario, fecha_creacion FROM solicitud_servicio WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?;
        
        if let Some(r) = row {
            Ok(Some(self.mapear_solicitud(r)?))
        } else {
            Ok(None)
        }
    }
    
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

#[async_trait]
impl RepositorioMensaje for RepositorioSQLite {
    async fn guardar(&self, mensaje: MensajeSolicitud) -> Result<MensajeSolicitud, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO mensaje_solicitud (solicitud_id, emisor_id, contenido) VALUES (?, ?, ?)")
            .bind(mensaje.solicitud_id).bind(mensaje.emisor_id).bind(&mensaje.contenido).execute(&self.pool).await?;
        Ok(MensajeSolicitud { id: Some(resultado.last_insert_rowid() as i32), ..mensaje })
    }
    async fn listar_por_solicitud(&self, solicitud_id: i32) -> Result<Vec<MensajeSolicitud>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query("SELECT id, solicitud_id, emisor_id, contenido, fecha_envio FROM mensaje_solicitud WHERE solicitud_id = ? ORDER BY fecha_envio ASC")
            .bind(solicitud_id).fetch_all(&self.pool).await?;
        
        use chrono::{DateTime, Utc};
        let mut mensajes = Vec::new();
        for r in rows {
            let fecha_str: String = r.get(4);
            mensajes.push(MensajeSolicitud {
                id: Some(r.get(0)),
                solicitud_id: r.get(1),
                emisor_id: r.get(2),
                contenido: r.get(3),
                fecha_envio: Some(DateTime::parse_from_str(&format!("{} +0000", fecha_str), "%Y-%m-%d %H:%M:%S %z")?.with_timezone(&Utc)),
            });
        }
        Ok(mensajes)
    }
}

#[async_trait]
impl RepositorioResennia for RepositorioSQLite {
    async fn guardar(&self, resennia: Resennia) -> Result<Resennia, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO resennia (solicitud_id, calificacion, comentario) VALUES (?, ?, ?)")
            .bind(resennia.solicitud_id).bind(resennia.calificacion).bind(&resennia.comentario).execute(&self.pool).await?;
        Ok(Resennia { id: Some(resultado.last_insert_rowid() as i32), ..resennia })
    }
    async fn buscar_por_solicitud(&self, solicitud_id: i32) -> Result<Option<Resennia>, Box<dyn Error + Send + Sync>> {
        Ok(sqlx::query_as::<_, Resennia>("SELECT id, solicitud_id, calificacion, comentario, fecha_creacion FROM resennia WHERE solicitud_id = ?").bind(solicitud_id).fetch_optional(&self.pool).await?)
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
