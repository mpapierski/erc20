#[cfg_attr(target_arch = "wasm32", no_std)]
#[cfg_attr(target_arch = "wasm32", no_main)]
extern crate alloc;

use alloc::string::String;

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_erc20::{
    constants::{
        ADDRESS_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME, DECIMALS_RUNTIME_ARG_NAME,
        NAME_RUNTIME_ARG_NAME, OWNER_RUNTIME_ARG_NAME, RECIPIENT_RUNTIME_ARG_NAME,
        SPENDER_RUNTIME_ARG_NAME, SYMBOL_RUNTIME_ARG_NAME, TOTAL_SUPPLY_RUNTIME_ARG_NAME,
    },
    Address, ERC20,
};
use casper_types::{CLValue, U256};

#[no_mangle]
pub extern "C" fn name() {
    let name = ERC20::default().name();
    runtime::ret(CLValue::from_t(name).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn symbol() {
    let symbol = ERC20::default().symbol();
    runtime::ret(CLValue::from_t(symbol).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn decimals() {
    let decimals = ERC20::default().decimals();
    runtime::ret(CLValue::from_t(decimals).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let total_supply = ERC20::default().total_supply();
    runtime::ret(CLValue::from_t(total_supply).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let address: Address = runtime::get_named_arg(ADDRESS_RUNTIME_ARG_NAME);
    let balance = ERC20::default().balance_of(address);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn transfer() {
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default()
        .transfer(recipient, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn approve() {
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default().approve(spender, amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn allowance() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let val = ERC20::default().allowance(owner, spender);
    runtime::ret(CLValue::from_t(val).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    ERC20::default()
        .transfer_from(owner, recipient, amount)
        .unwrap_or_revert();
}

#[no_mangle]
fn call() {
    let name: String = runtime::get_named_arg(NAME_RUNTIME_ARG_NAME);
    let symbol: String = runtime::get_named_arg(SYMBOL_RUNTIME_ARG_NAME);
    let decimals = runtime::get_named_arg(DECIMALS_RUNTIME_ARG_NAME);
    let total_supply = runtime::get_named_arg(TOTAL_SUPPLY_RUNTIME_ARG_NAME);

    let _token = ERC20::install(name, symbol, decimals, total_supply).unwrap_or_revert();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
mod standalone_support {

    #[allow(unused_variables)]
    pub mod native_ext_ffi {
        use std::{
            collections::BTreeMap,
            ptr,
            sync::{Arc, Mutex},
        };

        use rand::prelude::*;

        use casper_types::{
            api_error,
            bytesrepr::{self, ToBytes},
            contracts::NamedKeys,
            AccessRights, ApiError, CLValue, Key, RuntimeArgs, URef,
        };

        pub static mut RUNTIME_ARGS: Option<RuntimeArgs> = None;
        pub static mut NAMED_KEYS: Option<NamedKeys> = None;
        pub static mut HOST_BUFFER: Option<Vec<u8>> = None;
        pub static mut STORAGE: Option<BTreeMap<Key, CLValue>> = None;

        #[no_mangle]
        pub extern "C" fn casper_read_value(
            key_ptr: *const u8,
            key_size: usize,
            output_size: *mut usize,
        ) -> i32 {
            todo!("casper_read_value")
        }
        #[no_mangle]
        pub extern "C" fn casper_write(
            key_ptr: *const u8,
            key_size: usize,
            value_ptr: *const u8,
            value_size: usize,
        ) {
            todo!("casper_write")
        }
        #[no_mangle]
        pub extern "C" fn casper_add(
            key_ptr: *const u8,
            key_size: usize,
            value_ptr: *const u8,
            value_size: usize,
        ) {
            todo!("casper_add")
        }
        #[no_mangle]
        pub extern "C" fn casper_new_uref(
            uref_ptr: *mut u8,
            value_ptr: *const u8,
            value_size: usize,
        ) {
            let uref_bytes: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(uref_ptr, 33) };
            let value_bytes: &[u8] = unsafe { std::slice::from_raw_parts(value_ptr, value_size) };
            let value: CLValue =
                bytesrepr::deserialize(value_bytes.to_vec()).expect("should have cl value");
            let new_uref = runtime_new_uref(value).to_bytes().unwrap();
            uref_bytes.copy_from_slice(&new_uref[..])
        }
        #[no_mangle]
        pub extern "C" fn casper_load_named_keys(
            total_keys: *mut usize,
            result_size: *mut usize,
        ) -> i32 {
            todo!("casper_load_named_keys")
        }
        #[no_mangle]
        pub extern "C" fn casper_ret(value_ptr: *const u8, value_size: usize) -> ! {
            todo!("casper_ret")
        }
        #[no_mangle]
        pub extern "C" fn casper_get_key(
            name_ptr: *const u8,
            name_size: usize,
            output_ptr: *mut u8,
            output_size: usize,
            bytes_written_ptr: *mut usize,
        ) -> i32 {
            todo!("casper_get_key")
        }
        #[no_mangle]
        pub extern "C" fn casper_has_key(name_ptr: *const u8, name_size: usize) -> i32 {
            let name_bytes = unsafe { std::slice::from_raw_parts(name_ptr, name_size) };
            let name: String =
                bytesrepr::deserialize(name_bytes.to_vec()).expect("should deserialize");
            let named_keys = unsafe { NAMED_KEYS.get_or_insert_with(|| NamedKeys::default()) };
            let result = match named_keys.get(&name) {
                Some(_) => Ok(()),
                None => Err(ApiError::MissingKey),
            };
            api_error::i32_from(result)
        }
        #[no_mangle]
        pub extern "C" fn casper_put_key(
            name_ptr: *const u8,
            name_size: usize,
            key_ptr: *const u8,
            key_size: usize,
        ) {
            let named_keys = unsafe { NAMED_KEYS.get_or_insert_with(|| NamedKeys::default()) };
            let name_bytes: &[u8] = unsafe { std::slice::from_raw_parts(name_ptr, name_size) };
            let key_bytes: &[u8] = unsafe { std::slice::from_raw_parts(key_ptr, key_size) };
            let name: String =
                bytesrepr::deserialize(name_bytes.to_vec()).expect("should deserialize name");
            let key: Key =
                bytesrepr::deserialize(key_bytes.to_vec()).expect("should deserialize key");
            named_keys.insert(name, key);
        }
        #[no_mangle]
        pub extern "C" fn casper_remove_key(name_ptr: *const u8, name_size: usize) {
            todo!("casper_remove_key")
        }
        #[no_mangle]
        pub extern "C" fn casper_revert(status: i32) -> ! {
            let api_error = api_error::result_from(status).unwrap_err();
            panic!("casper_revert({:?}", api_error);
        }
        #[no_mangle]
        pub extern "C" fn casper_is_valid_uref(uref_ptr: *const u8, uref_size: usize) -> i32 {
            todo!("casper_is_valid_uref")
        }
        #[no_mangle]
        pub extern "C" fn casper_add_associated_key(
            account_hash_ptr: *const u8,
            account_hash_size: usize,
            weight: i32,
        ) -> i32 {
            todo!("casper_add_associated_key")
        }
        #[no_mangle]
        pub extern "C" fn casper_remove_associated_key(
            account_hash_ptr: *const u8,
            account_hash_size: usize,
        ) -> i32 {
            todo!("casper_remove_associated_key")
        }
        #[no_mangle]
        pub extern "C" fn casper_update_associated_key(
            account_hash_ptr: *const u8,
            account_hash_size: usize,
            weight: i32,
        ) -> i32 {
            todo!("casper_update_associated_key")
        }
        #[no_mangle]
        pub extern "C" fn casper_set_action_threshold(
            permission_level: u32,
            threshold: u32,
        ) -> i32 {
            todo!("casper_set_action_threshold")
        }
        #[no_mangle]
        pub extern "C" fn casper_get_caller(output_size: *mut usize) -> i32 {
            todo!("casper_get_caller")
        }
        #[no_mangle]
        pub extern "C" fn casper_get_blocktime(dest_ptr: *const u8) {
            todo!("casper_get_blocktime")
        }
        #[no_mangle]
        pub extern "C" fn casper_create_purse(purse_ptr: *const u8, purse_size: usize) -> i32 {
            todo!("casper_create_purse")
        }
        #[no_mangle]
        pub extern "C" fn casper_transfer_to_account(
            target_ptr: *const u8,
            target_size: usize,
            amount_ptr: *const u8,
            amount_size: usize,
            id_ptr: *const u8,
            id_size: usize,
            result_ptr: *const i32,
        ) -> i32 {
            todo!("casper_transfer_to_account")
        }
        #[no_mangle]
        pub extern "C" fn casper_transfer_from_purse_to_account(
            source_ptr: *const u8,
            source_size: usize,
            target_ptr: *const u8,
            target_size: usize,
            amount_ptr: *const u8,
            amount_size: usize,
            id_ptr: *const u8,
            id_size: usize,
            result_ptr: *const i32,
        ) -> i32 {
            todo!("casper_transfer_from_purse_to_account")
        }
        #[no_mangle]
        pub extern "C" fn casper_transfer_from_purse_to_purse(
            source_ptr: *const u8,
            source_size: usize,
            target_ptr: *const u8,
            target_size: usize,
            amount_ptr: *const u8,
            amount_size: usize,
            id_ptr: *const u8,
            id_size: usize,
        ) -> i32 {
            todo!("casper_transfer_from_purse_to_purse")
        }
        #[no_mangle]
        pub extern "C" fn casper_record_transfer(
            maybe_to_ptr: *const u8,
            maybe_to_size: usize,
            source_ptr: *const u8,
            source_size: usize,
            target_ptr: *const u8,
            target_size: usize,
            amount_ptr: *const u8,
            amount_size: usize,
            id_ptr: *const u8,
            id_size: usize,
        ) -> i32 {
            todo!("casper_record_transfer")
        }
        #[no_mangle]
        pub extern "C" fn casper_record_era_info(
            era_id_ptr: *const u8,
            era_id_size: usize,
            era_info_ptr: *const u8,
            era_info_size: usize,
        ) -> i32 {
            todo!("casper_record_era_info")
        }
        #[no_mangle]
        pub extern "C" fn casper_get_balance(
            purse_ptr: *const u8,
            purse_size: usize,
            result_size: *mut usize,
        ) -> i32 {
            todo!("casper_get_balance")
        }
        #[no_mangle]
        pub extern "C" fn casper_get_phase(dest_ptr: *mut u8) {
            todo!("casper_get_phase")
        }
        #[no_mangle]
        pub extern "C" fn casper_get_system_contract(
            system_contract_index: u32,
            dest_ptr: *mut u8,
            dest_size: usize,
        ) -> i32 {
            todo!("casper_get_system_contract")
        }
        #[no_mangle]
        pub extern "C" fn casper_get_main_purse(dest_ptr: *mut u8) {
            todo!("casper_get_main_purse")
        }
        #[no_mangle]
        pub extern "C" fn casper_read_host_buffer(
            dest_ptr: *mut u8,
            dest_size: usize,
            bytes_written: *mut usize,
        ) -> i32 {
            let current_host_buffer = unsafe { HOST_BUFFER.take() };
            let dest: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(dest_ptr, dest_size) };
            if let Some(host_buffer) = current_host_buffer {
                dest.copy_from_slice(&host_buffer[..]);
                let ptr = ptr::NonNull::new(bytes_written).expect("should have non null ptr");
                unsafe { ptr::write(ptr.as_ptr(), host_buffer.len()) }
            }
            0
        }
        #[no_mangle]
        pub extern "C" fn casper_create_contract_package_at_hash(
            hash_addr_ptr: *mut u8,
            access_addr_ptr: *mut u8,
            is_locked: bool,
        ) {
            todo!("casper_create_contract_package_at_hash")
        }
        #[no_mangle]
        pub extern "C" fn casper_create_contract_user_group(
            contract_package_hash_ptr: *const u8,
            contract_package_hash_size: usize,
            label_ptr: *const u8,
            label_size: usize,
            num_new_urefs: u8,
            existing_urefs_ptr: *const u8,
            existing_urefs_size: usize,
            output_size_ptr: *mut usize,
        ) -> i32 {
            todo!("casper_create_contract_user_group")
        }
        #[no_mangle]
        pub extern "C" fn casper_add_contract_version(
            contract_package_hash_ptr: *const u8,
            contract_package_hash_size: usize,
            version_ptr: *const u32,
            entry_points_ptr: *const u8,
            entry_points_size: usize,
            named_keys_ptr: *const u8,
            named_keys_size: usize,
            output_ptr: *mut u8,
            output_size: usize,
            bytes_written_ptr: *mut usize,
        ) -> i32 {
            todo!("casper_add_contract_version")
        }
        #[no_mangle]
        pub extern "C" fn casper_disable_contract_version(
            contract_package_hash_ptr: *const u8,
            contract_package_hash_size: usize,
            contract_hash_ptr: *const u8,
            contract_hash_size: usize,
        ) -> i32 {
            todo!("casper_disable_contract_version")
        }
        #[no_mangle]
        pub extern "C" fn casper_call_contract(
            contract_hash_ptr: *const u8,
            contract_hash_size: usize,
            entry_point_name_ptr: *const u8,
            entry_point_name_size: usize,
            runtime_args_ptr: *const u8,
            runtime_args_size: usize,
            result_size: *mut usize,
        ) -> i32 {
            todo!("casper_call_contract")
        }
        #[no_mangle]
        pub extern "C" fn casper_call_versioned_contract(
            contract_package_hash_ptr: *const u8,
            contract_package_hash_size: usize,
            contract_version_ptr: *const u8,
            contract_version_size: usize,
            entry_point_name_ptr: *const u8,
            entry_point_name_size: usize,
            runtime_args_ptr: *const u8,
            runtime_args_size: usize,
            result_size: *mut usize,
        ) -> i32 {
            todo!("casper_call_versioned_contract")
        }
        #[no_mangle]
        pub extern "C" fn casper_get_named_arg_size(
            name_ptr: *const u8,
            name_size: usize,
            dest_size: *mut usize,
        ) -> i32 {
            let name: &str = {
                let name_slice: &[u8] = unsafe { std::slice::from_raw_parts(name_ptr, name_size) };
                std::str::from_utf8(name_slice).expect("should have valid utf8 string in name")
            };
            let result = match unsafe { RUNTIME_ARGS.as_ref() } {
                Some(args) => {
                    match args.get(name) {
                        Some(value) => {
                            let inner_bytes_length: usize = {
                                let bytes = value.inner_bytes();
                                if bytes.len() > u32::max_value() as usize {
                                    panic!("Out of memory"); // Err(ApiError::OutOfMemory)
                                } else {
                                    bytes.len()
                                }
                            };

                            let bytes_le = inner_bytes_length.to_le_bytes();

                            let mut ptr = ptr::NonNull::new(dest_size)
                                .expect("should have non null dest_size ptr");
                            unsafe { ptr::write(ptr.as_ptr(), inner_bytes_length) }
                            // let dest: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(dest_size, bytes_le.len()) };
                            // dest.copy_from_slice(&bytes_le[..]);
                            Ok(())
                        }
                        None => Err(ApiError::MissingArgument),
                    }
                }
                None => todo!(),
            };

            api_error::i32_from(result)
        }
        #[no_mangle]
        pub extern "C" fn casper_get_named_arg(
            name_ptr: *const u8,
            name_size: usize,
            dest_ptr: *mut u8,
            dest_size: usize,
        ) -> i32 {
            let name: &str = {
                let name_slice: &[u8] = unsafe { std::slice::from_raw_parts(name_ptr, name_size) };
                std::str::from_utf8(name_slice).expect("should have valid utf8 string in name")
            };

            let dest: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(dest_ptr, dest_size) };

            let result = match unsafe { RUNTIME_ARGS.as_ref() } {
                Some(args) => match args.get(name) {
                    Some(value) => {
                        dest.copy_from_slice(value.inner_bytes().as_slice());
                        Ok(())
                    }
                    None => Err(ApiError::MissingArgument),
                },
                None => todo!(),
            };

            api_error::i32_from(result)
        }
        #[no_mangle]
        pub extern "C" fn casper_remove_contract_user_group(
            contract_package_hash_ptr: *const u8,
            contract_package_hash_size: usize,
            label_ptr: *const u8,
            label_size: usize,
        ) -> i32 {
            todo!("casper_remove_contract_user_group")
        }
        #[no_mangle]
        pub extern "C" fn casper_provision_contract_user_group_uref(
            contract_package_hash_ptr: *const u8,
            contract_package_hash_size: usize,
            label_ptr: *const u8,
            label_size: usize,
            value_size_ptr: *const usize,
        ) -> i32 {
            todo!("casper_provision_contract_user_group_uref")
        }
        #[no_mangle]
        pub extern "C" fn casper_remove_contract_user_group_urefs(
            contract_package_hash_ptr: *const u8,
            contract_package_hash_size: usize,
            label_ptr: *const u8,
            label_size: usize,
            urefs_ptr: *const u8,
            urefs_size: usize,
        ) -> i32 {
            todo!("casper_remove_contract_user_group_urefs")
        }
        #[no_mangle]
        pub extern "C" fn casper_blake2b(
            in_ptr: *const u8,
            in_size: usize,
            out_ptr: *mut u8,
            out_size: usize,
        ) -> i32 {
            todo!("casper_blake2b")
        }
        #[no_mangle]
        pub extern "C" fn casper_load_call_stack(
            call_stack_len_ptr: *mut usize,
            result_size_ptr: *mut usize,
        ) -> i32 {
        }

        fn runtime_new_uref(value: CLValue) -> URef {
            let uref = URef::new(rand::random(), AccessRights::READ_ADD_WRITE);
            let storage = unsafe { STORAGE.get_or_insert_with(|| BTreeMap::default()) };
            storage.insert(Key::from(uref), value);
            uref
        }

        #[cfg(feature = "test-support")]
        #[no_mangle]
        pub extern "C" fn casper_print(text_ptr: *const u8, text_size: usize) {
            todo!("casper_print")
        }

        #[no_mangle]
        pub extern "C" fn casper_new_dictionary(output_size_ptr: *mut usize) -> i32 {
            let uref = runtime_new_uref(CLValue::from_t(()).unwrap())
                .to_bytes()
                .expect("should serialize bytes");
            let mut output_size =
                ptr::NonNull::new(output_size_ptr).expect("should pass non null ptr");
            unsafe { ptr::write(output_size.as_ptr(), uref.len()) }
            0
        }
        #[no_mangle]
        pub extern "C" fn casper_dictionary_get(
            uref_ptr: *const u8,
            uref_size: usize,
            key_bytes_ptr: *const u8,
            key_bytes_size: usize,
            output_size: *mut usize,
        ) -> i32 {
            todo!("casper_dictionary_get")
        }
        #[no_mangle]
        pub extern "C" fn casper_dictionary_put(
            uref_ptr: *const u8,
            uref_size: usize,
            key_ptr: *const u8,
            key_size: usize,
            value_ptr: *const u8,
            value_size: usize,
        ) -> i32 {
            todo!("casper_dictionary_put")
        }
    }
}

#[cfg(test)]
mod tests {
    use casper_types::{runtime_args, RuntimeArgs};

    use super::*;

    fn native_exec(func: impl FnOnce(), args: RuntimeArgs) {
        // TODO: "export_name: &str" and dlsym to invoke exports by name
        unsafe {
            standalone_support::native_ext_ffi::RUNTIME_ARGS = Some(args);
        }

        func();
    }

    #[test]
    fn test_should_deploy() {
        // exec(call();
        let args = runtime_args! {
            NAME_RUNTIME_ARG_NAME => "Name",
            SYMBOL_RUNTIME_ARG_NAME => "Symbol",
            DECIMALS_RUNTIME_ARG_NAME => 100u8,
            TOTAL_SUPPLY_RUNTIME_ARG_NAME => U256::from(1_000_000),
        };
        native_exec(call, args);
    }
}
