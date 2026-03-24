# CLAUDE.md - 사주 상담 플랫폼 (AI Saju Consultation Platform)

## Project Overview

AI-powered Korean fortune-telling (사주/四柱) consultation platform with free saju card generation, paid AI consultations, daily fortunes, and compatibility checks.

## Project Structure

```
saju/
├── backend/           # Rust (Axum) API server
│   ├── src/
│   │   ├── api/       # Route handlers
│   │   ├── auth/      # JWT + OAuth (Kakao/Apple/Google)
│   │   ├── models/    # Database models (SQLx)
│   │   ├── saju/      # Saju calculation engine
│   │   │   └── tables/  # Manseryeok static lookup tables
│   │   └── services/  # Business logic layer
│   └── migrations/    # PostgreSQL migration files (001-010)
├── app/               # Flutter mobile app (iOS + Android)
├── web/               # Next.js 15 marketing site + share pages
├── shared/
│   ├── contracts/     # API schema (OpenAPI 3.1)
│   │   └── api-schema.json
│   └── design-tokens/ # Shared design tokens
│       └── tokens.json
├── docker-compose.yml # Local dev: PostgreSQL 16 + Redis 7 + backend
├── fly.toml           # Fly.io deployment config (nrt region)
└── .github/workflows/ # CI/CD pipelines
```

## Running Locally

### Prerequisites
- Docker & Docker Compose
- Rust 1.75+ (for backend development without Docker)
- Flutter 3.27+ (for mobile app)
- Node.js 20+ (for web)

### Start all services
```bash
cp .env.example .env   # Fill in real values
docker-compose up -d   # PostgreSQL, Redis, backend
```

### Backend only (development)
```bash
cd backend
cargo run               # Runs on http://localhost:8080
cargo test              # Run all tests
cargo clippy            # Lint
```

### Flutter app
```bash
cd app
flutter pub get
flutter run             # iOS simulator or Android emulator
flutter test
```

### Web (Next.js)
```bash
cd web
npm install
npm run dev             # http://localhost:3000
npm run lint
npm run build
```

## Key Files

| Purpose | Path |
|---------|------|
| API Contract | `shared/contracts/api-schema.json` |
| Design Tokens | `shared/design-tokens/tokens.json` |
| DB Migrations | `backend/migrations/001-010_*.sql` |
| Saju Tables | `backend/src/saju/tables/` |
| Environment Vars | `.env.example` |

## API Base URLs

- Production: `https://api.saju.app/v1`
- Local: `http://localhost:8080/v1`
- Health check: `GET /health`

## Database

PostgreSQL 16 via Supabase. Key tables:
- `users` - OAuth users (Kakao/Apple/Google) + has_profile flag
- `orders` - IAP payment orders (RevenueCat verified)
- `saju_profiles` - Permanent saju profiles (encrypted birth data + individual pillar columns)
- `saju_cards` - Cached free card results (by birth_hmac)
- `consultations` - Paid AI consultations (references orders, encrypted birth data)
- `chat_messages` - Consultation chat history
- `daily_fortunes` - Pre-generated daily fortunes (by ilju)
- `compatibility_results` - Cached compatibility scores
- `share_links` - Deep link redirect service (replaces Firebase Dynamic Links)
- `rate_limits` - Device-based rate limiting

## Coding Conventions

### Rust (Backend)
- Use `axum` for HTTP, `sqlx` for database, `serde` for JSON
- All handlers return `Result<Json<T>, AppError>`
- Use `tracing` for structured logging, not `println!`
- Database queries use parameterized SQLx queries (never string interpolation)
- All public functions must have doc comments
- Error types must implement `IntoResponse`

### Flutter (App)
- State management: Riverpod
- Folder structure: feature-first under `lib/features/`
- Use `freezed` for immutable models
- Design tokens from `shared/design-tokens/tokens.json`
- All strings in Korean, use `l10n` for any future i18n

### Next.js (Web)
- App Router (not Pages Router)
- TypeScript strict mode
- Tailwind CSS with design token values
- Server Components by default, `'use client'` only when needed
- API calls via server actions or route handlers

### General
- Commit messages in English, code comments in Korean where domain-specific
- Branch naming: `feat/`, `fix/`, `chore/`
- PR required for `main`, CI must pass
- Never commit `.env` files or secrets
- JSONB fields follow schemas defined in `api-schema.json`

## Saju Domain Notes

- 사주 (四柱) = Four Pillars of Destiny (year/month/day/hour pillars)
- Each pillar has a 천간 (Heavenly Stem) + 지지 (Earthly Branch)
- 오행 (五行) = Five Elements: Wood, Fire, Earth, Metal, Water
- 일주 (日柱) = Day Pillar, the primary identifier for daily fortunes
- 절기 (節氣) = Solar terms, used to determine saju month boundaries
- Month boundaries follow 절 (Jie) terms, NOT calendar months
- Birth hour uses 시진 (12 two-hour periods), "unknown" is valid
