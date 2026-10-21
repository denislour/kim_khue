//! Browser-only DOM bridges for APIs outside Topcoat 0.10's Rust vocabulary.
//! Application state and data flow remain in Rust signals and handlers.
use topcoat::runtime::{Expr, Signal, expr};

/// Native chapter disclosure follows the screen size and remains usable without JS.
pub fn chapter_index_open(initial: &Signal<bool>) -> Expr<bool> {
    expr!({
        let open = initial.get();
        raw!(
            r#"(() => {
                const media = matchMedia('(min-width: 960px)');
                if (!window.__kimKhueChapterMedia) {
                    media.addEventListener('change', event => {
                        const picker = document.querySelector('.chapter-picker');
                        if (picker) picker.open = event.matches;
                    });
                    window.__kimKhueChapterMedia = true;
                }
                return cx.hydrate(media.matches);
            })()"#,
            open
        )
    })
}

pub fn sync_library_url(
    query: &Signal<String>,
    genre: &Signal<String>,
    page: &Signal<usize>,
) -> Expr<String> {
    expr!({
        let q = query.get();
        let g = genre.get();
        let p = page.get();
        raw!(
            r#"(() => {
            const url = new URL(location.href);
            const q = String(${q}).trim();
            const genre = String(${g});
            if (q) url.searchParams.set('q', q); else url.searchParams.delete('q');
            if (genre !== 'all') url.searchParams.set('genre', genre); else url.searchParams.delete('genre');
            const page = Number(${p});
            if (page > 1) url.searchParams.set('page', String(page)); else url.searchParams.delete('page');
            if (url.href !== location.href) history.replaceState(null, '', url);
            return ${q};
        })()"#,
            {
                let _ = (g, p);
                q
            }
        )
    })
}

/// Apply the saved preference, falling back to the operating system's scheme.
pub fn site_theme(theme: &Signal<String>) -> Expr<String> {
    expr!({
        let value = theme.get();
        raw!(
            r#"(() => {
                const root = document.documentElement;
                const requested = String(${value});
                let preference = requested;
                if (requested === 'system') {
                    try { preference = localStorage.getItem('kim-khue-theme') || 'system'; } catch (_) {}
                }
                if (!['light', 'dark'].includes(preference)) preference = 'system';
                const media = matchMedia('(prefers-color-scheme: dark)');
                const apply = () => {
                    const selected = root.dataset.themePreference;
                    const dark = selected === 'dark' || (selected === 'system' && media.matches);
                    root.classList.toggle('dark', dark);
                    root.style.colorScheme = dark ? 'dark' : 'light';
                    document.querySelector('#site-theme')?.setAttribute('aria-pressed', String(dark));
                    document.querySelector('meta[name="theme-color"]')?.setAttribute('content', dark ? '#30271e' : '#ffffff');
                };
                root.dataset.themePreference = preference;
                if (requested !== 'system') {
                    try { localStorage.setItem('kim-khue-theme', preference); } catch (_) {}
                }
                apply();
                if (!root.__kimKhueThemeListener) {
                    media.addEventListener('change', () => {
                        if (root.dataset.themePreference === 'system') apply();
                    });
                    root.__kimKhueThemeListener = true;
                }
                return ${value};
            })()"#,
            value
        )
    })
}

/// Wait for a deliberate hover; Rust's signal still controls the open state.
pub fn book_hover_device() -> Expr<bool> {
    expr!(raw!(
        "cx.hydrate(matchMedia('(hover: hover) and (pointer: fine)').matches)",
        false
    ))
}

/// The timer is canceled when the pointer leaves the cover.
pub fn book_hover(open: &Signal<bool>, id: &str) -> Expr<()> {
    let stage_id = id.to_owned();
    let geometry = book_geometry(id);
    expr!({
        raw!(
            r#"(() => {
                ${geometry};
                const stage = document.getElementById(String(${stage_id}));
                if (stage) {
                    clearTimeout(stage.__bookHoverTimer);
                    if (matchMedia('(hover: hover) and (pointer: fine)').matches) {
                        stage.__bookHoverTimer = setTimeout(() => {
                            stage.__bookHoverTimer = null;
                            if (!stage.isConnected || !stage.matches(':hover')) return;
                            ${open}.set(cx.hydrate(true));
                        }, 1000);
                    }
                }
                return cx.hydrate(null);
            })()"#,
            {
                let _ = (stage_id, open, geometry);
            }
        )
    })
}

/// Keep a turning leaf within the viewport without moving or scaling its book.
pub fn book_geometry(id: &str) -> Expr<()> {
    let stage_id = id.to_owned();
    expr!({
        raw!(
            r#"(() => {
                const stage = document.getElementById(String(${stage_id}));
                if (stage) {
                    const rect = stage.getBoundingClientRect();
                    const room = Math.max(0, rect.left - 16);
                    let low = Math.PI / 2;
                    let high = Math.PI;
                    for (let i = 0; i < 24; i++) {
                        const angle = (low + high) / 2;
                        const reach = -rect.width * Math.cos(angle) * 2000 /
                            (2000 - rect.width * Math.sin(angle));
                        if (reach <= room) low = angle; else high = angle;
                    }
                    stage.style.setProperty('--book-open-angle', (-low * 180 / Math.PI) + 'deg');
                    stage.style.setProperty('--book-bend', Math.min(48, room / rect.width * 72) + 'deg');
                }
                return cx.hydrate(null);
            })()"#,
            {
                let _ = stage_id;
            }
        )
    })
}

pub fn cancel_book_hover(id: &str) -> Expr<()> {
    let stage_id = id.to_owned();
    expr!({
        raw!(
            r#"(() => {
                const stage = document.getElementById(String(${stage_id}));
                if (stage) {
                    clearTimeout(stage.__bookHoverTimer);
                    stage.__bookHoverTimer = null;
                }
                return cx.hydrate(null);
            })()"#,
            {
                let _ = stage_id;
            }
        )
    })
}
