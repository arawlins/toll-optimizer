# API Reference

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
