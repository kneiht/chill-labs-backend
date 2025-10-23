# Auth Domain

This domain handles authentication and authorization for the English Coaching application.

## Overview

The auth domain provides:
- User registration
- User login (via email or username)
- JWT token generation and validation
- Token refresh
- Current user retrieval from JWT

## Architecture

### Components

- **`model.rs`**: Request/response DTOs
  - `RegisterRequest`: User registration payload
  - `LoginRequest`: Login payload (supports email or username)
  - `RefreshTokenRequest`: Token refresh payload
  - `AuthResponse`: Authentication response with JWT and user info
  - `UserInfo`: User information in auth responses

- **`service.rs`**: Business logic
  - `AuthService`: Core authentication service
    - `register()`: Register new users (default role: Student)
    - `login()`: Authenticate users with email/username + password
    - `refresh_token()`: Generate new JWT from existing token
    - `verify_and_get_user()`: Validate JWT and return user
    - `verify_token()`: Validate JWT and return claims

- **`handler.rs`**: HTTP handlers
  - `register`: POST /api/auth/register
  - `login`: POST /api/auth/login
  - `refresh_token`: POST /api/auth/refresh
  - `get_current_user`: GET /api/auth/me (requires Authorization header)

- **`mod.rs`**: Module definition and route registration

## API Endpoints

### 1. Register User

**POST** `/api/auth/register`

Register a new user with email and/or username.

**Request Body:**
```json
{
  "display_name": "John Doe",
  "email": "john@example.com",      // Optional (but username or email required)
  "username": "johndoe",            // Optional (but username or email required)
  "password": "SecurePassword123"   // Required (min 8 characters)
}
```

**Response (201 Created):**
```json
{
  "success": true,
  "message": "User registered successfully",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user": {
      "id": "01997199-4f31-7718-a766-687e926dd0c7",
      "display_name": "John Doe",
      "username": "johndoe",
      "email": "john@example.com",
      "role": "Student",
      "status": "Pending"
    }
  }
}
```

### 2. Login

**POST** `/api/auth/login`

Authenticate user with email or username and password.

**Request Body:**
```json
{
  "login": "john@example.com",  // Can be email or username
  "password": "SecurePassword123"
}
```

**Response (200 OK):**
```json
{
  "success": true,
  "message": "Login successful",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user": {
      "id": "01997199-4f31-7718-a766-687e926dd0c7",
      "display_name": "John Doe",
      "username": "johndoe",
      "email": "john@example.com",
      "role": "Student",
      "status": "Active"
    }
  }
}
```

### 3. Refresh Token

**POST** `/api/auth/refresh`

Generate a new JWT from an existing valid token.

**Request Body:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Response (200 OK):**
```json
{
  "success": true,
  "message": "Token refreshed successfully",
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
}
```

### 4. Get Current User

**GET** `/api/auth/me`

Retrieve current user information from JWT token.

**Headers:**
```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Response (200 OK):**
```json
{
  "success": true,
  "message": "User retrieved successfully",
  "data": {
    "id": "01997199-4f31-7718-a766-687e926dd0c7",
    "display_name": "John Doe",
    "username": "johndoe",
    "email": "john@example.com",
    "role": "Student",
    "status": "Active"
  }
}
```

## Configuration

Set JWT configuration via environment variables:

```bash
APP__JWT__SECRET=your-secret-key-here
APP__JWT__EXPIRATION_HOURS=24
```

**Defaults:**
- `secret`: "default_secret_change_in_production"
- `expiration_hours`: 24

⚠️ **Important:** Always set a strong JWT secret in production!

## Security Features

1. **Password Hashing**: Uses Argon2 (industry standard)
2. **JWT Tokens**: Signed with HMAC-SHA256
3. **Token Expiration**: Configurable expiration time
4. **Account Status Check**: Suspended/inactive accounts cannot login
5. **Unique Constraints**: Email and username uniqueness enforced

## Integration

### Adding Auth to AppState

The auth service is automatically initialized in `src/state.rs`:

```rust
use crate::domain::auth::service::AuthService;

pub struct AppState {
    pub auth_service: AuthService,
    // ... other services
}
```

### Adding Auth Routes

Routes are registered in `src/server.rs`:

```rust
use crate::domain::auth::auth_routes;

let app = Router::new()
    .nest("/api/auth", auth_routes())
    // ... other routes
```

## Usage Examples

### Registering a User

```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "display_name": "John Doe",
    "email": "john@example.com",
    "username": "johndoe",
    "password": "SecurePassword123"
  }'
```

### Logging In

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "login": "john@example.com",
    "password": "SecurePassword123"
  }'
```

### Accessing Protected Resource

```bash
curl -X GET http://localhost:3000/api/auth/me \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

## Error Handling

The auth domain uses the centralized `AppError` system:

- **Validation errors**: Invalid input (400 Bad Request)
- **Unauthorized errors**: Invalid credentials, expired token (401 Unauthorized)
- **Forbidden errors**: Suspended account (403 Forbidden)
- **Conflict errors**: Email/username already exists (409 Conflict)
- **Internal errors**: Server errors (500 Internal Server Error)

## Testing

Use the provided playground presets in `playground/presets/auth.json` to test all auth endpoints.

## Future Enhancements

Potential improvements:
- Refresh token with rotation
- Role-based access control middleware
- OAuth2 integration
- Two-factor authentication
- Password reset flow
- Email verification
- Session management
- Rate limiting for login attempts