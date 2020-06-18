#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]

use hdk::{
    prelude::*,
    api::PROPERTIES
};
use hdk_proc_macros::zome;
use serde_json::{
    json,
    from_str
};
pub mod messages;

// see https://developer.holochain.org/api/0.0.47-alpha1/hdk/ for info on using the hdk library

#[zome]
mod messages_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        // Ok(())

        // let dna_properties: messages::Members = match from_str(&PROPERTIES.to_string()) {
        //     Ok(result) => result,
        //     _ => messages::Members::blank()
        // };

        // hdk::debug(format!("{}", dna_properties.recipient));

        if let EntryValidationData::Create{entry, ..} = validation_data {
            let agent = entry as AgentId;
            if agent.nick == "reject_agent::app" {
                Err("This agent will always be rejected".into())
            } else {
                Ok(())
            }
        } else {
            Err("Cannot update or delete an agent at this time".into())
        }

    }

    #[entry_def]
    fn message_def() -> ValidatingEntryType {
        messages::message_definition()
    }

    #[entry_def]
    fn message_anchor_def() -> ValidatingEntryType {
        messages::message_anchor_definition()
    }

    #[receive]
    fn receive(from: Address, payload: String) -> String {
        // format!("Received: {} from {}", payload, from)
        messages::handlers::receive(from, payload)
    }

    #[zome_fn("hc_public")]
    fn hello() -> ZomeApiResult<String> {
        Ok("hello!".to_owned())
    }

    #[zome_fn("hc_public")]
    fn send(author: Address, recipient: Address, message: String) -> ZomeApiResult<String> {
        messages::handlers::send(author, recipient, message, 6000)
    }
}
