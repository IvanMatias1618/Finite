# Flujo de Datos - finit

Este documento describe cómo viaja la información a través de las capas del sistema durante los procesos principales.

## Flujo 1: Registro de un Nuevo Colaborador
1. **API (HTTP POST)**: El cliente envía un JSON a `/colaboradores`.
2. **Manejador (`registrar_colaborador`)**: Convierte el JSON en estructuras de dominio.
3. **Caso de Uso (`ejecutar`)**:
   - Llama al `RepositorioUsuario` para asegurar la existencia del usuario base.
   - Llama al `RepositorioColaborador` para crear el perfil profesional.
   - Itera sobre los servicios, llamando al `RepositorioServicio` para guardar cada uno y sus precios por urgencia.
4. **Infraestructura (MySQL)**: Se ejecutan múltiples `INSERT` dentro de una transacción (pendiente de implementar formalmente) para asegurar la integridad.

## Flujo 2: Solicitud de Servicio con Emparejamiento Geográfico
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
