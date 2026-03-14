use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait EventContract: Serialize + DeserializeOwned {
    fn event_name(&self) -> String;

    fn rmq_exchange(&self) -> &String;
}
