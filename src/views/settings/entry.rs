use crate::ctxs::*;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum EntryStatus {
    New,
    Edit,
    View,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntryItem {
    NewIdx(usize),
    Idx(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntryCbAction {
    Delete(EntryItem),
    Check(EntryItem),
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct EntryProps {
    #[prop_or(false)]
    pub check: bool,
    #[prop_or(None)]
    pub idx: Option<usize>,
    pub info: RelayInfo,
    pub cb_action: Callback<EntryCbAction>,
}

#[derive(Debug)]
pub struct Entry {
    edit: bool,
    input_error_state: u8, // use bits tu judge the state of inputs
    relay_handle: RelayContext,
    relay_handle_listener: ContextHandle<RelayContext>,
    relay_node: NodeRef,
}

#[derive(Debug)]
pub enum EntryMsg {
    RelayCtx(RelayContext),
    Update,
    Delete,
    Confirm,
    Cancel,
    Check,
}

impl Entry {
    const RELAY_STATE: u8 = 1;

    fn validate_input_state(&mut self) {
        for (node, state) in vec![(&self.relay_node, Self::RELAY_STATE)] {
            if Self::is_ok(node) {
                match state {
                    Self::RELAY_STATE => {
                        if let Err(e) = nostr_sdk::nostr::Url::parse(&Self::input_node_val(node)) {
                            self.input_error_state |= state;
                            log::error!("relay error:{e:?}");
                        } else {
                            self.input_error_state &= !state;
                        }
                    }
                    _ => {
                        self.input_error_state &= !state;
                    }
                }
            } else {
                self.input_error_state |= state;
            }
        }
    }
    fn is_input_ok(&self) -> bool {
        if self.edit {
            self.input_error_state == 0
        } else {
            false
        }
    }
    fn is_ok(node: &NodeRef) -> bool {
        !Self::input_node_val(node).is_empty()
    }
    fn input_node_val(node: &NodeRef) -> String {
        if let Some(node) = &node.cast::<HtmlInputElement>() {
            node.value().trim().to_string()
        } else {
            "".to_string()
        }
    }
}

impl Component for Entry {
    type Message = EntryMsg;
    type Properties = EntryProps;

    fn create(ctx: &Context<Self>) -> Self {
        let pros = ctx.props();
        let (relay_handle, relay_handle_listener) = ctx
            .link()
            .context(ctx.link().callback(EntryMsg::RelayCtx))
            .expect("No UserContactContext Provided");
        Self {
            edit: pros.idx.is_some(),
            input_error_state: 0,
            relay_handle,
            relay_handle_listener,
            relay_node: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use EntryMsg::*;
        let mut update = false;
        let item = if let Some(idx) = ctx.props().idx {
            EntryItem::NewIdx(idx)
        } else {
            EntryItem::Idx(ctx.props().info.id)
        };
        match msg {
            Update => {
                self.edit = !self.edit;
                update = true;
            }
            Confirm => {
                if self.edit {
                    self.validate_input_state();
                    log::info!("{}", self.input_error_state);
                    if self.is_input_ok() {
                        let ralay = Self::input_node_val(&self.relay_node);
                        let relay = nostr_sdk::nostr::Url::parse(&ralay).unwrap();
                        self.edit = false;
                        self.relay_handle.dispatch(RelayAction::Add(relay));
                        ctx.link().send_message(Delete);
                    }
                }
                update = true;
            }
            Cancel => {
                if ctx.props().idx.is_some() {
                    ctx.props().cb_action.emit(EntryCbAction::Delete(item));
                }
                update = true;
                self.edit = false;
            }
            Delete => {
                if ctx.props().idx.is_some() {
                    ctx.props().cb_action.emit(EntryCbAction::Delete(item));
                } else {
                    self.relay_handle
                        .dispatch(RelayAction::Remove(ctx.props().info.id));
                }
                update = true;
            }
            Check => {
                ctx.props().cb_action.emit(EntryCbAction::Check(item));
                update = true;
            }
            RelayCtx(_) => {}
            _ => {
                log::info!("{msg:?}");
            }
        }
        update
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let is_edit = self.edit;
        let check = ctx.props().check;
        // log::info!("is_edit-:{is_edit}");
        let check_btn_clk = { ctx.link().callback(move |_: MouseEvent| EntryMsg::Check) };
        let update_confirm_btn_clk = ctx.link().callback(move |_: MouseEvent| {
            if is_edit {
                EntryMsg::Confirm
            } else {
                EntryMsg::Update
            }
        });
        let delete_cancel_btn_clk = {
            ctx.link().callback(move |_: MouseEvent| {
                if is_edit {
                    EntryMsg::Cancel
                } else {
                    EntryMsg::Delete
                }
            })
        };
        let is_relay_error = self.input_error_state & Self::RELAY_STATE == Self::RELAY_STATE;
        let input_cls_fn = |is_err: bool| if is_err { "is-danger" } else { "" };
        let relay_input_cls = input_cls_fn(is_relay_error);
        let info = &ctx.props().info;
        let relay_value = info
            .uri
            .as_ref()
            .map(|r| r.to_string())
            .or(Some(Self::input_node_val(&self.relay_node)))
            .unwrap();

        include!("./components/entry.html")
    }
}
