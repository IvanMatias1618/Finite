# --- ETAPA DE CONSTRUCCIÓN ---
FROM rust:1.75-slim-bookworm as builder

# Instalar dependencias necesarias para la compilación (SSL y SQLite)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar archivos de configuración para cachear dependencias
COPY Cargo.toml Cargo.lock ./

# Crear un esqueleto para compilar las dependencias
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Ahora copiar el código real y compilar el binario final
COPY src ./src
COPY infraestructura ./infraestructura
RUN touch src/main.rs && cargo build --release

# --- ETAPA DE EJECUCIÓN ---
FROM debian:bookworm-slim

# Instalar librerías mínimas necesarias para la ejecución
RUN apt-get update && apt-get install -y \
    libssl3 \
    libsqlite3-0 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar el binario desde la etapa de construcción
COPY --from=builder /app/target/release/finit /app/finit
# Asegurar que el directorio de la base de datos existe en el contenedor
RUN mkdir -p /app/infraestructura

# Exponer el puerto del motor (por defecto 3000)
EXPOSE 3000

# Comando para ejecutar el motor
CMD ["./finit"]
