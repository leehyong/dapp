use crate::ctxs::*;
use yew::prelude::*;

#[function_component(PleaseWait)]
pub fn please_wait() -> Html {
    let locale_ctx = use_context::<I18nLocaleContext>().unwrap();
    let lang = locale_ctx.to_string();
    let loading = rust_i18n::t!("loading...", locale = &lang);
    html! {
        <span class="icon-text has-text-info">
            <span class="icon">
                <i class="fas fa-spinner fa-pulse"></i>
            </span>
            <span>{loading}</span>
        </span>
    }
}
