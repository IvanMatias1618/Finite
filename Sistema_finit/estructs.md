# Estructura del Sistema - finit

Este documento describe la organización de archivos, carpetas y estructuras del proyecto siguiendo los principios de **Arquitectura Limpia (Clean Architecture)**.

## Directorios y Archivos

- `infraestructura/`: Contiene archivos externos al código Rust que definen el entorno.
  - `esquema.sql`: Definición de la base de datos MySQL (Tablas, ENUMs, Relaciones).
- `src/`: Raíz del código fuente.
  - `main.rs`: Punto de entrada de la aplicación. Configura la base de datos (MySQL), carga el `JWT_SECRET` y arranca el servidor Axum.
  - `dominio/`: El núcleo del negocio. No tiene dependencias externas.
    - `usuario.rs`: Estructura `Usuario`.
    - `colaborador.rs`: Estructuras `Colaborador`, `PerfilColaborador`, `TrabajoPortafolio`, `EstadoVerificacion` y `ResumenEstadisticasColaborador`.
    - `disponibilidad.rs`: Estructuras `Disponibilidad` y `HorarioSemanal`.
    - `configuracion_precio.rs`: Estructura `ConfiguracionPrecio`.
    - `categoria.rs`: Estructuras `Categoria` y `Subcategoria`.
    - `servicio.rs`: Estructura `Servicio` y `PrecioServicioUrgencia`.
    - `solicitud.rs`: Estructura `SolicitudServicio` y `EstadoSolicitud` (Deriva `PartialEq`).
    - `mensaje.rs`: Estructura `MensajeSolicitud`.
    - `resennia.rs`: Estructura `Resennia`.
    - `urgencia.rs`: ENUM `Urgencia`.
    - `puertos/`: **Interfaces (Traits)**.
      - `repositorio_usuario.rs`
      - `repositorio_colaborador.rs`: Incluye `eliminar_trabajo_portafolio` y `obtener_estadisticas`.
      - `repositorio_servicio.rs`
      - `repositorio_solicitud.rs`
      - `repositorio_mensaje.rs`
      - `repositorio_disponibilidad.rs`
      - `repositorio_configuracion_precio.rs`
      - `repositorio_resennia.rs`
  - `aplicacion/`: Orquestación del negocio. Implementa los "Casos de Uso".
    - `servicios/`:
      - `registro_colaborador.rs`: Registro con validación de JWT.
      - `actualizar_documentacion.rs`
      - `configurar_precios_dinamicos.rs`
      - `configurar_horarios.rs`
      - `calificar_servicio.rs`: Validación de estado `Terminado` y unicidad.
      - `consultar_perfil_colaborador.rs`
      - `consultar_estadisticas_colaborador.rs`: Dashboard del técnico.
      - `registrar_servicio_tecnico.rs`: Adición de servicios individuales.
      - `gestionar_portafolio.rs`: Alta y baja de trabajos realizados.
      - `listar_colaboradores_marketplace.rs`
      - `solicitud_servicio.rs`: Cálculo de precio final (Base + Distancia).
      - `gestionar_mensajes.rs`
  - `infraestructura/`: Implementación de detalles técnicos.
    - `mod.rs`: Estructura `RepositorioMySQL`.
    - `mysql_repositorio_*.rs`: Implementaciones para producción.
    - `sqlite_repositorio.rs`: **Repositorio de Pruebas**. Implementación completa de todos los puertos para ejecución de tests en memoria.
    - `api/`:
      - `rutas.rs`: Rutas unificadas y `EstadoApp`.
      - `manejadores.rs`: Handlers de Axum con deserialización segura.
- `tests/`: Pruebas de integración.
  - `colaborador_test.rs`, `identidad_test.rs`, `marketplace_test.rs`, `navegacion_test.rs`, `solicitud_test.rs`.

## Estructuras y Parámetros Clave

### Entidades de Dominio
- **Colaborador**: Incluye campos de documentación (`ine_frontal`, `ine_trasera`, `comprobante_domicilio`, `foto_selfie_ine`) almacenados como `LONGTEXT` para soportar Base64.
- **EstadoSolicitud**: Ahora implementa `PartialEq` para permitir validaciones lógicas en la capa de aplicación.
- **ResumenEstadisticasColaborador**: Agrupa `total_servicios`, `rating_promedio`, `ganancias_totales` y `servicios_pendientes`.

### Justificación de Diseño
- **Inyección de Secretos**: El `jwt_secret` se inyecta en los casos de uso que lo requieren desde `main.rs`, permitiendo una rotación segura de llaves sin cambiar la lógica.
- **Dualidad de Repositorios**: Se mantiene `sqlite_repositorio.rs` para garantizar que la suite de pruebas sea rápida, aislada y no requiera un servidor MySQL activo, mientras que `mysql_repositorio_*.rs` se usa para la operación real.
