use gloo_storage::LocalStorage;
use gloo_storage::Storage;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub trait LoadStoreKey: Sized + 'static {
    #[inline]
    fn load_store_key() -> &'static str;
}
pub trait LoadStore: Default + LoadStoreKey {
    fn load() -> Self
    where
        for<'de> Self: Deserialize<'de>;

    fn store(&self)
    where
        Self: Serialize;
}


impl<T> LoadStore for T where
    T: LoadStoreKey + Default
{
    fn load() -> Self
        where
                for<'de> Self: Deserialize<'de>,
    {
        log::info!("load:{}", Self::load_store_key());
        let ret = if let Ok(data) = LocalStorage::get::<Self>(Self::load_store_key()) {
            data
        } else {
            Self::default()
        };
        ret
    }

    fn store(&self)
        where
            Self: Serialize,
    {
        log::info!(
            "store:{}-{}",
            Self::load_store_key(),
            json!(self).to_string()
        );
        LocalStorage::set(Self::load_store_key(), self).expect("store error ")
    }
}
