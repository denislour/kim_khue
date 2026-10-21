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

Seed the three demo stories and their opening chapters:

```sh
cargo run --bin seed
```

The command is repeatable. Existing titles and chapters are preserved; missing
demo chapters are inserted, and known legacy demo text is upgraded:

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
The library reads story metadata and opening chapters from SQLite through the story service.
The seed command can insert three demo stories from `src/seed/data.rs`;
the interface does not import demo fixtures or fall back to them.

- `src/app.rs`: module router, root HTML layout, assets, runtime, and app context.
- `src/app/urls.rs`: encoded story/chapter links and filter-preserving library URLs.
- `src/app/tests.rs`: HTTP routing, layout, pagination, and draft-privacy regression tests.
- `src/app/_site.rs`: the site route group and its header/footer layout.
- `src/app/_site/_home.rs`: the `/` page, story service calls, and visible composition tree.
- `src/app/_site/stories.rs`: the English `/stories` route prefix.
- `src/app/_site/stories/slug.rs`: `/stories/{slug}`, the story detail and chapter reading page.
- `src/i18n.rs`: interface messages; Vietnamese is currently enabled.
- `src/seed/data.rs`: demo metadata used only by the seed command.
- `styles.css`: the white and earth-gold Topcoat UI theme and Tailwind configuration.
- `styles.custom.css`: typography, responsive layouts, and custom presentation.
- `src/components/ui/`: Topcoat UI component families managed by the CLI.
- `src/app/_site/_home/`: homepage-owned sections and story-library components.
- `src/app/_site/shared.rs` and `shared/`: site header and footer.
- `src/app/_site/stories/slug/`: chapter index and chapter content.
- `app.js`: a backup of the legacy JavaScript logic, currently excluded from the page.
- `src/fonts.rs`, `assets/fonts/`: self-hosted fonts with Vietnamese support and OFL licenses.
- `assets/*.svg`: custom logo, landscape artwork, and story covers.

The interface locale is selected by `static I18N` in `src/i18n.rs`:

```rust
pub(crate) static I18N: &Messages = messages(Locale::Vi);
```

Search matches titles and descriptions without case or Vietnamese diacritics.
Genre filters combine with the query; the result count, no-result state, and reset
button update through Topcoat signals and server-rendered page updates. `q`,
`genre`, and the one-based `page` parameter are synchronized to the URL and
restored on reload. The library shows up to 24 stories per page; previous/next
links preserve filters and work without JavaScript. Search or genre changes
reset the page to one. The result count includes all matches, not just the
visible page. Out-of-range pages show a link back to the first page.

Each title links to `/stories/{slug}`, a server-rendered reading page. The page
lists every chapter, marks unpublished drafts, and shows the persisted chapter
text with previous/next navigation. Chapter links keep their position in
`?chapter=N`, so a chapter is shareable and survives a reload. Chapter pages
render without JavaScript; the chapter index uses a native disclosure that opens
by default on wide screens when the runtime is available.

**Bến Cũ Đông Ngàn** opens at <http://127.0.0.1:3000/stories/ben-cu-dong-ngan>.
Click its cover or title in the library to start chapter one. Its 13 chapter
entries and complete prose are stored in SQLite. A fresh seed publishes all
13 chapters; subsequent seeding preserves deliberate publication changes.
The original chapter files are in `src/seed/dong_ngan/01_*.txt` through `13_*.txt`,
and the full story arc is in
`src/seed/dong_ngan/outline.md`. Running the seed command migrates the old
`river` demo in place, preserves subsequent author edits, and adds any missing
chapter entries or fills empty chapter drafts. The story update is atomic; rerunning
the seed command preserves nonempty author-edited chapters. The legacy story slug remains available at `/stories/river` and permanently redirects to the canonical story slug. The retired `/truyen/{slug}` route does not redirect.

Connecting to SQLite applies pending SQL files in `migrations/` through Toasty.
Applied versions and timestamps are stored in `schema_migrations`. Upgrades run
in a transaction: schema changes and migration history roll back together on
failure. Existing databases without history are adopted using the original
idempotent table migrations, preserving IDs and author content. Add future
numbered migrations to the `MIGRATIONS` list in `src/db.rs`; do not edit SQL
that has already been applied.

**Before the first upgraded startup, back up your SQLite database.** The new
publication migrations add `chapters.published` and mark existing non-whitespace
chapter content as published once, preserving previous visibility. Empty legacy
chapters remain drafts. New chapters default to unpublished even if they contain
text; publishing requires explicitly setting `published = TRUE`. Restarting or
seeding does not undo later decisions to unpublish. Public reading queries redact
unpublished bodies, and catalog previews exclude them; chapter titles remain
visible in the index with a draft label.

Startup never seeds story content. `--init` is still accepted by the seed command
for compatibility, but is no longer required. `main.rs` remains configuration,
connection, and startup.

To run the application directly, bundle its assets first:

```sh
topcoat asset bundle
cargo run
```

## Project structure

```text
src/
├── app.rs                    # Module router + root HTML layout
├── app/
│   ├── _site.rs              # Public-site layout
│   ├── tests.rs              # HTTP integration tests
│   ├── urls.rs               # Web navigation helpers
│   └── _site/
│       ├── _home.rs          # GET /
│       ├── _home/            # Homepage-owned components
│       ├── stories.rs        # /stories route prefix, no index page
│       ├── stories/slug.rs   # GET /stories/{slug}
│       └── shared.rs         # Site header/footer components
├── components.rs
├── components/ui.rs          # CLI-managed Topcoat UI modules
├── services.rs
├── services/story.rs        # Story queries and business logic
└── seed/
    ├── mod.rs                # Seed operations
    ├── data.rs               # Demo metadata
    └── dong_ngan/
        ├── mod.rs            # Story metadata and embedded prose
        └── ...               # Chapter text and outline
```

`src/app.rs` calls `module_router!()` and is the root of the route tree. This
project uses Rust's file-entry convention: a module with children is declared
in `name.rs`, and its child files live in `name/`. Topcoat registers only
compiled route modules; ordinary component modules do not create routes.
The Topcoat UI files remain in their current directory because
`components.toml` references those paths.

| Handler | Served URL / scope | Purpose |
| --- | --- | --- |
| `app::root` | All pages | Root HTML document, styles, fonts, and scripts. |
| `app::_site::site_layout` | The `_site` group | Shared site header, skip link, and footer. |
| `app::_site::_home::home_page` | `/` | Home page. |
| `app::_site::stories::slug::story_page` | `/stories/{slug}` | Story detail and chapter reading page. |

### Route groups and layouts

- `_site` is a route group, not a special layout filename. Its underscore
  removes the group name from public URLs. Its `#[layout]` function wraps
  only pages in that group.
- `_home` is another group module so its page serves `/`, not `/home`.
- `stories.rs` supplies the `/stories` prefix; it does not add a `/stories`
  index page. The detail handler uses `module_param!(slug)`.
- `slug.rs` uses `module_param!(slug)` to define the dynamic URL segment.
- Unlike Next.js, `layout.rs` and `page.rs` are not special filenames.
  Topcoat identifies handlers through attributes and derives paths from modules.

Layout chains are **Root → Site → Home** and **Root → Site → Reading**.
The root layout contains no site header or footer. A future `app/admin.rs`
with its own `#[layout]` and child modules can serve `/admin/...`
under **Root → Admin**, without inheriting the site layout. No admin routes
are added until that feature is needed.

This follows the module-routing pattern in Topcoat's
[official routing example](https://github.com/tokio-rs/topcoat/tree/main/examples/module-router)
and [admin demo](https://github.com/tokio-rs/topcoat/tree/main/demos/admin).

Pages call services and compose the main sections. The home page renders the
hero, story library, and author introduction. It passes the queried story models
to the library, which owns its search, filters, grid, and empty states.
The site layout composes the shared header and footer. Search and genre signals
live on the home page and are passed to the library.

### Component ownership

Use one rule: section-specific UI lives inside its route section; general UI
lives at the application level. Website components live in
`src/app/_site/` beside the route that owns them. General Topcoat UI components stay at
`src/components/ui/`, shared across sections, with CLI registry paths unchanged.

The site's `components` module contains only components, not route handlers.
It does not expose a `/components` URL. Folder placement communicates ownership;
Rust module visibility controls access. Database operations remain in `services`.

Page-specific components are colocated with their owning page:

```text
app/_site/
├── _home.rs
├── _home/
│   ├── home_hero.rs
│   ├── author_intro.rs
│   ├── story_library.rs
│   └── story_library/
│       ├── story_search.rs
│       ├── genre_filters.rs
│       ├── story_grid.rs
│       ├── story_card.rs
│       └── story_card/story_cover.rs
├── stories/slug.rs
├── stories/slug/
│   ├── chapter_index.rs
│   └── chapter_content.rs
├── shared.rs
└── shared/
    ├── site_header.rs
    └── site_footer.rs
```

The route files use explicit `mod` declarations to include their colocated
components. `#[path]` aliases are used when distinct names avoid collisions with
the component macro. To add a page, add a route module under `_site/`, declare
it in `_site.rs`, and add a `#[page]` handler. To add a shared site component,
put it under `shared/` and declare it in `shared.rs`. Story-detail components
belong under `stories/slug/` and are declared from `slug.rs`.

Parent components compose their private child components internally. Explicit
`#[path]` aliases retain distinct module names where needed to avoid collisions
with the names generated by Topcoat's `#[component]` macro.

Topcoat UI components use their original family files, with `components.toml`
retaining the registry paths and hashes used by the CLI.

Keep database queries and business operations in `services`. The home page
initializes query/genre/page signals and calls `services::story::search_page`;
its tracked signal reads let Topcoat refresh the rendered library. Components
receive the catalog and signal handles through parameters. Business data stays
out of i18n. Web navigation belongs in `app::urls`, not the story service.

An unfiltered or genre-only catalog uses database counts and ordered pagination.
Nonempty Vietnamese accent-insensitive searches scan title/summary projections
in batches of 128; memory stays bounded, but total search work is still O(N).
Complete story rows and published chapter-one previews are loaded only for the
selected page. At larger scale, an indexed normalized search column or full-text
search can replace that scan. The reader fetches one selected chapter body plus
a lightweight index projection, not every chapter's full prose.

`app::_site::stories::slug` calls `services::story::read` and renders the chapter index and
chapter content components. `browser.rs` contains small Topcoat `raw!` bridges
for theme persistence, URL history, and cover-hover geometry, which are outside
the 0.10 Rust runtime vocabulary. There is no application JavaScript file loaded:
`app.js` remains a backup, and `topcoat::runtime::script()` supplies the runtime.

`src/config.rs` loads `.env` before Tokio starts. `src/db.rs`, `src/model.rs`,
and `migrations/` own the Toasty connection, Story/Chapter models, and additive
SQLite setup. `src/bin/seed.rs`, `src/seed/mod.rs`, and `src/seed/data.rs` provide the
separate seed command and demo fixtures.

## Artwork and typography

The earlier `assets/landscape.svg` and `assets/logo.svg` were replaced by
`landscape-hero.svg`, `logo-kim-khue.svg`, and a separate small-size
`favicon.svg`; the previous drawings remain in git history. The new landscape retains the original
mountains, palette, and whitespace, with a leaning, conical-hatted boatman and an oblique pavilion drawn
with flowing eaves, tapered posts, open bays, and stone steps. The mountain
book cover uses the same refined artwork. The new mark keeps the roof,
open book, and single hexagonal outline, with fewer strokes. The author section
displays that mark without surrounding rings. Book covers carry
`seal-kim-khue.svg`: a cinnabar seal with hand-drawn Vietnamese “Kim Khuê”
letterforms and a lightly worn edge. The seal uses vector paths rather than
a font or the former “KK” text badge.

Visual references:

- [The Met: Chinese Gardens](https://www.metmuseum.org/exhibitions/listings/2012/chinese-gardens)
- [The Met: Pavilion on a pond](https://www.metmuseum.org/art/collection/search/51391)
- [Lucide: book-open](https://lucide.dev/icons/book-open),
  [search](https://lucide.dev/icons/search), moon, x, and directional arrows.

The logo and landscape are project SVG drawings; interface icons use Lucide
through Topcoat's Iconify integration.

The site uses one typeface throughout: **Cormorant Garamond**, licensed under OFL.
`fonts.rs` registers one local variable font covering Latin, Vietnamese, and
combining marks. It is subset from the [official Google Fonts source](https://github.com/google/fonts/tree/main/ofl/cormorantgaramond), version 4.001, with the original outlines preserved.
Headings, navigation, summaries, controls, covers, and chapter text all use it.
Small text sizes are adjusted for readability; the original glyphs and Vietnamese
diacritics are preserved. Other font files are not registered or loaded.
The license is retained in `assets/fonts/cormorantgaramond-OFL.txt`.

## Validation

```sh
cargo test --lib --locked --offline
cargo check --all-targets --locked --offline
cargo fmt --all -- --check
topcoat fmt --check src
```

The tests use in-memory SQLite, including simulated legacy schema upgrades.
HTTP tests exercise the actual module router and nested layouts. A test-only
asset catalog is discovered from the test executable, so no pre-bundling,
network requests, or real database changes are needed to run them.

## Learn more

- [Getting started](https://github.com/tokio-rs/topcoat/blob/main/docs/getting_started.md)
- [Guides](https://github.com/tokio-rs/topcoat/tree/main#learn-topcoat)


## Completed interface updates

- Extended the story introduction panels and original demo descriptions.
- Documented the original typefaces and the custom CSS applied to them.
- Replaced decorative text/CSS icons with simple Lucide line icons.
- Preserved the old landscape and added a new hero SVG with a refined boat/pavilion.
- Preserved the old logo and added a simpler roof/book mark and dedicated favicon.
- Restored library search, filters, and chapter reading with Topcoat signals and SQLite chapters.
