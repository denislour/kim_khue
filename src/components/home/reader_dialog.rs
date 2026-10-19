mod reader_content;
mod reader_toolbar;

use crate::components::ui::dialog::{dialog, dialog_content};
use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

#[component]
pub async fn reader_dialog() -> Result<impl View> {
    Ok(view! {
        dialog(
            open: false,
            attrs: attributes! {
                class="reader"
                id="reader"
                aria-labelledby="reader-title"
            },
            dialog_content(
                attrs: attributes! { class="reader-panel" },
                reader_toolbar::reader_toolbar()
                reader_content::reader_content()
            )
        )
    })
}
