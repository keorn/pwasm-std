#![no_std]
#![no_main]

#[macro_use]
extern crate wasm_std;

use wasm_std::{CallArgs, storage};
use wasm_std::hash::{H256, sha3};

#[no_mangle]
pub fn call(descriptor: *mut u8) {
    let mut call_args = unsafe { CallArgs::from_raw(descriptor) };

    let hs: H256 = sha3(b"");
    let k: [u8; 32] = hs.into();
    let v = [0u8; 32];
    storage::write(&k, &v);
    let mut vread = [0u8; 32];
    storage::read(&k, &mut vread);
    let mut vec = vec![0u8; 16384];
    vec[32..64].copy_from_slice(&v);

    *call_args.result_mut() = vec.into_boxed_slice();

    unsafe { call_args.save(descriptor); }
}
