# API Reference

## System

### `GET /health`
Simple health check endpoint to verify the API is running.

**Response (200 OK):**
```text
OK
```

---

## Authentication

### `POST /auth/register`
Creates a new user account.

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```

**Response (200 OK):**
```json
{
  "token": "<jwt_token>",
  "user": {
    "id": "<uuid>",
    "email": "user@example.com",
    "created_at": "2026-03-06T12:00:00Z"
  }
}
```

---

### `POST /auth/login`
Authenticates a user and returns a session token.

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```

**Response (200 OK):** Same as register.

---

## Security & Rate Limiting

### Rate Limiting
The `/auth` endpoints (`/register` and `/login`) are protected by a rate limiter to prevent brute-force attacks.
-   **Limit**: 2 requests per second.
-   **Burst**: Up to 5 requests.
-   **Bypass**: Can be disabled by setting the environment variable `DISABLE_RATE_LIMIT=true` (recommended for development and CI only).

### IP Extraction
Rate limiting is based on the client's IP address. If running behind a proxy, ensure the `X-Forwarded-For` header is set correctly. The API uses a `GlobalKeyExtractor` as a fallback if connection info is unavailable.

---

## Analysis & History

### `GET /api/history`
Returns the upload history for the authenticated user.

**Headers:**
*   `Authorization: Bearer <token>`

**Query Parameters:**
*   `limit` (optional): Maximum number of records to return (default: `20`).
*   `offset` (optional): Number of records to skip (default: `0`).

**Response (200 OK):**
```json
[
  {
    "id": "<uuid>",
    "filename": "statement.csv",
    "total_trips": 45,
    "cost_actual": "124.50",
    "cost_optimized": "112.00",
    "savings": "12.50",
    "uploaded_at": "2026-03-06T14:30:00Z"
  }
]
```

---

### `POST /api/analyze`
Accepts a CSV statement, processes it, and returns the detailed analysis.

**Headers:**
*   `Authorization: Bearer <token>`

**Request Body:** `multipart/form-data`
*   `file`: The ETR statement CSV file.

**Response (200 OK):**
```json
{
  "total_trips": 45,
  "total_cost": 124.50,
  "time_based_savings": 8.50,
  "distance_based_savings": 10.20,
  "time_analysis": [
    {
      "transponder_plate": "E87290C1",
      "direction": "Eastbound",
      "best_k": 3,
      "centroids": [
        {
          "centroid_time": "08:15",
          "average_entry_time": "08:12",
          "total_toll_charge": 15.20,
          "total_optimized_savings": 2.50,
          "optimization_advice": "Shift trips to before 07:00 or after 09:30 to save some $$$",
          "trips": [
            {
              "trip": { ... },
              "optimized_cost": 12.70,
              "optimized_saved": 2.50,
              "optimized_entry": "Hwy 401",
              "optimized_exit": "Hwy 407"
            }
          ]
        }
      ]
    }
  ],
  "distance_analysis": [
    {
      "transponder_plate": "E87290C1",
      "direction": "Eastbound",
      "centroids": [
        {
          "centroid_distance": 15.5,
          "representative_entry": "Hwy 401",
          "representative_exit": "Woodbine",
          "total_optimized_savings": 3.20,
          "optimization_advice": "Exit on Warden to save some $$$",
          "trips": [ ... ]
        }
      ]
    }
  ]
}
```

---

## Pricing

### `POST /api/pricing`
Returns the current and next timeslot prices for a given date and time based on the 2026 toll rates.

**Request Body:**
```json
{
  "date": "2026-04-13",
  "time": "06:30 AM"
}
```
*   `date`: Format `YYYY-MM-DD`.
*   `time`: Format `HH:MM AM/PM` or `HH:MM[:SS]` (24h).

**Response (200 OK):**
```json
{
  "current": {
    "timeslot": "5:00 AM",
    "average_wb": 69.10,
    "average_eb": 72.45
  },
  "next": {
    "timeslot": "7:00 AM",
    "average_wb": 89.35,
    "average_eb": 92.10
  },
  "day_type": "Weekday"
}
```

---

## Observability

### `GET /metrics`
Returns system performance metrics in Prometheus format.

**Access:** Requires Basic Authentication.
**Credentials:** `METRICS_USERNAME` and `METRICS_PASSWORD`.

**Response (200 OK):** Plain text (Prometheus format).
```text
# HELP axum_http_requests_total Total number of HTTP requests
# TYPE axum_http_requests_total counter
axum_http_requests_total{method="GET",path="/api/history",status="200"} 42
```
