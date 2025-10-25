# Frontend Notes Implementation Guide

This guide provides complete instructions for implementing note management in your frontend application using the Chill Labs API.

## Table of Contents
- [Base URL Configuration](#base-url-configuration)
- [Response Format](#response-format)
- [Notes Endpoints](#notes-endpoints)
- [Get All Notes](#get-all-notes)
- [Get Note](#get-note)
- [Create Note](#create-note)
- [Update Note](#update-note)
- [Delete Note](#delete-note)
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
- **201 Created**: Resource created
- **204 No Content**: Resource deleted
- **400 Bad Request**: Validation errors
- **401 Unauthorized**: Invalid or missing authentication
- **403 Forbidden**: Insufficient permissions
- **404 Not Found**: Resource not found
- **500 Internal Server Error**: Server error

---

## Notes Endpoints

All notes endpoints are prefixed with `/notes` and require authentication:

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| GET | `/notes` | Get all user's notes | Yes |
| GET | `/notes/{id}` | Get a specific note | Yes |
| POST | `/notes` | Create a new note | Yes |
| PUT | `/notes/{id}` | Update a note | Yes |
| DELETE | `/notes/{id}` | Delete a note | Yes |

---

## Get All Notes

### Endpoint
```
GET /notes
```

### Headers
```
Authorization: Bearer <access_token>
```

### Success Response (200 OK)
```json
{
  "success": true,
  "message": "Notes retrieved successfully",
  "status": "Ok",
  "data": [
    {
      "id": "01234567-89ab-cdef-0123-456789abcdef",
      "user_id": "01234567-89ab-cdef-0123-456789abcdef",
      "title": "My Note",
      "content": "Note content here",
      "created": "2025-10-24T12:00:00Z",
      "updated": "2025-10-24T12:00:00Z"
    }
  ]
}
```

### Example Code
```javascript
async function getAllNotes() {
  const accessToken = localStorage.getItem('access_token');
  
  if (!accessToken) {
    throw new Error('No access token available');
  }

  const response = await fetch(`${API_BASE_URL}/notes`, {
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
- **403 Forbidden**: Insufficient permissions (e.g., account suspended)

---

## Get Note

### Endpoint
```
GET /notes/{id}
```

### Headers
```
Authorization: Bearer <access_token>
```

### Success Response (200 OK)
```json
{
  "success": true,
  "message": "Note retrieved successfully",
  "status": "Ok",
  "data": {
    "id": "01234567-89ab-cdef-0123-456789abcdef",
    "user_id": "01234567-89ab-cdef-0123-456789abcdef",
    "title": "My Note",
    "content": "Note content here",
    "created": "2025-10-24T12:00:00Z",
    "updated": "2025-10-24T12:00:00Z"
  }
}
```

### Example Code
```javascript
async function getNote(noteId) {
  const accessToken = localStorage.getItem('access_token');
  
  if (!accessToken) {
    throw new Error('No access token available');
  }

  const response = await fetch(`${API_BASE_URL}/notes/${noteId}`, {
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
- **403 Forbidden**: No permission to access the note (not owner or admin)
- **404 Not Found**: Note does not exist

---

## Create Note

### Endpoint
```
POST /notes
```

### Headers
```
Authorization: Bearer <access_token>
```

### Request Body
```json
{
  "title": "My New Note",
  "content": "This is the content of my note."
}
```

### Field Requirements
- **title** (required): Non-empty string
- **content** (required): Non-empty string

### Success Response (201 Created)
```json
{
  "success": true,
  "message": "Note created successfully",
  "status": "Created",
  "data": {
    "id": "01234567-89ab-cdef-0123-456789abcdef",
    "user_id": "01234567-89ab-cdef-0123-456789abcdef",
    "title": "My New Note",
    "content": "This is the content of my note.",
    "created": "2025-10-24T12:00:00Z",
    "updated": "2025-10-24T12:00:00Z"
  }
}
```

### Example Code
```javascript
async function createNote(title, content) {
  const accessToken = localStorage.getItem('access_token');
  
  if (!accessToken) {
    throw new Error('No access token available');
  }

  const response = await fetch(`${API_BASE_URL}/notes`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      title: title,
      content: content
    })
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
- **400 Validation**: Missing or empty title/content
- **401 Unauthorized**: Missing, invalid, or expired access token

---

## Update Note

### Endpoint
```
PUT /notes/{id}
```

### Headers
```
Authorization: Bearer <access_token>
```

### Request Body
```json
{
  "title": "Updated Title",
  "content": "Updated content here."
}
```

### Field Requirements
- **title** (optional): Non-empty string if provided
- **content** (optional): Non-empty string if provided
- At least one field must be provided

### Success Response (200 OK)
```json
{
  "success": true,
  "message": "Note updated successfully",
  "status": "Ok",
  "data": {
    "id": "01234567-89ab-cdef-0123-456789abcdef",
    "user_id": "01234567-89ab-cdef-0123-456789abcdef",
    "title": "Updated Title",
    "content": "Updated content here.",
    "created": "2025-10-24T12:00:00Z",
    "updated": "2025-10-24T13:00:00Z"
  }
}
```

### Example Code
```javascript
async function updateNote(noteId, title, content) {
  const accessToken = localStorage.getItem('access_token');
  
  if (!accessToken) {
    throw new Error('No access token available');
  }

  const response = await fetch(`${API_BASE_URL}/notes/${noteId}`, {
    method: 'PUT',
    headers: {
      'Authorization': `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      title: title,
      content: content
    })
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
- **400 Validation**: Invalid or empty fields
- **401 Unauthorized**: Missing, invalid, or expired access token
- **403 Forbidden**: No permission to update the note (not owner or admin)
- **404 Not Found**: Note does not exist

---

## Delete Note

### Endpoint
```
DELETE /notes/{id}
```

### Headers
```
Authorization: Bearer <access_token>
```

### Success Response (204 No Content)
```json
{
  "success": true,
  "message": "Note deleted successfully",
  "status": "NoContent"
}
```

### Example Code
```javascript
async function deleteNote(noteId) {
  const accessToken = localStorage.getItem('access_token');
  
  if (!accessToken) {
    throw new Error('No access token available');
  }

  const response = await fetch(`${API_BASE_URL}/notes/${noteId}`, {
    method: 'DELETE',
    headers: {
      'Authorization': `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    }
  });

  if (response.status === 204) {
    return { success: true };
  } else {
    const data = await response.json();
    throw new Error(data.message);
  }
}
```

### Common Errors
- **401 Unauthorized**: Missing, invalid, or expired access token
- **403 Forbidden**: No permission to delete the note (not owner or admin)
- **404 Not Found**: Note does not exist

---

## Token Management

Refer to the [Frontend Authentication Implementation Guide](FRONTEND_AUTH_GUIDE.md) for token storage, refresh, and protected requests. All notes endpoints require a valid access token in the Authorization header.

### Protected API Requests
```javascript
async function makeProtectedRequest(endpoint, method = 'GET', body = null) {
  const accessToken = localStorage.getItem('access_token');
  
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

  return await fetch(`${API_BASE_URL}${endpoint}`, options).then(res => res.json());
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
    'Conflict': 'This resource already exists.',
    'Internal': 'An internal server error occurred. Please try again later.'
  };

  const errorType = data.status;
  const defaultMessage = errorMessages[errorType] || 'An error occurred.';
  
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
  const notes = await getAllNotes();
  console.log('Notes:', notes);
} catch (error) {
  const errorDetails = handleApiError(error);
  console.error('Failed to get notes:', errorDetails.message);
  // Display error to user
  showErrorNotification(errorDetails.message);
}
```

---

## Complete Example

Here's a complete notes service for your frontend:

```javascript
class NotesService {
  constructor(baseURL) {
    this.baseURL = baseURL;
  }

  // Get all notes
  async getAllNotes() {
    return this.makeRequest('/notes');
  }

  // Get a specific note
  async getNote(noteId) {
    return this.makeRequest(`/notes/${noteId}`);
  }

  // Create a new note
  async createNote(title, content) {
    return this.makeRequest('/notes', 'POST', { title, content });
  }

  // Update a note
  async updateNote(noteId, title, content) {
    return this.makeRequest(`/notes/${noteId}`, 'PUT', { title, content });
  }

  // Delete a note
  async deleteNote(noteId) {
    return this.makeRequest(`/notes/${noteId}`, 'DELETE');
  }

  // Make authenticated request with error handling
  async makeRequest(endpoint, method = 'GET', body = null) {
    const accessToken = localStorage.getItem('access_token');
    
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

    const response = await fetch(`${this.baseURL}${endpoint}`, options);
    const data = await response.json();
    
    if (data.success) {
      return data.data;
    } else {
      throw new Error(data.message || 'Request failed');
    }
  }
}

// Usage
const notesService = new NotesService('http://localhost:3000');

// Get all notes
try {
  const notes = await notesService.getAllNotes();
  console.log('All notes:', notes);
} catch (error) {
  console.error('Failed to get notes:', error.message);
}

// Create a note
try {
  const newNote = await notesService.createNote('My Note', 'Note content');
  console.log('Created note:', newNote);
} catch (error) {
  console.error('Failed to create note:', error.message);
}

// Update a note
try {
  const updatedNote = await notesService.updateNote('note-id', 'Updated Title', 'Updated content');
  console.log('Updated note:', updatedNote);
} catch (error) {
  console.error('Failed to update note:', error.message);
}

// Delete a note
try {
  await notesService.deleteNote('note-id');
  console.log('Note deleted successfully');
} catch (error) {
  console.error('Failed to delete note:', error.message);
}
```

---

## Best Practices

1. **Authentication**: Always include the access token in requests. Implement automatic token refresh as described in the auth guide.

2. **Error Handling**: Handle all possible error types and provide user-friendly messages.

3. **Validation**: Validate input on the frontend before sending requests to prevent unnecessary API calls.

4. **Permissions**: Users can only access their own notes unless they are admins.

5. **Optimistic Updates**: For better UX, consider optimistic updates for create, update, and delete operations.

6. **Loading States**: Show loading indicators during API requests.

7. **Retry Logic**: Implement retry logic for network failures.

8. **Data Synchronization**: If using real-time features, consider WebSockets or polling for updates.

---

## Support

For issues or questions about notes implementation, please refer to:
- API documentation at `/tester` endpoint
- Backend repository documentation
- Contact the backend team

---

**Last Updated**: October 2025