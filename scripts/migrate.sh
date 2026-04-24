#!/usr/bin/env bash
set -euo pipefail

: "${DATABASE_URL:?DATABASE_URL must be set}"

sqlx migrate run --database-url "$DATABASE_URL"
