use std::error::Error;
use sqlx::MySqlPool;

pub mod mysql_repositorio_usuario;
pub mod mysql_repositorio_colaborador;
pub mod mysql_repositorio_servicio;
pub mod mysql_repositorio_solicitud;
pub mod mysql_repositorio_categoria;
pub mod mysql_repositorio_mensaje;
pub mod mysql_repositorio_disponibilidad;
pub mod mysql_repositorio_configuracion_precio;
pub mod mysql_repositorio_resennia;
pub mod sqlite_repositorio;
pub mod api;

pub struct RepositorioMySQL {
    pub pool: MySqlPool,
}

impl RepositorioMySQL {
    pub fn nuevo(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn inicializar_tablas(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // 1. Crear Tablas base
        sqlx::query("CREATE TABLE IF NOT EXISTS usuario (id INT PRIMARY KEY AUTO_INCREMENT, nombre TEXT, correo VARCHAR(255) UNIQUE, contrasenna TEXT)")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS colaborador (id INT PRIMARY KEY AUTO_INCREMENT, usuario_id INT, telefono TEXT, sitio_web TEXT, foto_perfil TEXT, especialidad_resumen TEXT, es_verificado BOOLEAN DEFAULT FALSE, estado_verificacion ENUM('pendiente', 'verificado', 'rechazado') DEFAULT 'pendiente', ine_frontal TEXT, ine_trasera TEXT, comprobante_domicilio TEXT, foto_selfie_ine TEXT, medio_transporte TEXT, rating_promedio DECIMAL(3,2) DEFAULT 0.0, total_servicios INT DEFAULT 0)")
            .execute(&self.pool).await?;

        // Asegurar que estado_verificacion sea ENUM si ya existia como TEXT/VARCHAR
        let _ = sqlx::query("ALTER TABLE colaborador MODIFY COLUMN estado_verificacion ENUM('pendiente', 'verificado', 'rechazado') DEFAULT 'pendiente'")
            .execute(&self.pool).await;

        // Migraciones manuales: Comprobar si las columnas existen antes de añadirlas
        let columnas_esperadas = vec![
            ("estado_verificacion", "ENUM('pendiente', 'verificado', 'rechazado') DEFAULT 'pendiente'"),
            ("ine_frontal", "TEXT"),
            ("ine_trasera", "TEXT"),
            ("comprobante_domicilio", "TEXT"),
            ("foto_selfie_ine", "TEXT")
        ];

        for (columna, tipo) in columnas_esperadas {
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

        sqlx::query("CREATE TABLE IF NOT EXISTS portafolio_colaborador (id INT PRIMARY KEY AUTO_INCREMENT, colaborador_id INT, foto_antes TEXT, foto_despues TEXT, descripcion TEXT, FOREIGN KEY (colaborador_id) REFERENCES colaborador(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS disponibilidad_colaborador (id INT PRIMARY KEY AUTO_INCREMENT, colaborador_id INT, dia_semana TINYINT, hora_inicio VARCHAR(5), hora_fin VARCHAR(5), activo BOOLEAN DEFAULT TRUE, FOREIGN KEY (colaborador_id) REFERENCES colaborador(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS configuracion_precio_colaborador (id INT PRIMARY KEY AUTO_INCREMENT, colaborador_id INT, precio_por_kilometro DECIMAL(10,2), recargo_lluvia DECIMAL(10,2), recargo_domingo DECIMAL(10,2), recargo_nocturno DECIMAL(10,2), FOREIGN KEY (colaborador_id) REFERENCES colaborador(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS categoria (id INT PRIMARY KEY AUTO_INCREMENT, nombre VARCHAR(100) UNIQUE)")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS subcategoria (id INT PRIMARY KEY AUTO_INCREMENT, categoria_id INT, nombre TEXT, descripcion TEXT, FOREIGN KEY (categoria_id) REFERENCES categoria(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS servicio (id INT PRIMARY KEY AUTO_INCREMENT, colaborador_id INT, subcategoria_id INT, descripcion TEXT, distancia_maxima_kilometros DECIMAL(10,2), precio_por_kilometro DECIMAL(10,2), latitud DECIMAL(10,7), longitud DECIMAL(10,7), FOREIGN KEY (colaborador_id) REFERENCES colaborador(id), FOREIGN KEY (subcategoria_id) REFERENCES subcategoria(id))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS precio_servicio_urgencia (id INT PRIMARY KEY AUTO_INCREMENT, servicio_id INT, urgencia TEXT, precio DECIMAL(10,2))")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS solicitud_servicio (id INT PRIMARY KEY AUTO_INCREMENT, usuario_id INT, colaborador_id INT, subcategoria_id INT, servicio_id INT, urgencia TEXT, precio_final DECIMAL(10,2), estado TEXT, descripcion_detallada TEXT, fotos_evidencia_inicial TEXT, latitud_usuario DECIMAL(10,7), longitud_usuario DECIMAL(10,7), fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP)")
            .execute(&self.pool).await?;

        sqlx::query("CREATE TABLE IF NOT EXISTS mensaje_solicitud (id INT PRIMARY KEY AUTO_INCREMENT, solicitud_id INT, emisor_id INT, contenido TEXT, fecha_envio DATETIME DEFAULT CURRENT_TIMESTAMP, FOREIGN KEY (solicitud_id) REFERENCES solicitud_servicio(id))")
            .execute(&self.pool).await?;

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

        println!("✅ Base de datos MySQL inicializada con datos semilla.");
        Ok(())
    }
}

