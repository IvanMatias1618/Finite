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
  - **Lógica**: Valida al usuario y crea el perfil de colaborador con sus servicios asociados (vinculados a subcategorias).

### Consultar Perfil de Colaborador (`CasoUsoConsultarPerfilColaborador`)
- **`ejecutar`**:
  - **Parámetros**: `colaborador_id`.
  - **Lógica**: Obtiene los datos del colaborador (incluyendo foto, especialidad y estado de verificación), el nombre del usuario, la lista de servicios y el portafolio de trabajos realizados.

### Listar Colaboradores Marketplace (`CasoUsoListarColaboradoresMarketplace`)
- **`ejecutar`**:
  - **Parámetros**: `subcategoria_id`, `latitud`, `longitud`.
  - **Lógica**: 
    1. Busca todos los servicios vinculados a la subcategoría en el radio de cobertura.
    2. Cruza la información con el perfil Pro de los colaboradores.
    3. Calcula el "precio desde" (Urgencia Baja).
    4. Devuelve una lista ordenada por precio ascendente.

### Solicitud de Servicio (`CasoUsoSolicitudServicio`)
- **`crear_solicitud_directa`**:
  - **Parámetros**: `usuario_id`, `colaborador_id`, `subcategoria_id`, `urgencia`, `descripcion_detallada`, `fotos_evidencia_inicial`, `latitud`, `longitud`.
  - **Lógica**: 
    1. Valida que el colaborador ofrezca el servicio en la subcategoría indicada.
    2. Calcula el `precio_final = precio_base(urgencia) + (distancia * precio_por_km)`.
    3. Crea la solicitud en estado `PendienteDeRevision` adjuntando la evidencia del cliente.

- **`calcular_distancia_km`**: Implementa la fórmula de Haversine para determinar la distancia en kilómetros entre dos puntos geodésicos.

## Capa de Dominio (Puertos)

### Repositorio de Categorias (`RepositorioCategoria`)
- **`listar`**: Obtiene todas las categorias base.
- **`listar_subcategorias`**: Obtiene las subcategorias de una categoria específica.

### Repositorio de Servicios (`RepositorioServicio`)
- **`buscar_por_categoria_y_cercania`**: Filtra en base de datos aquellos servicios vinculados a la subcategoria cuyo radio de cobertura (`distancia_maxima_kilometros`) incluya la posición del usuario.
- **`buscar_por_colaborador`**: Obtiene todos los servicios registrados para un colaborador específico.
- **`buscar_precio_por_servicio_y_urgencia`**: Obtiene el costo base específico de un servicio para un nivel de urgencia dado.

## Capa de Infraestructura (API)

### Manejadores de Axum (`manejadores.rs`)
- **`registrar_colaborador`**: Punto de entrada HTTP que deserializa el JSON de entrada y delega la ejecución al caso de uso correspondiente.
- **`crear_solicitud`**: Recibe `subcategoria_id` y coordenadas para iniciar el proceso de matching.
