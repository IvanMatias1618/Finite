-- 1. Actualizar tabla de usuarios (Añadir contrasenna)
ALTER TABLE usuario ADD COLUMN contrasenna VARCHAR(255) NOT NULL;

-- 2. Actualizar tabla de colaboradores (Documentación y Verificación)
ALTER TABLE colaborador 
    ADD COLUMN foto_perfil VARCHAR(255),
    ADD COLUMN especialidad_resumen TEXT,
    ADD COLUMN es_verificado BOOLEAN DEFAULT FALSE,
    ADD COLUMN estado_verificacion ENUM('pendiente', 'verificado', 'rechazado') DEFAULT 'pendiente',
    ADD COLUMN ine_frontal VARCHAR(255),
    ADD COLUMN ine_trasera VARCHAR(255),
    ADD COLUMN comprobante_domicilio VARCHAR(255),
    ADD COLUMN foto_selfie_ine VARCHAR(255),
    ADD COLUMN medio_transporte VARCHAR(50),
    ADD COLUMN rating_promedio DECIMAL(3,2) DEFAULT 0.00,
    ADD COLUMN total_servicios INT DEFAULT 0;

-- 3. Crear tabla de disponibilidad (Formato nuevo compatible con dominio/disponibilidad.rs)
CREATE TABLE IF NOT EXISTS disponibilidad_colaborador (
    id INT AUTO_INCREMENT PRIMARY KEY,
    colaborador_id INT NOT NULL,
    dia_semana TINYINT NOT NULL, -- 0-6 (Domingo-Sábado)
    hora_inicio TIME NOT NULL,
    hora_fin TIME NOT NULL,
    activo BOOLEAN DEFAULT TRUE,
    FOREIGN KEY (colaborador_id) REFERENCES colaborador(id)
);

-- 4. Crear tabla de configuración de precios dinámicos
CREATE TABLE IF NOT EXISTS configuracion_precios_colaborador (
    colaborador_id INT PRIMARY KEY,
    precio_por_kilometro DECIMAL(10,2) DEFAULT 0.00,
    recargo_lluvia DECIMAL(5,2) DEFAULT 0.00,
    recargo_domingo DECIMAL(5,2) DEFAULT 0.00,
    recargo_nocturno DECIMAL(5,2) DEFAULT 0.00,
    FOREIGN KEY (colaborador_id) REFERENCES colaborador(id)
);

-- 5. Actualizar solicitudes para incluir detalles multimedia
ALTER TABLE solicitud_servicio 
    ADD COLUMN descripcion_detallada TEXT,
    ADD COLUMN fotos_evidencia_inicial TEXT; -- Rutas separadas por comas o JSON
