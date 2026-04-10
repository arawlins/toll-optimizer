# Monitoring & Observability

The Toll Optimizer system features a comprehensive monitoring stack based on the **Loki Stack**. This allows for real-time metrics collection, log aggregation, and automated alerting.

## 1. Architecture Overview

The monitoring stack consists of the following components:

-   **Grafana**: Visualization and alerting dashboard.
-   **Prometheus**: Time-series database for quantitative metrics.
-   **Loki**: Log aggregation system (optimized for JSON logs).
-   **Promtail**: Log shipper that discovers Docker container logs.
-   **Alertmanager**: Routes alerts to external services (e.g., Slack).

### Data Flow
1.  **Metrics**: `toll-optimizer-api` exposes a `/metrics` endpoint. Prometheus scrapes this endpoint every 15 seconds.
2.  **Logs**: All containers output logs to `stdout`/`stderr`. Promtail reads these logs from the Docker socket and ships them to Loki.
3.  **Alerting**: 
    -   Prometheus evaluates metric-based rules.
    -   Loki evaluates log-based rules (e.g., searching for "ERROR").
    -   Both send firing alerts to Alertmanager, which then notifies the configured Slack webhook.

---

## 2. Accessing the Tools

| Service | Local URL | Default Credentials |
| :--- | :--- | :--- |
| **Grafana** | [http://localhost:3001](http://localhost:3001) | `admin` / `admin` (or `GRAFANA_PASSWORD`) |
| **Prometheus** | [http://localhost:9090](http://localhost:9090) | N/A |
| **Alertmanager** | [http://localhost:9093](http://localhost:9093) | N/A |
| **Loki API** | [http://localhost:3100](http://localhost:3100) | `X-Scope-OrgID: fake` (Header required) |

---

## 3. Loki Security & Multi-tenancy

Loki is configured with `auth_enabled: true` to prevent unauthorized access. This enables multi-tenancy mode, which requires every request to include a tenant identifier.

### Tenant Configuration
-   **Default Tenant ID**: `fake`
-   **Clients**: Both Promtail and Grafana are configured to use the `fake` tenant ID.
-   **Ruler**: Alert rules are stored in a subdirectory matching the tenant ID (`monitoring/loki/rules/fake/`).

### Interacting with secured Loki
If querying Loki via `curl` or external tools, you MUST include the tenant header:
```bash
curl -H "X-Scope-OrgID: fake" http://localhost:3100/loki/api/v1/query...
```

---

## 4. Log-Based Alerting (Loki)

Alerts are defined in `monitoring/loki/rules/fake/rules.yml`.

### Current Alerts

#### 1. High Error Rate (`HighErrorRateLogs`)
Triggers if more than 10 logs containing the string `"ERROR"` are detected in a 5-minute window.

#### 2. Unknown Entry/Exit Points
These alerts trigger immediately (`for: 0m`) when the CSV parser encounters a location name that isn't mapped in the core library.
-   **`UnknownEntryPointDetected`**: Extracts the specific entry point name from the JSON log and includes it in the alert.
-   **`UnknownExitPointDetected`**: Extracts the specific exit point name from the JSON log and includes it in the alert.

### Adding New Log Alerts
To add a new alert, edit `monitoring/loki/rules/fake/rules.yml` and add a new rule to the `groups` list.
Example LogQL expression:
```logql
sum by (container) (count_over_time({container="toll_optimizer_app"} |= "some_pattern" [5m])) > 0
```

---

## 4. Metrics Alerting (Prometheus)

Prometheus is configured to send alerts to Alertmanager. Metrics-based alerts can be added by creating a `.rules.yml` file and referencing it in `monitoring/prometheus/prometheus.yml`.

---

## 5. External Notifications (Slack)

The system is configured to send alerts to a Slack webhook.
-   **Configuration File**: `monitoring/alertmanager/alertmanager.yml`
-   **Environment Variable**: `SLACK_WEBHOOK_URL` in your `.env` file.

The Slack message includes the alert severity, a summary, and the detailed description (which often includes dynamic labels like the specific unknown entry point found).

---

## 6. Troubleshooting

### Permission Denied (Loki)
If Loki fails to start with `mkdir /loki/chunks: permission denied`, ensure the `loki_data` volume is correctly initialized and that the `user: root` directive is present in `docker-compose.yml`.

### DNS Resolution (Promtail)
If Promtail logs `dial tcp: lookup loki: no such host`, ensure that Loki is running and healthy. Promtail is configured with `restart: always` to recover from temporary network initialization delays.
