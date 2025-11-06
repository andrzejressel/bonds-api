# Bonds API

Bonds API is a full-stack application for displaying data on Polish government bonds, built with a Rust backend (using the Loco framework) and an Angular frontend.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Prerequisites and Setup
- Install required tools:
  - `cargo install just --locked` (takes ~2 minutes)
  - `cargo install cargo-binstall --locked` (takes ~5 minutes)
  - `npm install -g @angular/cli`
  - Optional: `cargo install watchexec-cli` (for `just watch` command)

### Backend (Rust + Loco Framework)
Bootstrap, build, and test the backend:
- `cd backend`
- `just install-requirements` -- installs rustfmt, llvm-tools-preview, and cargo-llvm-cov. Takes ~1m30s. NEVER CANCEL.
- `cargo build` -- takes ~2m17s to complete. NEVER CANCEL. Set timeout to 180+ seconds.
- `just check` -- runs cargo fmt and clippy. Takes ~1 minute. NEVER CANCEL. Set timeout to 120+ seconds.
- `just test` -- runs all tests. Takes ~55 seconds. NEVER CANCEL. Set timeout to 120+ seconds.
- `just test-coverage` -- runs tests with coverage report. Takes ~2m49s. NEVER CANCEL. Set timeout to 200+ seconds.

Run the backend server:
- ALWAYS run the bootstrapping steps first.
- `cargo run --bin myapp-cli start` -- starts server on http://localhost:5150
- Server shows Loco ASCII art banner when ready
- Test with: `curl http://localhost:5150/bonds` (returns JSON array of bond IDs)

### Frontend (Angular + TypeScript)
Bootstrap, build, and test the frontend:
- `cd frontend`
- `npm ci` -- installs dependencies. Takes ~16 seconds.
- `npm run build:development` -- development build. Takes ~6 seconds.
- `npm run build:prod` -- production build. FAILS due to font loading from fonts.googleapis.com in sandboxed environments. Document this limitation.
- `npm run test-coverage` -- runs tests with coverage. Takes ~18 seconds.

Run the frontend server:
- ALWAYS run the dependencies installation first.
- `ng serve --host 0.0.0.0 --port 4200` -- starts dev server on http://localhost:4200
- Frontend connects to backend at http://localhost:5150 in development mode

## Validation

### Manual End-to-End Testing
ALWAYS run through complete end-to-end scenarios after making changes:

1. **Backend API Testing**:
   - Start backend: `cd backend && cargo run --bin myapp-cli start`
   - Test bonds list: `curl http://localhost:5150/bonds`
   - Test specific bond CSV: `curl http://localhost:5150/bonds/EDO0115/csv | head -5`
   - Expected: JSON array of bond IDs and CSV data with date,value headers

2. **Frontend Development Server**:
   - Start frontend: `cd frontend && ng serve --host 0.0.0.0`
   - Verify server starts without errors and shows "Application bundle generation complete"
   - Frontend should be accessible at http://localhost:4200

3. **Full Stack Integration**:
   - Run both backend (port 5150) and frontend (port 4200) simultaneously
   - Frontend environment.development.ts points to http://localhost:5150
   - Verify no CORS or connection errors in browser console

### Pre-commit Validation
ALWAYS run these commands before committing changes:
- Backend: `cd backend && just check && just test`
- Frontend: `cd frontend && npm run test-coverage`
- **CRITICAL**: NEVER CANCEL builds or tests. Build may take 2+ minutes, tests may take 1+ minute.

## Repository Structure and Key Files

### Backend (`/backend`)
- **Entry points**: `src/bin/main.rs` (main CLI), `src/bin/tool.rs` (utilities)
- **API routes**: Controllers define bonds endpoints (`/bonds`, `/bonds/{id}`, `/bonds/{id}/csv`)
- **Build system**: `Justfile` defines common tasks (check, test, fix, watch)
- **Configuration**: `config/` directory contains environment-specific settings
- **Tests**: `tests/requests/bonds.rs` contains integration tests
- **Key workspace crates**:
  - `crates/api` - Generated OpenAPI client code
  - `crates/bonds-reader` - Bond data reading and value calculation logic
  - `crates/model` - Data models
  - `crates/loco-rs-otel` - OpenTelemetry integration

### Frontend (`/frontend`)
- **Entry point**: `src/main.ts` bootstraps the Angular application
- **Configuration**: `angular.json`, `package.json`
- **Environment config**: `src/environments/` contains API endpoint configuration
- **Tests**: Unit tests alongside components, Karma configuration in `karma.conf.js`

### CI/CD
- **Backend**: `.github/workflows/backend.yml` - builds, tests, checks formatting, deploys to Fly.io
- **Frontend**: `.github/workflows/frontend.yml` - builds multiple configurations, runs tests

## Common Issues and Troubleshooting

### Known Limitations
- **Production frontend build fails** due to font loading from fonts.googleapis.com in sandboxed environments. Use development builds instead.
- **OpenTelemetry errors** appear in backend logs when OTEL endpoint (localhost:4318) is not available. These are non-blocking.

### Build Timeouts
- Backend initial build: ~2m17s (subsequent builds much faster due to caching)
- Backend tests: ~55s
- Backend test coverage: ~2m49s
- Frontend builds: ~6s (development), fails in production
- Frontend tests: ~18s

### Dependency Issues
- If `cargo binstall` is not available, install with: `cargo install cargo-binstall --locked`
- If Just is not available, install with: `cargo install just --locked`
- If Angular CLI is not available, install with: `npm install -g @angular/cli`

## Development Workflows

### Adding New Backend Features
1. Update API definitions in `crates/api/` if needed
2. Implement business logic in appropriate crates (`bonds-reader`, `model`)
3. Add controller endpoints in main application
4. Add integration tests in `tests/requests/`
5. Run `just check && just test` to validate

### Adding New Frontend Features
1. Use Angular CLI for scaffolding: `ng generate component component-name`
2. Update service classes for new API endpoints
3. Add unit tests alongside new components
4. Run `npm run test-coverage` to validate

### Performance Testing
- Backend handles bond data calculations and CSV generation
- Frontend displays charts and graphs using Angular Material and Plotly.js
- Always test with realistic bond data scenarios

## Command Reference

### Just Commands (Backend)
- `just --list` - Show all available commands
- `just install-requirements` - Install Rust components and tools
- `just watch` - Watch for changes and restart server (requires `watchexec`)
- `just check` - Format check and linting
- `just fix` - Auto-fix formatting and clippy issues
- `just test` - Run all tests
- `just test-coverage` - Run tests with coverage report
- `just gen-api` - Regenerate API client code

### NPM Scripts (Frontend)
- `npm run start` - Start development server (same as `ng serve`)
- `npm run build:development` - Development build
- `npm run build:prod` - Production build (fails in sandboxed environments)
- `npm run build:review` - Review build configuration
- `npm run test` - Run tests in watch mode
- `npm run test-coverage` - Run tests with coverage
- `npm run watch` - Build in watch mode