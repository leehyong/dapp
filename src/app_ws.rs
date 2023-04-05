use crate::ctxs::*;
use crate::route::*;
use nostr_sdk::{
    Client, ClientMessage, Contact, Filter, Kind, RelayMessage, RelayPoolNotification, Timestamp,
    Url,
};
use std::cell::RefCell;
use std::collections::HashMap;

use std::rc::Rc;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug)]
pub enum AppMsg {
    ReconnectWebsocket(Url, usize),
    RelayCtx(RelayContext),
    UserEventCtx(UserEventContext),
    UserCtx(UserContext),
    RelayPoolNotification(RelayPoolNotification),
    SendMsgEventCtx(SendMsgEventContext),
    UserContactCtx(UserContactContext),

    Noop,
}

#[derive(Debug, Clone)]
pub struct WsInfo {
    pub uri: Url,
    pub retry: i32, // 记录失败重连之后重试次数， 0表示是成功连接的
}
pub struct AppClient {
    client: Option<Rc<RefCell<Client>>>,
    cur_relays: Rc<RefCell<HashMap<usize, WsInfo>>>,
    send_msg_handler: SendMsgEventContext,
    _send_msg_listener: ContextHandle<SendMsgEventContext>,
    user_event_handle: UserEventContext,
    _user_event_listener: ContextHandle<UserEventContext>,
    relay_handle: RelayContext,
    _relay_listener: ContextHandle<RelayContext>,
    user_handle: UserContext,
    _user_handle_listener: ContextHandle<UserContext>,
    user_contact_listener: ContextHandle<UserContactContext>,
    user_contact_handle: UserContactContext,
}

impl AppClient {
    const MAX_RETRIES: i32 = 10;

    async fn reconnect(
        client: Rc<RefCell<Client>>,
        cur_relays: Rc<RefCell<HashMap<usize, WsInfo>>>,
        retry_connect: Callback<(Url, usize)>,
        uri: Url,
        id: usize,
    ) {
        if let Err(e) = client.borrow().connect_relay(uri.to_string()).await {
            log::error!("connect {uri} error{e:?}");
            if let nostr_sdk::client::Error::RelayPool(_re) = e {
                // retry
                if let Some(mut _info) = cur_relays.borrow_mut().get_mut(&id) {
                    if _info.retry < Self::MAX_RETRIES {
                        _info.retry += 1;
                        retry_connect.emit((uri.clone(), id));
                    }
                } else {
                    log::warn!("no such {uri}")
                }
            }
        }
    }

    fn connect(&mut self, ctx: &Context<Self>) -> bool {
        let mut update = false;
        let all_relays = self.relay_handle.list.borrow();
        let remove_relays = self
            .cur_relays
            .borrow()
            .iter()
            .filter(|(k, _)| !all_relays.contains_key(*k))
            .map(|(k, info)| (*k, info.clone()))
            .collect::<Vec<_>>();
        if !remove_relays.is_empty() {
            update = true;
            for (k, _r) in &remove_relays {
                self.cur_relays.borrow_mut().remove(k);
            }
            if let Some(client) = self.client.clone() {
                spawn_local(async move {
                    // disconnect old ones
                    for (_, relay) in remove_relays {
                        if let Err(e) = client.borrow().disconnect_relay(relay.uri).await {
                            log::warn!("{e:?}");
                        };
                    }
                });
            }
        }
        let news = all_relays
            .iter()
            .filter(|(k, info)| !self.cur_relays.borrow().contains_key(*k) && info.uri.is_some())
            .map(|(_, info)| info.clone())
            .collect::<Vec<_>>();
        if !news.is_empty() {
            update = true;
            for info in &news {
                self.cur_relays.borrow_mut().insert(
                    info.id,
                    WsInfo {
                        uri: info.uri.clone().unwrap(),
                        retry: 0,
                    },
                );
            }
            if let Some(client) = self.client.clone() {
                let relay_infos = self.cur_relays.clone();
                let retry_connect = ctx
                    .link()
                    .callback(|(uri, id)| AppMsg::ReconnectWebsocket(uri, id));
                spawn_local(async move {
                    let client = client;
                    let relay_infos = relay_infos;
                    let retry_connect = retry_connect;
                    for info in news {
                        let uri = info.uri.as_ref().unwrap();
                        if let Err(e) = client.borrow().add_relay(uri.to_string()).await {
                            log::warn!("add {uri} error:{e:?}");
                            continue;
                        }
                        Self::reconnect(
                            client.clone(),
                            relay_infos.clone(),
                            retry_connect.clone(),
                            uri.clone(),
                            info.id,
                        )
                        .await
                    }
                });
            }
        }
        if let Some(client) = self.client.clone() {
            let callback = ctx
                .link()
                .callback(|notification| AppMsg::RelayPoolNotification(notification));
            spawn_local(async move {
                loop {
                    let mut notifications = client.borrow().notifications();

                    while let Ok(notification) = notifications.recv().await {
                        match notification {
                            RelayPoolNotification::Shutdown => return, // exit the loop
                            _ => callback.emit(notification),
                        }
                    }
                }
            });
        }
        update
    }

    fn subscribe_contacts(&self) {
        if let Some(client) = &self.client {
            let keys = client.borrow().keys();
            let client = client.clone();
            let subscription = Filter::new().pubkey(keys.public_key());
            // .since(Timestamp::now());
            let subscription = subscription
                .pubkeys(
                    self.user_contact_handle
                        .data
                        .borrow()
                        .values()
                        .map(|contact| contact.pubkey.unwrap())
                        .collect::<Vec<_>>(),
                )
                .since(Timestamp::now());
            spawn_local(async move {
                client.borrow().subscribe(vec![subscription]).await;
            });
        }
    }

    fn set_nostr_contacts(&self) -> bool {
        let mut update = false;
        let contacts = self
            .user_contact_handle
            .data
            .borrow()
            .values()
            .map(|contact| Contact {
                pk: contact.pubkey.unwrap(),
                relay_url: contact.relay.as_ref().map(|u| u.to_string()),
                alias: contact.nickname.clone(),
            })
            .collect::<Vec<_>>();
        if let Some(client) = self.client.clone() {
            spawn_local(async move {
                if let Err(e) = client.borrow().set_contact_list(contacts).await {
                    log::warn!("{e:?}");
                }
            });
            update = true;
        }
        update
    }
}

impl Component for AppClient {
    type Message = AppMsg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (send_msg_handler, _send_msg_listener) = ctx
            .link()
            .context(ctx.link().callback(AppMsg::SendMsgEventCtx))
            .expect("No SendMsgEventContext Provided");
        let (user_event_handle, _user_event_listener) = ctx
            .link()
            .context(ctx.link().callback(AppMsg::UserEventCtx))
            .expect("No UserEventCtx Provided");
        let (user_contact_handle, user_contact_listener) = ctx
            .link()
            .context(ctx.link().callback(AppMsg::UserContactCtx))
            .expect("No UserEventCtx Provided");

        let (user_handle, _user_handle_listener) = ctx
            .link()
            .context(ctx.link().callback(AppMsg::UserCtx))
            .expect("No UserEventCtx Provided");

        let (relay_handle, _relay_listener) = ctx
            .link()
            .context(ctx.link().callback(AppMsg::RelayCtx))
            .expect("No UserEventCtx Context Provided");
        let mut _self = Self {
            user_event_handle,
            user_contact_listener,
            user_contact_handle,
            client: None,
            send_msg_handler,
            _send_msg_listener,
            _user_event_listener,
            _relay_listener,
            user_handle,
            _user_handle_listener,
            relay_handle,
            cur_relays: Rc::new(RefCell::new(HashMap::new())),
        };
        _self
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut _update = false;
        match msg {
            AppMsg::ReconnectWebsocket(uri, id) => {
                if let Some(client) = self.client.clone() {
                    let relay_infos = self.cur_relays.clone();
                    let retry_connect = ctx
                        .link()
                        .callback(|(uri, id)| AppMsg::ReconnectWebsocket(uri, id));
                    spawn_local(async move {
                        Self::reconnect(client, relay_infos, retry_connect, uri, id).await
                    });
                }
            }
            AppMsg::SendMsgEventCtx(msg_ctx) => {
                if let Some(_msg) = &msg_ctx.msg {
                    // _update = self.handle_client_msg(ctx, msg);
                    if let Some(client) = &self.client {
                        let client = client.clone();
                        let msg = _msg.clone();
                        let user_event_handle = self.user_event_handle.clone();
                        spawn_local(async move {
                            if let Err(e) = client.borrow().send_msg(msg.clone()).await {
                                log::debug!("{e:?}");
                            } else if let ClientMessage::Event(event) = msg {
                                user_event_handle.dispatch(UserEventAction::Add(*event.clone()));
                            }
                        })
                    } else {
                        log::debug!("no client, and then no send!");
                    }
                }
            }
            AppMsg::UserCtx(user_ctx) => {
                if let Some(keys) = user_ctx.keys.clone() {
                    if self.client.is_some() {
                        // disconnect old
                        if self.client.as_ref().unwrap().borrow().keys() != keys {
                            let client = self.client.take().unwrap();
                            log::warn!("disconnect old connections");
                            spawn_local(async move {
                                client.borrow().disconnect().await.unwrap();
                            });
                        }
                    }
                    // make new client
                    self.client = Some(Rc::new(RefCell::new(Client::new(&keys))));
                    _update = self.connect(ctx);
                    self.set_nostr_contacts();
                    self.subscribe_contacts();
                } else {
                    log::warn!("no user keys");
                }
            }
            AppMsg::RelayPoolNotification(notification) => {
                match notification {
                    RelayPoolNotification::Message(_, rmsg) => {
                        if let RelayMessage::Ok {
                            event_id,
                            status,
                            message: _,
                        } = rmsg
                        {
                            self.user_event_handle
                                .dispatch(UserEventAction::Visible(event_id, status));
                        }
                        // if let Some(msg) = &self.send_msg_handler.msg {
                        //     log::debug!("{msg:?}");
                        // }
                    }

                    RelayPoolNotification::Event(_, event) => {
                        if event.kind == Kind::TextNote {
                            self.user_event_handle
                                .dispatch(UserEventAction::AddVisible(event));
                        } else {
                            log::info!("notification {event:?}");
                        }
                    } // todo
                    RelayPoolNotification::Shutdown => {
                        log::info!("Shutdown");
                    } //todo
                }
            }
            AppMsg::RelayCtx(_) => _update = self.connect(ctx),
            AppMsg::UserContactCtx(_) => {
                self.subscribe_contacts();
                _update = self.set_nostr_contacts();
            }
            _ => {
                log::info!("default {:?}", msg);
            }
        }
        _update
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <BrowserRouter>
                <Switch<MainRoute> render={switch_main} />
            </BrowserRouter>
        )
    }
}
