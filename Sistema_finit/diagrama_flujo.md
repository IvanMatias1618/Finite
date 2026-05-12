# 🗺️ Mapas Mentales del Sistema Finite

## 1. Arquitectura Modular (Flujo de Datos)
Este mapa muestra cómo una petición (ej. Calificar un servicio) viaja a través de las capas de la Arquitectura Limpia.

```text
[ CLIENTE (Okupo) ]
       |
       | POST /calificaciones {solicitud_id, calificacion}
       v
[ CAPA API (Infraestructura) ]
       |
       |--> manejadores.rs (Valida JSON y extrae datos)
       |--> rutas.rs (Inyecta Dependencias)
       v
[ CAPA APLICACIÓN (Casos de Uso) ]
       |
       |--> calificar_servicio.rs (Lógica de Negocio)
       |    |-- ¿Existe la solicitud?
       |    |-- ¿Está terminada?
       |    |-- ¿Ya tiene calificación?
       v
[ CAPA DOMINIO (Entidades y Puertos) ]
       |
       |--> resennia.rs (Estructura de Datos)
       |--> puertos/repositorio_resennia.rs (Contrato/Interface)
       v
[ CAPA INFRAESTRUCTURA (Persistencia) ]
       |
       |--> mysql_repositorio_resennia.rs (Implementación SQL)
       |--> Base de Datos (MySQL)
```

---

## 2. Estructura de la Base de Datos (Mapa de Relaciones)
Visualización de cómo se conectan las tablas principales en el motor.

```text
  [ usuario ] <------- 1:1 ------- [ colaborador ]
      ^                                |
      |          +---------------------+ 1:N
      |          |                     |
      |          v                     v
      |   [ portafolio ]        [ servicio ] <-------- [ subcategoria ]
      |                                |                      ^
      |                                | 1:N                  |
      |                                v                      | 1:N
      |                         [ solicitud_servicio ] ------- [ categoria ]
      |                                |
      |                                | 1:1
      |                                v
      +------------------------ [ resennia ]
```

---

## 3. Módulos de Colaborador (Estructura Pro)
Mapa de los componentes que forman el perfil completo de un profesional.

```text
[ PERFIL COLABORADOR ]
         |
         +-- [ Documentación ] ----> (INE Frontal, INE Trasera, Comprobante)
         |
         +-- [ Verificación ] -----> (Estado: Pendiente | Verificado | Rechazado)
         |
         +-- [ Config. Precios ] --> (Precio/Km, Recargos: Lluvia, Domingo, Noche)
         |
         +-- [ Disponibilidad ] ---> (Agenda Semanal: L-D, Horas Inicio/Fin)
         |
         +-- [ Portafolio ] --------> (Gestión: Añadir | Eliminar trabajos)
         |
         +-- [ Estadísticas ] ------> (Ganancias, Rating, Total Servicios)
```
