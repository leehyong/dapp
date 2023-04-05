mod websocket_worker;

pub use websocket_worker::{WebsocketWorker, WebsocketWorkerInputMsg, WorkerOutput};

use crate::relay_ctx::RelayContext;

#[derive(Debug)]
pub enum WorkerMsg {
    Event(WorkerOutput),
    RelayInfo,
    Result,
    Auth,
}
