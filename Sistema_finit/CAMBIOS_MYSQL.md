# Documentación Técnica: Sistema Finit/Okupo

## 📅 Fecha de Actualización: 22 de Abril de 2026

## 🛠️ Transición a MySQL
Se ha completado la migración del backend de SQLite a MySQL 8.0 para mejorar la escalabilidad y concurrencia del motor.

### Cambios en Infraestructura
- **Base de Datos**: Ahora se ejecuta en un contenedor Docker (`mysql:8.0`).
- **Conector**: Se ha sustituido `sqlx-sqlite` por `sqlx-mysql` en el backend de Rust.
- **Esquema**: Se han normalizado los tipos de datos (e.g., `VARCHAR(255)`, `DECIMAL(10,2)` para precios y coordenadas).

### Implementación de Repositorios (Traits)
Se han creado los siguientes módulos en `src/infraestructura/`:
- `mysql_repositorio_usuario`: Manejo de perfiles y contraseñas.
- `mysql_repositorio_categoria`: Gestión de categorías y subcategorías semilla.
- `mysql_repositorio_colaborador`: Registro de prestadores de servicios.
- `mysql_repositorio_servicio`: Definición de precios y geolocalización.
- `mysql_repositorio_solicitud`: Flujo de órdenes de servicio.
- `mysql_repositorio_mensaje`: Historial de chat en tiempo real.

## 🚀 Configuración del Entorno (Local)
1. **Docker**: `sudo docker-compose up -d db`.
2. **Backend**:
   - Variables en `.env`: `DATABASE_URL=mysql://ivan:password@localhost:3306/finit`.
   - Ejecutar: `cargo run`.
3. **Frontend**:
   - Variables en `main.py`: `API_BASE_URL = "http://localhost:3000"`.
   - Ejecutar: `source venv/bin/activate && python main.py`.

## 📍 Estado de Estabilidad
- El flujo de **Pruebas E2E** (`pruebas_e2e.py`) pasa con éxito en todos los módulos críticos.
- El frontend está correctamente vinculado y las sesiones se manejan mediante JWT extraídos del backend.

---

## 📅 Fecha de Actualización: 11 de Mayo de 2026

### Optimización de Documentación y Carga de Archivos
Se han realizado mejoras críticas para soportar el flujo de registro de técnicos con fotografías de alta calidad.

- **Migración a LONGTEXT**: Las columnas `ine_frontal`, `ine_trasera`, `comprobante_domicilio` y `foto_selfie_ine` en la tabla `colaborador` han sido migradas de `TEXT` a `LONGTEXT`. Esto permite almacenar imágenes en formato Base64 de gran tamaño (anteriormente limitadas a 64KB).
- **Aumento de Límites de API**: Se ha configurado `DefaultBodyLimit` en el servidor Axum a **20MB**. Esto garantiza que las peticiones POST con múltiples imágenes pesadas no sean rechazadas por el servidor.
- **Actualización de Roles Automática**: El registro de un colaborador ahora actualiza automáticamente el `rol` del usuario a `colaborador` en la tabla `usuario`, eliminando la necesidad de actualizaciones manuales para habilitar el dashboard técnico.

