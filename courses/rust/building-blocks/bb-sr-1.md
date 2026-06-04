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

## You're ready when...

- [ ] You can explain 3 API versioning strategies and their tradeoffs
- [ ] You know what makes a change backward-compatible
- [ ] You understand OpenAPI/Swagger and why it matters
- [ ] You can describe a deprecation workflow

Next: [Project: API Design & Versioning](../projects/senior-1/README.md)
