use foxtive::FOXTIVE;
use foxtive::prelude::AppStateExt;
use foxtive::results::AppResult;
use serde::Serialize;
use crate::contracts::event_contract::EventContract;

pub struct Event;

impl Event {
    pub async fn emit<N, D>(name: N, data: &D) -> AppResult<()>
    where
        N: EventContract,
        D: Serialize,
    {
        Self::publish(name.rmq_exchange(), name.event_name(), data).await
    }

    async fn publish<E, P>(ex: &str, event: E, payload: &P) -> AppResult<()>
    where
        E: ToString,
        P: serde::Serialize,
    {
        let data = serde_json::to_string(payload)?;
        FOXTIVE
            .rabbitmq()
            .lock_owned()
            .await
            .publish(ex, event, data.as_bytes())
            .await
    }
}