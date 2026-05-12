# Funciones y Lógica de Negocio - finit

Este documento detalla las funciones principales del sistema, agrupadas por su responsabilidad.

## Capa de Aplicación (Casos de Uso)

### Registro de Usuario (`CasoUsoRegistroUsuario`)
- **`ejecutar`**: Valida unicidad de correo y aplica hash a la contrasenna (bcrypt).

### Registro de Colaborador (`CasoUsoRegistroColaborador`)
- **`ejecutar`**: Decodifica el `token_usuario` usando el `jwt_secret` para identificar al usuario, crea el perfil Pro y registra los servicios iniciales.

### Gestión de Portafolio (`CasoUsoGestionarPortafolio`)
- **`annadir_trabajo`**: Registra fotos "antes/después" y descripción de un trabajo realizado.
- **`eliminar_trabajo`**: Elimina un registro del portafolio por su ID.

### Registrar Servicio Técnico (`CasoUsoRegistrarServicioTecnico`)
- **`ejecutar`**: Permite al colaborador añadir nuevos servicios de forma individual, vinculándolos a subcategorías y definiendo sus precios por urgencia.

### Consultar Estadísticas (`CasoUsoConsultarEstadisticasColaborador`)
- **`ejecutar`**: Calcula el resumen de actividad (ganancias, servicios terminados, rating y pendientes).

### Calificar Servicio (`CasoUsoCalificarServicio`)
- **`ejecutar`**: 
  1. Verifica que la solicitud exista y esté en estado `Terminado`.
  2. Valida que no exista una calificación previa.
  3. Registra la reseña y comentario.

### Actualizar Documentación (`CasoUsoActualizarDocumentacion`)
- **`ejecutar`**: Actualiza enlaces de INE y comprobante. Reinicia el estado a `Pendiente` de verificación humana.

### Configurar Precios Dinámicos (`CasoUsoConfigurarPreciosDinamicos`)
- **`ejecutar`**: Define recargos por lluvia, domingo y noche.

### Configurar Horarios (`CasoUsoConfigurarHorarios`)
- **`ejecutar`**: Actualiza la disponibilidad semanal (limpia registros anteriores y crea los nuevos).

### Listar Colaboradores Marketplace (`CasoUsoListarColaboradoresMarketplace`)
- **`ejecutar`**: Filtra profesionales por cercanía geográfica y devuelve el precio base de urgencia baja como referencia.

### Solicitud de Servicio (`CasoUsoSolicitudServicio`)
- **`crear_solicitud_directa`**: 
  1. Identifica el servicio del colaborador.
  2. Calcula precio: `Base(Urgencia) + (Distancia * Precio_KM)`.
  3. Persiste en estado `PendienteDeRevision`.

## Capa de Dominio (Puertos)

### Repositorio de Colaboradores (`RepositorioColaborador`)
- **`obtener_estadisticas`**: Consulta agregada de servicios y ganancias.
- **`eliminar_trabajo_portafolio`**: Eliminación física del registro.

### Repositorio de Solicitudes (`RepositorioSolicitud`)
- **`buscar_por_id`**: Recuperación completa de una solicitud (implementado en MySQL y SQLite).
- **`actualizar_estado`**: Cambio en el ciclo de vida de la orden.

## Capa de Infraestructura (API)

### Manejadores de Axum (`manejadores.rs`)
- Se encargan de la deserialización de JSON, extracción de parámetros de ruta (`Path`) y query, y el manejo de errores del sistema transformándolos en códigos de estado HTTP adecuados.
- Implementan `axum::debug_handler` para facilitar la depuración de tipos en tiempo de compilación.
