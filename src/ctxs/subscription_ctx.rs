#![allow(missing_docs)]

use std::rc::Rc;

use indexmap::IndexMap;
use nostr_sdk::nostr::{Filter, SubscriptionId as NostrSubscriptionId};
use yew::prelude::*;

type SubscriptionId = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Subscription {
    subs: IndexMap<SubscriptionId, Vec<Filter>>,
}

// 每个relay 都可以发送 订阅
impl Default for Subscription {
    fn default() -> Self {
        Self {
            subs: IndexMap::with_capacity(10),
        }
    }
}

pub enum SubscriptionMessage {
    Remove(SubscriptionId),
    Add(Vec<Filter>),
    Update(SubscriptionId, Vec<Filter>),
}
impl Reducible for Subscription {
    type Action = SubscriptionMessage;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        use SubscriptionMessage::*;
        let mut clone = self.subs.clone();
        match action {
            Remove(id) => clone.remove(&id),
            Add(filters) => clone.insert(NostrSubscriptionId::generate().to_string(), filters),
            Update(id, filters) => clone.insert(id, filters),
        };
        Rc::new(Self { subs: clone })
    }
}

pub type SubscriptionContext = UseReducerHandle<Subscription>;
