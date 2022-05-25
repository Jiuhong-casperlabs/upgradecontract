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
    CLType, CLValue, ContractHash,ContractPackageHash
};


#[no_mangle]
pub extern "C" fn call() {
    let package_hash_str:String = runtime::get_named_arg("package_hash_str");
    let package_hash = ContractPackageHash::from_formatted_str(&package_hash_str).unwrap();
    let contract_hash_str:String = runtime::get_named_arg("contract_hash_str");
    let contract_hash = ContractHash::from_formatted_str(&contract_hash_str).unwrap();
    
    storage::disable_contract_version(package_hash,contract_hash);

}
