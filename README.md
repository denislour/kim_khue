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

## Giao diện Kim Khuê

Thiết kế thư viện cổ trang với tranh sơn thủy, logo SVG riêng và bìa sách đóng chỉ.
Các truyện hiện có là **nội dung minh họa**, có thể thay bằng sáng tác của bạn
trong `src/features.rs` (danh sách `SAMPLE_STORIES`).

- `src/app.rs`: layout và HTML qua macro `view!` của Topcoat.
- `src/i18n.rs`: thông điệp giao diện; hiện bật tiếng Việt.
- `src/features.rs`: cấu trúc dữ liệu `Story` và truyện mẫu.
- `styles.css`: theme vàng đất của Topcoat UI và cấu hình Tailwind.
- `styles.custom.css`: typography, responsive và cách trình bày riêng của Kim Khuê.
- `src/components/`: component Topcoat UI.
- `app.js`: bản sao lưu logic JavaScript cũ, hiện không được nhúng vào trang.
- `src/fonts.rs`, `assets/fonts/`: font tự lưu trên máy chủ, có dấu tiếng Việt và giấy phép OFL.
- `assets/*.svg`: logo, tranh và bìa sách vẽ riêng.

Chọn ngôn ngữ bằng `static TEXT` trong `src/app.rs`:

```rust
static TEXT: &Messages = messages(Locale::Vi);
```

Danh sách truyện và phần đọc thử dùng HTML `<details>`. Giao diện tìm kiếm và
bộ lọc đang ẩn; các thao tác trong dialog đọc chưa được nối bằng procedure/signal.
The application connects to SQLite through Toasty and registers the database
in the app context. The interface still reads `SAMPLE_STORIES`; the seed command
only inserts data into the database. Starting the server does not create tables
or seed data automatically.

Để chạy bản build trực tiếp, cần bundle tài nguyên trước:

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
