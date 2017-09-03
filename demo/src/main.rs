#![no_std]
#![no_main]

#[macro_use]
extern crate wasm_std;

use wasm_std::{CallArgs, storage};
use wasm_std::logger::debug;
use wasm_std::hash::{H256, sha3};
use wasm_std::tiny_keccak::Keccak;

#[no_mangle]
pub fn call(descriptor: *mut u8) {
    let mut call_args = unsafe { CallArgs::from_raw(descriptor) };
    // empty_keccak();
    // let _ = hash_cmp();
    let hs: H256 = sha3(format!(""));
    debug(&hs.hex());
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

fn hash_cmp() -> bool {
    let hs: H256 = sha3(b"");
    let hs2: H256 = sha3(b"");
    hs == hs2
}


fn empty_keccak() {
    let keccak = Keccak::new_keccak256();
    let mut res: [u8; 32] = [0; 32];
    keccak.finalize(&mut res);

    let expected = vec![
        0xc5, 0xd2, 0x46, 0x01, 0x86, 0xf7, 0x23, 0x3c,
        0x92, 0x7e, 0x7d, 0xb2, 0xdc, 0xc7, 0x03, 0xc0,
        0xe5, 0x00, 0xb6, 0x53, 0xca, 0x82, 0x27, 0x3b,
        0x7b, 0xfa, 0xd8, 0x04, 0x5d, 0x85, 0xa4, 0x70
    ];

    let ref_ex: &[u8] = &expected;
    let fixed: H256 = res.into();
    debug(&fixed.hex());
}
