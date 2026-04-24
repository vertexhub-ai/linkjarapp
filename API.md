# LinkJar API Contract

Base URL: `http://localhost:3000` (dev) / configured via `VITE_API_URL` (web client)

## Environment Variables

| Variable       | Used by | Description                              |
| -------------- | ------- | ---------------------------------------- |
| `DATABASE_URL` | api     | PostgreSQL connection string             |
| `VITE_API_URL` | web     | API base URL consumed by the React client |

---

## POST /links

Create a new short link.

**Request**
```json
{
  "url": "https://example.com/some/long/path"
}
```

**Responses**

| Status | Body                                                  | Notes               |
| ------ | ----------------------------------------------------- | ------------------- |
| 201    | `{ "code": "abc123", "url": "https://...", "short_url": "http://localhost:3000/abc123" }` | Created             |
| 400    | `{ "error": "url is required" }`                      | Missing field       |
| 422    | `{ "error": "invalid url" }`                          | Malformed URL       |
| 500    | `{ "error": "internal server error" }`               | Database or other   |

---

## GET /:code

Redirect to the original URL.

**Responses**

| Status | Headers / Body                  | Notes              |
| ------ | ------------------------------- | ------------------ |
| 302    | `Location: <original_url>`      | Increments `visits` |
| 404    | `{ "error": "not found" }`      | Unknown code       |

---

## GET /links/:code/stats

Return visit statistics for a short link.

**Responses**

| Status | Body                                                                                        | Notes        |
| ------ | ------------------------------------------------------------------------------------------- | ------------ |
| 200    | `{ "code": "abc123", "url": "https://...", "clicks": 42, "created_at": "2026-04-22T00:00:00Z" }` | Success      |
| 404    | `{ "error": "not found" }`                                                                  | Unknown code |
