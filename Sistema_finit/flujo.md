# Flujo de Datos - finit

Este documento describe cómo viaja la información a través de las capas del sistema durante los procesos principales.

## Flujo 1: Registro de un Nuevo Usuario
1. **API (HTTP POST)**: El cliente envía un JSON a `/usuarios` con `nombre`, `correo` y `contrasenna`.
2. **Manejador (`registrar_usuario`)**: Convierte el JSON en estructura de dominio.
3. **Caso de Uso (`ejecutar`)**:
   - Valida que el correo no esté duplicado.
   - Aplica Hash a la contrasenna.
   - Llama al `RepositorioUsuario` para persistir los datos.
4. **Respuesta**: Devuelve el ID del usuario creado.

## Flujo 2: Registro y Gestión de Colaborador (Modular)

### Etapa 0: Registro Base
1. **API (HTTP POST)**: Se envía JSON a `/colaboradores` con `token_usuario` (JWT).
2. **Caso de Uso**: Decodifica el token con el `jwt_secret` e identifica al usuario. Crea el perfil Pro.

### Etapa 1: Documentación e Identidad
1. **API (HTTP POST)**: Se envía JSON a `/colaboradores/:id/documentacion`.
2. **Caso de Uso**: Actualiza el perfil y asegura que el `estado_verificacion` sea `Pendiente`.

### Etapa 2: Gestión de Portafolio y Servicios
1. **API (HTTP POST)**: Se envía JSON a `/colaboradores/:id/portafolio`.
2. **Caso de Uso**: Registra evidencia de trabajos previos.
3. **API (HTTP POST)**: Se envía JSON a `/tecnico/servicios`.
4. **Caso de Uso**: Añade categorías y precios específicos para el marketplace.

### Etapa 3: Estadísticas del Panel
1. **API (HTTP GET)**: Se consulta `/colaboradores/:id/estadisticas`.
2. **Caso de Uso**: Calcula ganancias, rating y servicios en tiempo real desde los repositorios.

## Flujo 3: Solicitud de Servicio con Emparejamiento Geográfico
1. **Entrada**: El usuario solicita un servicio (ej. "Electricista") proporcionando sus coordenadas actuales.
2. **Filtrado Espacial**: El sistema consulta a MySQL todos los servicios de la categoría donde:
   `Distancia(Usuario, Servicio) <= Servicio.distancia_maxima`.
3. **Optimización de Precios (Matching)**:
   - Para cada servicio encontrado, el sistema calcula el precio final considerando la distancia y la urgencia.
   - Se selecciona el servicio con el `precio_final` más bajo.
4. **Persistencia**: Se crea un registro en `solicitud_servicio` con estado `PendienteDeRevision`.
5. **Retorno**: El sistema devuelve al usuario la solicitud creada.

## Flujo de Estados de Solicitud
`PendienteDeRevision` -> `AceptadoPorColaborador` -> `CitaProgramada` -> `Terminado` -> `Calificado` (Post-resennia).
*Nota: La transición a `Terminado` habilita el endpoint de calificaciones.*
