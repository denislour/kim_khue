use crate::{
    components::button::{ButtonSize, ButtonVariant, button_variants},
    i18n::I18N,
};
use topcoat::{
    Result,
    view::{View, class, component, view},
};

#[component]
pub async fn site_footer() -> Result<impl View> {
    Ok(view! {
        <footer class="site-footer shell">
            <a class="footer-brand" href="#top" translate="no">(I18N.brand)</a>
            <p>(I18N.footer)</p>
            <a
                class=(class!(
                    button_variants(ButtonVariant::Ghost, ButtonSize::Sm),
                    "back-to-top",
                ))
                href="#top"
            >
                (I18N.back_top)
                <span aria-hidden="true">" ↑"</span>
            </a>
        </footer>
    })
}
