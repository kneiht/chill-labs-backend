# Rotating Refresh Tokens Strategy

## What Changed?

**Previous Implementation:**
- Refresh endpoint returned only a new `access_token`
- Refresh token stayed the same forever
- Users had to re-login after refresh token expired (30 days)

**New Implementation (October 2025):**
- Refresh endpoint returns BOTH `access_token` AND `refresh_token`
- Each refresh gives you a fresh refresh token (resets the 30-day timer)
- Users stay logged in indefinitely as long as they use the app

---

## Why This Change?

This implements the **same strategy as Facebook, Google, and other major platforms**:

1. **Better User Experience**: Users don't need to login again after 30 days
2. **Continuous Sessions**: As long as you use the app regularly, you stay logged in forever
3. **Security**: Refresh tokens rotate, making stolen tokens expire faster
4. **Industry Standard**: This is how most modern applications work

---

## How It Works

### Token Lifecycle:

```
Day 0:  Login → Get Access Token (1h) + Refresh Token (30d)
Day 0:  After 1 hour → Access token expires
Day 0:  App auto-refreshes → Get NEW Access Token (1h) + NEW Refresh Token (30d)
Day 1:  After 1 hour → Access token expires again
Day 1:  App auto-refreshes → Get NEW tokens again
...
Day 29: App auto-refreshes → Get NEW tokens (30-day timer resets!)
Day 59: Still logged in! → Keep refreshing indefinitely
```

### If User Stops Using App:

```
Day 0:  Login → Get tokens
Day 15: User stops using app
Day 45: User tries to use app → Refresh token expired → MUST LOGIN AGAIN
```

---

## API Response Changes

### Before (OLD):
```json
POST /auth/refresh

Response:
{
  "success": true,
  "data": {
    "access_token": "new_access_token_here"
  }
}
```

### After (NEW):
```json
POST /auth/refresh

Response:
{
  "success": true,
  "data": {
    "access_token": "new_access_token_here",
    "refresh_token": "NEW_refresh_token_here",  ← MUST STORE THIS!
    "user": {
      "id": "...",
      "display_name": "...",
      "username": "...",
      "email": "...",
      "role": "...",
      "status": "..."
    }
  }
}
```

---

## Frontend Implementation

### ❌ WRONG Implementation:
```javascript
async refreshAccessToken() {
  const response = await fetch('/auth/refresh', {
    method: 'POST',
    body: JSON.stringify({ token: refreshToken })
  });
  
  const data = await response.json();
  
  // WRONG: Only storing access token
  localStorage.setItem('access_token', data.data.access_token);
  
  // This causes users to logout after 30 days!
}
```

### ✅ CORRECT Implementation:
```javascript
async refreshAccessToken() {
  const response = await fetch('/auth/refresh', {
    method: 'POST',
    body: JSON.stringify({ token: refreshToken })
  });
  
  const data = await response.json();
  
  if (data.success) {
    // CORRECT: Store ALL tokens (access + refresh + user)
    localStorage.setItem('access_token', data.data.access_token);
    localStorage.setItem('refresh_token', data.data.refresh_token);  // ← CRITICAL!
    localStorage.setItem('user', JSON.stringify(data.data.user));
    return data.data.access_token;
  }
}
```

---

## Complete Example (React/TypeScript)

```typescript
// auth.service.ts
interface AuthTokens {
  access_token: string;
  refresh_token: string;
  user: UserInfo;
}

class AuthService {
  private baseURL = 'http://localhost:3000';

  // Store all auth data
  private storeTokens(data: AuthTokens): void {
    localStorage.setItem('access_token', data.access_token);
    localStorage.setItem('refresh_token', data.refresh_token);
    localStorage.setItem('user', JSON.stringify(data.user));
  }

  // Refresh tokens (rotating strategy)
  async refreshTokens(): Promise<string> {
    const refreshToken = localStorage.getItem('refresh_token');
    
    if (!refreshToken) {
      throw new Error('No refresh token');
    }

    const response = await fetch(`${this.baseURL}/auth/refresh`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ token: refreshToken })
    });

    const result = await response.json();
    
    if (!result.success) {
      // Refresh token expired - user must login
      this.logout();
      throw new Error('Session expired');
    }

    // Store ALL new tokens
    this.storeTokens(result.data);
    
    return result.data.access_token;
  }

  // API call with automatic token refresh
  async apiCall<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
    const accessToken = localStorage.getItem('access_token');
    
    options.headers = {
      ...options.headers,
      'Authorization': `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    };

    let response = await fetch(`${this.baseURL}${endpoint}`, options);
    
    // If 401, refresh and retry
    if (response.status === 401) {
      try {
        const newAccessToken = await this.refreshTokens();
        
        // Retry with new token
        options.headers = {
          ...options.headers,
          'Authorization': `Bearer ${newAccessToken}`,
        };
        response = await fetch(`${this.baseURL}${endpoint}`, options);
      } catch {
        throw new Error('Session expired');
      }
    }

    const data = await response.json();
    return data.data;
  }

  logout(): void {
    localStorage.removeItem('access_token');
    localStorage.removeItem('refresh_token');
    localStorage.removeItem('user');
  }
}

export const authService = new AuthService();
```

---

## Axios Interceptor Example

```javascript
import axios from 'axios';

const api = axios.create({
  baseURL: 'http://localhost:3000',
});

// Request interceptor: Add access token
api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('access_token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => Promise.reject(error)
);

// Response interceptor: Handle 401 and refresh tokens
api.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config;

    // If 401 and haven't retried yet
    if (error.response?.status === 401 && !originalRequest._retry) {
      originalRequest._retry = true;

      try {
        const refreshToken = localStorage.getItem('refresh_token');
        
        const response = await axios.post(
          'http://localhost:3000/auth/refresh',
          { token: refreshToken }
        );

        const { access_token, refresh_token, user } = response.data.data;
        
        // Store ALL new tokens - CRITICAL!
        localStorage.setItem('access_token', access_token);
        localStorage.setItem('refresh_token', refresh_token);
        localStorage.setItem('user', JSON.stringify(user));

        // Retry original request with new token
        originalRequest.headers.Authorization = `Bearer ${access_token}`;
        return api(originalRequest);
      } catch {
        // Refresh failed - logout
        localStorage.clear();
        window.location.href = '/login';
        return Promise.reject(error);
      }
    }

    return Promise.reject(error);
  }
);

export default api;
```

---

## Testing Checklist

### Test Case 1: Token Refresh Works
1. Login to app
2. Wait 1 hour (or change token expiration to 1 minute for testing)
3. Make an API call
4. ✅ Should automatically refresh and succeed
5. ✅ Check localStorage - both tokens should be updated

### Test Case 2: Session Extension
1. Login to app
2. Use app regularly for 40 days
3. ✅ Should still be logged in (refresh token keeps resetting)

### Test Case 3: Inactivity Logout
1. Login to app
2. Don't use app for 31 days
3. Try to use app
4. ✅ Should be logged out (refresh token expired)

### Test Case 4: Old Token Rejected
1. Login to app
2. Trigger a refresh (wait 1 hour)
3. Try to use the OLD refresh token
4. ✅ Should fail (old token is no longer valid)

---

## Common Mistakes

### ❌ Mistake #1: Not Storing New Refresh Token
```javascript
// WRONG!
const data = await refreshEndpoint();
localStorage.setItem('access_token', data.access_token);
// Missing: localStorage.setItem('refresh_token', data.refresh_token);
```

**Result**: Users logout after 30 days even if active

---

### ❌ Mistake #2: Not Handling 401 Automatically
```javascript
// WRONG!
const response = await fetch('/api/users', {
  headers: { Authorization: `Bearer ${accessToken}` }
});

if (response.status === 401) {
  // Immediately redirect to login without trying refresh
  window.location.href = '/login';
}
```

**Result**: Users get logged out every hour

---

### ❌ Mistake #3: Concurrent Refresh Requests
```javascript
// WRONG!
// Multiple API calls happening at same time, each trying to refresh
apiCall1(); // Triggers refresh
apiCall2(); // Triggers refresh again
apiCall3(); // Triggers refresh again
```

**Result**: Race conditions, token conflicts

**Solution**: Implement a refresh lock/queue

---

## Migration Guide

If you have existing frontend code using the OLD API:

### Step 1: Update refresh function
Change from:
```javascript
localStorage.setItem('access_token', data.data.access_token);
```

To:
```javascript
localStorage.setItem('access_token', data.data.access_token);
localStorage.setItem('refresh_token', data.data.refresh_token);
localStorage.setItem('user', JSON.stringify(data.data.user));
```

### Step 2: Test thoroughly
- Test token refresh works
- Test session extends beyond 30 days
- Test inactive users get logged out

### Step 3: Deploy
- Backend and frontend can be deployed independently
- Backend is backward compatible (returns more data, but old frontends will ignore it)
- Update frontend at your convenience

---

## Token Expiration Configuration

Default settings (in `.env`):
```bash
APP__JWT__ACCESS_TOKEN_EXPIRATION_HOURS=1     # Access token: 1 hour
APP__JWT__REFRESH_TOKEN_EXPIRATION_HOURS=720  # Refresh token: 30 days
```

For testing, you can temporarily set:
```bash
APP__JWT__ACCESS_TOKEN_EXPIRATION_HOURS=0.0167  # 1 minute
APP__JWT__REFRESH_TOKEN_EXPIRATION_HOURS=0.0333  # 2 minutes
```

---

## Security Benefits

1. **Token Rotation**: Each refresh invalidates the old refresh token
2. **Reduced Window**: Stolen tokens have shorter validity
3. **Audit Trail**: Can track token refresh patterns for suspicious activity
4. **Revocation**: Can implement server-side token blacklisting easier

---

## Questions?

**Q: What if refresh token expires while app is open?**
A: User will get 401 on next API call → automatic refresh will fail → redirect to login

**Q: Can I still use the old refresh token after getting a new one?**
A: No, old refresh tokens become invalid once a new one is issued

**Q: Do I need to update all frontend apps immediately?**
A: No, the API is backward compatible. Old apps will work but won't get session extension benefits.

**Q: What happens if user has multiple devices?**
A: Each device gets its own tokens. Each refresh is independent per device.

---

**Last Updated**: October 2025
**Implementation Date**: October 2025
