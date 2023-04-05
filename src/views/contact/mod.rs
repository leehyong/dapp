mod entry;
mod recent;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::mem::swap;
use std::sync::atomic::{AtomicUsize, Ordering};

use self::entry::*;
use self::recent::*;
use crate::ctxs::*;

use yew::prelude::*;

pub struct Contact {
    i18n_handle: I18nLocaleContext,
    i18n_handle_listener: ContextHandle<I18nLocaleContext>,
    user_contact_handle_listner: ContextHandle<UserContactContext>,
    user_contact_handle: UserContactContext,
    new_contacts: Option<indexmap::IndexMap<usize, UserContactInfo>>, // 新增联系人列表
    update_contact: Option<UserContactInfo>,                          // 正在更新某个
    checked_contacts: HashSet<EntryItem>,
    check_all: bool, // 已经check的
}

#[derive(Debug, Clone)]
pub enum ContactMsg {
    I18nCtx(I18nLocaleContext),
    UserContactCtx(UserContactContext),
    NewUserContactInfo,
    CheckAllOrNot, // select all or not
    RevertAll,
    CbAction(EntryCbAction), // revert or not
    // CheckOne(EntryItem),
    // RemoveOne(EntryItem),
    DeleteCheckedUserContactInfo, // Delete all checked contacts
}
static PRIVATE_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
impl Contact {
    const MAX_NEW_CONTACTS: usize = 5;

    fn all_contact_pks(&self) -> HashSet<EntryItem> {
        (*self.user_contact_handle.data)
            .borrow()
            .values()
            .map(|c| c.pubkey.as_ref().unwrap().clone())
            .map(|pk| EntryItem::PubKey(pk))
            .collect()
    }

    fn all_contact_ids(&self) -> HashSet<EntryItem> {
        if let Some(new_list) = &self.new_contacts {
            new_list
                .iter()
                .map(|(idx, _)| EntryItem::Idx(*idx))
                .collect()
        } else {
            HashSet::with_capacity(0)
        }
    }
    fn new_contacts_view(&self, _ctx: &Context<Self>, cb_action: Callback<EntryCbAction>) -> Html {
        if let Some(new_contacts) = self.new_contacts.as_ref() {
            new_contacts
                .iter()
                .map(move |(idx, info)| {
                    // log::info!("{idx}-{}", serde_json::json!(info));
                    let idx = *idx;
                    let cb_action = cb_action.clone();
                    let check = self.checked_contacts.contains(&EntryItem::Idx(idx));
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

    fn contacts_view(&self, _ctx: &Context<Self>, cb_action: Callback<EntryCbAction>) -> Html {
        (*self.user_contact_handle.data)
            .borrow()
            .values()
            .map(move |info| {
                // log::info!("{idx}-{}", serde_json::json!(info));
                let cb_action = cb_action.clone();
                let pk = info.pubkey.unwrap();
                let key = pk.to_string();
                let check = self.checked_contacts.contains(&EntryItem::PubKey(pk));
                // log::info!("{check}");
                html!(
                    <Entry {key} {check} {cb_action} info={info.clone()}/>
                )
            })
            .collect::<Html>()
    }

    fn recents_view(&self, _ctx: &Context<Self>) -> Html {
        (*self.user_contact_handle.recent)
            .borrow()
            .iter()
            .enumerate()
            .map(move |(_idx, pk)| {
                let key = *pk;
                html!(
                    <Recent pubkey={key.clone()} key={key.to_string()} />
                )
            })
            .collect::<Html>()
    }
}

impl Component for Contact {
    type Message = ContactMsg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (i18n_handle, i18n_handle_listener) = ctx
            .link()
            .context(ctx.link().callback(ContactMsg::I18nCtx))
            .expect("No I18nLocaleContext Provided");
        let (user_contact_handle, user_contact_handle_listner) = ctx
            .link()
            .context(ctx.link().callback(ContactMsg::UserContactCtx))
            .expect("No UserContactContext Provided");

        Self {
            check_all: false,
            i18n_handle,
            i18n_handle_listener,
            user_contact_handle,
            user_contact_handle_listner,
            new_contacts: None,
            update_contact: None,
            checked_contacts: HashSet::new(),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // todo: set metadata
        // todo: save my contacts
        use ContactMsg::*;
        let mut update = false;
        match msg {
            NewUserContactInfo => {
                if self.new_contacts.is_none() {
                    self.new_contacts = Some(indexmap::IndexMap::new());
                }
                let cs = self.new_contacts.as_mut().unwrap();
                if cs.len() > Self::MAX_NEW_CONTACTS {
                    log::error!("no more add new contacts!");
                } else {
                    update = true;
                    let id = PRIVATE_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
                    cs.insert(id, UserContactInfo::empty());
                }
            }
            CheckAllOrNot => {
                if self.check_all {
                    self.checked_contacts.clear();
                } else {
                    let all_idxs = self.all_contact_ids();
                    let mut alls = self.all_contact_pks();
                    for idx in all_idxs {
                        alls.insert(idx);
                    }
                    swap(&mut self.checked_contacts, &mut alls);
                }
                self.check_all = !self.check_all;
                // log::info!("{:?}", self.checked_contacts);
                update = true;
            }
            RevertAll => {
                let all_pks = self.all_contact_pks();
                let mut all = self.all_contact_ids();

                if self.checked_contacts.is_empty() {
                    for pk in all_pks {
                        all.insert(pk);
                    }
                    swap(&mut self.checked_contacts, &mut all);
                } else {
                    all = all.union(&all_pks).into_iter().map(|f| *f).collect::<_>();
                    let diff = all
                        .difference(&self.checked_contacts)
                        .into_iter()
                        .map(|f| *f)
                        .collect::<HashSet<EntryItem>>();
                    self.checked_contacts.clear();
                    for item in diff {
                        self.checked_contacts.insert(item);
                    }
                }
                self.check_all = !self.checked_contacts.is_empty();
                update = true;
            }

            DeleteCheckedUserContactInfo => {
                if !self.checked_contacts.is_empty() {
                    let mut removes = HashSet::with_capacity(self.checked_contacts.len());
                    swap(&mut removes, &mut self.checked_contacts);
                    let mut pks = HashSet::new();
                    for item in removes {
                        match item {
                            EntryItem::PubKey(pk) => {
                                pks.insert(pk);
                            }
                            EntryItem::Idx(idx) => {
                                if let Some(ref mut new_contacts) = &mut self.new_contacts {
                                    // let mut tail = new_contacts.split_off(idx);
                                    // new_contacts.append(&mut tail);
                                    new_contacts.remove(&idx);
                                }
                            }
                        }
                    }
                    if !pks.is_empty() {
                        self.user_contact_handle
                            .dispatch(UserContactAction::RemoveBatch(pks));
                    }
                    update = true;
                }
            }

            CbAction(action) => match action {
                EntryCbAction::Delete(item) => match item {
                    EntryItem::Idx(idx) => {
                        if let Some(ref mut list) = self.new_contacts {
                            list.remove(&idx);
                            update = true;
                        }
                    }
                    _ => unreachable!(),
                },
                EntryCbAction::Check(item) => {
                    if self.checked_contacts.contains(&item) {
                        self.checked_contacts.remove(&item);
                    } else {
                        self.checked_contacts.insert(item);
                    }
                    update = true;
                }
            },
            /*DeleteNewUserContactInfo(idx) => {
                if let Some(uc) = &mut self.new_contacts {
                    if uc.len() > idx {
                        let mut after = uc.split_off(idx);
                        after.pop_front();
                        uc.append(&mut after);
                    } else {
                        log::warn!("len error:{}<{}", uc.len(), idx);
                    }
                }
            }
            DeleteUserContactInfo(pk) => {
                if let Some(_) = self.user_contact_handle.data.borrow().get(&pk) {
                    self.user_contact_handle
                        .dispatch(UserContactAction::Remove(pk));
                }
            }
            UpdateUserContactInfo(info) => {
                if let Some(_) = self
                    .user_contact_handle
                    .data
                    .borrow()
                    .get(&(***info).pubkey.unwrap())
                {
                    self.user_contact_handle
                        .dispatch(UserContactAction::Update(info));
                }
            }*/
            UserContactCtx(_) | I18nCtx(_) => update = true,
            _ => {
                log::info!("{:?}", msg);
            }
        }
        update
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let new_clk = ctx
            .link()
            .callback(|_e: MouseEvent| ContactMsg::NewUserContactInfo);
        let select_all_or_not_clk = ctx
            .link()
            .callback(|_: MouseEvent| ContactMsg::CheckAllOrNot);
        let revert_clk = ctx.link().callback(|_: MouseEvent| ContactMsg::RevertAll);
        let delete_checked_clk = ctx
            .link()
            .callback(|_: MouseEvent| ContactMsg::DeleteCheckedUserContactInfo);

        let cb_action = ctx.link().callback(|item| ContactMsg::CbAction(item));
        let new_contacts = self.new_contacts_view(ctx, cb_action.clone());
        let contacts = self.contacts_view(ctx, cb_action);

        let recents = self.recents_view(ctx);
        include!("../html/contact.html")
    }
}
