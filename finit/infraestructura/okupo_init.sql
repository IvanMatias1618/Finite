-- Esquema SQLite para okupo.db

CREATE TABLE IF NOT EXISTS usuario (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre TEXT NOT NULL,
    correo TEXT UNIQUE NOT NULL,
    contrasenna TEXT NOT NULL,
    fecha_registro DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS colaborador (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    usuario_id INTEGER NOT NULL,
    telefono TEXT,
    sitio_web TEXT,
    FOREIGN KEY (usuario_id) REFERENCES usuario(id)
);

CREATE TABLE IF NOT EXISTS categoria (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS servicio (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    colaborador_id INTEGER NOT NULL,
    categoria_id INTEGER NOT NULL,
    descripcion TEXT,
    distancia_maxima_kilometros DECIMAL(10,2),
    precio_por_kilometro DECIMAL(10,2),
    latitud DECIMAL(10,8),
    longitud DECIMAL(11,8),
    FOREIGN KEY (colaborador_id) REFERENCES colaborador(id),
    FOREIGN KEY (categoria_id) REFERENCES categoria(id)
);

CREATE TABLE IF NOT EXISTS precio_servicio_urgencia (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    servicio_id INTEGER NOT NULL,
    urgencia TEXT NOT NULL CHECK(urgencia IN ('baja', 'media', 'alta', 'critica')),
    precio DECIMAL(10,2) NOT NULL,
    UNIQUE(servicio_id, urgencia),
    FOREIGN KEY (servicio_id) REFERENCES servicio(id)
);

CREATE TABLE IF NOT EXISTS solicitud_servicio (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    usuario_id INTEGER NOT NULL,
    servicio_id INTEGER NOT NULL,
    urgencia TEXT NOT NULL CHECK(urgencia IN ('baja', 'media', 'alta', 'critica')),
    precio_final DECIMAL(10,2) NOT NULL,
    estado TEXT DEFAULT 'en_espera_de_pago' CHECK(estado IN ('pendiente', 'aceptado', 'terminado', 'cancelado', 'en_espera_de_pago')),
    latitud_usuario DECIMAL(10,8),
    longitud_usuario DECIMAL(11,8),
    fecha_creacion DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (usuario_id) REFERENCES usuario(id),
    FOREIGN KEY (servicio_id) REFERENCES servicio(id)
);

-- Datos de Prueba

INSERT INTO usuario (nombre, correo, contrasenna) VALUES 
('Juan Perez', 'juan@ejemplo.com', '123'),
('Maria Garcia', 'maria@ejemplo.com', '123'),
('Carlos Lopez', 'carlos@ejemplo.com', '123'),
('Ana Martinez', 'ana@ejemplo.com', '123');

INSERT INTO colaborador (usuario_id, telefono, sitio_web) VALUES 
(1, '555-0101', 'http://juanrepara.com'),
(2, '555-0102', 'http://mariaservicios.com'),
(3, '555-0103', NULL);

INSERT INTO categoria (nombre) VALUES 
('Fontaneria'),
('Plomeria'),
('Electricidad'),
('Pintar casas'),
('Electricista'),
('Limpieza');

-- Servicios para Juan (Colaborador 1) - Fontaneria y Plomeria
INSERT INTO servicio (colaborador_id, categoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud) VALUES 
(1, 1, 'Fontanero experto en fugas', 20.0, 5.0, 19.4326, -99.1332),
(1, 2, 'Plomeria general y destape de caños', 15.0, 6.0, 19.4326, -99.1332);

-- Servicios para Maria (Colaborador 2) - Electricidad y Limpieza
INSERT INTO servicio (colaborador_id, categoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud) VALUES 
(2, 3, 'Instalaciones electricas seguras', 30.0, 4.0, 19.4270, -99.1677),
(2, 6, 'Limpieza profunda de oficinas', 10.0, 10.0, 19.4270, -99.1677);

-- Servicios para Carlos (Colaborador 3) - Pintar casas y Electricista
INSERT INTO servicio (colaborador_id, categoria_id, descripcion, distancia_maxima_kilometros, precio_por_kilometro, latitud, longitud) VALUES 
(3, 4, 'Pintor de casas y fachadas', 25.0, 3.5, 19.4100, -99.1200),
(3, 5, 'Electricista certificado', 20.0, 7.0, 19.4100, -99.1200);

-- Precios de Urgencia para algunos servicios
INSERT INTO precio_servicio_urgencia (servicio_id, urgencia, precio) VALUES 
(1, 'baja', 200.0), (1, 'media', 300.0), (1, 'alta', 500.0), (1, 'critica', 800.0),
(3, 'baja', 150.0), (3, 'media', 250.0), (3, 'alta', 400.0), (3, 'critica', 700.0);
