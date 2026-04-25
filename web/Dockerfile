# syntax=docker/dockerfile:1.7

# ---------- Stage 1: build the Vite/React bundle ----------
FROM node:20-alpine AS builder

WORKDIR /app

# VITE_API_URL is baked into the JS bundle at build time.
# Default to "/api" so the bundle issues same-origin requests
# that nginx proxies to the api service.
ARG VITE_API_URL=/api
ENV VITE_API_URL=${VITE_API_URL}

# Install deps from the lockfile for reproducibility.
# `npm ci` fails fast if package-lock.json is missing — that is intentional.
COPY package.json package-lock.json* ./
RUN npm ci --no-audit --no-fund

# Copy source and produce dist/
COPY . .
RUN npm run build

# ---------- Stage 2: nginx runtime ----------
FROM nginx:alpine AS runtime

# Replace the stock server block with our SPA + /api proxy config.
RUN rm /etc/nginx/conf.d/default.conf
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Static assets only — no build tools, no source, no node_modules.
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget -qO- http://127.0.0.1/ >/dev/null 2>&1 || exit 1

CMD ["nginx", "-g", "daemon off;"]
