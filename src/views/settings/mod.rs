mod entry;
use yew::prelude::*;

use self::entry::*;
use crate::ctxs::*;

use std::borrow::Borrow;
use std::collections::{HashSet};
use std::mem::swap;



pub struct Settings {
    i18n_handle: I18nLocaleContext,
    i18n_handle_listener: ContextHandle<I18nLocaleContext>,
    relay_handle_listener: ContextHandle<RelayContext>,
    relay_handle: RelayContext,
    new_relays: Option<indexmap::IndexMap<usize, RelayInfo>>, // 新增联系人列表
    checked_relays: HashSet<EntryItem>,
    check_all: bool, // 已经check的
}

#[derive(Debug, Clone)]
pub enum SettingsMsg {
    I18nCtx(I18nLocaleContext),
    RelayCtx(RelayContext),
    NewRelayInfo,
    CheckAllOrNot, // select all or not
    RevertAll,
    CbAction(EntryCbAction), // revert or not
    // CheckOne(EntryItem),
    // RemoveOne(EntryItem),
    DeleteCheckedRelayInfo, // Delete all checked contacts
}
impl Settings {
    const MAX_NEW_RELAYS: usize = 5;

    fn all_relay_pks(&self) -> HashSet<EntryItem> {
        (*self.relay_handle.list)
            .borrow()
            .values()
            .map(|c| c.id)
            .map(|pk| EntryItem::Idx(pk))
            .collect()
    }

    fn all_relay_ids(&self) -> HashSet<EntryItem> {
        if let Some(new_list) = &self.new_relays {
            new_list
                .iter()
                .map(|(idx, _)| EntryItem::NewIdx(*idx))
                .collect()
        } else {
            HashSet::with_capacity(0)
        }
    }
    fn new_relays_view(&self, _ctx: &Context<Self>, cb_action: Callback<EntryCbAction>) -> Html {
        if let Some(new_relays) = self.new_relays.as_ref() {
            new_relays
                .iter()
                .map(move |(idx, info)| {
                    // log::info!("{idx}-{}", serde_json::json!(info));
                    let idx = *idx;
                    let cb_action = cb_action.clone();
                    let check = self.checked_relays.contains(&EntryItem::NewIdx(idx));
                    // let now = chrono::Utc::now();
                    html!(
                        <Entry key={idx} idx={Some(idx)} {check} {cb_action} info={info.clone()}/>
                    )
                })
                .collect::<Html>()
        } else {
            html!(<></>)
        }
    }

    fn relays_view(&self, _ctx: &Context<Self>, cb_action: Callback<EntryCbAction>) -> Html {
        (*self.relay_handle.list)
            .borrow()
            .values()
            .map(move |info| {
                // log::info!("{idx}-{}", serde_json::json!(info));
                let cb_action = cb_action.clone();
                let key = info.id;
                let check = self.checked_relays.contains(&EntryItem::Idx(key));
                // log::info!("{check}");
                html!(
                    <Entry {key} {check} {cb_action} info={info.clone()}/>
                )
            })
            .collect::<Html>()
    }
}

impl Component for Settings {
    type Message = SettingsMsg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (i18n_handle, i18n_handle_listener) = ctx
            .link()
            .context(ctx.link().callback(SettingsMsg::I18nCtx))
            .expect("No I18nLocaleContext Provided");
        let (relay_handle, relay_handle_listener) = ctx
            .link()
            .context(ctx.link().callback(SettingsMsg::RelayCtx))
            .expect("No UserContactContext Provided");

        Self {
            check_all: false,
            i18n_handle,
            i18n_handle_listener,
            relay_handle,
            relay_handle_listener,
            new_relays: None,
            checked_relays: HashSet::new(),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // todo: set metadata
        // todo: save my contacts
        use SettingsMsg::*;
        let mut update = false;
        match msg {
            NewRelayInfo => {
                if self.new_relays.is_none() {
                    self.new_relays = Some(indexmap::IndexMap::new());
                }
                let cs = self.new_relays.as_mut().unwrap();
                if cs.len() > Self::MAX_NEW_RELAYS {
                    log::error!("no more add new contacts!");
                } else {
                    update = true;
                    let relay_info = RelayInfo::empty();
                    cs.insert(relay_info.id, relay_info);
                }
            }
            CheckAllOrNot => {
                if self.check_all {
                    self.checked_relays.clear();
                } else {
                    let all_idxs = self.all_relay_ids();
                    let mut alls = self.all_relay_pks();
                    for idx in all_idxs {
                        alls.insert(idx);
                    }
                    swap(&mut self.checked_relays, &mut alls);
                }
                self.check_all = !self.check_all;
                update = true;
            }
            RevertAll => {
                let all_pks = self.all_relay_pks();
                let mut all = self.all_relay_ids();

                if self.checked_relays.is_empty() {
                    for pk in all_pks {
                        all.insert(pk);
                    }
                    swap(&mut self.checked_relays, &mut all);
                } else {
                    all = all.union(&all_pks).into_iter().map(|f| *f).collect::<_>();
                    let diff = all
                        .difference(&self.checked_relays)
                        .into_iter()
                        .map(|f| *f)
                        .collect::<HashSet<EntryItem>>();
                    self.checked_relays.clear();
                    for item in diff {
                        self.checked_relays.insert(item);
                    }
                }
                self.check_all = !self.checked_relays.is_empty();
                update = true;
            }

            DeleteCheckedRelayInfo => {
                if !self.checked_relays.is_empty() {
                    let mut removes = HashSet::with_capacity(self.checked_relays.len());
                    swap(&mut removes, &mut self.checked_relays);
                    let mut pks = HashSet::new();
                    for item in removes {
                        match item {
                            EntryItem::Idx(idx) => {
                                pks.insert(idx);
                            }
                            EntryItem::NewIdx(idx) => {
                                if let Some(ref mut new_relays) = &mut self.new_relays {
                                    new_relays.remove(&idx);
                                }
                            }
                        }
                    }
                    if !pks.is_empty() {
                        self.relay_handle.dispatch(RelayAction::RemoveBatch(pks));
                    }
                    update = true;
                }
            }

            CbAction(action) => match action {
                EntryCbAction::Delete(item) => match item {
                    EntryItem::Idx(idx) => {
                        if let Some(ref mut list) = self.new_relays {
                            list.remove(&idx);
                            update = true;
                        }
                    }
                    _ => unreachable!(),
                },
                EntryCbAction::Check(item) => {
                    if self.checked_relays.contains(&item) {
                        self.checked_relays.remove(&item);
                    } else {
                        self.checked_relays.insert(item);
                    }
                    update = true;
                }
            },
            RelayCtx(_) | I18nCtx(_) => update = true,
            _ => {
                log::info!("{:?}", msg);
            }
        }
        update
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let new_clk = ctx
            .link()
            .callback(|_e: MouseEvent| SettingsMsg::NewRelayInfo);
        let select_all_or_not_clk = ctx
            .link()
            .callback(|_: MouseEvent| SettingsMsg::CheckAllOrNot);
        let revert_clk = ctx.link().callback(|_: MouseEvent| SettingsMsg::RevertAll);
        let delete_checked_clk = ctx
            .link()
            .callback(|_: MouseEvent| SettingsMsg::DeleteCheckedRelayInfo);

        let cb_action = ctx.link().callback(|item| SettingsMsg::CbAction(item));
        let new_relays = self.new_relays_view(ctx, cb_action.clone());
        let relays = self.relays_view(ctx, cb_action);

        include!("../html/settings.html")
    }
}
