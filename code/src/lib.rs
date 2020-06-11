#![feature(proc_macro_hygiene)]

use hdk::prelude::*;
use hdk_proc_macros::zome;

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
        Ok(())
    }

    #[entry_def]
    fn message_def() -> ValidatingEntryType {
        messages::message_definition()

    }
    #[entry_def]
    fn message_anchor_def() -> ValidatingEntryType {
        messages::message_anchor_definition()
    }

    #[zome_fn("hc_public")]
    fn hello() -> ZomeApiResult<String> {
        Ok("hello!".to_owned())
    }
}
