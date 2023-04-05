use crate::ctxs::*;
use crate::{route::MainRoute, views::UserModal};
use yew::prelude::*;
use yew_hooks::{use_is_first_mount, use_mount};
use yew_router::prelude::*;
#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let route: MainRoute = use_route().unwrap_or(MainRoute::Home);
    let user_ctx = use_context::<UserContext>().unwrap();
    let locale_ctx = use_context::<I18nLocaleContext>().unwrap();
    let locale_ctx_clone = locale_ctx.clone();
    let cur_lang = locale_ctx.to_string();
    let eng_clk = Callback::from(move |_e: MouseEvent| locale_ctx.dispatch(I18nLocale::En));
    log::trace!("{}", &cur_lang);
    let zhcn_clk =
        Callback::from(move |_e: MouseEvent| locale_ctx_clone.dispatch(I18nLocale::Zhcn));
    let is_home = || route == MainRoute::Home;
    let is_contact = || route == MainRoute::Contact;
    let is_setting = || route == MainRoute::Settings || route == MainRoute::SettingsRoot;
    let is_first = use_is_first_mount();
    let user_ctx_clone = user_ctx.clone();
    let user_ctx_clone2 = user_ctx.clone();
    // 控制用户窗口是否显示
    let user_modal = use_state(|| is_first && user_ctx_clone.show_modal());

    let cb = {
        let user_modal_clone = user_modal.clone();
        Callback::from(move |val: bool| {
            user_modal_clone.set(val);
        })
    };

    use_mount(move || {
        user_ctx_clone2.dispatch(UserContextMessage::ModalCb(cb));
    });

    let profile_clk = {
        let user_modal_clone = user_modal.clone();
        Callback::from(move |_: MouseEvent| {
            user_modal_clone.set(true);
        })
    };
    include!("html/layout.html")
}
