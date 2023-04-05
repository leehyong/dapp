use std::rc::Rc;

use crate::ctxs::*;

use nostr_sdk::nostr::Keys;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(with = "keys_serde")]
    pub keys: Option<Keys>,
    pub nick_name: String,
    pub avatar_url: String,
    #[serde(skip)]
    pub show_modal_cb: Option<Callback<bool>>,
}

impl LoadStoreKey for User {
    fn load_store_key() -> &'static str {
        "nostr-user-info"
    }
}

impl User {
    // const USER_KEY: &'static str = "nostr-user-info";
    pub fn show_modal(&self) -> bool {
        self.keys.is_none()
    }

    pub fn new() -> User {
        let keys = Keys::generate();
        Self {
            keys: Some(keys),
            show_modal_cb: None,
            nick_name: "test".to_owned(),
            avatar_url:
                "https://c-ssl.duitang.com/uploads/blog/202208/01/20220801091938_56fad.jpeg"
                    .to_owned(),
        }
    }

    // pub fn load() -> Self {
    //     if let Ok(user) = LocalStorage::get::<User>(Self::USER_KEY) {
    //         user
    //     } else {
    //         Self::default()
    //     }
    // }

    // pub fn store(&self) {
    //     LocalStorage::set(Self::USER_KEY, self).expect("store user error ")
    // }
}

mod keys_serde {
    use nostr_sdk::nostr::{key::FromSkStr, Keys};
    use serde::{self, Deserialize, Deserializer, Serializer};
    pub fn serialize<S>(keys: &Option<Keys>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(keys) = keys {
            // only serialize the secret key, because the public key can generate from the secret key
            let sk = keys
                .secret_key()
                .map_err(serde::ser::Error::custom)?
                .display_secret()
                .to_string();
            serializer.serialize_str(sk.as_str())
        } else {
            serializer.serialize_str("")
        }
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Keys>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Ok(None);
        }
        let sk = Keys::from_sk_str(s.as_str()).map_err(serde::de::Error::custom)?;
        Ok(Some(sk))
    }
}

pub enum UserContextMessage {
    Renew,
    UserMsg(User),
    KeysMsg(Keys),
    Nickname(String),
    AvatarUrl(String),
    ModalCb(Callback<bool>),
}

impl Default for User {
    fn default() -> Self {
        Self {
            show_modal_cb: None,
            keys: None,
            nick_name: "test".to_owned(),
            avatar_url:
                "https://c-ssl.duitang.com/uploads/blog/202208/01/20220801091938_56fad.jpeg"
                    .to_owned(),
        }
    }
}

impl Reducible for User {
    type Action = UserContextMessage;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        use UserContextMessage::*;
        let ret = match action {
            Renew => User::new().into(),
            UserMsg(user) => Rc::new(user),
            KeysMsg(keys) => User {
                keys: Some(keys),
                nick_name: self.nick_name.clone(),
                avatar_url: self.avatar_url.clone(),
                show_modal_cb: self.show_modal_cb.clone(),
            }
            .into(),
            Nickname(nick_name) => User {
                nick_name,
                keys: self.keys.clone(),
                avatar_url: self.avatar_url.clone(),
                show_modal_cb: self.show_modal_cb.clone(),
            }
            .into(),
            AvatarUrl(avatar_url) => User {
                avatar_url,
                keys: self.keys.clone(),
                nick_name: self.nick_name.clone(),
                show_modal_cb: self.show_modal_cb.clone(),
            }
            .into(),
            ModalCb(cb) => User {
                avatar_url: self.avatar_url.clone(),
                keys: self.keys.clone(),
                nick_name: self.nick_name.clone(),
                show_modal_cb: Some(cb),
            }
            .into(),
        };
        ret.store();
        ret
    }
}

pub type UserContext = UseReducerHandle<User>;
