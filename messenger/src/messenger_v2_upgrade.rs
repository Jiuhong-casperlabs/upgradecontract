#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{string::ToString, vec};

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
    runtime::ret(CLValue::from_t("second".to_string()).unwrap_or_revert());
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

    // Get the package hash of the originally deployed contract.
    let messenger_package_hash = runtime::get_key("mymessenger_package_hash")
        .unwrap_or_revert()
        .into_hash()
        .unwrap()
        .into();
    // Overwrite the original contract with the new entry points. This works
    // because the original code stored the required access token into the accounts storage.
    let _ = storage::add_contract_version(messenger_package_hash, entry_points, Default::default());
}
