# Guía de Uso de la API - finit

Este documento es para que el desarrollador del Frontend pueda consumir los servicios del motor `finit`.

## Base URL
Por defecto local: `http://localhost:3000`
## Endpoints Principales

### 1. Estado de la API (Health Check)
**GET** `/`
- **Uso**: Verificar si el servidor está en línea.
- **Respuesta Esperada**: `200 OK`.

### 2. Registro de Usuario
**POST** `/usuarios`
- **Uso**: Crear una cuenta base.
- **Cuerpo (JSON)**:
  ```json
  {
    "nombre": "Ivan",
    "correo": "ivan@ejemplo.com",
    "contrasenna": "mi_password"
  }
  ```
- **Respuesta**: El ID del usuario.

### 3. Registro de Colaborador Completo
**POST** `/colaboradores`
- **Uso**: Convierte un usuario existente en colaborador.
- **Cuerpo (JSON)**:
  ```json
  {
    "token_usuario": "1",
    "telefono": "123456789",
    "sitio_web": "https://mi-sitio.com",
    "servicios": [
...
      [
        {
          "colaborador_id": 0,
          "categoria_id": 1,
          "descripcion": "Descripción del servicio",
          "distancia_maxima_kilometros": 15.0,
          "precio_por_kilometro": 5.5,
          "latitud": 19.432608,
          "longitud": -99.133209
        },
        [
          { "servicio_id": 0, "urgencia": "baja", "precio": 100.0 },
          { "servicio_id": 0, "urgencia": "alta", "precio": 250.0 }
        ]
      ]
    ]
  }
  ```
- **Nota**: El `colaborador_id` y `servicio_id` en el JSON pueden ser `0` o cualquier valor al registrar, ya que el servidor los asignará automáticamente.
- **Respuesta**: El ID del colaborador recién creado.

## Tipos de Datos y Formatos
- **Precios y Coordenadas**: Se envían como números de punto flotante en el JSON, pero se manejan con precisión decimal en el motor.
- **Urgencia**: Los valores permitidos son: `"baja"`, `"media"`, `"alta"`, `"critica"`.

## Ejemplo de Configuración de Túnel (ngrok)
Si necesitas compartir la API con el exterior:
`ngrok http 3000`
Luego proporciona la URL generada (`https://xxxx.ngrok-free.app`) a tu compañero del Front.
