#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]
#![no_main]
#![no_std]
#![allow(unused_imports)]
#![allow(unused_parens)]
extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec::Vec};
use alloc::vec;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    api_error::ApiError,
    contracts::{EntryPoint, EntryPointAccess, EntryPointType, EntryPoints},
    CLType, CLValue, Key, URef, Parameter
};

use casper_types::{CLTyped};
use casper_types::contracts::NamedKeys;
use crate::alloc::string::ToString;

const JSON_CALL: &str = "json_input";
const JSON_GET_CALL: &str = "json_get_data";
// const KEY: &str = "json_data";
const JSON_KEY: &str = "json_key";
const CONTRACT_NAME_KEY: &str = "JSON_DEMO";


#[no_mangle]
pub extern "C" fn json_input() {
    let json: String = runtime::get_named_arg("json");
    // let json_local_key = storage::new_uref(json);
    // let mut json_named_key: BTreeMap<String, Key> = BTreeMap::new();
    // let key_name = String::from(JSON_KEY);
    // json_named_key.insert(key_name, json_local_key.into());
    let data_ref: URef = storage::new_uref(json);
    let data_key: Key = data_ref.into();
    runtime::put_key(JSON_KEY, data_key);
}

#[no_mangle]
pub extern "C" fn json_get_data() {
    let uref: URef = runtime::get_key(JSON_KEY)
        .unwrap_or_revert_with(ApiError::MissingKey)
        .into_uref()
        .unwrap_or_revert_with(ApiError::UnexpectedKeyVariant);
    let result: String = storage::read(uref)
        .unwrap_or_revert_with(ApiError::Read)
        .unwrap_or_revert_with(ApiError::ValueNotFound);
    let typed_result = CLValue::from_t(result).unwrap_or_revert();
    runtime::ret(typed_result);
}

#[no_mangle]
pub extern "C" fn call() {
    // let json_local_key = storage::new_uref(0);
    // let mut json_named_key: BTreeMap<String, Key> = BTreeMap::new();
    // let key_name = String::from(JSON_KEY);
    // json_named_key.insert(key_name, json_local_key.into());
    let mut json_entry_points = EntryPoints::new();
    json_entry_points.add_entry_point(EntryPoint::new(
        JSON_CALL,
        vec![Parameter::new("json", String::cl_type())],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    json_entry_points.add_entry_point(EntryPoint::new(
        JSON_GET_CALL,
        Vec::new(),
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let (stored_contract_hash, _) =
        storage::new_locked_contract(json_entry_points, None, None, None);
    runtime::put_key(CONTRACT_NAME_KEY, stored_contract_hash.into());

}