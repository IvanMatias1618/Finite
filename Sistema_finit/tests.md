# Estrategia de Pruebas - finit

Este documento detalla el sistema de validación y pruebas del proyecto.

## Infraestructura de Pruebas

Se utiliza el framework nativo de Rust (`cargo test`) junto con **Tokio** para la ejecución de pruebas asíncronas.

### Pruebas en Memoria (SQLite)
Para la validación de la lógica de negocio y flujos de casos de uso, se utiliza `sqlite_repositorio.rs`:
- **Ventaja**: Ejecución extremadamente rápida y sin dependencias externas (Docker/MySQL).
- **Implementación**: El `RepositorioSQLite` implementa todos los traits de dominio, permitiendo probar flujos como:
  - Registro y Login (con verificación de JWT).
  - Creación de solicitudes y cálculo de precios.
  - Gestión de portafolio y estadísticas.

### Pruebas de Integración (MySQL)
Para validar la persistencia real y tipos de datos específicos de producción:
- **Archivo**: `tests/pruebas_temporales.rs` (Tablas temporales de MySQL).
- **Pruebas E2E**: El script `pruebas_e2e.py` valida la API completa corriendo contra el servidor real y la base de datos MySQL.

## Tipos de Pruebas Implementadas

1. **Flujo de Identidad**: Registro de usuarios, detección de correos duplicados y validación de login con hash de contrasenna.
2. **Marketplace y Navegación**: Filtrado de colaboradores por subcategoría y ubicación.
3. **Solicitudes**: Creación de órdenes y listado por usuario.
4. **Perfil Pro**: Consulta de perfiles con servicios y portafolio integrados.

## Comandos Útiles

- `cargo test`: Ejecuta la suite de pruebas en memoria.
- `cargo test -- --nocapture`: Muestra logs y detalles de pánico.
- `python pruebas_e2e.py`: Ejecuta el flujo completo de negocio contra el sistema activo.

## Notas Técnicas
- **JWT en Tests**: Los tests utilizan un `jwt_secret` de prueba para validar la generación y decodificación de tokens en los flujos de registro de colaboradores.
- **Decimales**: Se valida la precisión de los precios y coordenadas en todas las capas del sistema.
