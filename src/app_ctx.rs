use std::rc::Rc;

use crate::app_ws::*;
use crate::ctxs::*;
use crate::fallback::PleaseWait;
use yew::prelude::*;
#[derive(Clone, Debug, PartialEq, Default)]
pub enum Theme {
    Dark,
    #[default]
    Light,
}

#[function_component(AppCtx)]
pub fn app_ctx() -> Html {
    let Fallback = html! {<PleaseWait />};
    let theme = use_memo(|_| Theme::Light, ());
    let locale = use_reducer(|| I18nLocale::Zhcn);
    // let modal = use_reducer(|| Modal(false));
    let user = use_reducer(|| User::load());
    let user_contact = use_reducer(|| UserContact::load());
    let relay = use_reducer(|| Relay::load());
    let user_event = use_reducer(|| UserEvent::default());
    let subsciption = use_reducer(|| Subscription::default());
    let send_msg = use_reducer(|| SendMsgEvent::default());

    html!(
        <>
        <Suspense fallback={Fallback}>
            <ContextProvider<Rc<Theme>> context={theme}>
                    <ContextProvider<I18nLocaleContext> context={locale}>
                            <ContextProvider<UserContext> context={user}>
                                <ContextProvider<UserEventContext> context={user_event}>
                                    <ContextProvider<RelayContext> context={relay}>
                                        <ContextProvider<SendMsgEventContext> context={send_msg}>
                                            <ContextProvider<SubscriptionContext> context={subsciption}>
                                                <ContextProvider<UserContactContext> context={user_contact}>
                                                    <AppClient/>
                                                </ContextProvider<UserContactContext>>
                                            </ContextProvider<SubscriptionContext>>
                                        </ContextProvider<SendMsgEventContext>>
                                    </ContextProvider<RelayContext>>
                                </ContextProvider<UserEventContext>>
                        </ContextProvider<UserContext>>
                    </ContextProvider<I18nLocaleContext>>
            </ContextProvider<Rc<Theme>>>
        </Suspense>
    </>
    )
}
