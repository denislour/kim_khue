# kimkhue

A [Topcoat](https://github.com/tokio-rs/topcoat) application.

## Development

Install the Topcoat CLI:

```sh
cargo install topcoat-cli
```

Copy `.env.example` to `.env` and configure `DATABASE_URL`:

```sh
cp .env.example .env
```

Initialize a new database and seed the three demo stories:

```sh
cargo run --bin seed -- --init
```

For an existing database, seed without creating tables. Stories whose slugs
already exist are skipped:

```sh
cargo run --bin seed
```

Start the development server, which rebuilds the application when you edit its source:

```sh
topcoat dev
```

Then open <http://127.0.0.1:3000>.

## Kim Khuê interface

A literary library inspired by historical settings, with landscape artwork,
a custom SVG logo, and covers styled after thread-bound books.
The current stories are **demo content**. Replace them with your own writing
in the `SAMPLE_STORIES` list in `src/features.rs`.

- `src/app.rs`: page layout and HTML rendered with Topcoat's `view!` macro.
- `src/i18n.rs`: interface messages; Vietnamese is currently enabled.
- `src/features.rs`: the sample `Story` data structure and demo stories.
- `styles.css`: the white and earth-gold Topcoat UI theme and Tailwind configuration.
- `styles.custom.css`: typography, responsive layouts, and custom presentation.
- `src/components/`: Topcoat UI components.
- `app.js`: a backup of the legacy JavaScript logic, currently excluded from the page.
- `src/fonts.rs`, `assets/fonts/`: self-hosted fonts with Vietnamese support and OFL licenses.
- `assets/*.svg`: custom logo, landscape artwork, and story covers.

The interface locale is selected by `static TEXT` in `src/app.rs`:

```rust
static TEXT: &Messages = messages(Locale::Vi);
```

Story previews use native HTML `<details>` elements. Search and filter controls
are currently hidden, and reading dialog actions have not yet been connected
to procedures or signals.
The application connects to SQLite through Toasty and registers the database
in the app context. The interface still reads `SAMPLE_STORIES`; the seed command
only inserts data into the database. Starting the server does not create tables
or seed data automatically.

To run the application directly, bundle its assets first:

```sh
topcoat asset bundle
cargo run
```

## Project structure

| Path | Contents |
| --- | --- |
| `src/app.rs`, `src/app/` | The route tree: the router, layouts, pages, and routes. |
| `src/features.rs`, `src/features/` | Everything one feature owns, such as its data and business rules. |
| `src/config.rs` | Load `.env` and application configuration before starting Tokio. |
| `src/db.rs`, `src/model.rs` | Shared Toasty database connection and models. |
| `src/bin/seed.rs`, `src/seed.rs` | Separate seed command and demo data insertion. |

Pages in `app` use features and components. A feature can use shared components but never imports from `app`, and shared components never import from a feature.

## Learn more

- [Getting started](https://github.com/tokio-rs/topcoat/blob/main/docs/getting_started.md)
- [Guides](https://github.com/tokio-rs/topcoat/tree/main#learn-topcoat)
