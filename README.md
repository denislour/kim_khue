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

- `src/app/mod.rs`: module router, root HTML layout, assets, runtime, and app context.
- `src/app/urls.rs`: encoded story/chapter links and filter-preserving library URLs.
- `src/app/tests.rs`: HTTP routing, layout, pagination, and draft-privacy regression tests.
- `src/app/_site/mod.rs`: the site route group and its header/footer layout.
- `src/app/_site/_home.rs`: the `/` page, story service calls, and visible composition tree.
- `src/app/_site/stories/mod.rs`: the English-named module mapped to the `truyen` URL segment.
- `src/app/_site/stories/slug.rs`: `/truyen/{slug}`, the story detail and chapter reading page.
- `src/i18n.rs`: interface messages; Vietnamese is currently enabled.
- `src/seed/data.rs`: demo metadata used only by the seed command.
- `styles.css`: the white and earth-gold Topcoat UI theme and Tailwind configuration.
- `styles.custom.css`: typography, responsive layouts, and custom presentation.
- `src/components/ui/`: Topcoat UI component families managed by the CLI.
- `src/app/_site/components/home/`: home sections; smaller library components live under `story_library/`.
- `src/app/_site/components/shared/`: site-shared components; card internals live under their parent component directory.
- `src/app/_site/components/reading/`: chapter index and chapter content for the story detail page.
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

Each title links to `/truyen/{slug}`, a server-rendered reading page. The page
lists every chapter, marks unpublished drafts, and shows the persisted chapter
text with previous/next navigation. Chapter links keep their position in
`?chapter=N`, so a chapter is shareable and survives a reload. Chapter pages
render without JavaScript; the chapter index uses a native disclosure that opens
by default on wide screens when the runtime is available.

**Bến Cũ Đông Ngàn** opens at <http://127.0.0.1:3000/truyen/ben-cu-dong-ngan>.
Click its cover or title in the library to start chapter one. Its 13 chapter
entries and complete prose are stored in SQLite. A fresh seed publishes all
13 chapters; subsequent seeding preserves deliberate publication changes.
The original chapter files are in `src/seed/dong_ngan/01_*.txt` through `13_*.txt`,
and the full story arc is in
`src/seed/dong_ngan/outline.md`. Running the seed command migrates the old
`river` demo in place, preserves subsequent author edits, and adds any missing
chapter entries or fills empty chapter drafts. The story update is atomic; rerunning
the seed command preserves nonempty author-edited chapters. `/truyen/river` redirects to the renamed story.

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
├── app/
│   ├── mod.rs                # Module router + root HTML layout
│   ├── urls.rs               # Web navigation helpers
│   ├── tests.rs              # HTTP regression tests
│   └── _site/
│       ├── mod.rs            # Site group + header/footer layout
│       ├── _home.rs          # GET /, service calls and component tree
│       ├── stories/
│       │   ├── mod.rs        # URL segment renamed to truyen
│       │   └── slug.rs       # GET /truyen/{slug}, chapter reading page
│       └── components/
│           ├── mod.rs
│           ├── home/         # mod.rs + home-specific components
│           ├── shared/       # mod.rs + story cards, header, footer
│           └── reading/      # mod.rs + chapter index and content
├── components/
│   ├── mod.rs
│   └── ui/                   # mod.rs + CLI-managed Topcoat UI files
├── services/
│   ├── mod.rs
│   └── story.rs              # Reusable story queries
└── seed/
    ├── mod.rs                # Seed operations
    ├── data.rs               # Demo metadata
    └── dong_ngan/
        ├── mod.rs            # Story metadata and embedded prose
        └── ...               # Chapter text and outline
```

`src/app/mod.rs` calls `module_router!()` and is the root of the route tree.

### Rust module style

Modules with child files use `folder/mod.rs`, rather than a sibling `folder.rs`.
For example, `components/ui/mod.rs` declares `button.rs` and `card.rs` in the
same folder. Leaf modules remain regular `.rs` files. This keeps each section's
entry point inside its folder; it does not reduce the total file count.
Rust module names and imports stay the same, so Topcoat's route paths and
layout nesting are unchanged. UI component file paths in `components.toml`
also stay unchanged.
Pages and layouts use bare `#[page]` and `#[layout]` attributes; the router
registers them from compiled Rust modules. Each module still needs a `mod`
declaration: Topcoat does not scan the filesystem.

| Handler | Served URL / scope | Purpose |
| --- | --- | --- |
| `app::root` | All pages | Root HTML document, styles, fonts, and scripts. |
| `app::_site::site_layout` | The `_site` group | Shared site header, skip link, and footer. |
| `app::_site::_home::home_page` | `/` | Home page. |
| `app::_site::stories::slug::story_page` | `/truyen/{slug}` | Story detail and chapter reading page. |

### Route groups and layouts

- `_site` is a route group, not a special layout filename. Its underscore
  removes the group name from public URLs. Its `#[layout]` function wraps
  only pages in that group.
- `_home` is another group module so its page serves `/`, not `/home`.
- `stories/mod.rs` uses `segment!(rename = "truyen")`: code names stay English
  while existing Vietnamese URLs remain unchanged. It declares no `/truyen`
  index page.
- `slug.rs` uses `module_param!(slug)` to define the dynamic URL segment.
- Unlike Next.js, `layout.rs` and `page.rs` are not special filenames.
  Topcoat identifies handlers through attributes and derives paths from modules.

Layout chains are **Root → Site → Home** and **Root → Site → Reading**.
The root layout contains no site header or footer. A future `app/admin/mod.rs`
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
`src/app/_site/components/`. Future admin-only components belong in
`src/app/admin/components/`. General Topcoat UI components stay in
`src/components/ui/`, shared across sections, with CLI registry paths unchanged.

The site's `components` module contains only components, not route handlers.
It does not expose a `/components` URL. Folder placement communicates ownership;
Rust module visibility controls access. Database operations remain in `services`.

Component internals are organized below their owning component:

```text
src/app/_site/components/
├── mod.rs
├── home/
│   ├── mod.rs
│   ├── home_hero.rs
│   ├── author_intro.rs
│   └── story_library/
│       ├── mod.rs
│       ├── story_search.rs
│       ├── genre_filters.rs
│       └── story_grid.rs
├── shared/
│   ├── mod.rs
│   ├── site_header.rs
│   ├── site_footer.rs
│   └── story_card/
│       ├── mod.rs
│       └── story_cover.rs
└── reading/
    ├── mod.rs
    ├── chapter_index.rs
    └── chapter_content.rs
```

The home and shared `mod.rs` files use `#[path]` to give their internal modules
distinct names and re-export the components. Pages and layouts can import the
component names without repeating their module names:

```rust
use super::components::home::{
    author_intro, home_hero, story_library,
};
```

Parent components compose their private child components internally. They use
`#[path]` to give each child module a distinct name, then import the component
for a short call without a module prefix:

```rust
// Inside home/story_library/mod.rs:
#[path = "story_search.rs"]
mod story_search_component;
use story_search_component::story_search;
```

This avoids a collision between the module name and the type generated by
Topcoat's `#[component]` macro while keeping the nested files in place. Topcoat UI components use their original family files, with
`components.toml` retaining the registry paths and hashes used by the CLI.

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
