mod _home;
mod components;
mod stories;

use self::components::shared::{site_footer, site_header};
use crate::i18n::I18N;
use topcoat::{
    Result,
    router::{Slot, layout},
    view::{View, view},
};

#[layout]
pub(crate) async fn site_layout(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <a class="skip-link" href="#main">(I18N.skip_content)</a>
        site_header()
        (slot)
        site_footer()
    })
}
