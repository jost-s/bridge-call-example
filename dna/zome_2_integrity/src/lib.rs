use hdi::prelude::*;
use zome_2_types::Type2;

#[hdk_entry_defs]
#[unit_enum(EntryTypesUnit)]
pub enum EntryTypes {
    Type2(Type2),
}
