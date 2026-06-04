# Building Block Sr-1: API Design and Evolution

**Prerequisites**: Phase 5 complete.

Before starting [Project: API Design & Versioning](../projects/senior-1/README.md),
complete the readings below.

## What to read

- [API Versioning Strategies Comparison](https://blog.bitsrc.io/6-api-versioning-strategies/).
  URL path, query parameter, header, content negotiation. Tradeoffs of each.

- [Postel's Law and API Evolution](https://en.wikipedia.org/wiki/Robustness_principle).
  "Be conservative in what you send, liberal in what you accept." Essential for
  backward-compatible API evolution.

- [OpenAPI Specification Guide](https://swagger.io/specification/).
  The standard for describing REST APIs. Machine-readable, enables tooling.

- [utoipa documentation](https://docs.rs/utoipa/latest/utoipa/).
  Generate OpenAPI specs from Rust code with derive macros.

## Key concepts

### Backward Compatibility Rules
- Adding fields to responses is safe (clients should ignore unknown fields)
- Removing or renaming response fields is BREAKING
- Adding optional request fields is safe
- Making optional fields required is BREAKING
- Adding new endpoints is safe
- Changing URL structure is BREAKING

### Deprecation Flow
1. Mark endpoint as deprecated (Sunset header + docs)
2. Log warnings when deprecated endpoint is used
3. Give clients migration time (weeks/months)
4. Remove after sunset date

## Exercises

1. **Classify changes as breaking or safe.** For each change below, state
   whether it's backward-compatible. If breaking, describe how to make it safe:
   - Adding a `tags` field to a response body
   - Renaming `user_name` to `username` in a response body
   - Making the `description` request field required (was optional)
   - Changing `GET /users/:id` to `GET /v2/users/:id`
   - Changing a field from `string` to `number` in the response
   - Adding a new optional query parameter `?sort_by=`

2. **Design an API evolution.** You have `POST /orders` that accepts
   `{product_id, quantity}`. You need to add support for discount codes:
   `{product_id, quantity, discount_code}`. Write the exact steps to evolve
   the API without breaking existing clients, including what happens on the
   server when `discount_code` is absent.

3. **Write OpenAPI for a small API.** By hand (not generated), write the
   OpenAPI 3.0 YAML for an API with 3 endpoints: create task, get task, list
   tasks. Include request/response schemas, error responses, and pagination
   parameters.

## You're ready when...

- [ ] You can explain 3 API versioning strategies and their tradeoffs
- [ ] You know what makes a change backward-compatible
- [ ] You understand OpenAPI/Swagger and why it matters
- [ ] You can describe a deprecation workflow
- [ ] You can classify any API change as breaking or safe

Next: [Project: API Design & Versioning](../projects/senior-1/README.md)
