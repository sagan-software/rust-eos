use eosio_cdt_sys::{capi_checksum160, capi_checksum256, capi_checksum512};
use eosio_core::{Checksum160, Checksum256, Checksum512};

#[inline]
pub fn ripemd160(data: &str) -> Checksum160 {
    let data_ptr = data.as_ptr();
    let data_len = data.len() as u32;
    let mut c_hash = capi_checksum160::default();
    let c_hash_ptr: *mut capi_checksum160 =
        &mut c_hash as *mut _ as *mut capi_checksum160;
    unsafe { ::eosio_cdt_sys::ripemd160(data_ptr, data_len, c_hash_ptr) }
    c_hash.hash.into()
}

#[inline]
pub fn assert_ripemd160(checksum: &Checksum160, data: &str) {
    let data_ptr = data.as_ptr();
    let data_len = data.len() as u32;
    let c_hash = capi_checksum160 {
        hash: checksum.to_bytes(),
        __bindgen_padding_0: [0_u32; 3],
    };
    let c_hash_ptr: *const capi_checksum160 =
        &c_hash as *const capi_checksum160;
    unsafe { ::eosio_cdt_sys::assert_ripemd160(data_ptr, data_len, c_hash_ptr) }
}

#[inline]
pub fn sha1(data: &str) -> Checksum160 {
    let data_ptr = data.as_ptr();
    let data_len = data.len() as u32;
    let mut c_hash = capi_checksum160::default();
    let c_hash_ptr: *mut capi_checksum160 =
        &mut c_hash as *mut _ as *mut capi_checksum160;
    unsafe { ::eosio_cdt_sys::sha1(data_ptr, data_len, c_hash_ptr) }
    c_hash.hash.into()
}

#[inline]
pub fn assert_sha1(checksum: &Checksum160, data: &str) {
    let data_ptr = data.as_ptr();
    let data_len = data.len() as u32;
    let c_hash = capi_checksum160 {
        hash: checksum.to_bytes(),
        __bindgen_padding_0: [0_u32; 3],
    };
    let c_hash_ptr: *const capi_checksum160 =
        &c_hash as *const capi_checksum160;
    unsafe { ::eosio_cdt_sys::assert_sha1(data_ptr, data_len, c_hash_ptr) }
}

#[inline]
pub fn sha256(data: &str) -> Checksum256 {
    let data_ptr = data.as_ptr();
    let data_len = data.len() as u32;
    let mut c_hash = capi_checksum256::default();
    let c_hash_ptr: *mut capi_checksum256 =
        &mut c_hash as *mut _ as *mut capi_checksum256;
    unsafe { ::eosio_cdt_sys::sha256(data_ptr, data_len, c_hash_ptr) }
    c_hash.hash.into()
}

#[inline]
pub fn assert_sha256(checksum: &Checksum256, data: &str) {
    let data_ptr = data.as_ptr();
    let data_len = data.len() as u32;
    let c_hash = capi_checksum256 {
        hash: checksum.to_bytes(),
    };
    let c_hash_ptr: *const capi_checksum256 =
        &c_hash as *const capi_checksum256;
    unsafe { ::eosio_cdt_sys::assert_sha256(data_ptr, data_len, c_hash_ptr) }
}

#[inline]
pub fn sha512(data: &str) -> Checksum512 {
    let data_ptr = data.as_ptr();
    let data_len = data.len() as u32;
    let mut c_hash = capi_checksum512::default();
    let c_hash_ptr: *mut capi_checksum512 =
        &mut c_hash as *mut _ as *mut capi_checksum512;
    unsafe { ::eosio_cdt_sys::sha512(data_ptr, data_len, c_hash_ptr) }
    c_hash.hash.into()
}

#[inline]
pub fn assert_sha512(checksum: &Checksum512, data: &str) {
    let data_ptr = data.as_ptr();
    let data_len = data.len() as u32;
    let c_hash = capi_checksum512 {
        hash: checksum.to_bytes(),
    };
    let c_hash_ptr: *const capi_checksum512 =
        &c_hash as *const capi_checksum512;
    unsafe { ::eosio_cdt_sys::assert_sha512(data_ptr, data_len, c_hash_ptr) }
}
