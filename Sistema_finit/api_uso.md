# Guía de Uso de la API - finit

Este documento es para que el desarrollador del Frontend pueda consumir los servicios del motor `finit`.

## Base URL
Por defecto local: `http://localhost:3000`

## Endpoints Principales

### 1. Estado de la API (Health Check)
**GET** `/`
- **Uso**: Verificar si el servidor está en línea.
- **Respuesta**: `"Bienvenido al motor finit - API activa (okupo.db)"`

### 2. Registro de Usuario
**POST** `/usuarios`
- **Cuerpo (JSON)**:
  ```json
  {
    "nombre": "Ivan",
    "correo": "ivan@test.com",
    "contrasenna": "password123"
  }
  ```
- **Respuesta**: El ID del usuario (ej: `1`).

### 3. Inicio de Sesion
**POST** `/login`
- **Uso**: Validar credenciales y obtener token JWT.
- **Cuerpo (JSON)**:
  ```json
  {
    "correo": "ivan@test.com",
    "contrasenna": "password123"
  }
  ```
- **Respuesta**: Token JWT (String).

### 4. Listar Categorías (Lazy Load)
**GET** `/categorias`
- **Uso**: Obtener todas las categorías base (sin subcategorías).
- **Respuesta (JSON)**:
  ```json
  [
    {
      "id": 1,
      "nombre": "Hogar",
      "subcategorias": null
    }
  ]
  ```

### 5. Listar Subcategorías por Categoría
**GET** `/categorias/:id/subcategorias`
- **Uso**: Obtener las subcategorías vinculadas a una categoría específica.
- **Respuesta (JSON)**:
  ```json
  [
    { "id": 1, "categoria_id": 1, "nombre": "Fontaneria", "descripcion": "Reparación de fugas" },
    { "id": 2, "categoria_id": 1, "nombre": "Electricidad", "descripcion": "Instalaciones eléctricas" }
  ]
  ```

### 6. Consultar Perfil de Colaborador (Perfil Pro)
**GET** `/colaboradores/:id`
- **Uso**: Obtener información pública detallada, servicios y portafolio.
- **Respuesta (JSON)**:
  ```json
  {
    "id": 1,
    "nombre": "Ivan",
    "telefono": "123456789",
    "sitio_web": "http://test.com",
    "foto_perfil": "url_foto.jpg",
    "especialidad_resumen": "Experto en fugas complejas",
    "es_verificado": true,
    "medio_transporte": "Camioneta",
    "rating_promedio": "4.8",
    "total_servicios": 120,
    "servicios": [...],
    "portafolio": [
      {
        "id": 1,
        "colaborador_id": 1,
        "foto_antes": "antes.jpg",
        "foto_despues": "despues.jpg",
        "descripcion": "Cambio de tubería principal"
      }
    ]
  }
  ```

### 7. Marketplace de Colaboradores
**GET** `/subcategorias/:id/colaboradores?latitud=19.4326&longitud=-99.1332`
- **Uso**: Listar profesionales cercanos a una ubicación, ordenados por precio base.
- **Respuesta (JSON)**:
  ```json
  [
    {
      "colaborador_id": 1,
      "nombre": "Ivan",
      "descripcion_servicio": "Servicio de fontanería profesional",
      "precio_base": "50.00",
      "distancia_km": "1.5"
    }
  ]
  ```

### 8. Registro de Colaborador
**POST** `/colaboradores`
- **Uso**: Convierte un usuario en colaborador con sus servicios iniciales.

### 9. Crear Solicitud de Servicio (Con Evidencia)
**POST** `/solicitudes`
- **Uso**: Crear una solicitud vinculada a un colaborador específico tras seleccionarlo en el Marketplace.
- **Cuerpo (JSON)**:
  ```json
  {
    "usuario_id": 1,
    "colaborador_id": 1,
    "subcategoria_id": 1,
    "urgencia": "alta",
    "descripcion_detallada": "Tubería rota en el baño principal, sale mucha agua.",
    "fotos_evidencia_inicial": "foto1.jpg,foto2.jpg",
    "latitud": 19.4326,
    "longitud": -99.1332
  }
  ```
- **Respuesta (JSON)**: La solicitud creada con estado `pendiente_de_revision`.

### 10. Listar Solicitudes
**GET** `/solicitudes`
- **Uso**: Listar todas las solicitudes del sistema.
- **Filtros (Query Params)**:
  - `usuario_id` (opcional): Filtrar por el ID de un usuario específico.
- **Ejemplo**: `/solicitudes?usuario_id=1`
### 10. Listar Solicitudes
...
  ```json
  [
    {
      "id": 1,
      "usuario_id": 1,
      "colaborador_id": 1,
      "subcategoria_id": 1,
      "servicio_id": 1,
      "urgencia": "media",
      "precio_final": "150.50",
      "estado": "pendiente_de_revision",
      "descripcion_detallada": "...",
      "fotos_evidencia_inicial": "...",
      "latitud_usuario": "19.4326",
      "longitud_usuario": "-99.1332",
      "fecha_creacion": "2023-10-27T10:00:00Z"
    }
  ]
  ```

### 11. Enviar Mensaje (Chat de Solicitud)
**POST** `/solicitudes/:id/mensajes`
- **Uso**: Enviar un mensaje de texto dentro de una solicitud activa.
- **Cuerpo (JSON)**:
  ```json
  {
    "emisor_id": 1,
    "contenido": "¿A qué hora podría pasar a revisar la tubería?"
  }
  ```
- **Respuesta (JSON)**: El mensaje creado con su ID y fecha de envío.

### 12. Listar Historial de Mensajes
**GET** `/solicitudes/:id/mensajes`
- **Uso**: Obtener todos los mensajes intercambiados en una solicitud, ordenados cronológicamente.
- **Respuesta (JSON)**:
  ```json
  [
    {
      "id": 1,
      "solicitud_id": 1,
      "emisor_id": 1,
      "contenido": "¿A qué hora podría pasar?",
      "fecha_envio": "2023-10-27T11:00:00Z"
    }
  ]
  ```

## Tipos de Datos y Formatos
- **Urgencia**: `"baja"`, `"media"`, `"alta"`, `"critica"`.
- **Precios/Coordenadas**: En las respuestas se devuelven como Strings para mantener precisión decimal de `rust_decimal`.
