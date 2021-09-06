#![cfg_attr(
    not(target_arch = "wasm32"),
    crate_type = "target arch should be wasm32"
)]  
#![no_main]

extern crate alloc;

use casper_types::{
    bytesrepr::ToBytes, runtime_args::RuntimeArgs, runtime_args,  ApiError, ContractHash, Key,
};
use casper_types::{URef};
use casper_contract::{contract_api::{runtime, storage}, unwrap_or_revert::UnwrapOrRevert};

const JSON_CALL: &str = "json_input";
const JSON_GET_CALL: &str = "json_get_data";
const CONTRACT_NAME_KEY: &str = "JSON_DEMO";


#[no_mangle]
pub extern "C" fn call() {
    let contract_hash = {
        let json_uref = runtime::get_key(CONTRACT_NAME_KEY).unwrap_or_revert_with(ApiError::GetKey);
        if let Key::Hash(hash) = json_uref {
            ContractHash::new(hash)
        } else {
            runtime::revert(ApiError::User(66));
        }
    };
    let data: String = runtime::get_named_arg("data");
    let args = runtime_args! {
        "json" => data,
    };
    let _: () = runtime::call_contract(contract_hash, JSON_CALL, args);

    let json_value_store: String = 
        runtime::call_contract(contract_hash, JSON_GET_CALL, RuntimeArgs::new());

    let data_ref: URef = storage::new_uref(json_value_store);
    let data_key: Key = data_ref.into();
    runtime::put_key("Retrieved Data", data_key);
}