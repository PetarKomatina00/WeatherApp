# Paused: private trips and route planning

Status: PAUSED at the user's request on October 28, 2026.
Do not resume feature development until the user asks to continue.
On October 29, the user requested documentation-only publication on `main` and
removal of the previously published feature branch. The remote feature branch
was deleted; application changes are excluded from `main`.
The implementation remains on the local `feature/private-trips-mentoring` branch
at commit `8c7d89e9e1d3c1ca9258da1904d04ddaf5dd1d30` in the current workspace.
That local branch is not available through a fresh clone from GitHub.
Do not merge or publish its application changes without explicit authorization.
Feature development remains paused.

## Completed work preserved locally (not on main)

- PostgreSQL migration for private trips, applied in the current cloud environment.
- Create, paginated list, detail, and delete endpoints with ownership enforcement.
- GPS origin/destination, car/truck selection, and UTC-normalized departure time.
- Yew `/trips` page and a "My trips" link.
- Removed the authentication test bypass, tightened JWT validation, removed OAuth
  code/token logging, and added a CSRF mutation header and no-store responses.

## Verification recorded before the pause

- 14 relevant backend tests passed, including real PostgreSQL and Redis checks.
- Backend build, frontend check, and WASM build passed.
- Browser create/list/delete checks passed with mocked HTTP responses.
- The production HTTP guard rejected anonymous requests and the previous test token.
- Live Auth0 login was not verified.
- The existing weather test failed separately: it expects Barcelona data while the
  existing route returns the `Test city` benchmark mock. This predates the new code.

## Resuming

Read `docs/trip-planning.md` first and inspect `git status`, local branches, and
any existing diff. With the user's authorization to resume, locate the local
implementation branch before working on the feature. Preserve existing changes.
Do not reset the checkout or assume local processes, migrations, or the local
implementation branch are available in a new environment.

Next: select a routing service and calculate an actual road route between saved
coordinates. Truck routing requires checking provider support and vehicle dimensions
and restrictions. Then add forecasts along the route, private simulated road
closures/weather hazards, rerouting, and learning to crawl one verified source.

Simulations must remain private and clearly separate from confirmed public incidents.
Dispatcher access to private trips will require explicitly authorized sharing.

## Main files

- `backend/src/trips/`
- `backend/migrations/2026-10-28-000001_create_trips/`
- `shared/src/trips.rs`
- `frontend/src/pages/trips.rs`
- `frontend/src/api/trips.rs`
- `backend/src/jwt/jwt_guards.rs` and `jwt_utility.rs`
- `docs/trip-planning.md`
