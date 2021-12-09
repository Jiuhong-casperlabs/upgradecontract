#![no_std]
#![no_main]

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::string::String;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ContractPackageHash, RuntimeArgs};

#[no_mangle]
pub extern "C" fn call() {
    let messenger_package_hash: ContractPackageHash = runtime::get_key("messenger_package_hash")
        .unwrap_or_revert()
        .into_hash()
        .unwrap_or_revert()
        .into();

    let message1: String = runtime::call_versioned_contract(
        messenger_package_hash,
        Some(1),
        "get_message",
        runtime_args! {},
    );

    let message2: String = runtime::call_versioned_contract(
        messenger_package_hash,
        None,
        "get_message",
        runtime_args! {},
    );

    runtime::put_key("version1value", storage::new_uref(message1).into());
    runtime::put_key("version2value", storage::new_uref(message2).into());
}
