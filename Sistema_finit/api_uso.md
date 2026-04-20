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

### 4. Listar Categorías y Subcategorías
**GET** `/categorias`
- **Uso**: Obtener todas las categorías y sus subcategorías anidadas.
- **Respuesta (JSON)**:
  ```json
  [
    {
      "id": 1,
      "nombre": "Hogar",
      "subcategorias": [
        { "id": 1, "categoria_id": 1, "nombre": "Fontaneria", "descripcion": "Reparación de fugas" },
        { "id": 2, "categoria_id": 1, "nombre": "Electricidad", "descripcion": "Instalaciones eléctricas" }
      ]
    }
  ]
  ```

### 5. Consultar Perfil de Colaborador
**GET** `/colaboradores/:id`
- **Uso**: Obtener información pública de un colaborador y sus servicios.
- **Respuesta (JSON)**:
  ```json
  {
    "id": 1,
    "nombre": "Ivan",
    "telefono": "123456789",
    "sitio_web": "http://test.com",
    "servicios": [
      {
        "id": 1,
        "colaborador_id": 1,
        "subcategoria_id": 1,
        "descripcion": "Servicio de fontanería",
        "distancia_maxima_kilometros": "10.0",
        "precio_por_kilometro": "5.5",
        "latitud": "19.4326",
        "longitud": "-99.1332"
      }
    ]
  }
  ```

### 6. Registro de Colaborador
**POST** `/colaboradores`
- **Uso**: Convierte un usuario en colaborador con sus servicios iniciales.
- **Cuerpo (JSON)**:
  ```json
  {
    "token_usuario": "JWT_AQUÍ",
    "telefono": "123456789",
    "sitio_web": "http://test.com",
    "servicios": [
      [
        {
          "colaborador_id": 0,
          "subcategoria_id": 1,
          "descripcion": "Descripción del servicio",
          "distancia_maxima_kilometros": 15.0,
          "precio_por_kilometro": 5.5,
          "latitud": 19.4326,
          "longitud": -99.1332
        },
        [
          { "servicio_id": 0, "urgencia": "baja", "precio": 100.0 },
          { "servicio_id": 0, "urgencia": "alta", "precio": 250.0 }
        ]
      ]
    ]
  }
  ```

### 7. Crear Solicitud de Servicio (Matching)
**POST** `/solicitudes`
- **Uso**: Busca el colaborador más económico en una subcategoría cercana y crea la solicitud.
- **Cuerpo (JSON)**:
  ```json
  {
    "usuario_id": 1,
    "subcategoria_id": 1,
    "urgencia": "media",
    "latitud": 19.4326,
    "longitud": -99.1332
  }
  ```
- **Respuesta (JSON)**: La solicitud creada con el `precio_final` calculado.

### 8. Listar Solicitudes
**GET** `/solicitudes`
- **Uso**: Listar todas las solicitudes del sistema.
- **Filtros (Query Params)**:
  - `usuario_id` (opcional): Filtrar por el ID de un usuario específico.
- **Ejemplo**: `/solicitudes?usuario_id=1`
- **Respuesta (JSON)**:
  ```json
  [
    {
      "id": 1,
      "usuario_id": 1,
      "servicio_id": 1,
      "urgencia": "media",
      "precio_final": "150.50",
      "estado": "en_espera_de_pago",
      "latitud_usuario": "19.4326",
      "longitud_usuario": "-99.1332",
      "fecha_creacion": "2023-10-27T10:00:00Z"
    }
  ]
  ```

## Tipos de Datos y Formatos
- **Urgencia**: `"baja"`, `"media"`, `"alta"`, `"critica"`.
- **Precios/Coordenadas**: En las respuestas se devuelven como Strings para mantener precisión decimal de `rust_decimal`.
