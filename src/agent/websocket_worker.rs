use gloo_timers::callback::Timeout;
use nostr_sdk::nostr::Url;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};
use yew_agent::{HandlerId, Private, WorkerLink};

pub struct WebsocketWorker {
    link: WorkerLink<Self>,
    uri: Option<Url>,
    handle_id: Option<HandlerId>,
    id: u32,
}

#[derive(Serialize, Deserialize)]
pub enum WebsocketWorkerInputMsg {
    Uri(String),
    Exit,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkerOutput {
    pub value: u32,
    pub uri: String,
}

pub enum ThisWorkerMsg {
    Loop,
    Stop,
}
static PRIVATE_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

impl yew_agent::Worker for WebsocketWorker {
    type Input = WebsocketWorkerInputMsg;
    type Message = ThisWorkerMsg;
    type Output = WorkerOutput;
    type Reach = Private<Self>;

    fn create(link: WorkerLink<Self>) -> Self {
        let id = PRIVATE_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            id,
            link,
            uri: None,
            handle_id: None,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        // no messaging
        log::info!("WebsocketWorker update");
        match msg {
            ThisWorkerMsg::Loop => {
                let hdl = self.handle_id;
                let lnk = self.link.clone();
                let uri = format!("{}-{}", self.uri.as_ref().unwrap().as_str(), self.id);
                log::info!("{uri}");
                // let timeout = Timeout::new(1_000, move || {
                //     lnk.send_message(ThisWorkerMsg::Loop);
                //     if let Some(handle_id) = hdl {
                //         lnk.respond(handle_id, WorkerOutput { value: 1000, uri });
                //     }
                //     // Do something after the one second timeout is up!
                // });

                // // Since we don't plan on cancelling the timeout, call `forget`.
                // timeout.forget();
            }
            ThisWorkerMsg::Stop => {}
        }
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        match msg {
            WebsocketWorkerInputMsg::Uri(uri) => {
                if let Ok(_uri) = Url::parse(&uri) {
                    self.uri = Some(_uri);
                    self.link.send_message(ThisWorkerMsg::Loop);
                }
            }
            WebsocketWorkerInputMsg::Exit => self.link.send_message(ThisWorkerMsg::Stop),
        }
        self.handle_id = Some(id);
    }

    fn name_of_resource() -> &'static str {
        "worker.js"
    }

    fn resource_path_is_relative() -> bool {
        true
    }
}
