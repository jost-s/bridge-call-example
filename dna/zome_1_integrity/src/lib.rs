use hdi::prelude::*;

#[hdk_entry_helper]
pub struct Type1 {
    pub name: String,
}

#[hdk_entry_defs]
#[unit_enum(EntryTypesUnit)]
pub enum EntryTypes {
    Type1(Type1),
}
