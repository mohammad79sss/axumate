# Axumate

> NestJS-inspired scaffolding for [Axum](https://github.com/tokio-rs/axum) web applications.
> Quickly generate **Modules, Controllers, Services, DTOs, Entities, and Middlewares** with a clean, opinionated project structure.

[![Crates.io](https://img.shields.io/crates/v/axumate.svg)](https://crates.io/crates/axumate)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

##  Who is this for?

Axumate is designed for developers who want to:

1. Build **scalable and well-structured** Axum projects.
2. Enforce **opinionated conventions** without reinventing the wheel.
3. Get started with Axum faster, especially as a **Rust beginner**.
4. Transition from **NestJS to Rust** while keeping a familiar development experience.

---

## 📦 Getting Started with the CLI

### 1️⃣ Install the CLI

```bash
cargo install axumate
```

### 2️⃣ Create a new project

```bash
axumate new my_project
```

This generates a minimal Axum project and installs the required dependencies.

### 3️⃣ Generate a module

```bash
cd my_project
axumate generate module hero
```

This scaffolds the following structure:

```
src/
├── hero/
│   ├── dto/
│   │   └── hero_dto.rs
│   ├── entity/
│   │   └── hero_entity.rs
│   ├── hero_controller.rs
│   ├── hero_service.rs
│   └── mod.rs
├── lib.rs
└── main.rs
```

### 4️⃣ Run the project

```bash
cargo run
```

Then visit: [http://127.0.0.1:3000/hero](http://127.0.0.1:3000/hero)

---

##  Generate Other Components

In addition to modules, you can generate controllers, services, DTOs, entities, and middlewares individually.

For example, to generate a middleware:

```bash
axumate generate middleware logger
```

Other supported generators:

* `axumate generate controller <name>`
* `axumate generate service <name>`
* `axumate generate dto <name>`
* `axumate generate entity <name>`

---

##  Project Structure Philosophy

Axumate enforces a **modular, NestJS-inspired layout**, where each feature lives in its own directory. This helps keep large projects organized and maintainable:

* **Modules**: Group related controllers, services, DTOs, and entities.
* **Controllers**: Define routes and handle HTTP requests.
* **Services**: Encapsulate business logic.
* **DTOs**: Represent input/output data contracts.
* **Entities**: Define database models or core domain types.
* **Middlewares**: Handle cross-cutting concerns (e.g., logging, authentication).

This structure grows naturally as your project scales, without losing clarity.

---

##  Contribute

* Help me in improving project. contribute by opening issues — feedback is welcome!

---

**License:** MIT
