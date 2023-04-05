use crate::ctxs::*;

use indexmap::IndexMap;
use nostr_sdk::nostr::prelude::*;

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{HashSet, LinkedList};
use std::rc::Rc;
use std::str::FromStr;
use yew::prelude::*;

const MAX_RECENTS: usize = 10;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserContact {
    pub data: Rc<RefCell<IndexMap<XOnlyPublicKey, UserContactInfo>>>,
    pub recent: Rc<RefCell<LinkedList<XOnlyPublicKey>>>,
    #[serde(skip)]
    count: u64, //for making the reducible UserContact in responsetive to change,  add the count field
}

impl PartialEq for UserContact {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UserContactInfo {
    pub pubkey: Option<XOnlyPublicKey>,
    pub avatar_url: Option<String>,
    pub relay: Option<Url>,
    pub nickname: Option<String>,
}

impl UserContactInfo {
    pub fn empty() -> Self {
        Self {
            pubkey: None,
            avatar_url: None,
            relay: None,
            nickname: None,
        }
    }
}

impl Default for UserContact {
    fn default() -> Self {
        let pk =
            Keys::from_pk_str("npub180cvv07tjdrrgpa0j7j7tmnyl2yr6yr7l8j4s3evf6u64th6gkwsyjh6w6")
                .unwrap();
        let pk = pk.public_key();
        let mut map = IndexMap::new();
        let mut d = UserContactInfo {
            pubkey: Some(pk.clone()),
            avatar_url: None,
            relay: Some(Url::from_str("wss://relay.damus.io").unwrap()),
            nickname: Some("lhy1".to_owned()),
        };
        map.insert(pk.clone(), d.clone());
        let mut list = LinkedList::new();
        list.push_front(pk);
        let pk = nostr_sdk::nostr::Keys::generate().public_key();
        // let pk = pk.x_only_public_key().0;
        d.pubkey = Some(pk.clone());
        map.insert(pk.clone(), d);
        list.push_front(pk);

        Self {
            data: Rc::new(RefCell::new(map)),
            recent: Rc::new(RefCell::new(list)),
            count: 0,
        }
    }
}

impl LoadStoreKey for UserContact {
    fn load_store_key() -> &'static str {
        "user-contacts"
    }
}
impl UserContact {
    // const USER_CONTACT_KEY: &'static str = "user-contacts";

    // pub fn load() -> Self {
    //     let ret = if let Ok(user) = LocalStorage::get::<UserContact>(Self::USER_CONTACT_KEY) {
    //         user
    //     } else {
    //         Self::default()
    //     };
    //     // log::info!("{:?}", ret);
    //     ret
    // }

    // pub fn store(&self) {
    //     LocalStorage::set(Self::USER_CONTACT_KEY, self).expect("store user-contact error ")
    // }
}
pub enum UserContactAction {
    Remove(XOnlyPublicKey),
    RemoveBatch(HashSet<XOnlyPublicKey>),
    Recent(XOnlyPublicKey),

    Add(UserContactInfo),
    Update(UserContactInfo),
}

impl Reducible for UserContact {
    type Action = UserContactAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        use UserContactAction::*;
        let mut count = self.count;
        match action {
            Remove(pk) => {
                self.data.borrow_mut().remove(&pk);
            }
            Add(info) | Update(info) => {
                self.data
                    .borrow_mut()
                    .insert(info.pubkey.as_ref().unwrap().clone(), info);
            }
            Recent(pk) => {
                let mut recent = self.recent.borrow_mut();
                if recent.len() == MAX_RECENTS {
                    // gurantee MAX_RECENTS item
                    recent.pop_back();
                }
                // place the latest contact in the front of others
                recent.push_front(pk);
            }
            RemoveBatch(pks) => {
                for pk in pks {
                    self.data.borrow_mut().remove(&pk);
                }
            }
        };
        count = count.checked_add(1).or(Some(0)).unwrap();
        let ret = Rc::new(Self {
            data: self.data.clone(),
            recent: self.recent.clone(),
            count,
        });

        ret.store();
        ret
    }
}

pub type UserContactContext = UseReducerHandle<UserContact>;
