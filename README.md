# kim_khue

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

- `src/app.rs`: router configuration, assets, runtime, and app context.
- `src/app/_layout.rs`: the root document and shared layout component composition.
- `src/app/_home.rs`: home page section composition with Topcoat's `view!` macro.
- `src/i18n.rs`: interface messages; Vietnamese is currently enabled.
- `src/features.rs`: the sample `Story` data structure and demo stories.
- `styles.css`: the white and earth-gold Topcoat UI theme and Tailwind configuration.
- `styles.custom.css`: typography, responsive layouts, and custom presentation.
- `src/components/ui/`: Topcoat UI components managed by the CLI.
- `src/components/home/`: home page components, including the header, footer, hero, author introduction, reading dialog, story library, and story cards.
  The `story_library/`, `story_card/`, and `reader_dialog/` subdirectories contain their smaller child components.
- `app.js`: a backup of the legacy JavaScript logic, currently excluded from the page.
- `src/fonts.rs`, `assets/fonts/`: self-hosted fonts with Vietnamese support and OFL licenses.
- `assets/*.svg`: custom logo, landscape artwork, and story covers.

The interface locale is selected by `static I18N` in `src/i18n.rs`:

```rust
pub(crate) static I18N: &Messages = messages(Locale::Vi);
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
| `src/components/ui.rs`, `src/components/ui/` | Topcoat UI components; `components.toml` directs CLI additions here. |
| `src/components/home.rs`, `src/components/home/` | Home page and layout components. |
| `src/config.rs` | Load `.env` and application configuration before starting Tokio. |
| `src/db.rs`, `src/model.rs` | Shared Toasty database connection and models. |
| `src/bin/seed.rs`, `src/seed.rs` | Separate seed command and demo data insertion. |

Pages in `app` compose the components in `components/home`. These components
can use feature data and the building blocks in `components/ui`. Keep business
rules in `features`; Topcoat UI components remain independent of those features.

## Learn more

- [Getting started](https://github.com/tokio-rs/topcoat/blob/main/docs/getting_started.md)
- [Guides](https://github.com/tokio-rs/topcoat/tree/main#learn-topcoat)
