use hdk::holochain_json_api::{
    json::JsonString,
};
// use hdk::holochain_core_types::entry::cap_entries::CapTokenClaim;

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
use hdk::prelude::*;

pub(crate) fn request_subscription(provider_agent_id: Address) -> ZomeApiResult<Address> {
    let msg = json!(Message::RequestSubscription);
    let msg: String = msg.to_string();
    let r = hdk::send(
        provider_agent_id.clone(),
        msg,
        100000.into(),
    )?;

    let grant_address: Result<Address, _> = JsonString::from_json(&r).try_into()?;
    grant_address.and_then(|grant_address|{
        hdk::commit_capability_claim("is_subscribed", provider_agent_id, grant_address) //returns claim address
    })
}

pub(crate) fn request_content(provider_agent_id: Address, claim_address: Address) -> ZomeApiResult<Vec<Content>> {
    let token_result = hdk::get_entry(&claim_address);
    let token = match token_result {
        Ok(Some(Entry::CapTokenClaim(claim))) => claim,
        _ => panic!("this is busted"),
    };
    let r = hdk::send(
        provider_agent_id.clone(),
        json!(Message::RequestContent(token)).to_string(),
        100000.into(),
    )?;
    let content = JsonString::from_json(&r).try_into()?;
    content
}
