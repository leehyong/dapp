use std::rc::Rc;

use nostr_sdk::nostr::{prelude::FromSkStr, Keys};

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::ctxs::*;

#[derive(Clone, Copy, Default, PartialEq)]
enum Tab {
    #[default]
    Auto,
    Import,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub struct ValidateState {
    pub import: bool,
    pub auto: bool,
}

impl ValidateState {
    pub fn reset(&mut self) {
        self.auto = false;
        self.import = false;
    }
}
#[derive(Clone, PartialEq, Properties)]
pub struct UserModalProps {
    pub show: bool,
    // pub cb: Callback<bool>,
}

#[function_component(UserModal)]
pub fn user_modal(props: &UserModalProps) -> Html {
    let user_ctx = use_context::<UserContext>().unwrap();
    let locale_ctx = use_context::<I18nLocaleContext>().unwrap();
    let lang = locale_ctx.to_string();
    let tab = use_state(Tab::default);
    let tab1 = tab.clone();
    let tab2 = tab.clone();
    let is_active_import = || *tab1 == Tab::Import;
    let is_active_auto_generate = || *tab2 == Tab::Auto;
    let auto_tab = tab.clone();
    let import_tab = tab.clone();
    let auto_tab_clk = Callback::from(move |_: MouseEvent| auto_tab.set(Tab::Auto));
    let import_tab_clk = Callback::from(move |_: MouseEvent| import_tab.set(Tab::Import));
    let auto_private_node_ref = use_node_ref();
    let auto_public_node_ref = use_node_ref();
    let keys: UseStateHandle<Option<Keys>> = use_state(|| None);
    let validate = use_state(|| ValidateState::default());
    let user_ctx_clone1 = user_ctx.clone();

    let hide_modal_clk = {
        let user_ctx_clone = user_ctx.clone();
        Callback::from(move |_: MouseEvent| {
            let cb_clone = user_ctx_clone.show_modal_cb.as_ref().unwrap().clone();
            cb_clone.emit(false);
        })
    };
    let import_private_node_ref = use_node_ref();
    let import_private_node_ref1 = import_private_node_ref.clone();
    let set_keys = {
        let keys_clone = keys.clone();
        let private_node_ref_clone = auto_private_node_ref.clone();
        let public_node_ref_clone = auto_public_node_ref.clone();
        Rc::new(move |keys: Keys| {
            if let Some(pub_key) = private_node_ref_clone.cast::<HtmlInputElement>() {
                pub_key.set_value(&keys.public_key().to_string());
            }
            if let Some(pri_key) = public_node_ref_clone.cast::<HtmlInputElement>() {
                pri_key.set_value(&keys.secret_key().unwrap().display_secret().to_string());
            }
            keys_clone.set(Some(keys));
        })
    };
    let generate_clk = {
        let validate_clone = validate.clone();
        let set_keys_clone = set_keys.clone();
        Callback::from(move |_: MouseEvent| {
            let keys = Keys::generate();
            (*set_keys_clone)(keys);
            validate_clone.set(ValidateState::default());
        })
    };
    use_effect_with_deps(
        move |_| {
            if let Some(keys) = &user_ctx_clone1.keys {
                (*set_keys)(keys.clone());
                if let Some(import_node) = import_private_node_ref1.cast::<HtmlInputElement>() {
                    import_node.set_value(
                        keys.secret_key()
                            .unwrap()
                            .display_secret()
                            .to_string()
                            .as_str(),
                    );
                }
            }
        },
        tab.clone(),
    );
    let import_input = {
        let keys_clone = keys.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            match Keys::from_sk_str(&input.value()) {
                Ok(k) => keys_clone.set(Some(k)),
                Err(e) => log::error!("parse error: {}", e),
            }
        })
    };
    let clear_import_clk = {
        let import_private_node_ref_clone = import_private_node_ref.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(import_node) = import_private_node_ref_clone.cast::<HtmlInputElement>() {
                import_node.set_value("");
            }
        })
    };
    let _import_private_node_ref_clone = import_private_node_ref.clone();

    let confirm_clk = {
        let user_ctx_clone = user_ctx.clone();
        let validate_clone = validate.clone();
        let keys_clone = keys.clone();
        let tab3 = tab.clone();

        Callback::from(move |_: MouseEvent| {
            let cb_clone = user_ctx_clone.show_modal_cb.as_ref().unwrap().clone();
            if let Some(_keys) = &*keys_clone {
                validate_clone.set(ValidateState::default());
                user_ctx.dispatch(UserContextMessage::KeysMsg(_keys.clone()));
                cb_clone.emit(false);
            } else {
                if *tab3 == Tab::Import {
                    validate_clone.set(ValidateState {
                        import: true,
                        ..*validate_clone
                    });
                } else if *tab3 == Tab::Auto {
                    validate_clone.set(ValidateState {
                        auto: true,
                        ..*validate_clone
                    });
                }
                log::warn!("no keys");
            }
        })
    };
    include!("html/user_modal.html")
}
