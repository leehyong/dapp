use nostr_sdk::nostr::prelude::*;
use yew::Reducible;
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SendMsgEvent {
    pub msg: Option<ClientMessage>,
}

impl Reducible for SendMsgEvent {
    type Action = ClientMessage;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        std::rc::Rc::new(SendMsgEvent { msg: Some(action) })
    }
}
pub type SendMsgEventContext = yew::UseReducerHandle<SendMsgEvent>;
