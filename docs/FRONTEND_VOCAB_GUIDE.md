# Frontend Vocabulary Implementation Guide

This guide provides complete instructions for implementing vocabulary management in your frontend application using the Chill Labs API.

## Table of Contents
- [Base URL Configuration](#base-url-configuration)
- [Response Format](#response-format)
- [Vocabulary Endpoints](#vocabulary-endpoints)
- [Get All Vocabulary Lists](#get-all-vocabulary-lists)
- [Get Vocabulary List](#get-vocabulary-list)
- [Create Vocabulary List](#create-vocabulary-list)
- [Update Vocabulary List](#update-vocabulary-list)
- [Delete Vocabulary List](#delete-vocabulary-list)
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

## Vocabulary Endpoints

All vocabulary endpoints are prefixed with `/vocabs` and require authentication:

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| GET | `/vocabs` | Get all user's vocabulary lists | Yes |
| GET | `/vocabs/{id}` | Get a specific vocabulary list | Yes |
| POST | `/vocabs` | Create a new vocabulary list | Yes |
| PUT | `/vocabs/{id}` | Update a vocabulary list | Yes |
| DELETE | `/vocabs/{id}` | Delete a vocabulary list | Yes |

---

## Get All Vocabulary Lists

### Endpoint
```
GET /vocabs
```

### Headers
```
Authorization: Bearer <access_token>
```

### Success Response (200 OK)
```json
{
  "success": true,
  "message": "Vocabulary lists retrieved successfully",
  "status": "Ok",
  "data": [
    {
      "id": "01234567-89ab-cdef-0123-456789abcdef",
      "user_id": "01234567-89ab-cdef-0123-456789abcdef",
      "words": [
        {
          "word": "intelligence",
          "phonics": "/ɪnˈtɛlədʒəns/",
          "partOfSpeech": "n",
          "vietnameseMeaning": "trí thông minh",
          "sampleSentence": "Emotional intelligence is important.",
          "vietnameseTranslation": "Trí tuệ cảm xúc quan trọng.",
          "image": "",
          "wordPronunciation": "intelligence.mp3",
          "sentencePronunciation": "emotional-intelligence-important.mp3"
        }
      ],
      "created": "2025-10-24T12:00:00Z",
      "updated": "2025-10-24T12:00:00Z"
    }
  ]
}
```

### Example Code
```javascript
async function getAllVocabularyLists() {
  const accessToken = localStorage.getItem('access_token');
  
  if (!accessToken) {
    throw new Error('No access token available');
  }

  const response = await fetch(`${API_BASE_URL}/vocabs`, {
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

## Get Vocabulary List

### Endpoint
```
GET /vocabs/{id}
```

### Headers
```
Authorization: Bearer <access_token>
```

### Success Response (200 OK)
```json
{
  "success": true,
  "message": "Vocabulary list retrieved successfully",
  "status": "Ok",
  "data": {
    "id": "01234567-89ab-cdef-0123-456789abcdef",
    "user_id": "01234567-89ab-cdef-0123-456789abcdef",
    "words": [
      {
        "word": "intelligence",
        "phonics": "/ɪnˈtɛlədʒəns/",
        "partOfSpeech": "n",
        "vietnameseMeaning": "trí thông minh",
        "sampleSentence": "Emotional intelligence is important.",
        "vietnameseTranslation": "Trí tuệ cảm xúc quan trọng.",
        "image": "",
        "wordPronunciation": "intelligence.mp3",
        "sentencePronunciation": "emotional-intelligence-important.mp3"
      }
    ],
    "created": "2025-10-24T12:00:00Z",
    "updated": "2025-10-24T12:00:00Z"
  }
}
```

### Example Code
```javascript
async function getVocabularyList(vocabId) {
  const accessToken = localStorage.getItem('access_token');
  
  if (!accessToken) {
    throw new Error('No access token available');
  }

  const response = await fetch(`${API_BASE_URL}/vocabs/${vocabId}`, {
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
- **403 Forbidden**: No permission to access vocabulary list (not owner or admin)
- **404 Not Found**: Vocabulary list does not exist

---

## Create Vocabulary List

### Endpoint
```
POST /vocabs
```

### Headers
```
Authorization: Bearer <access_token>
Content-Type: application/json
```

### Request Body
```json
{
  "words": [
    {
      "word": "intelligence",
      "vietnameseMeaning": "trí thông minh",
      "phonics": "/ɪnˈtɛlədʒəns/",
      "partOfSpeech": "n",
      "sampleSentence": "Emotional intelligence is important.",
      "vietnameseTranslation": "Trí tuệ cảm xúc quan trọng.",
      "image": "",
      "wordPronunciation": "intelligence.mp3",
      "sentencePronunciation": "emotional-intelligence-important.mp3"
    },
    {
      "word": "minimal",
      "vietnameseMeaning": "tối thiểu"
    }
  ]
}
```

### Field Requirements
- **words** (required): Array of word objects
- **word** (required): Non-empty string
- **vietnameseMeaning** (required): Non-empty string
- All other fields are optional

### Success Response (201 Created)
```json
{
  "success": true,
  "message": "Vocabulary list created successfully",
  "status": "Created",
  "data": {
    "id": "01234567-89ab-cdef-0123-456789abcdef",
    "user_id": "01234567-89ab-cdef-0123-456789abcdef",
    "words": [
      {
        "word": "intelligence",
        "vietnameseMeaning": "trí thông minh",
        "phonics": "/ɪnˈtɛlədʒəns/",
        "partOfSpeech": "n",
        "sampleSentence": "Emotional intelligence is important.",
        "vietnameseTranslation": "Trí tuệ cảm xúc quan trọng.",
        "image": "",
        "wordPronunciation": "intelligence.mp3",
        "sentencePronunciation": "emotional-intelligence-important.mp3"
      }
    ],
    "created": "2025-10-24T12:00:00Z",
    "updated": "2025-10-24T12:00:00Z"
  }
}
```

### Example Code
```javascript
async function createVocabularyList(words) {
  const accessToken = localStorage.getItem('access_token');
  
  if (!accessToken) {
    throw new Error('No access token available');
  }

  const response = await fetch(`${API_BASE_URL}/vocabs`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      words: words
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
- **400 Validation**: Missing or empty required fields
- **401 Unauthorized**: Missing, invalid, or expired access token

---

## Update Vocabulary List

### Endpoint
```
PUT /vocabs/{id}
```

### Headers
```
Authorization: Bearer <access_token>
Content-Type: application/json
```

### Request Body
```json
{
  "words": [
    {
      "word": "intelligence",
      "vietnameseMeaning": "trí thông minh (updated)",
      "phonics": "/ɪnˈtɛlədʒəns/",
      "partOfSpeech": "noun",
      "sampleSentence": "Updated sample sentence.",
      "vietnameseTranslation": "Updated Vietnamese translation."
    }
  ]
}
```

### Field Requirements
- **words** (optional): Array of word objects to replace existing words
- At least one field must be provided (words or individual word fields)

### Success Response (200 OK)
```json
{
  "success": true,
  "message": "Vocabulary list updated successfully",
  "status": "Ok",
  "data": {
    "id": "01234567-89ab-cdef-0123-456789abcdef",
    "user_id": "01234567-89ab-cdef-0123-456789abcdef",
    "words": [
      {
        "word": "intelligence",
        "vietnameseMeaning": "trí thông minh (updated)",
        "phonics": "/ɪnˈtɛlədʒəns/",
        "partOfSpeech": "noun",
        "sampleSentence": "Updated sample sentence.",
        "vietnameseTranslation": "Updated Vietnamese translation."
      }
    ],
    "created": "2025-10-24T12:00:00Z",
    "updated": "2025-10-24T13:00:00Z"
  }
}
```

### Example Code
```javascript
async function updateVocabularyList(vocabId, words) {
  const accessToken = localStorage.getItem('access_token');
  
  if (!accessToken) {
    throw new Error('No access token available');
  }

  const response = await fetch(`${API_BASE_URL}/vocabs/${vocabId}`, {
    method: 'PUT',
    headers: {
      'Authorization': `Bearer ${accessToken}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      words: words
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
- **403 Forbidden**: No permission to update vocabulary list (not owner or admin)
- **404 Not Found**: Vocabulary list does not exist

---

## Delete Vocabulary List

### Endpoint
```
DELETE /vocabs/{id}
```

### Headers
```
Authorization: Bearer <access_token>
```

### Success Response (204 No Content)
```json
{
  "success": true,
  "message": "Vocabulary list deleted successfully",
  "status": "NoContent"
}
```

### Example Code
```javascript
async function deleteVocabularyList(vocabId) {
  const accessToken = localStorage.getItem('access_token');
  
  if (!accessToken) {
    throw new Error('No access token available');
  }

  const response = await fetch(`${API_BASE_URL}/vocabs/${vocabId}`, {
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
- **403 Forbidden**: No permission to delete vocabulary list (not owner or admin)
- **404 Not Found**: Vocabulary list does not exist

---

## Token Management

Refer to the [Frontend Authentication Implementation Guide](FRONTEND_AUTH_GUIDE.md) for token storage, refresh, and protected requests. All vocabulary endpoints require a valid access token in the Authorization header.

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
function handleVocabularyError(data) {
  const errorMessages = {
    'Validation': 'Please check your input and try again.',
    'Unauthorized': 'Invalid credentials or session expired.',
    'Forbidden': 'You do not have permission to perform this action.',
    'NotFound': 'The requested vocabulary list was not found.',
    'Conflict': 'This vocabulary list already exists.',
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
  const vocabLists = await getAllVocabularyLists();
  console.log('Vocabulary lists:', vocabLists);
} catch (error) {
  const errorDetails = handleVocabularyError(error);
  console.error('Failed to get vocabulary lists:', errorDetails.message);
  // Display error to user
  showErrorMessage(errorDetails.message);
}
```

---

## Complete Example

Here's a complete vocabulary service for your frontend:

```javascript
class VocabularyService {
  constructor(baseURL) {
    this.baseURL = baseURL;
  }

  // Get all vocabulary lists
  async getAllVocabularyLists() {
    return this.makeRequest('/vocabs');
  }

  // Get a specific vocabulary list
  async getVocabularyList(vocabId) {
    return this.makeRequest(`/vocabs/${vocabId}`);
  }

  // Create a new vocabulary list
  async createVocabularyList(words) {
    return this.makeRequest('/vocabs', 'POST', { words });
  }

  // Update a vocabulary list
  async updateVocabularyList(vocabId, words) {
    return this.makeRequest(`/vocabs/${vocabId}`, 'PUT', { words });
  }

  // Delete a vocabulary list
  async deleteVocabularyList(vocabId) {
    return this.makeRequest(`/vocabs/${vocabId}`, 'DELETE');
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
const vocabularyService = new VocabularyService('http://localhost:3000');

// Get all vocabulary lists
try {
  const vocabLists = await vocabularyService.getAllVocabularyLists();
  console.log('All vocabulary lists:', vocabLists);
} catch (error) {
  console.error('Failed to get vocabulary lists:', error.message);
}

// Create a new vocabulary list
const newWords = [
  {
    word: "intelligence",
    vietnameseMeaning: "trí thông minh",
    phonics: "/ɪnˈtɛlədʒəns/",
    partOfSpeech: "n",
    sampleSentence: "Emotional intelligence is important.",
    vietnameseTranslation: "Trí tuệ cảm xúc quan trọng."
  },
  {
    word: "minimal",
    vietnameseMeaning: "tối thiểu"
  }
];

try {
  const createdVocab = await vocabularyService.createVocabularyList(newWords);
  console.log('Created vocabulary list:', createdVocab);
} catch (error) {
  console.error('Failed to create vocabulary list:', error.message);
}

// Update a vocabulary list
try {
  const updatedVocab = await vocabularyService.updateVocabularyList('vocab-id', newWords);
  console.log('Updated vocabulary list:', updatedVocab);
} catch (error) {
  console.error('Failed to update vocabulary list:', error.message);
}

// Delete a vocabulary list
try {
  await vocabularyService.deleteVocabularyList('vocab-id');
  console.log('Vocabulary list deleted successfully');
} catch (error) {
  console.error('Failed to delete vocabulary list:', error.message);
}
```

---

## Best Practices

1. **Authentication**: Always include access token in requests. Implement automatic token refresh as described in auth guide.

2. **Error Handling**: Handle all possible error types and provide user-friendly messages.

3. **Validation**: Validate input on the frontend before sending requests to prevent unnecessary API calls.

4. **Permissions**: Users can only access their own vocabulary lists unless they are admins.

5. **Optimistic Updates**: For better UX, consider optimistic updates for create, update, and delete operations.

6. **Loading States**: Show loading indicators during API requests.

7. **Retry Logic**: Implement retry logic for network failures.

8. **Data Synchronization**: If using real-time features, consider WebSockets or polling for updates.

---

## Support

For issues or questions about vocabulary implementation, please refer to:
- API documentation at `/tester` endpoint
- Backend repository documentation
- Contact the backend team

---

**Last Updated**: November 2025