# Usa la imagen oficial de Rust
FROM rust:latest

# Crear un directorio de trabajo en el contenedor
WORKDIR /app

# Copiar los archivos del proyecto
COPY Cargo.toml Cargo.lock ./

# Precompilar dependencias
RUN mkdir src && echo 'fn main() {}' > src/main.rs && cargo build --release && rm -r src

# Copiar el código fuente
COPY . .

# Compilar la aplicación
RUN cargo build --release

# Exponer el puerto en el contenedor
EXPOSE 8080

# Ejecutar la aplicación
CMD ["./target/release/api_nueva"]
