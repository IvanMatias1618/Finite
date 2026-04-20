# Funciones y Lógica de Negocio - finit

Este documento detalla las funciones principales del sistema, agrupadas por su responsabilidad.

## Capa de Aplicación (Casos de Uso)

### Registro de Usuario (`CasoUsoRegistroUsuario`)
- **`ejecutar`**:
  - **Parámetros**: `nombre`, `correo`, `contrasenna`.
  - **Lógica**: Valida unicidad del correo y persiste el nuevo usuario.

### Registro de Colaborador (`CasoUsoRegistroColaborador`)
- **`ejecutar`**:
  - **Parámetros**: `token_usuario`, `telefono`, `sitio_web`, `servicios`.
  - **Lógica**: Valida al usuario y crea el perfil de colaborador con sus servicios asociados.

### Consultar Perfil de Colaborador (`CasoUsoConsultarPerfilColaborador`)
- **`ejecutar`**:
  - **Parámetros**: `colaborador_id`.
  - **Lógica**: Obtiene los datos del colaborador, el nombre del usuario asociado y la lista completa de sus servicios.

### Solicitud de Servicio (`CasoUsoSolicitudServicio`)
- **`emparejar_y_solicitar`**:
  - **Parámetros**: `usuario_id`, `categoria_id`, `urgencia`, `latitud`, `longitud`.
  - **Lógica**: 
    1. Busca servicios cercanos mediante SQL (`buscar_por_categoria_y_cercania`).
    2. Itera sobre los candidatos y calcula la distancia física usando `calcular_distancia_km`.
    3. Recupera el precio base según la `urgencia` solicitada.
    4. Calcula el `precio_final = precio_urgencia + (distancia * precio_por_km)`.
    5. Elige la opción más económica y crea la solicitud en estado `EnEsperaDePago`.
- **`calcular_distancia_km`**: Implementa la fórmula de Haversine para determinar la distancia en kilómetros entre dos puntos geodésicos.

## Capa de Dominio (Puertos)

### Repositorio de Servicios (`RepositorioServicio`)
- **`buscar_por_categoria_y_cercania`**: Filtra en base de datos aquellos servicios cuyo radio de cobertura (`distancia_maxima_kilometros`) incluya la posición del usuario.
- **`buscar_por_colaborador`**: Obtiene todos los servicios registrados para un colaborador específico.
- **`buscar_precio_por_servicio_y_urgencia`**: Obtiene el costo base específico de un servicio para un nivel de urgencia dado.

## Capa de Infraestructura (API)

### Manejadores de Axum (`manejadores.rs`)
- **`registrar_colaborador`**: Punto de entrada HTTP que deserializa el JSON de entrada y delega la ejecución al caso de uso correspondiente. Maneja la conversión de errores de negocio a respuestas HTTP.
