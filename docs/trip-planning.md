# Private trip planning

## Publication status

`main` contains only the documentation for this feature. The application changes
were removed from remote publication at the user's request on October 29, 2026.
The implementation described below is preserved only on the local
`feature/private-trips-mentoring` branch at commit
`8c7d89e9e1d3c1ca9258da1904d04ddaf5dd1d30` in the current workspace.
Source paths, commands, and verification below refer to that local implementation,
not the application code currently on `main`. See `PAUSED.md` before resuming.

## Step 1 — persistent storage (implemented locally; not published)

The `/trips` page lets an authenticated user save a trip and load it again after
refreshing the page. PostgreSQL stores the data independently of the session,
Redis cache, and application process.

A trip has a name, WGS84 origin and destination coordinates (latitude/longitude
in decimal degrees), a `car` or `truck` vehicle type, and a departure timestamp.
Input such as `2026-11-01T08:30:00+01:00` is stored as the same instant in UTC.
Coordinates are currently entered manually; they are neither generated nor
obtained through automatic location tracking.

Ownership comes exclusively from the validated Auth0 `sub` claim. There is no
`owner_id` in the input DTO. Every read/delete query also filters by owner.
Someone else's ID and a nonexistent ID return the same `404`. An administrator
role does not automatically grant access to private trips. Dispatcher sharing
will require an explicit business rule and permission; it is outside this step.

### Running the feature

1. Start PostgreSQL and apply `backend/migrations` using the existing migration
   service or Diesel CLI. From `backend`, for example:
   `diesel migration run --migration-dir ./migrations`.
   The new migration is `2026-10-28-000001_create_trips`.
2. Configure the existing Rocket database pool and Auth0 settings
   (`AUTH0_DOMAIN`, `AUDIENCE`, and OAuth callback/client configuration).
   Keep secret values in environment settings and out of logs.
3. Start the backend and frontend using the project's usual commands.
4. Log in and open "My trips". Saved names, coordinates, vehicles, and departure
   times remain available when the same account signs in again later.

In the prepared cloud environment, source `/workspace/.weather-setup/env.sh`,
then run `bash /workspace/.weather-setup/services.sh` to apply migrations.
For the frontend, keep Trunk output outside the checkout. From `frontend`:

```bash
env -u NO_COLOR trunk serve --locked \
  --dist /workspace/.weather-setup/dist \
  --root-certificate /usr/local/share/ca-certificates/environment-proxy-ca.crt \
  --port 8001
```

These cloud-specific parameters are not required in every local environment.

### HTTP contract

All routes require a validated JWT via the existing private cookie or a Bearer
header. There is no production test-token bypass. RS256 signature, issuer,
audience, expiration, and a nonempty subject are checked before accessing data.
Invalid tokens return `401`; unavailable auth configuration/JWKS returns `503`.

| Method | Route | Result |
|---|---|---|
| POST | `/backend/trips` | `201`, saved trip and a `Location` header |
| GET | `/backend/trips?limit=20&offset=0` | `200`, only the current user's trips |
| GET | `/backend/trips/{id}` | `200` or `404` |
| DELETE | `/backend/trips/{id}` | `204` or `404` |

POST and DELETE require `X-Requested-With: WeatherApp`. This prevents ordinary
cross-origin HTML forms from performing cookie-authenticated mutations. CORS
allows only known frontend origins and this header; do not open it to arbitrary
origins. Private responses, including errors, have `Cache-Control: no-store`.
Lists use stable `created_at DESC, id DESC` ordering, a limit of 1–100, and offset >= 0.

Example POST body:

```json
{
  "name": "Belgrade to Nis",
  "origin": {"latitude": 44.7866, "longitude": 20.4489},
  "destination": {"latitude": 43.3209, "longitude": 21.8958},
  "vehicle_type": "truck",
  "departure_at": "2026-11-01T08:30:00+01:00"
}
```

The backend validates finite GPS values and ranges, distinct endpoints, a name
of 1–100 characters, and a valid RFC3339 timestamp with an explicit timezone.
Unknown input fields and vehicle types are rejected. Valid past departure dates
are allowed for historical trips; forecast availability belongs to the next step.

### Verification

```bash
cargo test --locked -p backend jwt::
cargo test --locked -p backend trips::
# After applying migrations, explicitly enable the integration test:
TRIPS_TEST_DATABASE_URL="$DATABASE_URL" cargo test --locked -p backend trips:: -- --include-ignored
cargo check --locked -p frontend --target wasm32-unknown-unknown
```

The integration test uses real PostgreSQL and real HTTP handlers with a test
verifier injected only into the test application. It checks two distinct users,
owner-spoof rejection, cross-user read/delete denial, the CSRF header, pagination,
and persistence through a new DB connection. It removes only its own test rows.
Separate authentication tests verify locally signed RSA JWTs without network
access or Auth0 credentials.

These tests do not replace a live Auth0 login check. Production currently fetches
JWKS per request with a timeout; caching with key rotation support is a future
improvement if traffic warrants it. A saved trip does not imply that a road
route has been calculated. Selecting `truck` does not yet check height, weight,
or access restrictions.

Recorded verification for this step: 14 relevant backend tests passed, including
PostgreSQL integration and the existing Redis tests. Backend build, WASM build,
and frontend check passed. Browser form/list/delete checks used mocked HTTP
responses and did not verify live Auth0 login. The existing weather test failed
separately: it expects Barcelona data while the route uses the `Test city`
benchmark mock. This mismatch predates this step and was not changed here.

## Step 2 — actual routing and forecasts along the route (planned)

- A routing service calculates the road route, distance, and duration.
- Sample points along the actual route and estimate arrival times from routing
  data, explicitly labeling approximations when details are unavailable.
- A forecast API supplies weather for each location and expected arrival time.
  Outside the available forecast horizon, return "insufficient data" rather
  than invented weather.
- Store route/forecast snapshots with source and retrieval timestamps.

Beyond OpenWeather, this requires a routing API or a self-hosted routing service.
HERE/TomTom are candidates whose truck options, licensing, and coverage need
checking; a suitable Valhalla/openrouteservice setup may be an alternative.
A standard car profile does not establish truck access. Add vehicle dimensions,
weight, and supported restrictions before choosing a compatible truck provider.

A geocoding API is optional for addresses/place names and unnecessary for manual
coordinates. Browser Geolocation does not require a separate paid API key, but
requires user permission and HTTPS/loopback. Map display requires a library and
licensed/attributed tile data. No additional external API was called in step 1.

## Step 3 — private simulations and rerouting (planned)

The user will be able to add a temporary road closure or simulated weather hazard.
Each scenario belongs to the trip owner and has geometry, a validity interval,
and clearly labeled `simulation` provenance. It must not affect other users'
trips or appear as a confirmed public road incident.

Comparing original and revised routes should explain the avoided segment,
additional distance/duration, and reasons. The routing engine must actually
support avoiding the relevant segments/polygons. If no alternative exists,
report that outcome. Do not simulate rerouting with a changed line or invented ETA.

## Step 4 — learning to crawl one source (planned)

First check whether the selected roadworks/closure source offers an API or RSS,
and review its terms of use. The crawler is an adapter that:

1. Fetches documents with a timeout, size limit, and moderate request schedule.
2. Parses HTML into structured notices.
3. Normalizes location, direction, and validity interval.
4. Records source URL, retrieval timestamp, content hash, and parser version.
5. Detects duplicates and changes.

Learn and test the parser using local HTML fixtures, including changed page
structures. Parser failure marks the source as stale/unreliable; it does not
mean roads are open. Keep external notices separate from user simulations.

A dispatcher view follows these steps and requires explicitly authorized access
to trips. A dispatcher role alone should not expose every user's private trips.
