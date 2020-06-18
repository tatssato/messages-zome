use hdk::{
    prelude::*,
    api::AGENT_ADDRESS,
};
use super::{
    Message,
    MessageAnchor,
    strings::*,
    HolochainEntry,
};

pub fn send(author: Address, recipient: Address, message: String, timestamp: u64) -> ZomeApiResult<String> {
    // create anchor based on timestamp
    let message_anchor = MessageAnchor::new(
        recipient.clone(),
        timestamp,
        "text".into()
    );
    let message_anchor_entry = message_anchor.entry();
    let message_anchor_entry_address = message_anchor_entry.address();

    // create message
    let message_struct = Message::new(author.clone(), message.clone());
    let message_entry = message_struct.entry();
    let message_entry_address = message_entry.address();

    // commit entry
    hdk::commit_entry(&message_anchor_entry)?;
    hdk::commit_entry(&message_entry)?;

    // link message to agent_id (author)
    hdk::link_entries(
        &AGENT_ADDRESS,
        &message_entry_address,
        AUTHOR_MESSAGE_ANCHOR_LINK_TYPE,
        ""
    )?;

    hdk::link_entries(
        &message_anchor_entry_address, 
        &message_entry_address,
        MESSAGE_ANCHOR_LINK_TYPE,
        ""
    )?;

    // send to node
    hdk::send(recipient, message, 60000.into())
}

pub fn receive(from: Address, payload: String) -> String {
    format!("Received: {} from {}", payload, from)
}