# Estructura del Sistema - finit

Este documento describe la organización de archivos, carpetas y estructuras del proyecto siguiendo los principios de **Arquitectura Limpia (Clean Architecture)**.

## Directorios y Archivos

- `infraestructura/`: Contiene archivos externos al código Rust que definen el entorno.
  - `esquema.sql`: Definición de la base de datos MySQL (Tablas, ENUMs, Relaciones).
- `src/`: Raíz del código fuente.
  - `main.rs`: Punto de entrada de la aplicación. Configura la base de datos y el servidor web.
  - `dominio/`: El núcleo del negocio. No tiene dependencias externas.
    - `usuario.rs`: Estructura `Usuario`. Representa tanto a clientes como a colaboradores.
    - `colaborador.rs`: Estructuras `Colaborador`, `PerfilColaborador` y `TrabajoPortafolio`. Datos profesionales, verificación y evidencias de trabajos anteriores (antes/después).
    - `categoria.rs`: Estructuras `Categoria` y `Subcategoria`. Clasificación jerárquica de los servicios (Lazy Load).
    - `servicio.rs`: Estructura `Servicio`. Define qué se ofrece, ubicación y cobertura.
    - `solicitud.rs`: Estructura `SolicitudServicio` y `EstadoSolicitud`. Ciclo de vida con estados Pro (`PendienteDeRevision`, `AceptadoPorColaborador`, `CitaProgramada`) y soporte para fotos de evidencia inicial.
    - `urgencia.rs`: ENUM `Urgencia`. Define los niveles de prioridad del servicio.
    - `puertos/`: **Interfaces (Traits)**. Definen qué puede hacer el sistema sin decir cómo.
      - `repositorio_usuario.rs`: Trait `RepositorioUsuario`.
      - `repositorio_colaborador.rs`: Trait `RepositorioColaborador`.
      - `repositorio_servicio.rs`: Trait `RepositorioServicio`.
      - `repositorio_solicitud.rs`: Trait `RepositorioSolicitud`.
  - `aplicacion/`: Orquestación del negocio. Implementa los "Casos de Uso".
    - `servicios/`: Lógica de procesos complejos.
      - `registro_colaborador.rs`: Lógica para convertir un usuario en colaborador con sus servicios.
      - `consultar_perfil_colaborador.rs`: Lógica para obtener el perfil Pro de un colaborador con servicios y portafolio.
      - `listar_colaboradores_marketplace.rs`: Lógica para buscar y filtrar profesionales cercanos.
      - `solicitud_servicio.rs`: Creación de solicitudes con evidencia y cálculo de precios geolocalizados.
  - `infraestructura/`: Implementación de detalles técnicos y dependencias externas.
    - `mod.rs`: Definición de la estructura `RepositorioMySQL`.
    - `mysql_repositorio_*.rs`: Implementaciones concretas para producción.
    - `sqlite_repositorio.rs`: Implementación de respaldo para pruebas locales rápidas sin base de datos externa.
    - `api/`: Exposición del sistema mediante protocolo HTTP.
      - `rutas.rs`: Definición de endpoints y estructura `EstadoApp` para inyección de dependencias.
      - `manejadores.rs`: Lógica de entrada/salida para las peticiones HTTP (Axum).
- `tests/`: Pruebas de integración y validación del sistema.
  - `colaborador_test.rs`: Suite de pruebas para el perfil del colaborador.
  - `navegacion_test.rs`: Suite de pruebas para categorias y subcategorias.

## Estructuras y Parámetros Clave

### Entidades de Dominio
- **Servicio**: Incluye `latitud`, `longitud`, `subcategoria_id` y `distancia_maxima_kilometros`. Se decidió colocar las coordenadas en el servicio para permitir que un mismo colaborador ofrezca servicios en puntos geográficos distintos (ej. dos locales comerciales).
- **SolicitudServicio**: Incluye `precio_final` calculado en tiempo de emparejamiento.

### Justificación de Diseño
- **Inversión de Dependencias**: La capa de `aplicacion` depende de `puertos` (traits), no de la implementación de MySQL. Esto permite cambiar la base de datos sin tocar la lógica del negocio.
- **Ubicación Geográfica**: Se utiliza el tipo `Decimal` para precisión financiera y de coordenadas, convirtiéndose a `f64` solo para cálculos trigonométricos (Haversine).
- **Matching Dinámico**: No se asigna un colaborador fijo de inmediato en el esquema si no que se busca el "mejor" según distancia y precio de urgencia en el momento de la solicitud.
