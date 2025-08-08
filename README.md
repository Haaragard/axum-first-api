# Project

This project was made with the purpose of learn more about RUST, some crates and features

Study purposes:

- Rust
- Rust Crates
  - Axum (Web Application Framework)
    - Lifetime
    - Structure
    - Build
    - States
  - Tokio (Async multithread)
  - Crono (DateTime creation/management)
  - dotenv (Environment variables)
  - guid-create (GUUID creation)
- Rust Project Structure
- Rust Project Management

## Dependencies

It needs to have RUST with cargo installed

To run the application without problems, should have the lib sqlite3 too.

### libsqlite-3 Installation

**Ubuntu:**

```bash
sudo apt update && sudo apt install libsqlite3-dev
```

## Environment configs

To prepare the environment configurations just make a copy of the file `./env.example` into `./env` and change the values as you prefer.

```bash
cp .env.example .env
```

## Build

Just run

```bash
cargo build
```

## Run application

Just run

```bash
cargo run
```
