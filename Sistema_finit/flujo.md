# Flujo de Datos - finit

Este documento describe cómo viaja la información a través de las capas del sistema durante los procesos principales.

## Flujo 1: Registro de un Nuevo Usuario
1. **API (HTTP POST)**: El cliente envía un JSON a `/usuarios` con `nombre`, `correo` y `contrasenna`.
2. **Manejador (`registrar_usuario`)**: Convierte el JSON en estructura de dominio.
3. **Caso de Uso (`ejecutar`)**:
   - Valida que el correo no esté duplicado.
   - Llama al `RepositorioUsuario` para persistir los datos.
4. **Respuesta**: Devuelve el ID del usuario creado.

## Flujo 2: Registro de un Nuevo Colaborador
1. **API (HTTP POST)**: El cliente envía un JSON a `/colaboradores` con un `token_usuario` (ID temporal).
2. **Manejador (`registrar_colaborador`)**: Delega al caso de uso.
3. **Caso de Uso (`ejecutar`)**:
   - Valida la existencia del usuario mediante el `token_usuario`.
   - Crea el perfil profesional en `RepositorioColaborador`.
   - Registra los servicios y sus precios.
4. **Infraestructura (SQLite/MySQL)**: Persistencia en base de datos.

## Flujo 3: Solicitud de Servicio con Emparejamiento Geográfico
1. **Entrada**: El usuario solicita un servicio (ej. "Electricista") proporcionando sus coordenadas actuales.
2. **Filtrado Espacial**: El sistema consulta a MySQL todos los servicios de la categoría donde:
   `Distancia(Usuario, Servicio) <= Servicio.distancia_maxima`.
3. **Optimización de Precios (Matching)**:
   - Para cada servicio encontrado, el sistema calcula el precio final considerando la distancia y la urgencia (Baja, Media, Alta, Critica).
   - Se selecciona el servicio con el `precio_final` más bajo.
4. **Persistencia**: Se crea un registro en `solicitud_servicio` con estado `en_espera_de_pago`.
5. **Retorno**: El sistema devuelve al usuario la solicitud creada con el precio calculado para que proceda al pago.

## Flujo de Estados de Solicitud
`EnEsperaDePago` -> `Pendiente` (Pagado) -> `Aceptado` (Por colaborador) -> `Terminado` -> `Calificado` (Post-resennia).
*Nota: Actualmente implementado hasta la creación en espera de pago.*
