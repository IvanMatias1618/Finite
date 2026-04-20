# 🚀 Finite

**Finite** es un motor de marketplace genérico diseñado para el emparejamiento (*matching*) dinámico basado en proximidad geográfica, disponibilidad y urgencia. Desarrollado con **Rust** y **MySQL**, ofrece una API robusta y de alto rendimiento para plataformas de servicios bajo demanda.

---

## ✨ Características Principales

- 📍 **Matching Geográfico**: Búsqueda de colaboradores basada en latitud y longitud (Fórmula de Haversine).
- 💰 **Precios Dinámicos**: Cálculo de costos según la distancia y el nivel de urgencia solicitado.
- 🏗️ **Arquitectura Limpia**: Código desacoplado, fácil de mantener y escalar siguiendo principios de *Clean Architecture*.
- 🔒 **Seguridad**: Autenticación JWT y encriptación de contrasennas con bcrypt.
- 🛠️ **Genericidad**: Diseñado para ser el núcleo de cualquier plataforma de servicios (limpieza, mecánica, reparaciones, etc.).

---

## 🛠️ Tecnologías

- **Lenguaje**: [Rust](https://www.rust-lang.org/) (Eficiencia y seguridad de memoria).
- **Framework Web**: [Axum](https://github.com/tokio-rs/axum) (Basado en Tokio).
- **Base de Datos**: MySQL (Producción) y SQLite (Desarrollo/Tests).
- **ORM/Query Builder**: [SQLx](https://github.com/launchbadge/sqlx).

---

## 🚀 Inicio Rápido

### Requisitos
- Rust (v1.70+)
- MySQL (opcional para desarrollo local, usa SQLite por defecto).

### Instalación
```bash
# Clonar el repositorio
git clone https://github.com/ivanmatias1618/Finite.git
cd finite/finit

# Ejecutar el servidor
cargo run
```

La API estará disponible en `http://localhost:3000`.

---

## 📂 Estructura del Proyecto

- `src/dominio`: Entidades de negocio y puertos (interfaces).
- `src/aplicacion`: Casos de uso y lógica de orquestación.
- `src/infraestructura`: Implementaciones de persistencia y API REST.
- `Sistema_finit/`: Documentación técnica detallada (flujos, estructuras, API).

---

## 📜 Reglas de Desarrollo

Este proyecto sigue reglas estrictas de nomenclatura y estilo:
- **Idioma**: Español (excepto palabras reservadas).
- **Sin 'ñ'**: Se utiliza 'nn' como reemplazo.
- **Sin Abreviaturas**: Nombres descriptivos para mayor claridad.

---

## 📄 Licencia

Este proyecto está bajo la Licencia MIT.
