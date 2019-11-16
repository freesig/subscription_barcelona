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
use crate::Content;

pub(crate) fn request_subscription(agent_id: Address) -> ZomeApiResult<Address> {
    let r = hdk::send(
        agent_id.clone(),
        json!(Message::RequestSubscription).to_string(),
        100000.into(),
    )?;
    let claim_address = JsonString::from_json(&r).try_into()?;
    hdk::commit_capability_claim("is_subscribed", agent_id, claim_address)
}

pub(crate) fn request_content(agent_id: Address, claim_address: Address) -> ZomeApiResult<Vec<Content>> {
    let r = hdk::send(
        agent_id.clone(),
        json!(Message::RequestContent(claim_address)).to_string(),
        100000.into(),
    )?;
    let content = JsonString::from_json(&r).try_into()?;
    content
}
