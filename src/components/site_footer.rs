use crate::{
    components::button::{ButtonSize, ButtonVariant, button_variants},
    i18n::TEXT,
};
use topcoat::{
    Result,
    view::{View, class, component, view},
};

#[component]
pub async fn site_footer() -> Result<impl View> {
    Ok(view! {
        <footer class="site-footer shell">
            <a class="footer-brand" href="#top" translate="no">(TEXT.brand)</a>
            <p>(TEXT.footer)</p>
            <a
                class=(class!(
                    button_variants(ButtonVariant::Ghost, ButtonSize::Sm),
                    "back-to-top",
                ))
                href="#top"
            >
                (TEXT.back_top)
                <span aria-hidden="true">" ↑"</span>
            </a>
        </footer>
    })
}
