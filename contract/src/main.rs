#![no_std]
#![no_main]

extern crate alloc;
use core::convert::TryInto;

use alloc::boxed::Box;
use alloc::{collections::BTreeSet, string::String, vec::Vec};

use alloc::vec;
use casper_contract::contract_api::{runtime, storage};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::bytesrepr::{FromBytes, ToBytes};
use casper_types::{
    CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter,
};

#[no_mangle]
pub extern "C" fn store_list_keys() {
    read_and_store::<(String, Vec<Key>)>();
}

#[no_mangle]
pub extern "C" fn call() {
    let (contract_package_hash, _) = storage::create_contract_package_at_hash();
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(endpoint(
        "store_list_keys",
        CLType::List(Box::new(Key::cl_type())),
    ));

    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, Default::default());
    runtime::put_key("mykv", contract_hash.into());
}

fn endpoint(name: &str, value_type: CLType) -> EntryPoint {
    EntryPoint::new(
        String::from(name),
        vec![
            Parameter::new("name", CLType::String),
            Parameter::new("value", value_type),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn read_and_store<T: CLTyped + FromBytes + ToBytes>() {
    let name: String = runtime::get_named_arg("name");
    let value: T = runtime::get_named_arg("value");
    set_key(name.as_str(), value);
}

fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}
