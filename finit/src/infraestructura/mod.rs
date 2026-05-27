use std::error::Error;
use sqlx::MySqlPool;
use bcrypt;

pub mod mysql_repositorio_usuario;
pub mod mysql_repositorio_colaborador;
pub mod mysql_repositorio_servicio;
pub mod mysql_repositorio_solicitud;
pub mod mysql_repositorio_categoria;
pub mod mysql_repositorio_mensaje;
pub mod mysql_repositorio_disponibilidad;
pub mod mysql_repositorio_configuracion_precio;
pub mod mysql_repositorio_resennia;
pub mod mysql_repositorio_cotizacion_especial;
pub mod mysql_repositorio_soporte;
pub mod sqlite_repositorio;
pub mod social;
pub mod api;

use crate::dominio::puertos::repositorio_motor::RepositorioMotor;

pub struct RepositorioMySQL {
    pub pool: MySqlPool,
}

#[async_trait::async_trait]
impl RepositorioMotor for RepositorioMySQL {
    async fn inicializar_tablas(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.inicializar_tablas().await
    }

    async fn limpiar_y_sembrar(&self, admin_password: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.limpiar_y_sembrar(admin_password).await
    }

    async fn ejecutar_query(&self, sql: &str) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
        self.ejecutar_query(sql).await
    }
}

impl RepositorioMySQL {
    pub fn nuevo(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn inicializar_tablas(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // 1. Crear Tablas base
        sqlx::query("CREATE TABLE IF NOT EXISTS usuario (id INT PRIMARY KEY AUTO_INCREMENT, nombre TEXT, correo VARCHAR(255) UNIQUE, contrasenna TEXT, rol VARCHAR(50) DEFAULT 'usuario')")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS colaborador (id INT PRIMARY KEY AUTO_INCREMENT, usuario_id INT, telefono TEXT, telefono_verificacion TEXT, zona_trabajo TEXT, sitio_web TEXT, foto_perfil TEXT, especialidad_resumen TEXT, es_verificado BOOLEAN DEFAULT FALSE, estado_verificacion ENUM('pendiente', 'verificado', 'rechazado') DEFAULT 'pendiente', ine_frontal LONGTEXT, ine_trasera LONGTEXT, comprobante_domicilio LONGTEXT, foto_selfie_ine LONGTEXT, medio_transporte TEXT, conekta_receptor_id TEXT, rating_promedio DECIMAL(3,2) DEFAULT 0.0, total_servicios INT DEFAULT 0)")
            .execute(&self.pool).await?;

        // Asegurar que estado_verificacion sea ENUM si ya existia como TEXT/VARCHAR
        let _ = sqlx::query("ALTER TABLE colaborador MODIFY COLUMN estado_verificacion ENUM('pendiente', 'verificado', 'rechazado') DEFAULT 'pendiente'")
            .execute(&self.pool).await;

        // Asegurar que las columnas de documentos sean LONGTEXT si ya existen
        let columnas_documentos = vec!["ine_frontal", "ine_trasera", "comprobante_domicilio", "foto_selfie_ine"];
        for col in columnas_documentos {
            let _ = sqlx::query(&format!("ALTER TABLE colaborador MODIFY COLUMN {} LONGTEXT", col))
                .execute(&self.pool).await;
        }

        // Migraciones manuales: Comprobar si las columnas existen antes de añadirlas
        let columnas_esperadas_colaborador = vec![
            ("estado_verificacion", "ENUM('pendiente', 'verificado', 'rechazado') DEFAULT 'pendiente'"),
            ("ine_frontal", "LONGTEXT"),
            ("ine_trasera", "LONGTEXT"),
            ("comprobante_domicilio", "LONGTEXT"),
            ("foto_selfie_ine", "LONGTEXT"),
            ("telefono_verificacion", "TEXT"),
            ("zona_trabajo", "TEXT"),
            ("conekta_receptor_id", "TEXT")
        ];

        for (columna, tipo) in columnas_esperadas_colaborador {
            let query = format!(
                "SELECT COUNT(*) FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'colaborador' AND COLUMN_NAME = '{}'",
                columna
            );
            let existe: i64 = sqlx::query_scalar(&query).fetch_one(&self.pool).await.unwrap_or(0);
            
            if existe == 0 {
                let alter = format!("ALTER TABLE colaborador ADD COLUMN {} {}", columna, tipo);
                sqlx::query(&alter).execute(&self.pool).await?;
                println!("🛠️  Columna '{}' annadida a la tabla colaborador.", columna);
            }
        }

        // Migracion para usuario: añadir rol si no existe
        let existe_rol: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'usuario' AND COLUMN_NAME = 'rol'")
            .fetch_one(&self.pool).await.unwrap_or(0);
        if existe_rol == 0 {
            sqlx::query("ALTER TABLE usuario ADD COLUMN rol VARCHAR(50) DEFAULT 'usuario'")
                .execute(&self.pool).await?;
            println!("🛠️  Columna 'rol' annadida a la tabla usuario.");
        }

        sqlx::query("CREATE TABLE IF NOT EXISTS portafolio_colaborador (id INT PRIMARY KEY AUTO_INCREMENT, colaborador_id INT, titulo TEXT, imagen TEXT, descripcion TEXT, FOREIGN KEY (colaborador_id) REFERENCES colaborador(id))")
            .execute(&self.pool).await?;

        // Migracion para portafolio_colaborador: cambiar foto_antes/foto_despues por titulo/imagen si es necesario
        let existe_titulo: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'portafolio_colaborador' AND COLUMN_NAME = 'titulo'")
            .fetch_one(&self.pool).await.unwrap_or(0);
        if existe_titulo == 0 {
            let _ = sqlx::query("ALTER TABLE portafolio_colaborador ADD COLUMN titulo TEXT").execute(&self.pool).await;
            let _ = sqlx::query("ALTER TABLE portafolio_colaborador ADD COLUMN imagen TEXT").execute(&self.pool).await;
            let _ = sqlx::query("UPDATE portafolio_colaborador SET titulo = 'Trabajo anterior', imagen = foto_despues").execute(&self.pool).await;
            println!("🛠️  Tabla portafolio_colaborador actualizada a la nueva estructura.");
        }

        sqlx::query("CREATE TABLE IF NOT EXISTS reporte_soporte (id INT PRIMARY KEY AUTO_INCREMENT, usuario_id INT, descripcion TEXT, fotos_evidencia TEXT, fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP, FOREIGN KEY (usuario_id) REFERENCES usuario(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS disponibilidad_colaborador (id INT PRIMARY KEY AUTO_INCREMENT, colaborador_id INT, dia_semana TINYINT, hora_inicio VARCHAR(5), hora_fin VARCHAR(5), activo BOOLEAN DEFAULT TRUE, FOREIGN KEY (colaborador_id) REFERENCES colaborador(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS configuracion_precio_colaborador (id INT PRIMARY KEY AUTO_INCREMENT, colaborador_id INT, precio_por_kilometro DECIMAL(10,2), recargo_lluvia DECIMAL(10,2), recargo_domingo DECIMAL(10,2), recargo_nocturno DECIMAL(10,2), FOREIGN KEY (colaborador_id) REFERENCES colaborador(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS categoria (id INT PRIMARY KEY AUTO_INCREMENT, nombre VARCHAR(100) UNIQUE)")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS subcategoria (id INT PRIMARY KEY AUTO_INCREMENT, categoria_id INT, nombre TEXT, descripcion TEXT, precio_base DECIMAL(10,2) DEFAULT 0.0, FOREIGN KEY (categoria_id) REFERENCES categoria(id))")
            .execute(&self.pool).await?;

        // Migracion para subcategoria: annadir precio_base si no existe
        let existe_precio_base: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'subcategoria' AND COLUMN_NAME = 'precio_base'")
            .fetch_one(&self.pool).await.unwrap_or(0);
        if existe_precio_base == 0 {
            sqlx::query("ALTER TABLE subcategoria ADD COLUMN precio_base DECIMAL(10,2) DEFAULT 0.0")
                .execute(&self.pool).await?;
            println!("🛠️  Columna 'precio_base' annadida a la tabla subcategoria.");
        }

        sqlx::query("CREATE TABLE IF NOT EXISTS servicio (id INT PRIMARY KEY AUTO_INCREMENT, colaborador_id INT, subcategoria_id INT, descripcion TEXT, distancia_maxima_kilometros DECIMAL(10,2), precio_por_kilometro DECIMAL(10,2), latitud DECIMAL(10,7), longitud DECIMAL(10,7), FOREIGN KEY (colaborador_id) REFERENCES colaborador(id), FOREIGN KEY (subcategoria_id) REFERENCES subcategoria(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS precio_servicio_urgencia (id INT PRIMARY KEY AUTO_INCREMENT, servicio_id INT, urgencia TEXT, precio DECIMAL(10,2))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS solicitud_servicio (id INT PRIMARY KEY AUTO_INCREMENT, usuario_id INT, colaborador_id INT, subcategoria_id INT, servicio_id INT, urgencia TEXT, precio_final DECIMAL(10,2), estado TEXT, descripcion_detallada TEXT, fotos_evidencia_inicial TEXT, fotos_evidencia_final TEXT, latitud_usuario DECIMAL(10,7), longitud_usuario DECIMAL(10,7), conekta_order_id TEXT, fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP)")
            .execute(&self.pool).await?;

        // Migracion para solicitud_servicio: annadir conekta_order_id y fotos_evidencia_final si no existen
        let columnas_solicitud = vec![
            ("conekta_order_id", "TEXT"),
            ("fotos_evidencia_final", "TEXT")
        ];
        for (col, tipo) in columnas_solicitud {
            let query = format!("SELECT COUNT(*) FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'solicitud_servicio' AND COLUMN_NAME = '{}'", col);
            let existe: i64 = sqlx::query_scalar(&query).fetch_one(&self.pool).await.unwrap_or(0);
            if existe == 0 {
                sqlx::query(&format!("ALTER TABLE solicitud_servicio ADD COLUMN {} {}", col, tipo))
                    .execute(&self.pool).await?;
                println!("🛠️  Columna '{}' annadida a la tabla solicitud_servicio.", col);
            }
        }

        sqlx::query("CREATE TABLE IF NOT EXISTS mensaje_solicitud (id INT PRIMARY KEY AUTO_INCREMENT, solicitud_id INT, emisor_id INT, contenido TEXT, fecha_envio DATETIME DEFAULT CURRENT_TIMESTAMP, FOREIGN KEY (solicitud_id) REFERENCES solicitud_servicio(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS resennia (id INT PRIMARY KEY AUTO_INCREMENT, solicitud_id INT, calificacion TINYINT, aspectos TEXT, comentario TEXT, fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP, FOREIGN KEY (solicitud_id) REFERENCES solicitud_servicio(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS cotizacion_especial (id INT PRIMARY KEY AUTO_INCREMENT, usuario_id INT, descripcion_trabajo TEXT, fotos_evidencia TEXT, presupuesto_estimado DECIMAL(10,2), nivel_urgencia VARCHAR(50), fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP, FOREIGN KEY (usuario_id) REFERENCES usuario(id))")
            .execute(&self.pool).await?;

        // Migracion para resennia: añadir aspectos si no existe
        let existe_aspectos: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = 'resennia' AND COLUMN_NAME = 'aspectos'")
            .fetch_one(&self.pool).await.unwrap_or(0);
        if existe_aspectos == 0 {
            sqlx::query("ALTER TABLE resennia ADD COLUMN aspectos TEXT")
                .execute(&self.pool).await?;
            println!("🛠️  Columna 'aspectos' annadida a la tabla resennia.");
        }

        // 2. Insertar Datos Semilla (Categorias 1-4)
        let categorias = vec![
            (1, "Cerrajeria"),
            (2, "Plomeria"),
            (3, "Electricidad"),
            (4, "Limpieza")
        ];

        for (id, nombre) in categorias {
            let _ = sqlx::query("INSERT IGNORE INTO categoria (id, nombre) VALUES (?, ?)")
                .bind(id).bind(nombre).execute(&self.pool).await;
        }

        // 3. Insertar Subcategorias
        let subcats = vec![
            (1, "Apertura de Puertas", "Apertura de cerraduras sin llaves"),
            (1, "Cambio de Cerraduras", "Instalacion de nuevas chapas"),
            (1, "Duplicado de Llaves", "Copia de llaves residenciales"),
            (2, "Reparacion de Fugas", "Arreglo de goteras y tuberias rotas"),
            (2, "Instalacion de Sanitarios", "Montaje de baños y mingitorios"),
            (2, "Destape de Cannierias", "Limpieza de drenajes obstruidos"),
            (3, "Cortocircuitos", "Reparacion de fallas electricas urgentes"),
            (3, "Instalacion de Lamparas", "Colocacion de luminarias y focos"),
            (3, "Cableado General", "Instalacion electrica completa"),
            (4, "Limpieza de Casas", "Limpieza profunda residencial"),
            (4, "Limpieza de Oficinas", "Mantenimiento de espacios laborales"),
            (4, "Lavado de Alfombras", "Limpieza profesional de textiles")
        ];

        for (cat_id, nombre, desc) in subcats {
            let _ = sqlx::query("INSERT IGNORE INTO subcategoria (categoria_id, nombre, descripcion) VALUES (?, ?, ?)")
                .bind(cat_id).bind(nombre).bind(desc).execute(&self.pool).await;
        }

        // 4. Asegurar usuario admin
        let admin_correo = std::env::var("ADMIN_CORREO").unwrap_or_else(|_| "admin@okupo.com".to_string());
        let admin_pass = std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin".to_string());

        let existe_admin: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM usuario WHERE correo = ?")
            .bind(&admin_correo)
            .fetch_one(&self.pool)
            .await
            .unwrap_or((0,));

        if existe_admin.0 == 0 {
            println!("👤 Creando usuario administrador ({})", admin_correo);
            let hash = bcrypt::hash(&admin_pass, bcrypt::DEFAULT_COST)?;
            sqlx::query("INSERT INTO usuario (nombre, correo, contrasenna, rol) VALUES (?, ?, ?, ?)")
                .bind("Administrador")
                .bind(&admin_correo)
                .bind(hash)
                .bind("admin")
                .execute(&self.pool)
                .await?;
        }

        println!("✅ Base de datos MySQL inicializada con datos semilla.");
        Ok(())
    }

    pub async fn limpiar_y_sembrar(&self, admin_password: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("🧹 Limpiando base de datos...");

        let mut conn = self.pool.acquire().await?;

        // Desactivar temporalmente las restricciones de llaves foraneas
        sqlx::query("SET FOREIGN_KEY_CHECKS = 0").execute(&mut *conn).await?;

        let tablas = vec![
            "mensaje_solicitud", "resennia", "solicitud_servicio", "precio_servicio_urgencia",
            "servicio", "subcategoria", "categoria", "configuracion_precio_colaborador",
            "disponibilidad_colaborador", "reporte_soporte", "portafolio_colaborador",
            "colaborador", "usuario", "cotizacion_especial"
        ];

        for tabla in tablas {
            sqlx::query(&format!("TRUNCATE TABLE {}", tabla)).execute(&mut *conn).await?;
        }

        sqlx::query("SET FOREIGN_KEY_CHECKS = 1").execute(&mut *conn).await?;

        // Re-inicializar tablas y datos semilla
        self.inicializar_tablas().await?;

        // El usuario admin ya se crea en inicializar_tablas usando ADMIN_CORREO y ADMIN_PASSWORD de entorno
        // Pero limpiar_y_sembrar recibe un admin_password especifico (usado en el CLI)
        // Si queremos forzar el password que viene por parametro:
        let admin_correo = std::env::var("ADMIN_CORREO").unwrap_or_else(|_| "admin@okupo.com".to_string());
        let hash = bcrypt::hash(admin_password, bcrypt::DEFAULT_COST)?;
        
        sqlx::query("UPDATE usuario SET contrasenna = ? WHERE correo = ?")
            .bind(hash)
            .bind(&admin_correo)
            .execute(&mut *conn)
            .await?;

        println!("✨ Base de datos reseteada con éxito. Admin: {}", admin_correo);
        Ok(())
    }

    pub async fn ejecutar_query(&self, sql: &str) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>> {
        use sqlx::{Column, Row};
        
        // Determinar si es una consulta SELECT o una operacion de modificacion
        let sql_trimmed = sql.trim().to_uppercase();
        if sql_trimmed.starts_with("SELECT") || sql_trimmed.starts_with("SHOW") || sql_trimmed.starts_with("DESCRIBE") {
            let rows = sqlx::query(sql).fetch_all(&self.pool).await?;
            let mut resultados = Vec::new();

            for row in rows {
                let mut objeto = serde_json::Map::new();
                for column in row.columns() {
                    let name = column.name();
                    
                    // Intento de mapeo de tipos básicos a JSON
                    let value = if let Ok(v) = row.try_get::<String, _>(name) {
                        serde_json::Value::String(v)
                    } else if let Ok(v) = row.try_get::<i32, _>(name) {
                        serde_json::Value::Number(v.into())
                    } else if let Ok(v) = row.try_get::<i64, _>(name) {
                        serde_json::Value::Number(v.into())
                    } else if let Ok(v) = row.try_get::<f64, _>(name) {
                        if let Some(n) = serde_json::Number::from_f64(v) {
                            serde_json::Value::Number(n)
                        } else {
                            serde_json::Value::Null
                        }
                    } else if let Ok(v) = row.try_get::<bool, _>(name) {
                        serde_json::Value::Bool(v)
                    } else {
                        // Para tipos mas complejos o nulos, devolvemos un string descriptivo o null
                        match row.try_get_raw(name) {
                            Ok(raw) if sqlx::ValueRef::is_null(&raw) => serde_json::Value::Null,
                            _ => serde_json::Value::String(format!("<{:?}>", column.type_info()))
                        }
                    };
                    objeto.insert(name.to_string(), value);
                }
                resultados.push(serde_json::Value::Object(objeto));
            }
            Ok(serde_json::Value::Array(resultados))
        } else {
            let result = sqlx::query(sql).execute(&self.pool).await?;
            Ok(serde_json::json!({
                "filas_afectadas": result.rows_affected(),
                "ultimo_id_insertado": result.last_insert_id()
            }))
        }
    }
}

