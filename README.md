# Weather Dashboard

A full stack weather and camping application built primarily in **Rust**, with **Yew/WebAssembly frontend**. Tools and technologies used: Rust backend, Redis caching, PostgreSQL persistence, authentication and authorization through Auth0, an MCP-powered AI assistant, Docker infrastructure and Kubernetes.

The project is intentionally focused on going deeper into backend, specifically in concurrency, caching, distributed system concepts and infrastructure rather than continuously adding unrelated features.

---

## Engineering Highlights from Most valuable to Least valuable (My opinion)

### 1. Combining duplicate requests per city with Tokio `Notify`

Weather data is cached in Redis, but a cache miss creates a problem. If many users request the same city simultaneously, every request could call the OpenWeather API.

OpenWeather offers 60 request per minute on a free plan. 

60 requests for city London means 60 independent and simultaneously calls to OpenWeather API.

Illustration: 

```text
60 requests for London
        |
        v
    Redis MISS
        |
        v
60 OpenWeather requests
```

To prevent this `cache stampede`, the backend implements a **per-city single-flight mechanism** using Mutex, hashMap and tokio Notify.

For every active city request, the backend keeps an in memory `flight`:

```text
HashMap<City, Flight>
```

The first request for a city becomes the **leader**.

All subsequent requests for the same city become **followers** and wait on a Tokio `Notify`.

Illustration:

```text
                  London
                    |
              Active Flight
                    |
        +-----------+-----------+
        |           |           |
      Leader     Follower    Follower
        |           |           |
 OpenWeather       wait        wait
        |
    Redis SET
        |
 notify_waiters()
        |
        +-----------> followers wake
                      |
                      v
                  Redis GET
```

The first approach was using a Mutex on every city. The problem with this approach is when the leader finishes, every other follower will lock the mutex, get the data from redis, unlock the mutex. and so on and so fort.

Illustration:

Request A ──> acquires Mutex ──> OpenWeather ──> Redis SET ──> unlock
Request B ──> waits ───────────> acquires Mutex ──> Redis GET ──> unlock
Request C ──> waits ─────────────────────────────> acquires Mutex ──> Redis GET ──> unlock
Request D ──> waits ───────────────────────────────────────────────> acquires Mutex ──> Redis GET ──> unlock

This adds an additional overhead.

The implementation also maintains explicit flight state using atomics so that followers do not depend only on receiving a notification and therefore avoid missed notification races.

The flight entry is removed from the `HashMap` after completion so that only currently active operations remain tracked.

> `tokio::Notify` provides coordination between asynchronous tasks **inside one backend process**. 

Cross process coordination between multiple backend instances is a separate distributed systems problem and is planned through Redis based distributed locking. **This is yet to be implemented.**

### Preliminary benchmark

Initial testing response time without Mutex or tokio notify to OpenWeatherAPI (60 requests) is approximately **2.4 seconds**.

Preliminary measurements showed:

| Implementation | Observed time | API | Request count
|---|---:|
| Per city `Mutex` | ~360 ms | OpenWeather API | 60
| Tokio `Notify` | ~240 ms | MOCK Local OpenWeather API | 60
| Tokio `Notify` | ~3.6 sec | MOCK Local OpenWeather API | 10 000


The key result is that 10,000 concurrent requests for the same `uncached city` resulted in only one upstream API call.

```text
10,000 concurrent requests
           |
           v
      Redis MISS
           |
           v
   1 single-flight leader
           |
           +----------------------+
           |                      |
           v                      v
    1 mock API call         followers wait
        (2.4 s)                   |
           |                      |
           v                      |
       Redis SET                  |
           |                      |
           v                      |
    notify_waiters() -------------+
           |
           v
   followers wake up
           |
           v
       Redis GET
           |
           v
   10,000 × HTTP 200
```
## 2. Redis caching

Weather responses are cached in Redis to reduce latency and avoid unnecessary calls to the external weather provider.

City keys are normalized before being used:

```text
London
LONDON
 london
```

become:

```text
london
```

This prevents logically identical cities from creating different cache entries.

The request flow is currently:

```text
Request
   |
   v
Redis GET
   |
   +---- HIT ----> return cached weather
   |
   v
 MISS
   |
   v
Single-flight coordination
   |
   v
OpenWeather API
   |
   v
Redis SET
```

Redis entries use expiration times (TTL) so that weather information is automatically refreshed after becoming stale.

---

## 3. MCP-powered camping AI assistant

The application includes a separate **MCP client and MCP server** used by an AI camping assistant.

The assistant is intentionally scoped to the camping domain through its system prompt.

The MCP layer allows the model to call application tools instead of relying only on text context.

For weather-related questions:

```text
User question
     |
     v
AI Assistant
     |
     +---- city already available in context
     |          |
     |          v
     |      use context
     |
     +---- city missing from context
                |
                v
             MCP tool
                |
                v
          Weather backend API
```


The MCP client and server are deployed as separate services from the main frontend and backend. 

**More mcp tools are yet to be added.**

---

## 4. Authentication, authorization, and audit logging

Authentication is handled through **Auth0**.

The backend validates JWT access tokens and applies authorization rules based on the authenticated user's permissions or role.

The application distinguishes between regular users and administrators.

```text
Authenticated User
       |
       +---- Regular user
       |       |
       |       +---- application functionality
       |
       +---- Admin
               |
               +---- application functionality
               +---- audit/log access
```

Administrative log data is not exposed to ordinary users.

Application logs are persisted in **PostgreSQL**.

Security-related backend responsibilities include:

- JWT validation
- protected API routes
- authorization checks
- admin only functionality
- PostgreSQL backed application logs

---

## 5. Containerized multi-service architecture

The development environment is containerized using **Docker Compose**.

The system consists of multiple independently running services:

```text
                         Frontend
                        Yew / WASM
                            |
                            v
                      Rust Backend
                      /     |      \
                     /      |       \
                    v       v        v
                 Redis   PostgreSQL  OpenWeather API


    Startup dependency order:

    PostgreSQL
        |
        v
    Migration service
    (run database migrations)
        |
        v
    Rust Backend
        |
        v
    Frontend


    AI / MCP flow:

    AI Assistant
        |
        v
    MCP Client
        |
        v
    MCP Server
        |
        v
    Rust Backend API
        |
        +------> Redis
        |
        +------> PostgreSQL
        |
        +------> OpenWeather API
```

The local environment is orchestrated with Docker Compose and contains separate services for the frontend, Rust backend, Redis, PostgreSQL, database migrations, MCP client, and MCP server.

Before the backend is started, PostgreSQL must be available and the dedicated migration service applies the required database migrations. This ensures that the backend starts against the expected database schema instead of attempting to initialize or modify the schema during normal application startup.

```text
PostgreSQL ready
      |
      v
Run migrations
      |
      v
Start backend
```

This keeps the backend as the primary application boundary: the MCP server does not access Redis, PostgreSQL, or the external weather provider directly, but instead consumes the same backend API used by the rest of the application.
The environment includes containers for:

- Rust backend
- Yew/WebAssembly frontend
- Redis
- PostgreSQL
- MCP client
- MCP server

The backend uses a **multi-stage Docker build** so that compilation dependencies remain in the build stage while the final runtime image contains only what is required to execute the application.

This reduces the size of the runtime container.

---

## 6. Kubernetes

The application also contains Kubernetes configuration for running the services in an orchestrated environment.

Kubernetes is used as the next deployment layer after validating the architecture locally with Docker Compose.

This allows the project to explore concepts such as:

- backend replicas
- service discovery
- container orchestration
- application configuration
- health and lifecycle management
- horizontal deployment of stateless backend instances

The distributed locking work is particularly relevant once several backend replicas can process requests concurrently.

---

## 7. External weather integration

Weather information is retrieved through the **OpenWeather API**.

The mock provider can reproduce the measured OpenWeather latency without API rate limits, allowing tests with thousands of concurrent requests.

---

## 8. API documentation

Backend endpoints are documented through **Swagger / OpenAPI**.

The API documentation provides a browsable description of available routes, request parameters, response models, and authentication requirements.

---

# Technology Stack

### Backend

- Rust (Rocket)
- Tokio
- Redis
- PostgreSQL
- Auth0 / JWT
- OpenWeather API
- OpenAPI / Swagger

### Frontend
- Rust
- Yew
- WebAssembly

### AI

- Model Context Protocol (MCP)
- Separate MCP client
- Separate MCP server
- Tool enabled camping assistant
- Prompt and behavior tests

### Infrastructure

- Docker
- Docker Compose
- Multi stage Docker builds
- Kubernetes

---

# Rust Concepts Used

The project has also served as a practical environment for working with core Rust concepts rather than only implementing CRUD endpoints.

Concepts currently used include:

- ownership and borrowing
- structs and data models
- functions and methods
- pattern matching and match arms
- macros
- async/await
- Tokio tasks and synchronization
- `Arc`
- `Mutex`
- atomics
- `HashMap`
- request coordination with `Notify`
- shared application state
- serialization and deserialization
- error handling with `Result` and `Option`


---

# Current Engineering Focus

The current progression is:

```text
Redis cache
     |
     v
Cache stampede discovered
     |
     v
Per city Mutex
     |
     v
Tokio Notify single-flight
     |
     v
Controlled mock benchmark
```

The objective is not merely to add features, but to understand how the application's behavior changes as concurrency and deployment complexity increase.