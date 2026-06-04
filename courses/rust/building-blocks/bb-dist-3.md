# Building Block Dist-3: Service Discovery and Distributed Tracing

**Prerequisites**: [Percolator Lab](../../dss/percolator/README.md) complete.

Before starting [Project: Service Mesh Fundamentals](../projects/dist-3/README.md),
complete the readings and exercises below.

## What to read

- [Service Discovery Patterns](https://microservices.io/patterns/service-discovery.html).
  Client-side vs server-side discovery. Service registries, health checks, and
  DNS-based discovery.

- [Saga Pattern (Chris Richardson)](https://microservices.io/patterns/data/saga.html).
  Managing distributed transactions across microservices using a sequence of
  local transactions with compensating actions.

- [Distributed Tracing with OpenTelemetry](https://opentelemetry.io/docs/concepts/signals/traces/).
  How traces propagate across service boundaries using context propagation.

- [Load Balancing Algorithms](https://samwho.dev/load-balancing/).
  Interactive visualization of round-robin, random, least-connections, and
  weighted algorithms.

## Key concepts

### Service Registry
```
Service A starts → registers with registry (name, address, health endpoint)
Service B needs A → queries registry → gets A's address
Registry health-checks A periodically → removes if unhealthy
```

### Saga Pattern
```
Create Task → Notify Users → Update Search Index
    ↓ (if notify fails)
Compensate: Delete Task
```

Each step has a compensating action. If any step fails, previous steps are
undone in reverse order.

### Context Propagation
Distributed tracing works by passing a trace ID in HTTP headers across services:
```
traceparent: 00-{trace_id}-{span_id}-01
```

### Idempotency Keys
To handle retries safely, every operation should be idempotent. Use client-generated
idempotency keys: repeating a request with the same key produces the same result.

## Exercises

**Exercise 1**: Implement round-robin load balancing over a list of addresses.

**Exercise 2**: Design a saga for a 3-step operation with compensating actions.

**Exercise 3**: Propagate a request ID header through two HTTP services using reqwest.

## You're ready when...

- [ ] You can explain client-side vs server-side service discovery
- [ ] You understand the saga pattern and compensating transactions
- [ ] You know how distributed tracing context propagates
- [ ] You can implement basic load balancing algorithms

Next: [Project: Service Mesh Fundamentals](../projects/dist-3/README.md)
