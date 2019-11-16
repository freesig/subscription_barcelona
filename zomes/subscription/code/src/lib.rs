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

use serde_json::json;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
    entry::cap_entries::{CapFunctions, CapabilityType},
};

use hdk::holochain_json_api::{
    json::JsonString,
    error::JsonError
};

use hdk::holochain_persistence_api::{
    cas::content::Address
};

use hdk_proc_macros::zome;

use std::convert::TryInto;

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
        entry!(
            name: "content",
            description: "this is some content",
            sharing: Sharing::Private,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | validation_data: hdk::EntryValidationData<Content>| {
                match validation_data {
                    hdk::EntryValidationData::Create{ entry, .. } => {
                        if entry.content.len() > 200 {
                            Err("Conent too long".into())
                        } else {
                            Ok(())
                        }
                    },
                    _ => Ok(()),
                }
            }
        )
    }

    #[zome_fn("hc_public")]
    pub fn add_content(content: Content) -> ZomeApiResult<Address> {
        let entry = Entry::App("content".into(), content.into());
        hdk::commit_entry(&entry)
    }

    #[zome_fn("hc_public")]
    pub fn request_subscription(agent_id: Address) -> ZomeApiResult<Address> {
        let r = hdk::send(agent_id, json!(Message::RequestSubscription).to_string(), 100000.into())?;
        let r = JsonString::from_json(&r);
        let r = r.try_into()?;
        r
    }

    #[receive]
    pub fn receive(from: Address, msg: JsonString) -> String {
        let msg: Result<Message, _> = JsonString::from_json(&msg).try_into();
        match msg {
            Ok(Message::RequestSubscription) => {
                let mut functions = CapFunctions::new();
                functions.insert("subscription".into(), vec!["get_content".into()]);
                let r = hdk::commit_capability_grant(
                    "is_subscribed".to_string(),
                    CapabilityType::Assigned,
                    Some(vec![from]),
                    functions);
                json!(r).to_string()
            },
            Err(err) => format!("message error {}", err),
        }
    }
}
