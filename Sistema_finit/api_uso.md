# Guía de Uso de la API - finit

Este documento es para que el desarrollador del Frontend pueda consumir los servicios del motor `finit`.

## Base URL
Por defecto local: `http://localhost:3000`

## Seguridad (JWT)
La mayoría de los endpoints protegidos requieren un encabezado de autorización:
`Authorization: Bearer <TOKEN>`

---

## Endpoints Principales

### 1. Estado de la API (Health Check)
**GET** `/`
- **Uso**: Verificar si el servidor está en línea.
- **Respuesta**: `"Bienvenido al motor finit - API activa (okupo.db)"`

### 2. Autenticación y Usuarios
- **POST** `/usuarios`: Registro de nuevo usuario.
- **POST** `/login`: Obtiene el token JWT.

---

### 3. Marketplace y Catálogos
- **GET** `/categorias`: Listar categorías base.
- **GET** `/categorias/:id/subcategorias`: Listar subcategorías.
- **GET** `/subcategorias/:id/colaboradores?latitud=X&longitud=Y`: Listar profesionales activos y cercanos (ordenados por cercanía y precio).
- **POST** `/cotizar`: Calcula el precio total (base + distancia + recargos nocturnos/domingo).
  - **Cuerpo**: `{"colaborador_id": 1, "subcategoria_id": 2, "urgencia": "alta", "latitud": 19.4, "longitud": -99.1}`

---

### 4. Gestión de Colaboradores (Técnicos)
- **GET** `/colaboradores/:id`: Perfil público.
- **GET** `/colaboradores/:id/estadisticas`: Métricas del dashboard.
- **POST** `/colaboradores/:id/documentacion`: Subir fotografías de identidad en Base64.
  - **Cuerpo**: `{"ine_frontal": "data:image/...", "ine_trasera": "...", "comprobante_domicilio": "...", "foto_selfie_ine": "..."}`
  - **Nota**: El servidor soporta hasta **20MB** por petición para permitir imágenes de alta resolución.
- **POST** `/colaboradores/:id/horarios`: Configurar disponibilidad semanal.
- **POST** `/colaboradores/:id/precios-dinamicos`: Configurar recargos por distancia/clima/hora.
- **POST** `/tecnico/servicios`: Registrar un nuevo servicio ofrecido.

---

### 5. Ciclo de Vida de Solicitudes
- **POST** `/solicitudes`: Crear nueva solicitud.
- **GET** `/solicitudes?usuario_id=X`: Listar mis solicitudes.
- **Acciones de Estado (Protegidas)**:
  - **POST** `/solicitudes/:id/aceptar`: El técnico acepta el trabajo.
  - **POST** `/solicitudes/:id/finalizar`: El técnico marca el trabajo como terminado.
  - **POST** `/solicitudes/:id/cancelar`: Cualquiera de las partes cancela la solicitud.

---

### 6. Administración (Panel de Control Tauri)
- **GET** `/admin/colaboradores/pendientes`: Lista colaboradores en espera de validación.
- **POST** `/colaboradores/:id/verificar`: Aprobar o rechazar a un colaborador.
  - **Cuerpo**: `{"estado": "verificado", "comentario": "Documentación correcta"}`

---

### 7. Comunicación y Multimedia (Próximamente)
- **POST** `/solicitudes/:id/mensajes`: Chat entre partes.
- **GET** `/solicitudes/:id/mensajes`: Historial de chat.
- **POST** `/calificaciones`: Calificar servicio terminado.

## Tipos de Datos y Formatos
- **Urgencia**: `"baja"`, `"media"`, `"alta"`, `"critica"`.
- **Estado de Verificación**: `"pendiente"`, `"verificado"`, `"rechazado"`.
- **Precios/Coordenadas**: Devueltos como Strings para precisión decimal.
