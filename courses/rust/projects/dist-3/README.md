# Project: Service Mesh Fundamentals

In this project you will build the core components of a service mesh from
scratch: a service registry, client-side load balancing, distributed tracing,
a saga orchestrator, and idempotency middleware. You will wire these together
into a small system of three cooperating microservices.

## Prerequisites

- Completion of dist-1 (Raft) and dist-2 (Percolator), or equivalent
  distributed systems experience.
- Comfortable with Axum, Tokio, and async Rust patterns.
- Familiarity with HTTP middleware concepts.

---

## Part 1: Service Registry

**Goal:** build an in-memory service registry that tracks live service instances.

1. In `src/registry.rs`, implement `ServiceRegistry` with these methods:
   - `register(instance: ServiceInstance)` — add an instance to the registry.
   - `deregister(instance_id: &str)` — remove an instance by ID.
   - `discover(service_name: &str) -> Vec<ServiceInstance>` — return all
     healthy instances of a named service.
   - `health_check(&self)` — iterate all instances; remove any whose
     `last_heartbeat` is older than the configured timeout.

2. `ServiceInstance` should contain at least: `id`, `service_name`, `address`,
   and `last_heartbeat` (a `chrono::DateTime<Utc>`).

3. Use `dashmap::DashMap` for interior mutability so the registry can be shared
   across Axum handlers without an external `Mutex`.

4. Write tests: `registry_register_and_discover`,
   `registry_deregister_removes_service`, `registry_health_check_removes_unhealthy`.

---

## Part 2: Client-Side Load Balancing

**Goal:** implement a `LoadBalancer` trait with three strategies.

1. Define a trait in `src/load_balancer.rs`:

   ```rust
   #[async_trait]
   pub trait LoadBalancer: Send + Sync {
       async fn next_instance(&self, instances: &[ServiceInstance]) -> Option<ServiceInstance>;
   }
   ```

2. Implement three strategies:
   - **RoundRobin** — cycle through instances in order.
   - **Random** — pick a random instance.
   - **LeastConnections** — pick the instance with the fewest active
     connections (track connection counts in an `Arc<DashMap>`).

3. Write tests: `round_robin_distributes_evenly`, `random_distributes_to_all`,
   `least_connections_prefers_idle`.

---

## Part 3: Distributed Tracing

**Goal:** propagate a `x-trace-id` header across HTTP calls between services.

1. In `src/tracing_propagation.rs`, implement Axum middleware that:
   - Reads an incoming `x-trace-id` header. If missing, generates a new UUID.
   - Stores the trace ID in request extensions so handlers can access it.
   - Adds the trace ID to every outgoing response header.

2. Implement `TracedClient`, a thin wrapper around `reqwest::Client` that
   automatically injects the current trace ID into outgoing requests.

3. Collect `Span` structs (service name, operation, start/end timestamps,
   trace ID) and store them in a shared `Vec<Span>`.

4. Write tests: `trace_id_propagated_across_services`,
   `trace_id_generated_if_missing`.

---

## Part 4: Saga Orchestrator

**Goal:** implement the saga pattern for multi-service distributed transactions.

1. In `src/saga.rs`, define:
   - `SagaStep` — a pair of async functions: `action` (the forward operation)
     and `compensation` (the rollback).
   - `Saga` — an ordered list of `SagaStep`s.
   - `SagaOrchestrator` — executes a `Saga`: runs each step's action in order;
     on failure, runs compensations in reverse order for all previously
     completed steps.

2. `SagaOrchestrator::execute` returns `Ok(())` on full success or
   `Err(SagaError)` if a step fails (after compensations have run).

3. Write tests: `saga_completes_all_steps`,
   `saga_compensates_on_failure` (step 3 fails, steps 2 and 1 are compensated),
   `saga_partial_compensation` (step 2 fails, only step 1 is compensated).

---

## Part 5: Idempotency

**Goal:** add idempotency-key middleware so that retried requests are not
processed twice.

1. In `src/idempotency.rs`, implement middleware that:
   - Reads an `idempotency-key` header from the request.
   - If the key has been seen before, returns the cached response immediately.
   - If the key is new, processes the request, caches the response, and returns
     it.
   - If no key is present, passes the request through without caching
     (non-idempotent endpoint).

2. Use `dashmap::DashMap` for the response cache.

3. Write tests: `idempotency_key_prevents_duplicate`,
   `idempotency_key_missing_is_allowed`.

---

## Part 6: Integration

**Goal:** wire everything together into a working multi-service system.

1. Create three service binaries in `src/bin/`:
   - `service_a.rs` — "Tasks" service (CRUD for tasks).
   - `service_b.rs` — "Notifications" service (accepts notification requests).
   - `service_c.rs` — "Search" service (accepts indexing requests).

2. Each service:
   - Registers itself with the `ServiceRegistry` on startup.
   - Sends periodic heartbeats.
   - Uses `TracedClient` for all outgoing HTTP calls.
   - Applies idempotency middleware to mutating endpoints.

3. Implement a "create task" saga that:
   - Step 1: Create a task in Service A.
   - Step 2: Send a notification via Service B.
   - Step 3: Index the task in Service C.
   - Compensations undo each step on failure.

4. Use the load balancer to choose instances when a service has multiple
   replicas registered.

5. Write test: `full_integration_create_task_saga`.

---

## Estimated Time

2 to 3 weeks.
