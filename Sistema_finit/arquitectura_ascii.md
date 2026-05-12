# 🏗️ Arquitectura del Sistema: Okupo + Finite

Este documento describe la interacción entre el Frontend (Okupo) y el Motor de Servicios (Finite).

## 📊 Diagrama de Módulos y Flujo

```ascii
+---------------------------------------+      +------------------------------------------+
|          APLICACIÓN OKUPO             |      |              MOTOR FINITE                |
|       (Flask / Jinja2 / Python)       |      |           (Axum / Rust / MySQL)          |
+---------------------------------------+      +------------------------------------------+
|                                       |      |                                          |
|  [ WEB INTERFACE ]                    |      |  [ API REST (Handlers) ]                 |
|  - Rutas: Autenticación, Pedidos      | <--> |  - Auth, Catálogos, Solicitudes          |
|  - Templates: Dashboard, Marketplace   |      |  - Middleware: Validación JWT             |
|                                       |      |                                          |
|  [ CLIENTE API ]                      |      |  [ CASOS DE USO (Aplicación) ]           |
|  - cliente_api.py (Requests)          |      |  - RegistroUsuario, CotizarServicio      |
|                                       |      |  - GestionarVerificación, Mensajería     |
|                                       |      |                                          |
|  [ SESIÓN / ESTADO ]                  |      |  [ PUERTOS (Traits) ]                    |
|  - Token JWT en Session               |      |  - RepositorioUsuario, RepositorioColab  |
|                                       |      |                                          |
+---------------------------------------+      |  [ INFRAESTRUCTURA ]                     |
                                               |  - RepositorioMySQL (sqlx)               |
                                               |  - RepositorioSQLite                     |
                                               +------------------------------------------+
                                                                    |
                                                                    v
                                                       +--------------------------+
                                                       |      BASE DE DATOS       |
                                                       |     (MySQL / okupo.db)   |
                                                       +--------------------------+
```

## 🔄 Flujo de Datos Principal

1.  **Autenticación**:
    - `Okupo` envía credenciales a `POST /login`.
    - `Finite` valida contra la DB y devuelve un **JWT**.
    - `Okupo` almacena el JWT en la sesión del servidor para futuras peticiones.

2.  **Marketplace**:
    - `Okupo` solicita categorías y subcategorías.
    - `Okupo` envía coordenadas a `GET /subcategorias/:id/colaboradores`.
    - `Finite` realiza el cálculo de distancia geográfica y devuelve técnicos ordenados.

3.  **Contratación**:
    - `Okupo` solicita cotización a `POST /cotizar`.
    - `Finite` aplica reglas de negocio (precios dinámicos, urgencia).
    - El cliente confirma y se crea una `SolicitudServicio`.

4.  **Seguridad de Administración**:
    - Todas las peticiones a `/admin/*` y verificaciones requieren el JWT.
    - El Middleware en `Finite` decodifica los `Claims` para asegurar la identidad.
