# CRUD API

Api básica creada con Actix Web en Rust, frontend con React y
base de datos MariaDB (compatible con MySQL).

## Requisitos

- Python || Opcional, para correr el proyecto completo con un solo comando.
- Cargo (Rust)
- pnpm
- MySQL o MariaDB

## Base de datos

Inicia y crea la base de datos antes de ejecutar (según `DATABASE_URL`).

Crea `crud_api/.env` copiando `crud_api/.env.example` y ajusta la URL.

## Ejecución

```bash
# En la raíz
python run.py o python3 run.py # Ejecuta todo el proyecto + migraciones

# O por separado:

# Dentro de crud_api/
cargo run #Instala dependencias y corre el backend

#Dentro de frontend/
pnpm run dev # Ejecuta el frontend
```

`run.py` espera que la base de datos ya esté corriendo, Inicia migraciones,
API y frontend, puede que siempre necesites hacer `pnpm install` dentro de frontend.
