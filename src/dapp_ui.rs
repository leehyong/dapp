use crate::app_ctx::*;
use serde_json::json;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// todo
#[function_component(App)]
pub fn app() -> Html {
    html!(
        <AppCtx/>
    )
}

pub fn run_app() {
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    #[cfg(not(debug_assertions))]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::Renderer::<App>::new().render();
    log::info!("available_locales:{:?}", crate::available_locales());
    for i in 0..8 {
        log::info!(
            "Pubkey{i}-{:?}",
            json!(nostr_sdk::nostr::Keys::generate()
                .key_pair()
                .unwrap()
                .public_key())
            .as_str()
        );
    }
    rust_i18n::set_locale("zh-cn");
}
