use crate::ctxs::*;

use indexmap::IndexMap;
use nostr_sdk::nostr::Url;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use yew::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relay {
    #[serde(skip)]
    count: u64,
    pub list: Rc<RefCell<IndexMap<usize, RelayInfo>>>,
}

impl PartialEq for Relay {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelayInfo {
    pub id: usize,
    pub uri: Option<Url>,
}

pub enum RelayAction {
    Remove(usize),
    RemoveBatch(HashSet<usize>),
    Add(Url),
    Update(usize, Url),
}
impl LoadStoreKey for Relay {
    fn load_store_key() -> &'static str {
        "relay"
    }
}

impl Default for Relay {
    fn default() -> Self {
        Self {
            list: Rc::new(RefCell::new(
                vec![
                    "wss://relay.damus.io",
                    "wss://nostr.oxtr.dev",
                    "wss://nostr.bitcoiner.social",
                    "wss://nostr.openchain.fr",
                ]
                .into_iter()
                .map(|ws| {
                    let info = RelayInfo::new(ws);
                    (info.id, info)
                })
                .collect::<IndexMap<usize, _>>(),
            )),
            count: 0,
        }
    }
}

static RELAY_ID: AtomicUsize = AtomicUsize::new(0);

impl RelayInfo {
    #[inline]
    fn generate_id() -> usize {
        RELAY_ID.fetch_add(1, Ordering::AcqRel)
    }

    fn new(uri: &str) -> Self {
        Self::new2(Url::parse(uri).ok())
    }
    pub fn new2(uri: Option<Url>) -> Self {
        Self {
            id: Self::generate_id(),
            uri,
        }
    }
    pub fn empty() -> Self {
        Self {
            id: Self::generate_id(),
            uri: None,
        }
    }
}

// 最大relay个数
const MAX_RELAY_SIZE: usize = 50;

impl Reducible for Relay {
    type Action = RelayAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        use RelayAction::*;
        let mut count = self.count;
        let list = match action {
            Remove(i) => {
                let list = self.list.clone();
                list.borrow_mut().remove(&i);
                list
            }
            RemoveBatch(idxs) => {
                let list = self.list.clone();
                for idx in idxs {
                    list.borrow_mut().remove(&idx);
                }
                list
            }
            Add(relay) => {
                if self.list.borrow().len() == MAX_RELAY_SIZE {
                    return self.clone();
                }
                let relay = RelayInfo::new2(Some(relay));
                let list = self.list.clone();
                list.borrow_mut().insert(relay.id, relay);
                list
            }
            Update(i, relay) => {
                let list = self.list.clone();
                let relay_info = RelayInfo {
                    id: i,
                    uri: Some(relay),
                };
                list.borrow_mut().insert(i, relay_info);
                list
            }
        };
        count = count.checked_add(1).or(Some(0)).unwrap();
        list.borrow_mut().sort_by(|_, v1, _, v2| v2.id.cmp(&v1.id));
        let ret = Rc::new(Relay { list, count });
        ret.store();
        ret
    }
}

pub type RelayContext = UseReducerHandle<Relay>;
