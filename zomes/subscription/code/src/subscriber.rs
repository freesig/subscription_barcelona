use hdk::holochain_json_api::{
    json::JsonString,
};

use serde_json::json;
use crate::Message;

use hdk::holochain_persistence_api::{
    cas::content::Address
};

use hdk::{
    error::ZomeApiResult,
};

use std::convert::TryInto;

pub fn request_subscription(agent_id: Address) -> ZomeApiResult<Address> {
    let r = hdk::send(
        agent_id,
        json!(Message::RequestSubscription).to_string(),
        100000.into(),
    )?;
    let r = JsonString::from_json(&r).try_into()?;
    r
}
