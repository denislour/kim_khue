use crate::i18n::I18N;
use topcoat::{
    Result,
    view::{View, component, view},
};

#[component]
pub async fn site_footer() -> Result<impl View> {
    let year = time::OffsetDateTime::now_utc().year();

    Ok(view! {
        <footer class="site-footer shell">
            <p>(I18N.footer)</p>
            <p class="footer-copyright">
                "© "
                (year)
                " "
                <span translate="no">(I18N.brand)</span>
            </p>
        </footer>
        <a
            class="back-to-top"
            href="#top"
            aria-label=(I18N.back_top)
            title=(I18N.back_top)
        >
            <svg
                class="back-to-top-icon"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 40 44"
                fill="none"
                aria-hidden="true"
                focusable="false"
            >
                <defs>
                    <linearGradient id="back-top-earth" x1="0" y1="0" x2="1" y2="1">
                        <stop stop-color="var(--brand-gold)" />
                        <stop offset="1" stop-color="var(--brand-brown)" />
                    </linearGradient>
                </defs>
                <path
                    d="M20 5C16 13 11 19 5 23L6 26C12 23 17 18 20 12C23 18 28 23 34 26L35 23C29 19 24 13 20 5Z"
                    fill="url(#back-top-earth)"
                />
                <path
                    d="M20 15C17 21 23 25 19 31L15 37C22 34 24 30 22 25C20 21 21 19 20 15Z"
                    fill="url(#back-top-earth)"
                />
                <path
                    d="M7 38Q13 34 20 38Q27 34 33 38"
                    stroke="url(#back-top-earth)"
                    stroke-width="1.3"
                    stroke-linecap="round"
                />
            </svg>
        </a>
    })
}
