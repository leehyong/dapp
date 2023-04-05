use crate::ctxs::*;
use crate::utils::*;
use nostr_sdk::nostr::secp256k1::XOnlyPublicKey;
use yew::prelude::*;
#[derive(Debug, Clone, Copy, Properties, PartialEq)]
pub struct RecentProps {
    pub pubkey: XOnlyPublicKey,
}
pub enum RecentMsg {
    UserContactCtx(UserContactContext),
}
pub struct Recent {
    user_contact_handle: UserContactContext,
}

impl Component for Recent {
    type Message = RecentMsg;
    type Properties = RecentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (user_contact_handle, _) = ctx
            .link()
            .context(ctx.link().callback(RecentMsg::UserContactCtx))
            .expect("No UserContactContext Provided");
        Self {
            user_contact_handle,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let check_btn_clk = { ctx.link().callback(move |_: MouseEvent| EntryMsg::Check) };
        // let update_confirm_btn_clk = ctx.link().callback(move |_: MouseEvent| EntryMsg::Update);
        // let delete_cancel_btn_clk = { ctx.link().callback(|_: MouseEvent| EntryMsg::Delete) };
        let pk = ctx.props().pubkey;
        let bw = self.user_contact_handle.data.borrow();
        let info = bw.get(&pk);
        if let Some(info) = info {
            let info = info.clone();
            include!("./components/recent.html")
        } else {
            html!(<></>)
        }
    }
}
