#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(alloc)]

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate alloc;
extern crate memchr;

mod bindings;
pub mod cstr;
pub mod ctypes;

pub mod action {
    pub use super::bindings::{
        action_data_size, current_receiver, has_auth, is_account, publication_time,
        read_action_data, require_auth, require_auth2, require_read_lock, require_recipient,
        require_write_lock, send_context_free_inline, send_inline,
    };
}

pub mod chain {
    pub use super::bindings::get_active_producers;
}

pub mod crypto {
    pub use super::bindings::{
        assert_recover_key, assert_ripemd160, assert_sha1, assert_sha256, assert_sha512,
        recover_key, ripemd160, sha1, sha256, sha512,
    };
}

pub mod db {
    pub use super::bindings::{
        db_end_i64, db_find_i64, db_get_i64, db_idx128_end, db_idx128_find_primary,
        db_idx128_find_secondary, db_idx128_lowerbound, db_idx128_next, db_idx128_previous,
        db_idx128_remove, db_idx128_store, db_idx128_update, db_idx128_upperbound, db_idx256_end,
        db_idx256_find_primary, db_idx256_find_secondary, db_idx256_lowerbound, db_idx256_next,
        db_idx256_previous, db_idx256_remove, db_idx256_store, db_idx256_update,
        db_idx256_upperbound, db_idx64_end, db_idx64_find_primary, db_idx64_find_secondary,
        db_idx64_lowerbound, db_idx64_next, db_idx64_previous, db_idx64_remove, db_idx64_store,
        db_idx64_update, db_idx64_upperbound, db_idx_double_end, db_idx_double_find_primary,
        db_idx_double_find_secondary, db_idx_double_lowerbound, db_idx_double_next,
        db_idx_double_previous, db_idx_double_remove, db_idx_double_store, db_idx_double_update,
        db_idx_double_upperbound, db_idx_long_double_end, db_idx_long_double_find_primary,
        db_idx_long_double_find_secondary, db_idx_long_double_lowerbound, db_idx_long_double_next,
        db_idx_long_double_previous, db_idx_long_double_remove, db_idx_long_double_store,
        db_idx_long_double_update, db_idx_long_double_upperbound, db_lowerbound_i64, db_next_i64,
        db_previous_i64, db_remove_i64, db_store_i64, db_update_i64, db_upperbound_i64,
    };
}

pub mod permission {
    pub use super::bindings::{
        check_permission_authorization, check_transaction_authorization, get_account_creation_time,
        get_permission_last_used,
    };
}

pub mod print {
    pub use super::bindings::{
        printdf, printhex, printi, printi128, printn, printqf, prints, prints_l, printsf, printui,
        printui128,
    };
}

pub mod privileged {
    pub use super::bindings::{
        activate_feature, get_blockchain_parameters_packed, get_resource_limits, is_privileged,
        set_active_producers, set_blockchain_parameters_packed, set_privileged,
        set_proposed_producers, set_resource_limits,
    };
}

pub mod system {
    pub use super::bindings::{
        current_time, eosio_assert, eosio_assert_code, eosio_assert_message, eosio_exit,
    };
}

pub mod transaction {
    pub use super::bindings::{
        cancel_deferred, expiration, get_action, get_context_free_data, read_transaction,
        send_deferred, tapos_block_num, tapos_block_prefix, transaction_size,
    };
}

pub mod types {
    pub use super::bindings::{
        account_name, account_permission, action_name, block_id_type, checksum160, checksum256,
        checksum512, permission_name, public_key, scope_name, signature, table_name, time,
        transaction_id_type, weight_type,
    };
}

pub mod prelude {
    pub use super::action::*;
    pub use super::chain::*;
    pub use super::crypto::*;
    pub use super::cstr::*;
    pub use super::ctypes::*;
    pub use super::db::*;
    pub use super::permission::*;
    pub use super::print::*;
    pub use super::privileged::*;
    pub use super::system::*;
    pub use super::transaction::*;
    pub use super::types::*;
}
