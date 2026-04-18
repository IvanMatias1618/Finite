# Detalles Técnicos y Configuraciones (Etc) - finit

Este documento recopila configuraciones específicas, dependencias y detalles de implementación técnica organizados por su ubicación o ámbito de uso.

## Raíz del Proyecto (Configuración)

### `Cargo.toml` (Dependencias Críticas)
- **`sqlx`**: Se utiliza la versión `0.7` con las siguientes features:
  - `runtime-tokio-rustls`: Para soporte asíncrono con Tokio.
  - `mysql`: Driver específico para la base de datos de producción.
  - `sqlite`: Implementado como motor de respaldo para pruebas rápidas y demostraciones locales sin dependencias externas.
  - `chrono`: Para mapear tipos de fecha/hora.
  - `rust_decimal`: Usado para el manejo de tipos monetarios y coordenadas sin pérdida de precisión.
- **`rust_decimal`**: Configurado con la feature `serde-float`.
- **`async-trait`**: Para compatibilidad `dyn` en traits de repositorios.

### `docker-compose.yml` (Entorno de Datos)
- **Imagen**: `mysql:8.0`. Fundamental para funciones geoespaciales.

## Carpeta: `infraestructura/`

### `sqlite_repositorio.rs` (Capa de Respaldo)
- Se utiliza `SqlitePool` para gestionar conexiones en memoria.
- Implementa los mismos traits de dominio, permitiendo la inyección de dependencias en `main.rs`.
- `inicializar_tablas()` crea el esquema dinámicamente al arrancar.

### `esquema.sql` (Base de Datos)
- **Tipos ENUM**: Se utilizan `ENUM` nativos en MySQL. En SQLite, estos se mapean como TEXT con validación en la capa de aplicación.

## Carpeta: `src/dominio/`

### `urgencia.rs` y `solicitud.rs`
- **Mapeo Serde**: Se utiliza `#[serde(rename_all = "lowercase")]` para que la API sea amigable con el frontend, permitiendo enviar valores en minúsculas (ej: "baja", "alta").

## Carpeta: `src/aplicacion/`

### `solicitud_servicio.rs` (Constantes Físicas)
- **Radio de la Tierra**: Se utiliza la constante `6371.0` km para el cálculo de Haversine.
