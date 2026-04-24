# linkjarapp

A self-hosted link bookmarking service with a REST API and web UI.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) 24+
- [Docker Compose](https://docs.docker.com/compose/) v2

## Quickstart

```bash
git clone https://github.com/your-org/linkjarapp.git
cd linkjarapp
docker compose up --build
```

Save a link:

```bash
curl -X POST http://localhost:8080/links \
  -H 'Content-Type: application/json' \
  -d '{"url":"https://example.com"}'
```

Open the web UI: http://localhost:3000

## Smoke test

```bash
bash scripts/smoke.sh
```
