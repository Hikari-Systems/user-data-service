# User Data Service

A REST API service for managing user profiles, OAuth profiles, terms acceptance, and access requests in a PostgreSQL database. Built with Express.js, TypeScript, and Knex.js.

## Features

- **User Management**: Create, read, update, and delete user profiles
- **OAuth Profile Management**: Store and manage OAuth provider profiles linked to users
- **Terms Acceptance Tracking**: Track user acceptance of terms and conditions by version
- **Access Request Management**: Manage access requests with grant/deny functionality and time-based access windows
- **Database Migrations**: Automatic database schema migrations on startup
- **Health Check Endpoint**: Built-in health check for monitoring and load balancers

## Prerequisites

- Node.js 22+ (for local development)
- Docker and Docker Compose (for containerized deployment)
- PostgreSQL database (included in Docker Compose setup)
- GitHub Personal Access Token (for accessing private npm packages during build)

## Installation

### Using Docker Compose (Recommended)

1. Clone the repository:
```bash
git clone <repository-url>
cd user-data-service
```

2. Set up your GitHub API key as an environment variable:
```bash
export GH_API_KEY=your_github_personal_access_token
```

3. Build and start the services:
```bash
docker-compose up --build
```

The service will be available at `http://localhost:3000`.

### Local Development Setup

1. Install dependencies:
```bash
npm install
```

2. Set up your environment variables or configuration file (see Configuration section)

3. Ensure PostgreSQL is running and accessible

4. Build the project:
```bash
npm run build
npm run build:esbuild
```

5. Start the server:
```bash
npm start
```

## Configuration

Configuration is managed through environment variables or a `config.json` file. The service uses the `@hikari-systems/hs.utils` config system which supports nested configuration keys.

### Configuration Options

#### Server Configuration
- `server:port` - Port number for the HTTP server (default: `3000`)

#### Database Configuration
- `db:host` - PostgreSQL hostname (default: `user-data-service-db`)
- `db:port` - PostgreSQL port (default: `5432`)
- `db:database` - Database name (default: `user-data-service`)
- `db:username` - Database username (default: `user-data-service`)
- `db:password` - Database password (default: `user-data-service`)
- `db:debug` - Enable Knex query debugging (default: `false`)
- `db:minpool` - Minimum connection pool size (default: `0`)
- `db:maxpool` - Maximum connection pool size (default: `10`)

#### SSL Configuration (Optional)
- `db:ssl:enabled` - Enable SSL connections (default: `false`)
- `db:ssl:verify` - Verify SSL certificate (default: `false`)
- `db:ssl:caCertFile` - Path to CA certificate file (optional)

#### Logging Configuration
- `log:level` - Log level (default: `debug`)

### Example Configuration

Create a `config.json` file:
```json
{
  "server": {
    "port": 3000
  },
  "log": {
    "level": "info"
  },
  "db": {
    "host": "localhost",
    "port": "5432",
    "database": "user-data-service",
    "username": "user-data-service",
    "password": "your-password"
  }
}
```

Or use environment variables:
```bash
export db__host=localhost
export db__port=5432
export db__database=user-data-service
export db__username=user-data-service
export db__password=your-password
export server__port=3000
```

## Docker Container Usage

### Building the Container

The Dockerfile uses a multi-stage build:
1. **Builder stage**: Installs dependencies and builds the application
2. **Runtime stage**: Creates a minimal Debian-based image with only the compiled application

To build manually:
```bash
docker build --secret id=ghapikey,env=GH_API_KEY -t user-data-service .
```

### Running the Container

```bash
docker run -p 3000:3000 \
  -e db__host=your-db-host \
  -e db__username=your-username \
  -e db__password=your-password \
  -e db__database=your-database \
  user-data-service
```

### Docker Compose

The included `docker-compose.yml` sets up:
- **app**: The user-data-service application
- **user-data-service-db**: PostgreSQL database with health checks

The database automatically initializes with the configured credentials and the application runs migrations on startup.

## API Documentation

All API endpoints are prefixed with `/api`. The service also serves static files from the `/static` directory.

### Health Check

```
GET /healthcheck
```

Returns `200 OK` if the service is running.

### User Endpoints

#### Get User by ID
```
GET /api/user/:id
```

Returns a user by their UUID.

**Response**: `200 OK` with user object, or `204 No Content` if not found

#### Get User by Email
```
GET /api/user/byEmail?email=user@example.com
```

Returns a user by their email address.

**Response**: `200 OK` with user object, or `204 No Content` if not found

#### Create User
```
POST /api/user
Content-Type: application/json

{
  "email": "user@example.com",
  "name": "John Doe",
  "picture": "https://example.com/avatar.jpg",
  "pictureImageServiceId": "uuid-here"
}
```

Creates a new user. The `id` is automatically generated as a UUID.

**Response**: `201 Created` with the created user object

#### Update User
```
PUT /api/user/:id
Content-Type: application/json

{
  "email": "user@example.com",
  "name": "John Doe",
  "picture": "https://example.com/avatar.jpg",
  "pictureImageServiceId": "uuid-here"
}
```

Updates an existing user or creates one if it doesn't exist (upsert).

**Response**: `200 OK` with the updated user object

#### Delete User
```
DELETE /api/user/:id
```

Deletes a user by ID.

**Response**: `203 No Content` on success, or `204 No Content` if user not found

### OAuth Profile Endpoints

#### Get OAuth Profile by User ID
```
GET /api/oauthProfile/byUserId/:userId
```

Returns the OAuth profile for a given user ID.

**Response**: `200 OK` with OAuth profile object, or `204 No Content` if not found

#### Get OAuth Profile by Sub
```
GET /api/oauthProfile/bySub?sub=oauth-subject-id
```

Returns the OAuth profile by the OAuth provider's subject ID.

**Response**: `200 OK` with OAuth profile object, or `204 No Content` if not found

#### Create or Update OAuth Profile
```
PUT /api/oauthProfile
Content-Type: application/json

{
  "sub": "oauth-subject-id",
  "userId": "user-uuid",
  "profileJson": "{\"name\":\"John Doe\",\"email\":\"user@example.com\"}"
}
```

Creates or updates an OAuth profile. The `profileJson` should be a JSON string.

**Response**: `200 OK` with the OAuth profile object

#### Delete OAuth Profile
```
DELETE /api/oauthProfile?sub=oauth-subject-id
```

Deletes an OAuth profile by subject ID.

**Response**: `203 No Content` on success, or `204 No Content` if not found

### User Terms Acceptance Endpoints

#### Get Terms Acceptance by ID
```
GET /api/userTermsAcceptance/:id
```

Returns a terms acceptance record by ID.

**Response**: `200 OK` with acceptance object, or `204 No Content` if not found

#### Get Terms Acceptances by User ID
```
GET /api/userTermsAcceptance/byUserId/:userId
```

Returns all terms acceptance records for a user, ordered by acceptance date (newest first).

**Response**: `200 OK` with array of acceptance objects

#### Get Latest Terms Acceptance by User ID
```
GET /api/userTermsAcceptance/byUserId/:userId/latest
```

Returns the most recent terms acceptance for a user.

**Response**: `200 OK` with acceptance object, or `204 No Content` if not found

#### Create Terms Acceptance
```
POST /api/userTermsAcceptance
Content-Type: application/json

{
  "userId": "user-uuid",
  "termsVersion": "1.0"
}
```

Creates a new terms acceptance record. The `id` and `acceptedAt` timestamp are automatically generated.

**Response**: `201 Created` with the created acceptance object

#### Delete Terms Acceptance
```
DELETE /api/userTermsAcceptance/:id
```

Deletes a terms acceptance record by ID.

**Response**: `203 No Content` on success, or `204 No Content` if not found

### Access Request Endpoints

#### Get Access Request
```
GET /api/user/:id/accessRequest/:key
```

Returns access requests for a user and key combination.

**Response**: `200 OK` with array of access request objects

#### Create Access Request
```
POST /api/user/:userId/accessRequest/:key
```

Creates a new access request. The `id` is automatically generated as a UUID.

**Response**: `201 Created` with the created access request object

## Database Schema

The service uses PostgreSQL with the following main tables:

### user
- `id` (UUID, Primary Key)
- `email` (String, Required)
- `name` (String, Optional)
- `picture` (String, Optional)
- `pictureImageServiceId` (UUID, Optional)
- `createdAt` (Timestamp)
- `updatedAt` (Timestamp)

### oauthProfile
- `sub` (String, Primary Key) - OAuth provider subject ID
- `userId` (UUID, Foreign Key → user.id)
- `profileJson` (JSONB) - OAuth provider profile data
- `createdAt` (Timestamp)
- `updatedAt` (Timestamp)

### userTermsAcceptance
- `id` (UUID, Primary Key)
- `userId` (UUID, Required)
- `termsVersion` (String, Required)
- `acceptedAt` (Timestamp, Required)
- `createdAt` (Timestamp)
- `updatedAt` (Timestamp)

### accessRequest
- `id` (UUID, Primary Key)
- `userId` (UUID, Foreign Key → user.id)
- `key` (String, Required)
- `granted` (Boolean, Optional)
- `decidedAt` (Timestamp, Optional)
- `grantedFrom` (Timestamp, Optional)
- `grantedUntil` (Timestamp, Optional)
- `createdAt` (Timestamp)
- `updatedAt` (Timestamp)

## Development

### Project Structure

```
user-data-service/
├── lib/
│   ├── model/          # Database models
│   ├── route/          # API route handlers
│   ├── index.ts        # Express router setup
│   ├── server.ts       # Server entry point
│   └── knexfile.ts     # Knex configuration
├── migrations/         # Database migrations
├── static/             # Static files
├── __tests__/          # Test files
├── Dockerfile          # Container build definition
├── docker-compose.yml  # Docker Compose configuration
└── config.json         # Configuration file
```

### Available Scripts

- `npm test` - Run tests
- `npm run testci` - Run tests in CI mode (bail on first failure)
- `npm run lint` - Run ESLint
- `npm run build` - Build TypeScript to JavaScript
- `npm run build:esbuild` - Bundle JavaScript with esbuild
- `npm run watch` - Watch mode for TypeScript compilation
- `npm start` - Start the server (requires build first)

### Code Style

The project uses:
- ESLint with Airbnb base configuration
- Prettier for code formatting
- TypeScript for type safety

### Database Migrations

Migrations are automatically run on server startup. To create a new migration:

1. Create a new file in `migrations/` with the format: `YYYYMMDDHHMM_description.ts`
2. Export `up` and `down` functions that return Knex schema operations
3. The migration will run automatically on next server start

Example migration:
```typescript
import { Knex } from 'knex';

export const up = (knex: Knex) =>
  knex.schema.createTable('example', (t) => {
    t.uuid('id').primary();
    t.string('name').notNullable();
    t.timestamps();
  });

export const down = (knex: Knex) =>
  knex.schema.dropTable('example');
```

## Testing

Run tests with:
```bash
npm test
```

For CI environments:
```bash
npm run testci
```

## License

Copyright 2026 Rick Knowles <rick.knowles@hikari-systems.com>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
