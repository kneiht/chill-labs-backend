# Frontend Authentication Implementation Guide

This guide provides complete instructions for implementing authentication in your frontend application using the Chill Labs API.

## Table of Contents
- [Base URL Configuration](#base-url-configuration)
- [Response Format](#response-format)
- [Authentication Endpoints](#authentication-endpoints)
- [User Registration](#user-registration)
- [User Login](#user-login)
- [Token Refresh](#token-refresh)
- [Get Current User](#get-current-user)
- [Token Management](#token-management)
- [Error Handling](#error-handling)
- [Complete Example](#complete-example)

---

## Base URL Configuration

The API runs on port **3000** by default. Configure your base URL:

```javascript
const API_BASE_URL = 'http://localhost:3000';
```

For production, replace with your actual domain:
```javascript
const API_BASE_URL = 'https://api.yourdomain.com';
```

---

## Response Format

All API responses follow a standardized format:

### Success Response
```json
{
  "success": true,
  "message": "Operation successful message",
  "status": "Ok" | "Created" | "NoContent",
  "data": { /* response data */ },
  "pagination": { /* optional pagination info */ }
}
```

### Error Response
```json
{
  "success": false,
  "message": "Error message",
  "status": "Validation" | "NotFound" | "Unauthorized" | "Forbidden" | "Internal" | "Conflict",
  "error": "Detailed error description (optional)"
}
```

### HTTP Status Codes
- **200 OK**: Successful request
- **400 Bad Request**: Validation errors
- **401 Unauthorized**: Invalid or missing authentication
- **403 Forbidden**: Insufficient permissions or account suspended
- **404 Not Found**: Resource not found
- **409 Conflict**: Username/email already exists
- **500 Internal Server Error**: Server error

---

## Authentication Endpoints

All authentication endpoints are prefixed with `/auth`:

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| POST | `/auth/register` | Register new user | No |
| POST | `/auth/login` | Login user | No |
| POST | `/auth/refresh` | Refresh access token | No |
| GET | `/auth/me` | Get current user info | Yes |

---

## User Registration

### Endpoint
```
POST /auth/register
```

### Request Body
```json
{
  "display_name": "John Doe",
  "username": "johndoe",
  "email": "john@example.com",
  "password": "SecureP@ss123"
}
```

### Field Requirements
- **display_name** (optional): Non-empty string
- **username** (optional): Minimum 3 characters
- **email** (optional): Valid email format
- **password** (required): Minimum 8 characters
- **Note**: At least one of `username` or `email` must be provided

### Success Response (201 Created)
```json
{
  "success": true,
  "message": "User registered successfully",
  "status": "Created",
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user": {
      "id": "01234567-89ab-cdef-0123-456789abcdef",
      "display_name": "John Doe",
      "username": "johndoe",
      "email": "john@example.com",
      "role": "Student",
      "status": "Pending"
    }
  }
}
```

### User Roles
- **Student**: Default role for new registrations
- **Teacher**: Assigned by admin
- **Admin**: Highest permission level

### User Status
- **Pending**: Newly registered (can still login)
- **Active**: Fully activated account
- **Suspended**: Account disabled (cannot login)

### Example Code
```javascript
async function register(displayName, username, email, password) {
  const response = await fetch(`${API_BASE_URL}/auth/register`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      display_name: displayName,
      username: username,
      email: email,
      password: password
    })
  });

  const data = await response.json();
  
  if (data.success) {
    // Store tokens
    localStorage.setItem('access_token', data.data.access_token);
    localStorage.setItem('refresh_token', data.data.refresh_token);
    localStorage.setItem('user', JSON.stringify(data.data.user));
    return data.data;
  } else {
    throw new Error(data.message);
  }
}
```

### Common Errors
- **400 Validation**: Missing required fields or invalid format
- **409 Conflict**: Username or email already exists

---

## User Login

### Endpoint
```
POST /auth/login
```

### Request Body
```json
{
  "login": "johndoe",
  "password": "SecureP@ss123"
}
```

### Field Requirements
- **login** (required): Username or email (automatically detected)
- **password** (required): User's password

**Note**: The API automatically detects if `login` contains `@` to determine if it's an email or username.

### Success Response (200 OK)
```json
{
  "success": true,
  "message": "Login successful",
  "status": "Ok",
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user": {
      "id": "01234567-89ab-cdef-0123-456789abcdef",
      "display_name": "John Doe",
      "username": "johndoe",
      "email": "john@example.com",
      "role": "Student",
      "status": "Active"
    }
  }
}
```

### Example Code
```javascript
async function login(loginIdentifier, password) {
  const response = await fetch(`${API_BASE_URL}/auth/login`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      login: loginIdentifier,
      password: password
    })
  });

  const data = await response.json();
  
  if (data.success) {
    // Store tokens
    localStorage.setItem('access_token', data.data.access_token);
    localStorage.setItem('refresh_token', data.data.refresh_token);
    localStorage.setItem('user', JSON.stringify(data.data.user));
    return data.data;
  } else {
    throw new Error(data.message);
  }
}
```

### Common Errors
- **400 Validation**: Missing credentials
- **401 Unauthorized**: Invalid credentials
- **403 Forbidden**: Account is suspended
- **404 Not Found**: User does not exist

---

## Token Refresh

### Endpoint
```
POST /auth/refresh
```

### Request Body
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### Field Requirements
- **token** (required): The refresh token received during login/registration

### Success Response (200 OK)
```json
{
  "success": true,
  "message": "Token refreshed successfully",
  "status": "Ok",
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "user": {
      "id": "01234567-89ab-cdef-0123-456789abcdef",
      "display_name": "John Doe",
      "username": "johndoe",
      "email": "john@example.com",
      "role": "Student",
      "status": "Active"
    }
  }
}
```

**Important Notes - Rotating Refresh Token Strategy**:
- **BOTH access token AND refresh token are returned**
- The new refresh token **MUST replace the old one** in storage
- This is called "rotating refresh tokens" - each refresh extends your session
- Sessions remain active indefinitely as long as you refresh within the expiration window
- Default configuration: Access tokens expire in 1 hour, refresh tokens in 30 days
- If you don't use the app for 30 days, you'll need to login again
- This strategy is used by major platforms like Facebook to keep users logged in

### Example Code
```javascript
async function refreshAccessToken() {
  const refreshToken = localStorage.getItem('refresh_token');
  
  if (!refreshToken) {
    throw new Error('No refresh token available');
  }

  const response = await fetch(`${API_BASE_URL}/auth/refresh`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      token: refreshToken
    })
  });

  const data = await response.json();
  
  if (data.success) {
    // Update BOTH tokens - critical for rotating refresh token strategy!
    localStorage.setItem('access_token', data.data.access_token);
    localStorage.setItem('refresh_token', data.data.refresh_token);
    localStorage.setItem('user', JSON.stringify(data.data.user));
    return data.data.access_token;
  } else {
    // Refresh token is invalid or expired - redirect to login
    localStorage.clear();
    throw new Error('Session expired. Please login again.');
  }
}
```

### Common Errors
- **401 Unauthorized**: Invalid or expired refresh token (redirect to login)
- **403 Forbidden**: Account suspended

---

## Get Current User

### Endpoint
```
GET /auth/me
```

### Headers
```
Authorization: Bearer <access_token>
```

### Success Response (200 OK)
```json
{
  "success": true,
  "message": "User retrieved successfully",
  "status": "Ok",
  "data": {
    "id": "01234567-89ab-cdef-0123-456789abcdef",
    "display_name": "John Doe",
    "username": "johndoe",
    "email": "john@example.com",
    "role": "Student",
    "status": "Active"
  }
}
```

### Example Code
```javascript
async function getCurrentUser() {
  const accessToken = localStorage.getItem('access_token');
  
  if (!accessToken) {
    throw new Error('No access token available');
  }

  const response = await fetch(`${API_BASE_URL}/auth/me`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    }
  });

  const data = await response.json();
  
  if (data.success) {
    return data.data;
  } else {
    throw new Error(data.message);
  }
}
```

### Common Errors
- **401 Unauthorized**: Missing, invalid, or expired access token
- **403 Forbidden**: Account suspended

---

## Token Management

### Token Storage
Store tokens securely in your application:

```javascript
// After successful login or registration
function storeAuthData(authData) {
  localStorage.setItem('access_token', authData.access_token);
  localStorage.setItem('refresh_token', authData.refresh_token);
  localStorage.setItem('user', JSON.stringify(authData.user));
}

// Retrieve tokens
function getAccessToken() {
  return localStorage.getItem('access_token');
}

function getRefreshToken() {
  return localStorage.getItem('refresh_token');
}

// Clear tokens on logout
function logout() {
  localStorage.removeItem('access_token');
  localStorage.removeItem('refresh_token');
  localStorage.removeItem('user');
  // Redirect to login page
}
```

### Automatic Token Refresh
Implement automatic token refresh when API calls fail with 401:

```javascript
async function apiCall(url, options = {}) {
  const accessToken = getAccessToken();
  
  // Add authorization header
  options.headers = {
    ...options.headers,
    'Authorization': `Bearer ${accessToken}`,
    'Content-Type': 'application/json',
  };

  let response = await fetch(url, options);
  
  // If unauthorized, try refreshing token
  if (response.status === 401) {
    try {
      const newAccessToken = await refreshAccessToken();
      
      // Retry the original request with new token
      options.headers['Authorization'] = `Bearer ${newAccessToken}`;
      response = await fetch(url, options);
    } catch (error) {
      // Refresh failed - redirect to login
      logout();
      window.location.href = '/login';
      throw new Error('Session expired');
    }
  }

  return response.json();
}
```

### Protected API Requests
Always include the access token in the Authorization header:

```javascript
async function makeProtectedRequest(endpoint, method = 'GET', body = null) {
  const accessToken = getAccessToken();
  
  if (!accessToken) {
    throw new Error('Not authenticated');
  }

  const options = {
    method: method,
    headers: {
      'Authorization': `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    }
  };

  if (body) {
    options.body = JSON.stringify(body);
  }

  return await apiCall(`${API_BASE_URL}${endpoint}`, options);
}
```

---

## Error Handling

### Comprehensive Error Handler
```javascript
function handleApiError(data) {
  const errorMessages = {
    'Validation': 'Please check your input and try again.',
    'Unauthorized': 'Invalid credentials or session expired.',
    'Forbidden': 'You do not have permission to perform this action.',
    'NotFound': 'The requested resource was not found.',
    'Conflict': 'This username or email is already taken.',
    'Internal': 'An internal server error occurred. Please try again later.'
  };

  const errorType = data.status;
  const defaultMessage = errorMessages[errorType] || 'An error occurred.';
  
  // Use the specific error message from API if available
  const errorMessage = data.error || data.message || defaultMessage;
  
  return {
    type: errorType,
    message: errorMessage
  };
}
```

### Usage Example
```javascript
try {
  const userData = await login('johndoe', 'password123');
  console.log('Login successful:', userData);
} catch (error) {
  const errorDetails = handleApiError(error);
  console.error('Login failed:', errorDetails.message);
  // Display error to user
  showErrorNotification(errorDetails.message);
}
```

---

## Complete Example

Here's a complete authentication service for your frontend:

```javascript
class AuthService {
  constructor(baseURL) {
    this.baseURL = baseURL;
  }

  // Store tokens and user data
  storeAuthData(authData) {
    localStorage.setItem('access_token', authData.access_token);
    localStorage.setItem('refresh_token', authData.refresh_token);
    localStorage.setItem('user', JSON.stringify(authData.user));
  }

  // Get stored tokens
  getAccessToken() {
    return localStorage.getItem('access_token');
  }

  getRefreshToken() {
    return localStorage.getItem('refresh_token');
  }

  getUser() {
    const user = localStorage.getItem('user');
    return user ? JSON.parse(user) : null;
  }

  // Check if user is authenticated
  isAuthenticated() {
    return !!this.getAccessToken();
  }

  // Register new user
  async register(displayName, username, email, password) {
    const response = await fetch(`${this.baseURL}/auth/register`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        display_name: displayName,
        username: username,
        email: email,
        password: password
      })
    });

    const data = await response.json();
    
    if (data.success) {
      this.storeAuthData(data.data);
      return data.data;
    } else {
      throw new Error(data.message || 'Registration failed');
    }
  }

  // Login user
  async login(loginIdentifier, password) {
    const response = await fetch(`${this.baseURL}/auth/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        login: loginIdentifier,
        password: password
      })
    });

    const data = await response.json();
    
    if (data.success) {
      this.storeAuthData(data.data);
      return data.data;
    } else {
      throw new Error(data.message || 'Login failed');
    }
  }

  // Refresh access token (rotating refresh token strategy)
  async refreshAccessToken() {
    const refreshToken = this.getRefreshToken();
    
    if (!refreshToken) {
      throw new Error('No refresh token available');
    }

    const response = await fetch(`${this.baseURL}/auth/refresh`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ token: refreshToken })
    });

    const data = await response.json();
    
    if (data.success) {
      // Store BOTH new tokens - critical for rotating refresh token strategy!
      this.storeAuthData(data.data);
      return data.data.access_token;
    } else {
      this.logout();
      throw new Error('Session expired. Please login again.');
    }
  }

  // Get current user info
  async getCurrentUser() {
    const accessToken = this.getAccessToken();
    
    if (!accessToken) {
      throw new Error('Not authenticated');
    }

    const response = await fetch(`${this.baseURL}/auth/me`, {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${accessToken}`,
        'Content-Type': 'application/json'
      }
    });

    const data = await response.json();
    
    if (data.success) {
      localStorage.setItem('user', JSON.stringify(data.data));
      return data.data;
    } else {
      throw new Error(data.message || 'Failed to get user info');
    }
  }

  // Make authenticated API request with auto-refresh
  async apiCall(endpoint, options = {}) {
    const accessToken = this.getAccessToken();
    
    if (!accessToken) {
      throw new Error('Not authenticated');
    }

    options.headers = {
      ...options.headers,
      'Authorization': `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    };

    let response = await fetch(`${this.baseURL}${endpoint}`, options);
    
    // Auto-refresh on 401
    if (response.status === 401) {
      try {
        await this.refreshAccessToken();
        
        // Retry with new token
        options.headers['Authorization'] = `Bearer ${this.getAccessToken()}`;
        response = await fetch(`${this.baseURL}${endpoint}`, options);
      } catch (error) {
        this.logout();
        throw new Error('Session expired');
      }
    }

    return response.json();
  }

  // Logout
  logout() {
    localStorage.removeItem('access_token');
    localStorage.removeItem('refresh_token');
    localStorage.removeItem('user');
  }
}

// Usage
const auth = new AuthService('http://localhost:3000');

// Register
try {
  const user = await auth.register('John Doe', 'johndoe', 'john@example.com', 'password123');
  console.log('Registered:', user);
} catch (error) {
  console.error('Registration failed:', error.message);
}

// Login
try {
  const user = await auth.login('johndoe', 'password123');
  console.log('Logged in:', user);
} catch (error) {
  console.error('Login failed:', error.message);
}

// Get current user
try {
  const user = await auth.getCurrentUser();
  console.log('Current user:', user);
} catch (error) {
  console.error('Failed to get user:', error.message);
}

// Make protected API call
try {
  const data = await auth.apiCall('/users/profile');
  console.log('Profile data:', data);
} catch (error) {
  console.error('API call failed:', error.message);
}

// Logout
auth.logout();
```

---

## Best Practices

1. **Rotating Refresh Tokens**: **CRITICAL** - Always update BOTH access and refresh tokens when calling the `/auth/refresh` endpoint. The API returns a new refresh token that extends your session. Failing to update the refresh token will cause users to be logged out after 30 days.

2. **Secure Token Storage**: Consider using secure storage mechanisms appropriate for your platform (e.g., httpOnly cookies for web, secure storage for mobile)

3. **Automatic Token Refresh**: Access tokens expire in 1 hour. Implement automatic refresh logic in your API interceptor to provide seamless user experience. With rotating refresh tokens, users stay logged in indefinitely as long as they use the app within 30 days.

4. **Error Handling**: Always handle authentication errors gracefully and redirect users to login when sessions expire

5. **HTTPS Only**: Always use HTTPS in production to protect tokens during transmission

6. **Logout on Critical Actions**: Clear tokens on logout, and consider clearing them when detecting potential security issues

7. **User Feedback**: Provide clear feedback to users about authentication status and errors

8. **Token Validation**: Validate tokens before making API requests to avoid unnecessary network calls

9. **Retry Logic**: Implement retry logic with exponential backoff for network failures

10. **Role-Based UI**: Use the `role` field from user data to show/hide features based on permissions

11. **Status Checks**: Check `status` field to handle suspended accounts appropriately

12. **Token Expiration Timing**: The default configuration is:
    - Access Token: 1 hour expiration
    - Refresh Token: 30 days expiration (reset on each refresh)
    - This means users stay logged in forever as long as they use the app at least once every 30 days

---

## Support

For issues or questions about authentication implementation, please refer to:
- API documentation at `/tester` endpoint
- Backend repository documentation
- Contact the backend team

---

**Last Updated**: October 2025
