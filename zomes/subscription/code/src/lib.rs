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
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
};

use hdk::holochain_json_api::{
    json::JsonString,
    error::JsonError
};

use hdk::holochain_persistence_api::{
    cas::content::Address
};

use hdk_proc_macros::zome;

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
                            Err("Conent too long")
                        } else {
                            Ok(())
                        }
                    },
                    _ => Ok(()),
                }
            }
        )
    }

    #[hc_public]
    pub fn get_content(agent_id: Address, claim: Address) -> ZomeApiResult<Vec<Content>> {
        Err("Not subscribed".into())

    }

    #[hc_public]
    pub fn add_content(content: Content) -> ZomeApiResult<Address> {
        let entry = Entry::App("content".into(), content.into());
        hdk::commit_entry(&entry)
    }

    #[hc_public]
    pub fn request_subscription(agent_id: Address) -> ZomeApiResult<Address> {
        let claim_address = hdk::send(agent_id, Message.into(), 100000.into())?;
    }

    #[receive]
    pub fn receive(from: Address, msg: JsonString) -> String {
        let msg: Result<Message, _> = msg.try_into();
        match msg {
            Ok(Message::RequestSubscription) => {

            },
            Err(err) => Err(format!("message error {}", err)),
            _ => "Error passing message".into(),
        }
    }
}
