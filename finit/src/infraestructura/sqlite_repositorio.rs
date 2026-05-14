use crate::dominio::puertos::repositorio_categoria::RepositorioCategoria;
use crate::dominio::puertos::repositorio_usuario::RepositorioUsuario;
use crate::dominio::puertos::repositorio_colaborador::RepositorioColaborador;
use crate::dominio::puertos::repositorio_servicio::RepositorioServicio;
use crate::dominio::puertos::repositorio_solicitud::RepositorioSolicitud;
use crate::dominio::puertos::repositorio_mensaje::RepositorioMensaje;
use crate::dominio::puertos::repositorio_resennia::RepositorioResennia;
use crate::dominio::puertos::repositorio_cotizacion_especial::RepositorioCotizacionEspecial;
use crate::dominio::categoria::{Categoria, Subcategoria};
use crate::dominio::usuario::Usuario;
use crate::dominio::colaborador::{Colaborador, TrabajoPortafolio, EstadoVerificacion, ResumenEstadisticasColaborador};
use crate::dominio::servicio::{Servicio, PrecioServicioUrgencia};
use crate::dominio::solicitud::{SolicitudServicio, EstadoSolicitud};
use crate::dominio::mensaje::MensajeSolicitud;
use crate::dominio::resennia::Resennia;
use crate::dominio::cotizacion_especial::CotizacionEspecial;
use crate::dominio::urgencia::Urgencia;
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
        sqlx::query("CREATE TABLE IF NOT EXISTS usuario (id INTEGER PRIMARY KEY AUTOINCREMENT, nombre TEXT, correo TEXT UNIQUE, contrasenna TEXT, rol TEXT DEFAULT 'usuario')")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS colaborador (id INTEGER PRIMARY KEY AUTOINCREMENT, usuario_id INTEGER, telefono TEXT, telefono_verificacion TEXT, zona_trabajo TEXT, sitio_web TEXT, foto_perfil TEXT, especialidad_resumen TEXT, es_verificado INTEGER DEFAULT 0, estado_verificacion TEXT DEFAULT 'pendiente', ine_frontal TEXT, ine_trasera TEXT, comprobante_domicilio TEXT, foto_selfie_ine TEXT, medio_transporte TEXT, conekta_receptor_id TEXT, rating_promedio TEXT DEFAULT '0.0', total_servicios INTEGER DEFAULT 0)")
            .execute(&self.pool).await?;

        // Migraciones manuales para 'colaborador' (por si ya existe la tabla)
        let _ = sqlx::query("ALTER TABLE colaborador ADD COLUMN estado_verificacion TEXT DEFAULT 'pendiente'").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE colaborador ADD COLUMN ine_frontal TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE colaborador ADD COLUMN ine_trasera TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE colaborador ADD COLUMN comprobante_domicilio TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE colaborador ADD COLUMN foto_selfie_ine TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE colaborador ADD COLUMN telefono_verificacion TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE colaborador ADD COLUMN zona_trabajo TEXT").execute(&self.pool).await;
        let _ = sqlx::query("ALTER TABLE colaborador ADD COLUMN conekta_receptor_id TEXT").execute(&self.pool).await;

        // Migracion para usuario: añadir rol si no existe
        let _ = sqlx::query("ALTER TABLE usuario ADD COLUMN rol TEXT DEFAULT 'usuario'").execute(&self.pool).await;

        sqlx::query("CREATE TABLE IF NOT EXISTS portafolio_colaborador (id INTEGER PRIMARY KEY AUTOINCREMENT, colaborador_id INTEGER, foto_antes TEXT, foto_despues TEXT, descripcion TEXT, FOREIGN KEY (colaborador_id) REFERENCES colaborador(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS disponibilidad_colaborador (id INTEGER PRIMARY KEY AUTOINCREMENT, colaborador_id INTEGER, dia_semana INTEGER, hora_inicio TEXT, hora_fin TEXT, activo INTEGER DEFAULT 1, FOREIGN KEY (colaborador_id) REFERENCES colaborador(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS configuracion_precio_colaborador (id INTEGER PRIMARY KEY AUTOINCREMENT, colaborador_id INTEGER, precio_por_kilometro TEXT, recargo_lluvia TEXT, recargo_domingo TEXT, recargo_nocturno TEXT, FOREIGN KEY (colaborador_id) REFERENCES colaborador(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS categoria (id INTEGER PRIMARY KEY AUTOINCREMENT, nombre TEXT UNIQUE)")
            .execute(&self.pool).await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS subcategoria (id INTEGER PRIMARY KEY AUTOINCREMENT, categoria_id INTEGER, nombre TEXT, descripcion TEXT, precio_base TEXT DEFAULT '0.0', FOREIGN KEY (categoria_id) REFERENCES categoria(id))")
            .execute(&self.pool).await?;

        // Migracion para subcategoria: annadir precio_base si no existe
        let _ = sqlx::query("ALTER TABLE subcategoria ADD COLUMN precio_base TEXT DEFAULT '0.0'").execute(&self.pool).await;

        sqlx::query("CREATE TABLE IF NOT EXISTS servicio (id INTEGER PRIMARY KEY AUTOINCREMENT, colaborador_id INTEGER, subcategoria_id INTEGER, descripcion TEXT, distancia_maxima_kilometros TEXT, precio_por_kilometro TEXT, latitud TEXT, longitud TEXT, FOREIGN KEY (colaborador_id) REFERENCES colaborador(id), FOREIGN KEY (subcategoria_id) REFERENCES subcategoria(id))")
            .execute(&self.pool).await?;
        
        sqlx::query("CREATE TABLE IF NOT EXISTS precio_servicio_urgencia (id INTEGER PRIMARY KEY AUTOINCREMENT, servicio_id INTEGER, urgencia TEXT, precio TEXT)")
            .execute(&self.pool).await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS solicitud_servicio (id INTEGER PRIMARY KEY AUTOINCREMENT, usuario_id INTEGER, colaborador_id INTEGER, subcategoria_id INTEGER, servicio_id INTEGER, urgencia TEXT, precio_final TEXT, estado TEXT, descripcion_detallada TEXT, fotos_evidencia_inicial TEXT, latitud_usuario TEXT, longitud_usuario TEXT, conekta_order_id TEXT, fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP)")
            .execute(&self.pool).await?;

        // Migracion para solicitud_servicio: annadir conekta_order_id si no existe
        let _ = sqlx::query("ALTER TABLE solicitud_servicio ADD COLUMN conekta_order_id TEXT").execute(&self.pool).await;

        sqlx::query("CREATE TABLE IF NOT EXISTS mensaje_solicitud (id INTEGER PRIMARY KEY AUTOINCREMENT, solicitud_id INTEGER, emisor_id INTEGER, contenido TEXT, fecha_envio DATETIME DEFAULT CURRENT_TIMESTAMP, FOREIGN KEY (solicitud_id) REFERENCES solicitud_servicio(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS resennia (id INTEGER PRIMARY KEY AUTOINCREMENT, solicitud_id INTEGER, calificacion INTEGER, aspectos TEXT, comentario TEXT, fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP, FOREIGN KEY (solicitud_id) REFERENCES solicitud_servicio(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS cotizacion_especial (id INTEGER PRIMARY KEY AUTOINCREMENT, usuario_id INTEGER, descripcion_trabajo TEXT, fotos_evidencia TEXT, presupuesto_estimado TEXT, nivel_urgencia TEXT, fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP, FOREIGN KEY (usuario_id) REFERENCES usuario(id))")
            .execute(&self.pool).await?;

        // Migracion para resennia: añadir aspectos si no existe
        let _ = sqlx::query("ALTER TABLE resennia ADD COLUMN aspectos TEXT").execute(&self.pool).await;

        Ok(())
    }
}

#[async_trait]
impl RepositorioCategoria for RepositorioSQLite {
    async fn listar(&self) -> Result<Vec<Categoria>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query("SELECT id, nombre FROM categoria")
            .fetch_all(&self.pool).await?;
        let mut categorias = Vec::new();
        for r in rows {
            categorias.push(Categoria {
                id: Some(r.get(0)),
                nombre: r.get(1),
                subcategorias: None,
            });
        }
        Ok(categorias)
    }
    async fn listar_subcategorias(&self, categoria_id: i32) -> Result<Vec<Subcategoria>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query("SELECT id, categoria_id, nombre, descripcion, precio_base FROM subcategoria WHERE categoria_id = ?")
            .bind(categoria_id)
            .fetch_all(&self.pool).await?;
        let mut subcategorias = Vec::new();
        for r in rows {
            subcategorias.push(Subcategoria {
                id: Some(r.get(0)),
                categoria_id: r.get(1),
                nombre: r.get(2),
                descripcion: r.get(3),
                precio_base: r.get::<String, _>(4).parse().unwrap_or(Decimal::ZERO),
            });
        }
        Ok(subcategorias)
    }
    async fn buscar_subcategoria_por_id(&self, id: i32) -> Result<Option<Subcategoria>, Box<dyn Error + Send + Sync>> {
        let r = sqlx::query("SELECT id, categoria_id, nombre, descripcion, precio_base FROM subcategoria WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool).await?;
        if let Some(r) = r {
            Ok(Some(Subcategoria {
                id: Some(r.get(0)),
                categoria_id: r.get(1),
                nombre: r.get(2),
                descripcion: r.get(3),
                precio_base: r.get::<String, _>(4).parse().unwrap_or(Decimal::ZERO),
            }))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl RepositorioUsuario for RepositorioSQLite {
    async fn guardar(&self, usuario: Usuario) -> Result<Usuario, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO usuario (nombre, correo, contrasenna, rol) VALUES (?, ?, ?, ?)")
            .bind(&usuario.nombre).bind(&usuario.correo).bind(&usuario.contrasenna).bind(&usuario.rol).execute(&self.pool).await?;
        Ok(Usuario { id: Some(resultado.last_insert_rowid() as i32), ..usuario })
    }
    async fn actualizar(&self, usuario: Usuario) -> Result<Usuario, Box<dyn Error + Send + Sync>> {
        sqlx::query("UPDATE usuario SET nombre = ?, correo = ?, contrasenna = ?, rol = ? WHERE id = ?")
            .bind(&usuario.nombre).bind(&usuario.correo).bind(&usuario.contrasenna).bind(&usuario.rol).bind(usuario.id).execute(&self.pool).await?;
        Ok(usuario)
    }
    async fn buscar_por_id(&self, id: i32) -> Result<Option<Usuario>, Box<dyn Error + Send + Sync>> {
        let r = sqlx::query("SELECT id, nombre, correo, contrasenna, rol FROM usuario WHERE id = ?").bind(id).fetch_optional(&self.pool).await?;
        if let Some(r) = r {
            Ok(Some(Usuario {
                id: Some(r.get(0)),
                nombre: r.get(1),
                correo: r.get(2),
                contrasenna: r.get(3),
                rol: r.get(4),
            }))
        } else { Ok(None) }
    }
    async fn buscar_por_correo(&self, correo: &str) -> Result<Option<Usuario>, Box<dyn Error + Send + Sync>> {
        let r = sqlx::query("SELECT id, nombre, correo, contrasenna, rol FROM usuario WHERE correo = ?").bind(correo).fetch_optional(&self.pool).await?;
        if let Some(r) = r {
            Ok(Some(Usuario {
                id: Some(r.get(0)),
                nombre: r.get(1),
                correo: r.get(2),
                contrasenna: r.get(3),
                rol: r.get(4),
            }))
        } else { Ok(None) }
    }
}

#[async_trait]
impl RepositorioColaborador for RepositorioSQLite {
    async fn guardar(&self, colaborador: Colaborador) -> Result<Colaborador, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO colaborador (usuario_id, telefono, telefono_verificacion, zona_trabajo, sitio_web, foto_perfil, especialidad_resumen, es_verificado, estado_verificacion, ine_frontal, ine_trasera, comprobante_domicilio, foto_selfie_ine, medio_transporte, conekta_receptor_id, rating_promedio, total_servicios) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(colaborador.usuario_id).bind(&colaborador.telefono)
            .bind(&colaborador.telefono_verificacion).bind(&colaborador.zona_trabajo)
            .bind(&colaborador.sitio_web)
            .bind(&colaborador.foto_perfil).bind(&colaborador.especialidad_resumen).bind(colaborador.es_verificado)
            .bind(colaborador.estado_verificacion.a_cadena_sqlite())
            .bind(&colaborador.ine_frontal).bind(&colaborador.ine_trasera).bind(&colaborador.comprobante_domicilio).bind(&colaborador.foto_selfie_ine)
            .bind(&colaborador.medio_transporte)
            .bind(&colaborador.conekta_receptor_id)
            .bind(colaborador.rating_promedio.to_string()).bind(colaborador.total_servicios)
            .execute(&self.pool).await?;
        Ok(Colaborador { id: Some(resultado.last_insert_rowid() as i32), ..colaborador })
    }
    async fn buscar_por_id(&self, id: i32) -> Result<Option<Colaborador>, Box<dyn Error + Send + Sync>> {
        let registro = sqlx::query("SELECT id, usuario_id, telefono, telefono_verificacion, zona_trabajo, sitio_web, foto_perfil, especialidad_resumen, es_verificado, estado_verificacion, ine_frontal, ine_trasera, comprobante_domicilio, foto_selfie_ine, medio_transporte, conekta_receptor_id, rating_promedio, total_servicios FROM colaborador WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?;
        if let Some(row) = registro {
            let estado_str: String = row.get(9);
            Ok(Some(Colaborador {
                id: Some(row.get(0)),
                usuario_id: row.get(1),
                telefono: row.get(2),
                telefono_verificacion: row.get(3),
                zona_trabajo: row.get(4),
                sitio_web: row.get(5),
                foto_perfil: row.get(6),
                especialidad_resumen: row.get(7),
                es_verificado: row.get::<i32, _>(8) != 0,
                estado_verificacion: EstadoVerificacion::desde_cadena_sqlite(&estado_str),
                ine_frontal: row.get(10),
                ine_trasera: row.get(11),
                comprobante_domicilio: row.get(12),
                foto_selfie_ine: row.get(13),
                medio_transporte: row.get(14),
                conekta_receptor_id: row.get(15),
                rating_promedio: row.get::<String, _>(16).parse().unwrap_or(Decimal::ZERO),
                total_servicios: row.get(17),
            }))
        } else { Ok(None) }
    }
    async fn actualizar(&self, colaborador: Colaborador) -> Result<Colaborador, Box<dyn Error + Send + Sync>> {
        sqlx::query("UPDATE colaborador SET telefono = ?, telefono_verificacion = ?, zona_trabajo = ?, sitio_web = ?, foto_perfil = ?, especialidad_resumen = ?, es_verificado = ?, estado_verificacion = ?, ine_frontal = ?, ine_trasera = ?, comprobante_domicilio = ?, foto_selfie_ine = ?, medio_transporte = ?, conekta_receptor_id = ?, rating_promedio = ?, total_servicios = ? WHERE id = ?")
            .bind(&colaborador.telefono).bind(&colaborador.telefono_verificacion).bind(&colaborador.zona_trabajo)
            .bind(&colaborador.sitio_web).bind(&colaborador.foto_perfil).bind(&colaborador.especialidad_resumen)
            .bind(colaborador.es_verificado).bind(colaborador.estado_verificacion.a_cadena_sqlite())
            .bind(&colaborador.ine_frontal).bind(&colaborador.ine_trasera).bind(&colaborador.comprobante_domicilio).bind(&colaborador.foto_selfie_ine)
            .bind(&colaborador.medio_transporte)
            .bind(&colaborador.conekta_receptor_id)
            .bind(colaborador.rating_promedio.to_string()).bind(colaborador.total_servicios)
            .bind(colaborador.id).execute(&self.pool).await?;
        Ok(colaborador)
    }
    async fn listar_todos(&self) -> Result<Vec<Colaborador>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query("SELECT id, usuario_id, telefono, telefono_verificacion, zona_trabajo, sitio_web, foto_perfil, especialidad_resumen, es_verificado, estado_verificacion, ine_frontal, ine_trasera, comprobante_domicilio, foto_selfie_ine, medio_transporte, conekta_receptor_id, rating_promedio, total_servicios FROM colaborador")
            .fetch_all(&self.pool).await?;
        let mut colaboradores = Vec::new();
        for row in rows {
            let estado_str: String = row.get(9);
            colaboradores.push(Colaborador {
                id: Some(row.get(0)),
                usuario_id: row.get(1),
                telefono: row.get(2),
                telefono_verificacion: row.get(3),
                zona_trabajo: row.get(4),
                sitio_web: row.get(5),
                foto_perfil: row.get(6),
                especialidad_resumen: row.get(7),
                es_verificado: row.get::<i32, _>(8) != 0,
                estado_verificacion: EstadoVerificacion::desde_cadena_sqlite(&estado_str),
                ine_frontal: row.get(10),
                ine_trasera: row.get(11),
                comprobante_domicilio: row.get(12),
                foto_selfie_ine: row.get(13),
                medio_transporte: row.get(14),
                conekta_receptor_id: row.get(15),
                rating_promedio: row.get::<String, _>(16).parse().unwrap_or(Decimal::ZERO),
                total_servicios: row.get(17),
            });
        }
        Ok(colaboradores)
    }
    async fn guardar_trabajo_portafolio(&self, trabajo: TrabajoPortafolio) -> Result<TrabajoPortafolio, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO portafolio_colaborador (colaborador_id, foto_antes, foto_despues, descripcion) VALUES (?, ?, ?, ?)")
            .bind(trabajo.colaborador_id).bind(&trabajo.foto_antes).bind(&trabajo.foto_despues).bind(&trabajo.descripcion).execute(&self.pool).await?;
        Ok(TrabajoPortafolio { id: Some(resultado.last_insert_rowid() as i32), ..trabajo })
    }
    async fn buscar_portafolio_por_colaborador(&self, colaborador_id: i32) -> Result<Vec<TrabajoPortafolio>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query("SELECT id, colaborador_id, foto_antes, foto_despues, descripcion FROM portafolio_colaborador WHERE colaborador_id = ?")
            .bind(colaborador_id).fetch_all(&self.pool).await?;
        let mut portafolio = Vec::new();
        for r in rows {
            portafolio.push(TrabajoPortafolio { id: Some(r.get(0)), colaborador_id: r.get(1), foto_antes: r.get(2), foto_despues: r.get(3), descripcion: r.get(4) });
        }
        Ok(portafolio)
    }
    async fn eliminar_trabajo_portafolio(&self, trabajo_id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query("DELETE FROM portafolio_colaborador WHERE id = ?").bind(trabajo_id).execute(&self.pool).await?;
        Ok(())
    }
    async fn obtener_estadisticas(&self, colaborador_id: i32) -> Result<ResumenEstadisticasColaborador, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query("SELECT total_servicios, rating_promedio FROM colaborador WHERE id = ?").bind(colaborador_id).fetch_one(&self.pool).await?;
        let total_servicios: i32 = row.get(0);
        let rating_promedio: Decimal = row.get::<String, _>(1).parse().unwrap_or(Decimal::ZERO);
        let row_ganancias = sqlx::query("SELECT SUM(CAST(precio_final AS REAL)) FROM solicitud_servicio WHERE colaborador_id = ? AND estado = 'terminado'").bind(colaborador_id).fetch_one(&self.pool).await?;
        let ganancias_totales: Decimal = row_ganancias.get::<Option<f64>, _>(0).map(|f| Decimal::from_f64_retain(f).unwrap_or(Decimal::ZERO)).unwrap_or(Decimal::ZERO);
        let row_pendientes = sqlx::query("SELECT COUNT(*) FROM solicitud_servicio WHERE colaborador_id = ? AND estado = 'pendiente'").bind(colaborador_id).fetch_one(&self.pool).await?;
        let servicios_pendientes: i32 = row_pendientes.get(0);
        Ok(ResumenEstadisticasColaborador { total_servicios, rating_promedio, ganancias_totales, servicios_pendientes })
    }
}

#[async_trait]
impl RepositorioServicio for RepositorioSQLite {
    async fn guardar(&self, servicio: Servicio) -> Result<Servicio, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO servicio (colaborador_id, subcategoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(servicio.colaborador_id).bind(servicio.subcategoria_id).bind(&servicio.descripcion).bind(servicio.distancia_maxima_kilometros.to_string())
            .bind(servicio.precio_por_kilometro.to_string()).bind(servicio.latitud.to_string()).bind(servicio.longitud.to_string()).execute(&self.pool).await?;
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
        let mut servicios = Vec::new();
        for r in rows {
            servicios.push(Servicio {
                id: Some(r.get(0)), colaborador_id: r.get(1), subcategoria_id: r.get(2), descripcion: r.get(3),
                distancia_maxima_kilometros: r.get::<String, _>(4).parse().unwrap_or(Decimal::ZERO),
                precio_por_kilometro: r.get::<String, _>(5).parse().unwrap_or(Decimal::ZERO),
                latitud: r.get::<String, _>(6).parse().unwrap_or(Decimal::ZERO),
                longitud: r.get::<String, _>(7).parse().unwrap_or(Decimal::ZERO),
            });
        }
        Ok(servicios)
    }
    async fn buscar_por_categoria_y_cercania(&self, subcategoria_id: i32, _latitud: f64, _longitud: f64) -> Result<Vec<Servicio>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query("SELECT id, colaborador_id, subcategoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud FROM servicio WHERE subcategoria_id = ?").bind(subcategoria_id).fetch_all(&self.pool).await?;
        let mut servicios = Vec::new();
        for r in rows {
            servicios.push(Servicio {
                id: Some(r.get(0)), colaborador_id: r.get(1), subcategoria_id: r.get(2), descripcion: r.get(3),
                distancia_maxima_kilometros: r.get::<String, _>(4).parse().unwrap_or(Decimal::ZERO),
                precio_por_kilometro: r.get::<String, _>(5).parse().unwrap_or(Decimal::ZERO),
                latitud: r.get::<String, _>(6).parse().unwrap_or(Decimal::ZERO),
                longitud: r.get::<String, _>(7).parse().unwrap_or(Decimal::ZERO),
            });
        }
        Ok(servicios)
    }
    async fn buscar_precio_por_servicio_y_urgencia(&self, servicio_id: i32, urgencia: Urgencia) -> Result<Option<Decimal>, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query("SELECT precio FROM precio_servicio_urgencia WHERE servicio_id = ? AND urgencia = ?")
            .bind(servicio_id).bind(urgencia.a_cadena()).fetch_optional(&self.pool).await?;
        if let Some(r) = row {
            Ok(Some(r.get::<String, _>(0).parse().unwrap_or(Decimal::ZERO)))
        } else { Ok(None) }
    }
}

#[async_trait]
impl RepositorioSolicitud for RepositorioSQLite {
    async fn crear(&self, solicitud: SolicitudServicio) -> Result<SolicitudServicio, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO solicitud_servicio (usuario_id, colaborador_id, subcategoria_id, servicio_id, urgencia, precio_final, estado, descripcion_detallada, fotos_evidencia_inicial, latitud_usuario, longitud_usuario, conekta_order_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(solicitud.usuario_id).bind(solicitud.colaborador_id).bind(solicitud.subcategoria_id).bind(solicitud.servicio_id)
            .bind(solicitud.urgencia.a_cadena()).bind(solicitud.precio_final.to_string()).bind("pendiente_de_revision").bind(&solicitud.descripcion_detallada)
            .bind(&solicitud.fotos_evidencia_inicial).bind(solicitud.latitud_usuario.map(|d| d.to_string())).bind(solicitud.longitud_usuario.map(|d| d.to_string()))
            .bind(&solicitud.conekta_order_id)
            .execute(&self.pool).await?;
        Ok(SolicitudServicio { id: Some(resultado.last_insert_rowid() as i32), ..solicitud })
    }
    async fn buscar_por_id(&self, id: i32) -> Result<Option<SolicitudServicio>, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query("SELECT id, usuario_id, colaborador_id, subcategoria_id, servicio_id, urgencia, precio_final, estado, descripcion_detallada, fotos_evidencia_inicial, latitud_usuario, longitud_usuario, conekta_order_id, fecha_creacion FROM solicitud_servicio WHERE id = ?").bind(id).fetch_optional(&self.pool).await?;
        if let Some(r) = row { Ok(Some(self.mapear_solicitud(r)?)) } else { Ok(None) }
    }
    async fn buscar_por_orden_conekta(&self, orden_id: &str) -> Result<Option<SolicitudServicio>, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query("SELECT id, usuario_id, colaborador_id, subcategoria_id, servicio_id, urgencia, precio_final, estado, descripcion_detallada, fotos_evidencia_inicial, latitud_usuario, longitud_usuario, conekta_order_id, fecha_creacion FROM solicitud_servicio WHERE conekta_order_id = ?").bind(orden_id).fetch_optional(&self.pool).await?;
        if let Some(r) = row { Ok(Some(self.mapear_solicitud(r)?)) } else { Ok(None) }
    }
    async fn listar_por_usuario(&self, usuario_id: i32) -> Result<Vec<SolicitudServicio>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query("SELECT id, usuario_id, colaborador_id, subcategoria_id, servicio_id, urgencia, precio_final, estado, descripcion_detallada, fotos_evidencia_inicial, latitud_usuario, longitud_usuario, conekta_order_id, fecha_creacion FROM solicitud_servicio WHERE usuario_id = ? ORDER BY fecha_creacion DESC").bind(usuario_id).fetch_all(&self.pool).await?;
        let mut solicitudes = Vec::new();
        for r in rows {
            solicitudes.push(self.mapear_solicitud(r)?);
        }
        Ok(solicitudes)
    }
    async fn listar_todas(&self) -> Result<Vec<SolicitudServicio>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query("SELECT id, usuario_id, colaborador_id, subcategoria_id, servicio_id, urgencia, precio_final, estado, descripcion_detallada, fotos_evidencia_inicial, latitud_usuario, longitud_usuario, conekta_order_id, fecha_creacion FROM solicitud_servicio ORDER BY fecha_creacion DESC").fetch_all(&self.pool).await?;
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
            EstadoSolicitud::Pagado => "pagado",
            };

        sqlx::query("UPDATE solicitud_servicio SET estado = ? WHERE id = ?").bind(estado_str).bind(id).execute(&self.pool).await?;
        Ok(())
    }
    async fn actualizar_orden_conekta(&self, id: i32, orden_id: String) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query("UPDATE solicitud_servicio SET conekta_order_id = ? WHERE id = ?").bind(orden_id).bind(id).execute(&self.pool).await?;
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
                id: Some(r.get(0)), solicitud_id: r.get(1), emisor_id: r.get(2), contenido: r.get(3),
                fecha_envio: Some(DateTime::parse_from_str(&format!("{} +0000", fecha_str), "%Y-%m-%d %H:%M:%S %z")?.with_timezone(&Utc)),
            });
        }
        Ok(mensajes)
    }
}

#[async_trait]
impl RepositorioResennia for RepositorioSQLite {
    async fn guardar(&self, resennia: Resennia) -> Result<Resennia, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO resennia (solicitud_id, calificacion, aspectos, comentario) VALUES (?, ?, ?, ?)")
            .bind(resennia.solicitud_id).bind(resennia.calificacion).bind(&resennia.aspectos).bind(&resennia.comentario).execute(&self.pool).await?;
        Ok(Resennia { id: Some(resultado.last_insert_rowid() as i32), ..resennia })
    }
    async fn buscar_por_solicitud(&self, solicitud_id: i32) -> Result<Option<Resennia>, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query("SELECT id, solicitud_id, calificacion, aspectos, comentario, fecha_creacion FROM resennia WHERE solicitud_id = ?").bind(solicitud_id).fetch_optional(&self.pool).await?;
        if let Some(r) = row {
            Ok(Some(Resennia {
                id: Some(r.get(0)),
                solicitud_id: r.get(1),
                calificacion: r.get(2),
                aspectos: r.get(3),
                comentario: r.get(4),
                fecha_creacion: None, // Simplificado
            }))
        } else { Ok(None) }
    }
}

#[async_trait]
impl RepositorioCotizacionEspecial for RepositorioSQLite {
    async fn guardar(&self, cotizacion: CotizacionEspecial) -> Result<CotizacionEspecial, Box<dyn Error + Send + Sync>> {
        let resultado = sqlx::query("INSERT INTO cotizacion_especial (usuario_id, descripcion_trabajo, fotos_evidencia, presupuesto_estimado, nivel_urgencia) VALUES (?, ?, ?, ?, ?)")
            .bind(cotizacion.usuario_id).bind(&cotizacion.descripcion_trabajo).bind(&cotizacion.fotos_evidencia)
            .bind(cotizacion.presupuesto_estimado.map(|d| d.to_string())).bind(cotizacion.nivel_urgencia.a_cadena())
            .execute(&self.pool).await?;
        Ok(CotizacionEspecial { id: Some(resultado.last_insert_rowid() as i32), ..cotizacion })
    }
    async fn listar_por_usuario(&self, usuario_id: i32) -> Result<Vec<CotizacionEspecial>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query("SELECT id, usuario_id, descripcion_trabajo, fotos_evidencia, presupuesto_estimado, nivel_urgencia, fecha_creacion FROM cotizacion_especial WHERE usuario_id = ? ORDER BY fecha_creacion DESC")
            .bind(usuario_id).fetch_all(&self.pool).await?;
        let mut cotizaciones = Vec::new();
        for r in rows {
            cotizaciones.push(self.mapear_cotizacion_especial(r)?);
        }
        Ok(cotizaciones)
    }
    async fn buscar_por_id(&self, id: i32) -> Result<Option<CotizacionEspecial>, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query("SELECT id, usuario_id, descripcion_trabajo, fotos_evidencia, presupuesto_estimado, nivel_urgencia, fecha_creacion FROM cotizacion_especial WHERE id = ?")
            .bind(id).fetch_optional(&self.pool).await?;
        if let Some(r) = row {
            Ok(Some(self.mapear_cotizacion_especial(r)?))
        } else { Ok(None) }
    }
}

impl RepositorioSQLite {
    fn mapear_cotizacion_especial(&self, r: sqlx::sqlite::SqliteRow) -> Result<CotizacionEspecial, Box<dyn Error + Send + Sync>> {
        use chrono::{DateTime, Utc};
        let urgencia_str: String = r.get(5);
        let fecha_str: String = r.get(6);
        Ok(CotizacionEspecial {
            id: Some(r.get(0)),
            usuario_id: r.get(1),
            descripcion_trabajo: r.get(2),
            fotos_evidencia: r.get(3),
            presupuesto_estimado: r.get::<Option<String>, _>(4).and_then(|s| s.parse().ok()),
            nivel_urgencia: Urgencia::desde_cadena(&urgencia_str).unwrap_or(Urgencia::Baja),
            fecha_creacion: Some(DateTime::parse_from_str(&format!("{} +0000", fecha_str), "%Y-%m-%d %H:%M:%S %z")?.with_timezone(&Utc)),
        })
    }

    fn mapear_solicitud(&self, r: sqlx::sqlite::SqliteRow) -> Result<SolicitudServicio, Box<dyn Error + Send + Sync>> {
        use crate::dominio::urgencia::Urgencia;
        use crate::dominio::solicitud::EstadoSolicitud;
        use chrono::{DateTime, Utc};
        let urgencia_str: String = r.get(5);
        let estado_str: String = r.get(7);
        let fecha_str: String = r.get(13);
        Ok(SolicitudServicio {
            id: Some(r.get(0)), usuario_id: r.get(1), colaborador_id: r.get(2), subcategoria_id: r.get(3), servicio_id: r.get(4),
            urgencia: Urgencia::desde_cadena(&urgencia_str).unwrap_or(Urgencia::Baja),
            precio_final: r.get::<String, _>(6).parse().unwrap_or(Decimal::ZERO),
            estado: EstadoSolicitud::desde_cadena(&estado_str).unwrap_or(EstadoSolicitud::EnEsperaDePago),
            descripcion_detallada: r.get(8), fotos_evidencia_inicial: r.get(9),
            latitud_usuario: r.get::<Option<String>, _>(10).and_then(|s| s.parse().ok()),
            longitud_usuario: r.get::<Option<String>, _>(11).and_then(|s| s.parse().ok()),
            conekta_order_id: r.get(12),
            fecha_creacion: Some(DateTime::parse_from_str(&format!("{} +0000", fecha_str), "%Y-%m-%d %H:%M:%S %z")?.with_timezone(&Utc)),
        })
    }
}
