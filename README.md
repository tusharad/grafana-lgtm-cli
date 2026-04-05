# grafana-lgtm-cli

grafana-lgtm-cli is a Rust CLI and small HTTP service that turns a natural language prompt into a Prometheus query, executes it, and returns a short summary.

Current scope is Prometheus only.

## Install

Requirements:
- Rust and Cargo
- A reachable Prometheus server
- A Gemini API key

Build:

```bash
cargo build
```

## Usage

Set API key:

```bash
export GEMINI_API_KEY="your_api_key"
```

Run as CLI:

```bash
cargo run -- ask "show cpu usage for the last hour"
```

Run as API server:

```bash
cargo run -- serve --port 8080
```

Send a request:

```bash
curl -X POST http://localhost:8080/api/v1/ask \
	-H "Content-Type: application/json" \
	-d '{"prompt":"show me current up status"}'
```

## Configuration

Runtime configuration currently available:
- `GEMINI_API_KEY`: required environment variable
- `--port`: API server port (default `8080`)

Code-level defaults:
- Prometheus base URL is `http://localhost:9090`
- Prometheus timeout is `300` seconds
- Gemini model defaults to `Gemini25FlashLite`

To change these defaults, update:
- `src/client/prometheus.rs` for Prometheus URL/timeout
- `src/llm/gemini.rs` for model and LLM prompts

## Examples

CLI command:

```bash
cargo run -- ask "what is the current request rate"
```

Typical CLI output:

```text
Processing query...

Result
Request rate is stable at around N requests/sec over the selected interval.
```

API request:

```http
POST /api/v1/ask
Content-Type: application/json

{"prompt":"show me error rate"}
```

Success response:

```json
{"summary":"Error rate is low and within normal range."}
```
