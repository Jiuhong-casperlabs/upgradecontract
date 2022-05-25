#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec,
};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    CLType, CLValue,
};

#[no_mangle]
pub extern "C" fn get_message() {
    runtime::ret(CLValue::from_t(String::from("first")).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "get_message",
        vec![],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let mut named_keys = BTreeMap::new();
    named_keys.insert("name_v1".to_string(), storage::new_uref("value_v1").into());

    // Introduce the contract itself to the account, and save it's package hash and access token
    // to the account's storage as "messenger_package_hash" and "messenger_access_token" respectively.
    let _ = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some("mymessenger_package_hash".to_string()),
        Some("aaa".to_string()),
    );
}
