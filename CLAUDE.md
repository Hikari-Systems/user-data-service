# CLAUDE.md — user-data-service-rs

Guidance for AI assistants working on this codebase.

---

## What this service does

Stores and retrieves user accounts, OAuth provider profiles, access requests, and terms acceptances. Exposes a REST API over PostgreSQL. No S3, no file I/O, no external services beyond the database. This is a Rust/actix-web rewrite of the original TypeScript/Express/Knex service and must remain a drop-in replacement.

---

## Codebase map

```
src/
  main.rs            — startup: config load, pool build, migrations, server bind
  config.rs          — AppConfig and sub-structs; serde_json loader + env var override walker
  db.rs              — PgPool builder (SSL mode, pool sizing)
  models/
    user.rs          — User struct (FromRow + Serialize), CreateUser/UpsertUser input, DB functions
    oauth_profile.rs — OauthProfile struct, UpsertOauthProfile input, DB functions
    access_request.rs — AccessRequest struct, DB functions
    user_terms_acceptance.rs — UserTermsAcceptance struct, CreateUserTermsAcceptance input, DB functions
  routes/
    mod.rs           — registers all routes on ServiceConfig; specific routes before wildcards
    user.rs          — user CRUD + accessRequest sub-routes
    oauth_profile.rs — OAuth profile CRUD
    user_terms_acceptance.rs — terms acceptance CRUD
static/
  index.html         — root page (embedded via include_str! at compile time)
migrations/
  20240620153200_baseline.sql — single consolidated migration, IF NOT EXISTS throughout
config.json          — default config values baked into the image
```

---

## Critical implementation decisions

### Config loading (`config.rs`)
**Do not use the `config` crate.** It normalises JSON keys to lowercase, which breaks `camelCase` fields like `caCertFile`. The custom loader reads `config.json` with `serde_json::Value`, applies env var overrides by walking the key tree with exact-case segment matching, then deserialises in one pass.

Config priority (lowest → highest):
1. `config.json` in the working directory (or `CONFIG_PATH` env var)
2. Env vars with `__` separator, exact camelCase path segments (e.g. `db__host=postgres`)

### Route registration (`routes/mod.rs`)
Routes are registered directly on `web::ServiceConfig`, **never inside `web::scope("")`**. An empty scope matches all paths and swallows 404s.

Route order matters at two places:
- `GET /api/user/byEmail` must be registered **before** `GET /api/user/{id}` (static segment wins, but register first anyway).
- `GET /api/userTermsAcceptance/byUserId/{userId}/latest` and `GET /api/userTermsAcceptance/byUserId/{userId}` must both be registered **before** `GET /api/userTermsAcceptance/{id}`. actix-web's trie gives literal segments priority over parameters, but the explicit ordering makes it unambiguous.

### AppState (`main.rs`)
`AppState` is a **concrete struct** with a `PgPool` field — not a trait object. `web::Data<AppState>` extraction in handlers is straightforward. All handlers take `state: web::Data<AppState>` as their first parameter.

### sqlx queries (`models/`)
Use **runtime queries** (`query_as::<_, Row>(sql).bind(value)`) not compile-time macros (`query_as!`). The macro requires `DATABASE_URL` at compile time, which breaks Docker builds without a live database.

### JSONB field — OauthProfile.profile_json (`models/oauth_profile.rs`)
`profile_json` is a `JSONB` column stored as `serde_json::Value` in the struct. The `#[sqlx(json)]` attribute on the field instructs sqlx's `FromRow` derive to decode the raw JSON bytes directly. For INSERT/UPDATE binds, wrap with `sqlx::types::Json(&parsed)`.

`profileJson` arrives in the PUT request body as a **JSON-encoded string** (not a JSON object), matching the original TS behaviour. The handler parses it with `serde_json::from_str` before passing to the model.

### Empty string → None (`models/user.rs`)
The original TS service treated empty strings for optional fields as `undefined` (null). The Rust structs for `CreateUser` and `UpsertUser` use a custom serde deserializer (`empty_string_as_none`) on `name` and `picture` fields: if the client sends `""`, the field becomes `None`. `pictureImageServiceId` is `Option<Uuid>` — an empty string would be a 400 parse error, which is acceptable.

### Timestamps — NaiveDateTime, not DateTime<Utc>
The original Knex migrations used `t.timestamps()` which creates `TIMESTAMP WITHOUT TIME ZONE` PostgreSQL columns. sqlx maps `TIMESTAMP` to `chrono::NaiveDateTime` and `TIMESTAMPTZ` to `chrono::DateTime<Utc>` — they are not interchangeable at the wire level. All timestamp fields in the structs use `Option<NaiveDateTime>` (or `NaiveDateTime` for `accepted_at`). Serialisation format is `"2024-06-20T15:32:00"` (no timezone suffix).

### Baseline migration (`migrations/20240620153200_baseline.sql`)
A single migration creates all four tables in their **final state** using `CREATE TABLE IF NOT EXISTS` and `ADD COLUMN IF NOT EXISTS`. This makes it safe to run against a database that was already migrated by the TypeScript/Knex service (all statements are no-ops, the migration is recorded in `_sqlx_migrations`, and the service starts normally). Do not split this back into incremental migrations.

### oauthProfile delete bug — fixed
The original TS model had `del = (db) => (id) => db.del().where('id', id)` for the `oauthProfile` table, which has no `id` column (primary key is `sub`). This silently deleted nothing. The Rust version correctly uses `DELETE FROM oauth_profile WHERE sub = $1`.

### Response codes
These match the original TS service exactly:
- `200` — found / updated
- `201` — created
- `203 Non-Authoritative Information` — deleted successfully
- `204 No Content` — not found (used instead of 404)
- `400` — invalid input (actix-web returns this automatically for bad path/query params)
- `500` — internal error (mapped from `anyhow::Error` via `ErrorInternalServerError`)

### Static file serving
`static/index.html` is embedded via `include_str!("../static/index.html")` and served from the `root_page` handler at `GET /`. Do not use `actix-files` — the relative path `./static` resolves relative to the process CWD, which is unreliable inside Docker.

### TLS / OpenSSL
sqlx is configured with `runtime-tokio-rustls`. This eliminates `openssl`/`pkg-config` from the build entirely. The runtime image (`debian:bookworm-slim`) provides only `libc`, `libm`, `libgcc_s` — all that is needed.

---

## Docker build notes

Two-stage build:
1. **`builder`** (`rust:1-bookworm`) — stub `src/main.rs` (`fn main() {}`) is compiled first with `cargo build --release --locked` to cache all dependency compilation in a layer. The stub artifacts are deleted, real source is overlaid, and `cargo build --release --locked` runs again to compile only the service code.
2. **`runtime`** (`debian:bookworm-slim`) — copies binary, migrations, static, and config.json. Runs as unprivileged `appuser` (UID 1000).

No native deps beyond `ca-certificates`. Cold first build takes ~25–35 min; subsequent builds with the dep-cache layer hit are ~2 min.

**Do not use alpine/musl.** Proc-macro crates require the dynamic linker at build time.

**cargo-chef is not worth it** for this single-crate service. The stub `main.rs` pattern achieves the same caching.

---

## Adding new endpoints

1. Add the DB query function to the relevant `models/` file.
2. Add the handler function to the relevant `routes/` file (make any new path/query structs `pub(crate)` — private types in public handler signatures cause compile errors).
3. Register the route in `routes/mod.rs` — place specific/static paths before wildcard paths.

## FK dependency order for user deletion

`access_request`, `oauth_profile`, and `user_terms_acceptance` all have FK constraints pointing at `user`. Deleting a user via `DELETE /api/user/:id` will fail with a 500 if any child rows exist. The correct cleanup order is:

1. `DELETE /api/userTermsAcceptance/:id` for each acceptance
2. `DELETE /api/oauthProfile?sub=X`
3. `DELETE /api/accessRequest/:id` for each access request
4. `DELETE /api/user/:id`

`DELETE /api/accessRequest/:id` is a route added in the Rust rewrite — the original TS service had `del` in the model but never exposed it as a route, making it impossible to delete users with access requests via the API.

---

## Common gotchas

- Config key names are **case-sensitive**. `db__host` not `db__Host`.
- Path/query structs used in `pub` handler signatures must be `pub(crate)` — otherwise actix-web's monomorphisation produces a compile error about private types in public interfaces.
- `_sqlx_migrations` (sqlx) is separate from `knex_migrations` (the original TS service). Both tables can coexist in the same database without conflict.
- The `"user"` table name requires quoting in SQL because `user` is a reserved word in PostgreSQL. All queries against it use `"user"`.
- Database migrations run automatically on startup. The `_sqlx_migrations` table is idempotent — safe to run against an already-migrated database.
- Timestamp serialisation format is `"2024-06-20T15:32:00"` (NaiveDateTime, no Z suffix) rather than the TS service's `"2024-06-20T15:32:00.000Z"`. This is a known minor behavioural difference due to the original schema using `TIMESTAMP` (no timezone).
