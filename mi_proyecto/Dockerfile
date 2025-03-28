# Build stage
FROM rust:1.72 as builder

WORKDIR /app

# Actualizar Cargo a la última versión
RUN rustup update stable && rustup default stable

# Instalar dependencias necesarias
RUN apt-get update && apt-get install -y libpq-dev pkg-config openssl

# Copiar el código fuente
COPY . .

# Construir el proyecto en modo release
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Instalar dependencias necesarias para ejecutar el binario
RUN apt-get update && apt-get install -y libpq5 openssl ca-certificates

WORKDIR /app

# Copiar el binario y otros archivos necesarios desde la etapa de construcción
COPY --from=builder /app/target/release/laliga-tracker .
COPY --from=builder /app/Frontend ./Frontend
COPY --from=builder /app/partidos.sql .

# Comando para ejecutar el backend
CMD ["./laliga-tracker"]