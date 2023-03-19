use hdk::prelude::*;
use zome_1_integrity::{EntryTypes, Type1};
use zome_2_integrity::Type2;

#[hdk_extern]
fn create_an_entry(_: ()) -> ExternResult<ActionHash> {
    let action_hash = create_entry(EntryTypes::Type1(Type1 {
        name: "name".to_string(),
    }))?;
    Ok(action_hash)
}

#[hdk_extern]
fn bridge_call(_: ()) -> ExternResult<()> {
    call(
        CallTargetCell::Local,
        "zome_2_coordinator",
        "test_call".into(),
        None,
        Type2 {
            name: "hello".to_string(),
        },
    )?;
    Ok(())
}
