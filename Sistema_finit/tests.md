# Estrategia de Pruebas - finit

Este documento detalla el sistema de validación y pruebas del proyecto.

## Infraestructura de Pruebas

Se utiliza el framework nativo de Rust (`cargo test`) junto con **Tokio** para la ejecución de pruebas asíncronas.

### Pruebas con Tablas Temporales
Para evitar la dependencia de un estado persistente en la base de datos y permitir pruebas aisladas, se ha implementado una estrategia basada en **Tablas Temporales de MySQL**:
- **Archivo**: `tests/pruebas_temporales.rs`
- **Funcionamiento**: Cada prueba crea sus propias tablas dentro de la sesión de base de datos. Al finalizar la conexión, MySQL elimina automáticamente estas tablas, garantizando que una prueba no afecte a la siguiente.
- **Ventaja**: Permite probar la lógica de los repositorios sin necesidad de limpiar manualmente la base de datos real.

## Tipos de Pruebas Implementadas

1. **Pruebas de Integración de Infraestructura**:
   - Validan la conexión con MySQL y la ejecución de consultas dinámicas.
   - Verifican el mapeo de estructuras (FromRow) y tipos complejos (Decimal, Enums).

2. **Pruebas de Lógica Geográfica (Próximamente)**:
   - Validación del cálculo de distancia Haversine en el caso de uso de solicitud de servicio.

## Comandos Útiles

- `cargo test`: Ejecuta todas las pruebas del proyecto.
- `cargo test -- --nocapture`: Ejecuta las pruebas mostrando los mensajes de log (`println!`).

## Notas de Diseño
- **Inyección de Dependencias**: Gracias al uso de `Arc<dyn Trait>`, podemos inyectar repositorios falsos (Mocks) en el futuro para probar la capa de aplicación sin tocar la base de datos.
