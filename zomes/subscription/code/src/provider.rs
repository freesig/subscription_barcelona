use crate::Content;
use hdk::entry_definition::ValidatingEntryType;
use hdk::error::ZomeApiResult;
use hdk::holochain_core_types::entry::cap_entries::CapTokenClaim;
use hdk::holochain_core_types::{dna::entry_types::Sharing, entry::Entry};
use hdk::holochain_persistence_api::cas::content::Address;
use serde_json::json;

use hdk::holochain_json_api::json::JsonString;
//
use crate::Message;

use hdk::holochain_core_types::entry::cap_entries::{CapFunctions, CapabilityType};

use hdk::prelude::*;
use std::convert::TryInto;

pub(crate) fn add_content(content: Content) -> ZomeApiResult<Address> {
    let entry = Entry::App("content".into(), content.into());
    hdk::commit_entry(&entry)
}

pub(crate) fn content_def() -> ValidatingEntryType {
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

pub(crate) fn receive(from: Address, msg: String) -> String {
    let msg: Result<Message, _> = JsonString::from_json(&msg).try_into();
    match msg {
        Ok(Message::RequestSubscription) => {
            let mut functions = CapFunctions::new();
            functions.insert("subscription".into(), vec!["get_content".into()]);
            let grant_address_result = hdk::commit_capability_grant(
                "is_subscribed".to_string(),
                CapabilityType::Assigned,
                Some(vec![from]),
                functions,
            );
            json!(grant_address_result).to_string()
        }
        Ok(Message::RequestContent(claim)) => json!(try_get_content(from, claim)).to_string(),
        Err(err) => format!("message error {}", err),
    }
}

fn try_get_content(_agent_id: Address, claim: CapTokenClaim) -> ZomeApiResult<Vec<Content>> {
    let grant_address = claim.token();
    let content = hdk::call(
        hdk::THIS_INSTANCE,
        "subscription",
        grant_address.clone(),
        "get_content",
        json!({}).into(),
    )?
    .into();

    Ok(vec![Content { content: content }])
    /*
    let entries = hdk::query_result(
        EntryType::CapTokenGrant.into(),
        QueryArgsOptions {
            entries: true,
            ..Default::default()
        },
    )?;
    match entries {
        QueryResult::Entries(entries) => {
            let token = entries.iter().filter(|(addr, _)| &grant_address == addr).next();
            if let Some((_, Entry::CapTokenGrant(token))) = token {
                let assignees = token.assignees();
                if let Some(assignees) = assignees {
                    if assignees.contains(&agent_id) {
                        //  now call the zome and function in the CapFunctions
                        let zomes = token.functions();
                        let mut sss = String::new();
                        for (zome, functions) in &zomes {
                            sss = format!("{} {:?}: \"{:?}\"", sss, zome, functions);
                            for function in functions {
                                sss = hdk::call(hdk::THIS_INSTANCE, zome, claim_address.clone(), function, "".into())?.into();
                            }
                        }
                        Ok(vec![Content{ content: sss }])
                    } else {
                        Err("Agents is not an assignee".to_string().into())
                    }
                } else {
                    Err("Agents is not an assignee".to_string().into())
                }
            } else {
                Err("No capability token found".to_string().into())
            }
        }
        _ => Err("No entries".to_string().into()),
    }
    */
}
