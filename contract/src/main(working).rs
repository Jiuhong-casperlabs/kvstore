#![no_std]
#![no_main]

extern crate alloc;
use core::convert::TryInto;

use alloc::boxed::Box;
use alloc::{collections::BTreeSet, string::String, vec::Vec};

use alloc::vec;
use casper_contract::contract_api::{runtime, storage};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{
    CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter,
};

#[no_mangle]
pub extern "C" fn store_list_keys() {
    let name: String = runtime::get_named_arg("name");
    let value: Vec<Key> = runtime::get_named_arg("value");
    // let key = storage::new_uref(value).into();
    // runtime::put_key(name.as_str(), key);

    match runtime::get_key(name.as_str()) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name.as_str(), key);
        }
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _) = storage::create_contract_package_at_hash();
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        "store_list_keys",
        vec![
            Parameter::new("name", CLType::String),
            Parameter::new("value", CLType::List(Box::new(Key::cl_type()))),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, Default::default());
    runtime::put_key("mykv", contract_hash.into());
}
