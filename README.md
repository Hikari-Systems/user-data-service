# user-data-service-rs

A Rust/actix-web microservice for storing and retrieving user accounts, OAuth profiles, access requests, and terms acceptances. Drop-in replacement for the original TypeScript/Express service; all API endpoints, JSON config keys, and response shapes are identical.

---

## Features

- User, OAuth profile, access request, and terms acceptance CRUD backed by PostgreSQL
- `profileJson` (JSONB) for raw OAuth provider profile storage
- Automatic schema migrations on startup (safe against existing Knex-migrated databases)
- Config layering: baked defaults → env vars
- Empty strings in request bodies treated as `null` (matches original TS behaviour)

---

## API Endpoints

### `GET /healthcheck`
Returns `200 OK` with body `OK`.

```bash
curl http://localhost:3000/healthcheck
```

---

### `GET /api/user/:id`
Returns a single user by UUID. Returns `204 No Content` if not found.

```bash
curl http://localhost:3000/api/user/550e8400-e29b-41d4-a716-446655440000
```

**Response `200 OK`**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "alice@example.com",
  "name": "Alice",
  "picture": "https://cdn.example.com/alice.jpg",
  "pictureImageServiceId": null,
  "createdAt": "2024-06-20T15:32:00",
  "updatedAt": null
}
```

---

### `GET /api/user/byEmail?email=X`
Returns a single user by email address. Returns `204 No Content` if not found.

```bash
curl "http://localhost:3000/api/user/byEmail?email=alice@example.com"
```

---

### `POST /api/user`
Creates a new user. Auto-generates a UUID. Empty strings for optional fields are treated as absent.

```bash
curl -X POST http://localhost:3000/api/user \
  -H 'Content-Type: application/json' \
  -d '{
    "email": "alice@example.com",
    "name": "Alice",
    "picture": "https://cdn.example.com/alice.jpg"
  }'
```

**Body**
| Field | Type | Required | Description |
|---|---|---|---|
| `email` | string | yes | Email address |
| `name` | string | no | Display name |
| `picture` | string | no | Profile picture URL |
| `pictureImageServiceId` | UUID | no | Ref to image-service record |

**Response `201 Created`** — created user object.

---

### `PUT /api/user/:id`
Upserts a user by ID (insert or update on conflict).

```bash
curl -X PUT http://localhost:3000/api/user/550e8400-e29b-41d4-a716-446655440000 \
  -H 'Content-Type: application/json' \
  -d '{
    "email": "alice@example.com",
    "name": "Alice B.",
    "picture": ""
  }'
```

Empty string for `picture` clears the field to `null`. **Response `200 OK`** — updated user object.

---

### `DELETE /api/user/:id`
Deletes a user by ID. Returns `203 Non-Authoritative Information` on success, `204 No Content` if not found.

```bash
curl -X DELETE http://localhost:3000/api/user/550e8400-e29b-41d4-a716-446655440000
```

---

### `GET /api/user/:id/accessRequest/:key`
Returns all access requests for a user+key combination, ordered by `createdAt` descending.

```bash
curl http://localhost:3000/api/user/550e8400-e29b-41d4-a716-446655440000/accessRequest/api-access
```

**Response `200 OK`**
```json
[
  {
    "id": "661f9511-f3ac-52e5-b827-557766551111",
    "userId": "550e8400-e29b-41d4-a716-446655440000",
    "key": "api-access",
    "granted": true,
    "decidedAt": "2024-11-21T12:30:00",
    "grantedFrom": "2024-11-21T00:00:00",
    "grantedUntil": null,
    "createdAt": "2024-11-21T12:00:00",
    "updatedAt": null
  }
]
```

---

### `POST /api/user/:userId/accessRequest/:key`
Creates a new access request for a user+key. All decision fields start as `null`.

```bash
curl -X POST http://localhost:3000/api/user/550e8400-e29b-41d4-a716-446655440000/accessRequest/api-access
```

**Response `201 Created`** — created access request object.

---

### `GET /api/oauthProfile/byUserId/:userId`
Returns the OAuth profile for a user. Returns `204 No Content` if not found.

```bash
curl http://localhost:3000/api/oauthProfile/byUserId/550e8400-e29b-41d4-a716-446655440000
```

**Response `200 OK`**
```json
{
  "sub": "google|109876543210987654321",
  "userId": "550e8400-e29b-41d4-a716-446655440000",
  "profileJson": { "email": "alice@gmail.com", "name": "Alice" },
  "createdAt": "2024-06-20T15:32:00",
  "updatedAt": null
}
```

---

### `GET /api/oauthProfile/bySub?sub=X`
Returns the OAuth profile by provider subject ID. Returns `204 No Content` if not found.

```bash
curl "http://localhost:3000/api/oauthProfile/bySub?sub=google|109876543210987654321"
```

---

### `PUT /api/oauthProfile`
Upserts an OAuth profile by `sub` (insert or update on conflict). `profileJson` must be sent as a **JSON-encoded string**.

```bash
curl -X PUT http://localhost:3000/api/oauthProfile \
  -H 'Content-Type: application/json' \
  -d '{
    "sub": "google|109876543210987654321",
    "userId": "550e8400-e29b-41d4-a716-446655440000",
    "profileJson": "{\"email\":\"alice@gmail.com\",\"name\":\"Alice\"}"
  }'
```

**Response `200 OK`** — upserted profile object (with `profileJson` as a parsed object).

---

### `DELETE /api/oauthProfile?sub=X`
Deletes an OAuth profile by subject ID. Returns `203` on success, `204` if not found.

```bash
curl -X DELETE "http://localhost:3000/api/oauthProfile?sub=google|109876543210987654321"
```

---

### `GET /api/userTermsAcceptance/:id`
Returns a single terms acceptance record by UUID. Returns `204 No Content` if not found.

```bash
curl http://localhost:3000/api/userTermsAcceptance/772a0622-g4bd-63f6-c938-668877662222
```

---

### `GET /api/userTermsAcceptance/byUserId/:userId`
Returns all terms acceptances for a user, ordered by `acceptedAt` descending.

```bash
curl http://localhost:3000/api/userTermsAcceptance/byUserId/550e8400-e29b-41d4-a716-446655440000
```

**Response `200 OK`** — array of acceptance objects.

---

### `GET /api/userTermsAcceptance/byUserId/:userId/latest`
Returns the most recent terms acceptance for a user. Returns `204 No Content` if none.

```bash
curl http://localhost:3000/api/userTermsAcceptance/byUserId/550e8400-e29b-41d4-a716-446655440000/latest
```

**Response `200 OK`**
```json
{
  "id": "772a0622-g4bd-63f6-c938-668877662222",
  "userId": "550e8400-e29b-41d4-a716-446655440000",
  "termsVersion": "2.0",
  "acceptedAt": "2025-04-09T17:30:00",
  "createdAt": "2025-04-09T17:30:00",
  "updatedAt": null
}
```

---

### `POST /api/userTermsAcceptance`
Records a user accepting a terms version. Auto-generates ID and sets `acceptedAt` to now.

```bash
curl -X POST http://localhost:3000/api/userTermsAcceptance \
  -H 'Content-Type: application/json' \
  -d '{
    "userId": "550e8400-e29b-41d4-a716-446655440000",
    "termsVersion": "2.0"
  }'
```

**Response `201 Created`** — created acceptance object.

---

### `DELETE /api/accessRequest/:id`
Deletes an access request by UUID. Returns `203` on success, `204` if not found.

```bash
curl -X DELETE http://localhost:3000/api/accessRequest/b340b21b-227d-42c6-ab55-5f182450a383
```

> **Note:** access requests hold a FK to `user`, so they must be deleted before their parent user can be deleted.

---

### `DELETE /api/userTermsAcceptance/:id`
Deletes a terms acceptance record. Returns `203` on success, `204` if not found.

```bash
curl -X DELETE http://localhost:3000/api/userTermsAcceptance/772a0622-g4bd-63f6-c938-668877662222
```

---

## End-to-end test sequence

All steps verified passing. Expected final output: `203 203 203 203 203 204` (four FK-child deletes, user delete, confirm gone).

```bash
BASE=http://localhost:3000

# Unique suffix per run so leftover rows from previous runs don't interfere.
TS=$(date +%s)
EMAIL="alice+${TS}@example.com"
SUB="google|${TS}"

# 1. Create a user
USER=$(curl -s -X POST $BASE/api/user \
  -H 'Content-Type: application/json' \
  -d "{\"email\":\"$EMAIL\",\"name\":\"Alice\"}")
echo $USER | python3 -m json.tool
USER_ID=$(echo $USER | python3 -c "import sys,json; print(json.load(sys.stdin)['id'])")

# 2. Fetch by ID
curl -s $BASE/api/user/$USER_ID | python3 -m json.tool

# 3. Fetch by email — unambiguous because $EMAIL is unique to this run
curl -s "$BASE/api/user/byEmail?email=$EMAIL" | python3 -m json.tool

# 4. Update name, clear picture (empty string → null)
curl -s -X PUT $BASE/api/user/$USER_ID \
  -H 'Content-Type: application/json' \
  -d "{\"email\":\"$EMAIL\",\"name\":\"Alice B.\",\"picture\":\"\"}" \
  | python3 -m json.tool

# 5. Upsert an OAuth profile (profileJson is a JSON-encoded string)
curl -s -X PUT $BASE/api/oauthProfile \
  -H 'Content-Type: application/json' \
  -d "{\"sub\":\"$SUB\",\"userId\":\"$USER_ID\",\"profileJson\":\"{\\\"email\\\":\\\"$EMAIL\\\"}\"}" \
  | python3 -m json.tool

# 6. Fetch OAuth profile by userId
curl -s $BASE/api/oauthProfile/byUserId/$USER_ID | python3 -m json.tool

# 7. Fetch OAuth profile by sub
curl -s "$BASE/api/oauthProfile/bySub?sub=$SUB" | python3 -m json.tool

# 8. Create an access request — capture its ID for cleanup
AR=$(curl -s -X POST "$BASE/api/user/$USER_ID/accessRequest/api-access")
echo $AR | python3 -m json.tool
AR_ID=$(echo $AR | python3 -c "import sys,json; print(json.load(sys.stdin)['id'])")

# 9. Fetch access requests for user+key
curl -s "$BASE/api/user/$USER_ID/accessRequest/api-access" | python3 -m json.tool

# 10. Accept terms v1.0
ACCEPTANCE=$(curl -s -X POST $BASE/api/userTermsAcceptance \
  -H 'Content-Type: application/json' \
  -d "{\"userId\":\"$USER_ID\",\"termsVersion\":\"1.0\"}")
echo $ACCEPTANCE | python3 -m json.tool
ACCEPTANCE_ID=$(echo $ACCEPTANCE | python3 -c "import sys,json; print(json.load(sys.stdin)['id'])")

# 11. Accept terms v2.0
V2=$(curl -s -X POST $BASE/api/userTermsAcceptance \
  -H 'Content-Type: application/json' \
  -d "{\"userId\":\"$USER_ID\",\"termsVersion\":\"2.0\"}")
echo $V2 | python3 -m json.tool
V2_ID=$(echo $V2 | python3 -c "import sys,json; print(json.load(sys.stdin)['id'])")

# 12. Get latest terms acceptance (should be v2.0)
curl -s "$BASE/api/userTermsAcceptance/byUserId/$USER_ID/latest" | python3 -m json.tool

# 13. Get all terms acceptances for user
curl -s "$BASE/api/userTermsAcceptance/byUserId/$USER_ID" | python3 -m json.tool

# 14. Cleanup — delete in FK-safe order before deleting the user.
#     access_request, user_terms_acceptance, and oauth_profile all FK to user.

# 14a. Delete the v1.0 acceptance (expect 203)
curl -s -X DELETE "$BASE/api/userTermsAcceptance/$ACCEPTANCE_ID" -w "%{http_code}\n"

# 14b. Delete the v2.0 acceptance (expect 203)
curl -s -X DELETE "$BASE/api/userTermsAcceptance/$V2_ID" -w "%{http_code}\n"

# 14c. Delete the OAuth profile (expect 203)
curl -s -X DELETE "$BASE/api/oauthProfile?sub=$SUB" -w "%{http_code}\n"

# 14d. Delete the access request (expect 203)
curl -s -X DELETE "$BASE/api/accessRequest/$AR_ID" -w "%{http_code}\n"

# 15. Delete the user (expect 203)
curl -s -X DELETE "$BASE/api/user/$USER_ID" -w "%{http_code}\n"

# 16. Confirm gone (expect 204)
curl -s -o /dev/null -w "%{http_code}\n" "$BASE/api/user/$USER_ID"
```

---

## Configuration

Config is loaded from `config.json` (or `CONFIG_PATH` env var), then overridden by environment variables using `__` as the path separator with **exact camelCase key names**.

### `config.json` reference

```json
{
  "server": {
    "port": 3000
  },
  "log": {
    "level": "debug"
  },
  "db": {
    "database": "user-data-service",
    "host": "user-data-service-db",
    "port": "5432",
    "username": "user-data-service",
    "password": "user-data-service",
    "ssl": {
      "enabled": false,
      "verify": true,
      "caCertFile": ""
    },
    "minpool": 0,
    "maxpool": 10
  }
}
```

### Environment variable examples

```bash
db__host=postgres
db__password=secret
db__ssl__enabled=true
db__ssl__caCertFile=/certs/ca.pem
log__level=info
server__port=8080
```

---

## Docker / Deployment

### Build and run locally

```bash
docker compose up --build
```

Service listens on `http://localhost:3000`.

### Multi-stage Dockerfile

1. **`builder`** (`rust:1-bookworm`) — stub `main.rs` compiled first to cache all dependencies as a Docker layer; real source compiled second. No native deps required beyond the Rust toolchain.
2. **`runtime`** (`debian:bookworm-slim`) — binary, migrations, static file, and config only. Runs as unprivileged `appuser` (UID 1000).

Cold build: ~25–35 min. Subsequent builds with dep-cache layer hit: ~2 min.

---

## Development

### Prerequisites

- Rust 1.75+
- PostgreSQL 14+

### Run locally

```bash
# start postgres first, e.g. via docker compose up user-data-service-db
db__host=localhost cargo run
```

### Compile check (no database needed)

```bash
cargo check
```
