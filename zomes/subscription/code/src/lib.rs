#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    error::ZomeApiResult,
};

use hdk::entry_definition::ValidatingEntryType;

use hdk::holochain_json_api::{
    json::JsonString,
    error::JsonError
};

use hdk::holochain_persistence_api::{
    cas::content::Address
};

use hdk_proc_macros::zome;


mod subscriber;
mod provider;

// see https://developer.holochain.org/api/0.0.38-alpha14/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct Content {
    content: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub enum Message {
    RequestSubscription,
}

#[zome]
mod subscription {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn content_def() -> ValidatingEntryType {
        provider::content_def()
    }

    #[zome_fn("hc_public")]
    pub fn add_content(content: Content) -> ZomeApiResult<Address> {
        provider::add_content(content)
    }

    #[zome_fn("hc_public")]
    pub fn request_subscription(agent_id: Address) -> ZomeApiResult<Address> {
        subscriber::request_subscription(agent_id)
    }

    #[receive]
    pub fn receive(from: Address, msg: JsonString) -> String {
        provider::receive(from, msg)
    }
}
