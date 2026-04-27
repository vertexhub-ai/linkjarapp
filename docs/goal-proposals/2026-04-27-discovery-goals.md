# LinkjarApp — Proposed Discovery Goals (2026-04-27 01:40 UTC cycle)

Project id: `7a1eab90-44f6-4db4-9c2c-eaf01427e5dc`
Author: Solution Architect (agent 0100)
Cycle: V-470

## Why this doc exists

The vertexagents MCP server failed to surface its tool schemas in this run
(same failure mode as V-469, lesson "Adapter surfaced errors during the run").
I cannot call `vertexagents_create_issue`, `vertexagents_promote_goal`, or
`vertexagents_record_goal_review`. The next cycle should pick up these proposals
and file them as a proposal issue once the MCP is healthy.

## Current project snapshot (read from repo)

- Stack already chosen by prior cycle: Rust API (`api/Cargo.toml`) + React/Vite
  web (`web/package.json`) + Postgres (per `DATABASE_URL` in `API.md`).
- API contract (`API.md`) defines POST /links, GET /:code redirect, GET /links/:code/stats.
- Scaffold commit: `6f11a95 feat(V-74): scaffold repo structure and define API contract`.
- No additional code beyond scaffold; no migrations, no real handler bodies, no UI.
- Project has 0 goals.

Conclusion: project is at "scaffold complete, MVP not yet functional" — the
right discovery goals frame the path from scaffold to a usable shortener.

## Proposed discovery goals (1–3)

### G1 — Functional MVP: end-to-end shorten + redirect + stats
**Outcome:** A user can POST /links, get a short code, hit `GET /:code` and be
302'd, then read `GET /links/:code/stats` and see the click counter increment.
Backed by Postgres with a migration; web UI has a form and a table; smoke test
covers the happy path.

**Acceptance signals**
- Migration creates `links(code TEXT PK, url TEXT, visits INT, created_at TIMESTAMPTZ)`.
- All three endpoints from `API.md` pass an integration test against a real Postgres.
- React UI lists links and lets the user create one.
- Local `docker-compose up` (or equivalent) brings the whole thing up.

**Why discovery, not in_progress:** the team hasn't decided which Rust web
framework to standardize on (axum vs actix), nor how to seed local Postgres.
That's a 1-day scoping conversation, not a 4-week build — discovery is right.

**Suggested owner:** Tech Lead (Rust) for decomposition.

---

### G2 — Reliability baseline: health, logs, error handling
**Outcome:** `/healthz` returns 200 when DB is reachable; structured JSON logs
on every request with status + latency + code; non-2xx paths return the error
shape from `API.md` consistently.

**Acceptance signals**
- `GET /healthz` exists, returns 503 when DB is down.
- `tracing` (or chosen crate) emits one JSON log per request.
- Error envelope `{ "error": "..." }` is shared across handlers (not duplicated).
- One smoke test exercises a forced DB-down to confirm 503 + log line.

**Why discovery:** depends on G1 landing first. Don't promote until MVP exists.

**Suggested owner:** SRE/Observability.

---

### G3 — Anti-abuse minimum: URL validation + naive rate-limit
**Outcome:** Malformed URLs rejected with 422 (already in contract); per-IP
rate limit on POST /links to keep a single client from exhausting the table;
optional link expiry left as a follow-up.

**Acceptance signals**
- Library-grade URL parser rejects `javascript:`, missing scheme, etc.
- Rate-limit middleware: 60 POST /links per IP per hour, configurable.
- Test confirms 429 on 61st request from same IP within window.

**Why discovery:** scope question — do we want Redis-backed rate limit (matches
VertexShortener's stack) or in-process for the MVP? Worth a 30-minute decision
before promoting.

**Suggested owner:** Backend/Auth specialty.

## Recommendation to human reviewer

Promote G1 first. Hold G2 and G3 in discovery until G1 lands — they're
meaningless without a working request path.

## Cycle bookkeeping

- No existing discovery goals to age out (project has 0 goals).
- No abandoned goals to cancel.
- Cycle review record will be written when MCP is reachable, against G1 once promoted.
