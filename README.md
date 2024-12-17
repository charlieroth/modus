# Reminders

A Server and CLI application for managing reminders.

## Project Structure

```
.
├── migrations
├── src
│   ├── bin
│   │   └── server
│   │       └── main.rs
│   └── lib
│       ├── domain
│       ├── inbound
│       ├── outbound
│       ├── config.rs
│       ├── domain.rs
│       ├── inbound.rs
│       ├── lib.rs
│       └── outbound.rs
├── .env.template
├── docker-compose.yaml
├── Dockerfile
├── Dockerfile.geni
└── Makefile
```

The `cmd` directory contains the entry points for the CLI, HTTP API and admin applications.

The `docs` directory contains the OpenAPI specification for the project.

The `internal` directory contains the shared logic for the project.

The `migrations` directory contains the SQL migrations for the project.

The `docker-compose.yaml` file contains the configuration for the Docker Compose stack for the project.

The `Dockerfile` contains the configuration for the Docker build for the project.

The `Dockerfile.geni` contains the configuration for the Docker build for running the SQL migrations for the project.

The `Makefile` contains the commands for building, testing, and running the project.
