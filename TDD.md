## Grafana-lgtm CLI

Grafana lgtm cli is a command line tool for generating and summarizing prometheus/loki/tempo telemetry data from natural language.

### Technical design documenation

The application is designed in below phases.

1. The Client layer: This layer contains code for calling prometheus/loki/tempo api.
2. The AI layer: This abstraction contains code for calling LLM with prompt & config and returns response.
3. The API layer: An api is available to send the natural language question and getting prompt.
4. The CLI layer: The cli for prompt and config.

## Steps

### Define the Client layer.

This layer shall contain code for prometheus/tempo/loki queries.
Either use existing clients for these apps or use api for calling these things.

Inputs for this layer:

1. Prometheus:
    - Query
    - Query parameters (limit, offset, time interval etc)
2. Loki:
    - Query
    - Query parameters (limit, offset, time interval etc)
2. Tempo:
    - Query
    - Query parameters (limit, offset, time interval etc)

Output for this layer:

Metrics/Logs/Traces of respective client.

### The AI layer

This layer will handle the input and output that will come and go from LLM. 

Input for this layer:
    1. Prompt
    2. Config (BaseURL, Model, async?)

Output:
    1. Structured json of the query.
    2. Summarization of result.

### API layer

The API layer lets end users to send input (natural language prompt) and receive summarization (result).

### CLI layer

CLI does the same as API layer
