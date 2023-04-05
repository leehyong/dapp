use crate::ctxs::*;
use crate::utils::*;
use nostr_sdk::nostr::prelude::*;

use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let locale_ctx = use_context::<I18nLocaleContext>().unwrap();
    let _user_ctx = use_context::<UserContext>().unwrap();
    let lang = locale_ctx.to_string();
    let send_event_ctx = use_context::<SendMsgEventContext>().unwrap();
    let txtarea_ref = use_node_ref();
    let empty_txt_area = use_state(|| false);
    let empty_txt_area1 = empty_txt_area.clone();
    let empty_txt_area2 = empty_txt_area.clone();
    let area_change = Callback::from(move |_: yew::Event| {
        empty_txt_area1.set(false);
    });
    let user_events_ctx = use_context::<UserEventContext>().unwrap();
    let send_clk = {
        let txtarea_ref_clone = txtarea_ref.clone();
        let _user_ctx_clone = _user_ctx.clone();
        Callback::from(move |_e: MouseEvent| {
            if _user_ctx_clone.show_modal() {
                let cb = _user_ctx_clone.show_modal_cb.as_ref().unwrap().clone();
                cb.emit(true);
            } else if let Some(txt_area) = txtarea_ref_clone.cast::<HtmlTextAreaElement>() {
                if !txt_area.value().is_empty() {
                    // when message is not empty, send!
                    let keys = _user_ctx_clone.keys.as_ref().unwrap();
                    match EventBuilder::new_text_note(txt_area.value().as_str(), &vec![])
                        .to_event(keys)
                    {
                        Ok(e) => {
                            txt_area.set_value("");
                            send_event_ctx.dispatch(ClientMessage::new_event(e));
                        }
                        Err(e) => log::warn!("{e}"),
                    }
                } else {
                    empty_txt_area2.set(true);
                }
            }
        })
    };
    include!("html/home.html")
}
