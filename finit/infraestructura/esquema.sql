CREATE TABLE IF NOT EXISTS usuario (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    correo VARCHAR(150) UNIQUE NOT NULL,
    fecha_registro TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS colaborador (
    id INT AUTO_INCREMENT PRIMARY KEY,
    usuario_id INT NOT NULL,
    telefono VARCHAR(20),
    sitio_web VARCHAR(255),
    FOREIGN KEY (usuario_id) REFERENCES usuario(id)
);

CREATE TABLE IF NOT EXISTS categoria (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS servicio (
    id INT AUTO_INCREMENT PRIMARY KEY,
    colaborador_id INT NOT NULL,
    categoria_id INT NOT NULL,
    descripcion TEXT,
    distancia_maxima_kilometros DECIMAL(10,2),
    precio_por_kilometro DECIMAL(10,2),
    latitud DECIMAL(10,8),
    longitud DECIMAL(11,8),
    FOREIGN KEY (colaborador_id) REFERENCES colaborador(id),
    FOREIGN KEY (categoria_id) REFERENCES categoria(id)
);

CREATE TABLE IF NOT EXISTS precio_servicio_urgencia (
    id INT AUTO_INCREMENT PRIMARY KEY,
    servicio_id INT NOT NULL,
    urgencia ENUM('baja', 'media', 'alta', 'critica') NOT NULL,
    precio DECIMAL(10,2) NOT NULL,
    UNIQUE(servicio_id, urgencia),
    FOREIGN KEY (servicio_id) REFERENCES servicio(id)
);

CREATE TABLE IF NOT EXISTS agenda_colaborador (
    id INT AUTO_INCREMENT PRIMARY KEY,
    colaborador_id INT NOT NULL,
    dia_semana ENUM('lunes', 'martes', 'miercoles', 'jueves', 'viernes', 'sabado', 'domingo') NOT NULL,
    hora_inicio TIME NOT NULL,
    hora_fin TIME NOT NULL,
    FOREIGN KEY (colaborador_id) REFERENCES colaborador(id)
);

CREATE TABLE IF NOT EXISTS solicitud_servicio (
    id INT AUTO_INCREMENT PRIMARY KEY,
    usuario_id INT NOT NULL,
    servicio_id INT NOT NULL,
    urgencia ENUM('baja', 'media', 'alta', 'critica') NOT NULL,
    precio_final DECIMAL(10,2) NOT NULL,
    estado ENUM('pendiente', 'aceptado', 'terminado', 'cancelado', 'en_espera_de_pago') DEFAULT 'en_espera_de_pago',
    latitud_usuario DECIMAL(10,8),
    longitud_usuario DECIMAL(11,8),
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (usuario_id) REFERENCES usuario(id),
    FOREIGN KEY (servicio_id) REFERENCES servicio(id)
);

CREATE TABLE IF NOT EXISTS resennia (
    id INT AUTO_INCREMENT PRIMARY KEY,
    solicitud_id INT NOT NULL UNIQUE,
    calificacion TINYINT CHECK (calificacion BETWEEN 1 AND 5),
    comentario TEXT,
    fecha_creacion TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (solicitud_id) REFERENCES solicitud_servicio(id)
);
