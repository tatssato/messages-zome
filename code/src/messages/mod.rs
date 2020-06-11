#![allow(dead_code)]

use hdk::{
    api::AGENT_ADDRESS,
    prelude::*,
};

pub mod handlers;
pub mod strings;
use holochain_entry_utils::HolochainEntry;
use strings::*;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MessageAnchor {
    author: Address,
    timestamp: u64,
    receiver: Address,
    message_type: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Message {
    anchor: Address,
    payload: String,
}

impl HolochainEntry for MessageAnchor {
    fn entry_type() -> String {
        MESSAGE_ANCHOR_ENTRY_NAME.to_owned()
    }
}

impl MessageAnchor {
    fn new(timestamp: u64, receiver: Address, message_type: String) -> Self {
        MessageAnchor {
            author: AGENT_ADDRESS.to_string().into(),
            timestamp,
            receiver,
            message_type,
        }
    }
}

impl HolochainEntry for Message {
    fn entry_type() -> String {
        MESSAGE_ENTRY_NAME.to_owned()
    }
}

impl Message {
    fn new(anchor: Address, payload: String) -> Self {
        Message {
            anchor,
            payload,
        }
    }
}

pub fn message_definition() -> ValidatingEntryType {
    entry!(
        name: Message::entry_type(),
        description: "this is the message entry defintion",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Message>| {
            Ok(())
        },
        links: [
            from!(
                MessageAnchor::entry_type(),
                link_type: MESSAGE_ANCHOR_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )
        ]
    )
}

pub fn message_anchor_definition() -> ValidatingEntryType {
    entry!(
        name: MessageAnchor::entry_type(),
        description: "this is the message anchor entry defintion",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<MessageAnchor>| {
            Ok(())
        },
        links: [
            from!(
                EntryType::AgentId.to_string(),
                link_type: AGENT_MESSAGE_ANCHOR_LINK_TYPE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )
        ]
    )
}


