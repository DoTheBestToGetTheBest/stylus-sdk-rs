// Copyright 2022-2023, Offchain Labs, Inc.
// For licensing, see https://github.com/OffchainLabs/stylus-sdk-rs/blob/stylus/licenses/COPYRIGHT.md

#![allow(unused)]

use crate::trace::{FrameReader, HostioKind::*};
use function_name::named;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::{mem, ptr::copy_nonoverlapping as memcpy};

lazy_static! {
    pub static ref FRAME: Mutex<Option<FrameReader>> = Mutex::new(None);
    pub static ref INK_LEFT: Mutex<u64> = Mutex::new(0);
}

macro_rules! frame {
    ($dec:pat) => {
        let $dec = FRAME
            .lock()
            .as_mut()
            .unwrap()
            .next_hostio(function_name!())
            .kind
        else {
            unreachable!()
        };
    };
}

/// Gets the ETH balance in wei of the account at the given address.
/// The semantics are equivalent to that of the EVM's [`BALANCE`] opcode.
///
/// [`BALANCE`]: https://www.evm.codes/#31
#[named]
#[no_mangle]
pub unsafe extern "C" fn account_balance(address: *const u8, dest: *mut u8) {
    //let hostio = frame!();
    todo!()
}

/// Gets the code hash of the account at the given address. The semantics are equivalent
/// to that of the EVM's [`EXT_CODEHASH`] opcode. Note that the code hash of an account without
/// code will be the empty hash
/// `keccak("") = c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470`.
///
/// [`EXT_CODEHASH`]: https://www.evm.codes/#3F
#[named]
#[no_mangle]
pub unsafe extern "C" fn account_codehash(address: *const u8, dest: *mut u8) {
    todo!()
}

/// Reads a 32-byte value from permanent storage. Stylus's storage format is identical to
/// that of the EVM. This means that, under the hood, this hostio is accessing the 32-byte
/// value stored in the EVM state trie at offset `key`, which will be `0` when not previously
/// set. The semantics, then, are equivalent to that of the EVM's [`SLOAD`] opcode.
///
/// [`SLOAD`]: https://www.evm.codes/#54
#[named]
#[no_mangle]
pub unsafe extern "C" fn storage_load_bytes32(key: *const u8, dest: *mut u8) {
    todo!()
}

/// Stores a 32-byte value to permanent storage. Stylus's storage format is identical to that
/// of the EVM. This means that, under the hood, this hostio is storing a 32-byte value into
/// the EVM state trie at offset `key`. Furthermore, refunds are tabulated exactly as in the
/// EVM. The semantics, then, are equivalent to that of the EVM's [`SSTORE`] opcode.
///
/// [`SSTORE`]: https://www.evm.codes/#55
#[named]
#[no_mangle]
pub unsafe extern "C" fn storage_store_bytes32(key: *const u8, value: *const u8) {
    todo!()
}

/// Gets the basefee of the current block. The semantics are equivalent to that of the EVM's
/// [`BASEFEE`] opcode.
///
/// [`BASEFEE`]: https://www.evm.codes/#48
#[named]
#[no_mangle]
pub unsafe extern "C" fn block_basefee(basefee: *mut u8) {
    todo!()
}

/// Gets the coinbase of the current block, which on Arbitrum chains is the L1 batch poster's
/// address. This differs from Ethereum where the validator including the transaction
/// determines the coinbase.
#[named]
#[no_mangle]
pub unsafe extern "C" fn block_coinbase(coinbase: *mut u8) {
    todo!()
}

/// Gets the gas limit of the current block. The semantics are equivalent to that of the EVM's
/// [`GAS_LIMIT`] opcode. Note that as of the time of this writing, `evm.codes` incorrectly
/// implies that the opcode returns the gas limit of the current transaction.  When in doubt,
/// consult [`The Ethereum Yellow Paper`].
///
/// [`GAS_LIMIT`]: https://www.evm.codes/#45
/// [`The Ethereum Yellow Paper`]: https://ethereum.github.io/yellowpaper/paper.pdf
#[named]
#[no_mangle]
pub unsafe extern "C" fn block_gas_limit() -> u64 {
    todo!()
}

/// Gets a bounded estimate of the L1 block number at which the Sequencer sequenced the
/// transaction. See [`Block Numbers and Time`] for more information on how this value is
/// determined.
///
/// [`Block Numbers and Time`]: https://developer.arbitrum.io/time
#[named]
#[no_mangle]
pub unsafe extern "C" fn block_number() -> u64 {
    todo!()
}

/// Gets a bounded estimate of the Unix timestamp at which the Sequencer sequenced the
/// transaction. See [`Block Numbers and Time`] for more information on how this value is
/// determined.
///
/// [`Block Numbers and Time`]: https://developer.arbitrum.io/time
#[named]
#[no_mangle]
pub unsafe extern "C" fn block_timestamp() -> u64 {
    todo!()
}

/// Gets the unique chain identifier of the Arbitrum chain. The semantics are equivalent to
/// that of the EVM's [`CHAIN_ID`] opcode.
///
/// [`CHAIN_ID`]: https://www.evm.codes/#46
#[named]
#[no_mangle]
pub unsafe extern "C" fn chainid() -> u64 {
    23011913
}

/// Calls the contract at the given address with options for passing value and to limit the
/// amount of gas supplied. The return status indicates whether the call succeeded, and is
/// nonzero on failure.
///
/// In both cases `return_data_len` will store the length of the result, the bytes of which can
/// be read via the `read_return_data` hostio. The bytes are not returned directly so that the
/// programmer can potentially save gas by choosing which subset of the return result they'd
/// like to copy.
///
/// The semantics are equivalent to that of the EVM's [`CALL`] opcode, including callvalue
/// stipends and the 63/64 gas rule. This means that supplying the `u64::MAX` gas can be used
/// to send as much as possible.
///
/// [`CALL`]: https://www.evm.codes/#f1
#[named]
#[no_mangle]
pub unsafe extern "C" fn call_contract(
    contract: *const u8,
    calldata: *const u8,
    calldata_len: usize,
    value: *const u8,
    gas: u64,
    return_data_len: *mut usize,
) -> u8 {
    frame!(CallContract {
        address,
        data,
        gas,
        value,
        outs_len,
        status,
        frame,
    });
    //memcpy(address.as_ptr(), dest, mem::size_of_val(address))
    status
}

/// Gets the address of the current program. The semantics are equivalent to that of the EVM's
/// [`ADDRESS`] opcode.
///
/// [`ADDRESS`]: https://www.evm.codes/#30
#[named]
#[no_mangle]
pub unsafe extern "C" fn contract_address(dest: *mut u8) {
    frame!(ContractAddress { address });
    memcpy(address.as_ptr(), dest, mem::size_of_val(&address))
}

/// Deploys a new contract using the init code provided, which the EVM executes to construct
/// the code of the newly deployed contract. The init code must be written in EVM bytecode, but
/// the code it deploys can be that of a Stylus program. The code returned will be treated as
/// WASM if it begins with the EOF-inspired header `0xEFF000`. Otherwise the code will be
/// interpreted as that of a traditional EVM-style contract. See [`Deploying Stylus Programs`]
/// for more information on writing init code.
///
/// On success, this hostio returns the address of the newly created account whose address is
/// a function of the sender and nonce. On failure the address will be `0`, `return_data_len`
/// will store the length of the revert data, the bytes of which can be read via the
/// `read_return_data` hostio. The semantics are equivalent to that of the EVM's [`CREATE`]
/// opcode, which notably includes the exact address returned.
///
/// [`Deploying Stylus Programs`]: https://developer.arbitrum.io/TODO
/// [`CREATE`]: https://www.evm.codes/#f0
#[named]
#[no_mangle]
pub unsafe extern "C" fn create1(
    code: *const u8,
    code_len: usize,
    endowment: *const u8,
    contract: *mut u8,
    revert_data_len: *mut usize,
) {
    todo!()
}

/// Deploys a new contract using the init code provided, which the EVM executes to construct
/// the code of the newly deployed contract. The init code must be written in EVM bytecode, but
/// the code it deploys can be that of a Stylus program. The code returned will be treated as
/// WASM if it begins with the EOF-inspired header `0xEFF000`. Otherwise the code will be
/// interpreted as that of a traditional EVM-style contract. See [`Deploying Stylus Programs`]
/// for more information on writing init code.
///
/// On success, this hostio returns the address of the newly created account whose address is a
/// function of the sender, salt, and init code. On failure the address will be `0`,
/// `return_data_len` will store the length of the revert data, the bytes of which can be read
/// via the `read_return_data` hostio. The semantics are equivalent to that of the EVM's
/// `[CREATE2`] opcode, which notably includes the exact address returned.
///
/// [`Deploying Stylus Programs`]: https://developer.arbitrum.io/TODO
/// [`CREATE2`]: https://www.evm.codes/#f5
#[named]
#[no_mangle]
pub unsafe extern "C" fn create2(
    code: *const u8,
    code_len: usize,
    endowment: *const u8,
    salt: *const u8,
    contract: *mut u8,
    revert_data_len: *mut usize,
) {
    todo!()
}

/// Delegate calls the contract at the given address, with the option to limit the amount of
/// gas supplied. The return status indicates whether the call succeeded, and is nonzero on
/// failure.
///
/// In both cases `return_data_len` will store the length of the result, the bytes of which
/// can be read via the `read_return_data` hostio. The bytes are not returned directly so that
/// the programmer can potentially save gas by choosing which subset of the return result
/// they'd like to copy.
///
/// The semantics are equivalent to that of the EVM's [`DELEGATE_CALL`] opcode, including the
/// 63/64 gas rule. This means that supplying `u64::MAX` gas can be used to send as much as
/// possible.
///
/// [`DELEGATE_CALL`]: https://www.evm.codes/#F4
#[named]
#[no_mangle]
pub unsafe extern "C" fn delegate_call_contract(
    contract: *const u8,
    calldata: *const u8,
    calldata_len: usize,
    gas: u64,
    return_data_len: *mut usize,
) -> u8 {
    todo!()
}

/// Emits an EVM log with the given number of topics and data, the first bytes of which should
/// be the 32-byte-aligned topic data. The semantics are equivalent to that of the EVM's
/// [`LOG0`], [`LOG1`], [`LOG2`], [`LOG3`], and [`LOG4`] opcodes based on the number of topics
/// specified. Requesting more than `4` topics will induce a revert.
///
/// [`LOG0`]: https://www.evm.codes/#a0
/// [`LOG1`]: https://www.evm.codes/#a1
/// [`LOG2`]: https://www.evm.codes/#a2
/// [`LOG3`]: https://www.evm.codes/#a3
/// [`LOG4`]: https://www.evm.codes/#a4
#[named]
#[no_mangle]
pub unsafe extern "C" fn emit_log(data: *const u8, len: usize, topics: usize) {
    todo!()
}

/// Gets the amount of gas left after paying for the cost of this hostio. The semantics are
/// equivalent to that of the EVM's [`GAS`] opcode.
///
/// [`GAS`]: https://www.evm.codes/#5a
#[named]
#[no_mangle]
pub unsafe extern "C" fn evm_gas_left() -> u64 {
    todo!()
}

/// Gets the amount of ink remaining after paying for the cost of this hostio. The semantics
/// are equivalent to that of the EVM's [`GAS`] opcode, except the units are in ink. See
/// [`Ink and Gas`] for more information on Stylus's compute pricing.
///
/// [`GAS`]: https://www.evm.codes/#5a
/// [`Ink and Gas`]: https://developer.arbitrum.io/TODO
#[named]
#[no_mangle]
pub unsafe extern "C" fn evm_ink_left() -> u64 {
    todo!()
}

/// The `entrypoint!` macro handles importing this hostio, which is required if the
/// program's memory grows. Otherwise compilation through the `ArbWasm` precompile will revert.
/// Internally the Stylus VM forces calls to this hostio whenever new WASM pages are allocated.
/// Calls made voluntarily will unproductively consume gas.
#[named]
#[no_mangle]
pub unsafe extern "C" fn memory_grow(pages: u16) {
    //let hostio = frame!();
    todo!()
}

/// Whether the current call is reentrant.
#[named]
#[no_mangle]
pub unsafe extern "C" fn msg_reentrant() -> bool {
    todo!()
}

/// Gets the address of the account that called the program. For normal L2-to-L2 transactions
/// the semantics are equivalent to that of the EVM's [`CALLER`] opcode, including in cases
/// arising from [`DELEGATE_CALL`].
///
/// For L1-to-L2 retryable ticket transactions, the top-level sender's address will be aliased.
/// See [`Retryable Ticket Address Aliasing`] for more information on how this works.
///
/// [`CALLER`]: https://www.evm.codes/#33
/// [`DELEGATE_CALL`]: https://www.evm.codes/#f4
/// [`Retryable Ticket Address Aliasing`]: https://developer.arbitrum.io/arbos/l1-to-l2-messaging#address-aliasing
#[named]
#[no_mangle]
pub unsafe extern "C" fn msg_sender(sender: *mut u8) {
    todo!()
}

/// Get the ETH value in wei sent to the program. The semantics are equivalent to that of the
/// EVM's [`CALLVALUE`] opcode.
///
/// [`CALLVALUE`]: https://www.evm.codes/#34
#[named]
#[no_mangle]
pub unsafe extern "C" fn msg_value(dest: *mut u8) {
    frame!(MsgValue { value });
    memcpy(value.as_ptr(), dest, mem::size_of_val(&value))
}

/// Efficiently computes the [`keccak256`] hash of the given preimage.
/// The semantics are equivalent to that of the EVM's [`SHA3`] opcode.
///
/// [`keccak256`]: https://en.wikipedia.org/wiki/SHA-3
/// [`SHA3`]: https://www.evm.codes/#20
#[named]
#[no_mangle]
pub unsafe extern "C" fn native_keccak256(bytes: *const u8, len: usize, output: *mut u8) {
    todo!()
}

/// Reads the program calldata. The semantics are equivalent to that of the EVM's
/// [`CALLDATA_COPY`] opcode when requesting the entirety of the current call's calldata.
///
/// [`CALLDATA_COPY`]: https://www.evm.codes/#37
#[named]
#[no_mangle]
pub unsafe extern "C" fn read_args(dest: *mut u8) {
    frame!(ReadArgs { args });
    memcpy(args.as_ptr(), dest, args.len());
}

/// Copies the bytes of the last EVM call or deployment return result. Does not revert if out of
/// bounds, but rather copies the overlapping portion. The semantics are otherwise equivalent
/// to that of the EVM's [`RETURN_DATA_COPY`] opcode.
///
/// [`RETURN_DATA_COPY`]: https://www.evm.codes/#3e
#[named]
#[no_mangle]
pub unsafe extern "C" fn read_return_data(dest: *mut u8, offset: usize, size: usize) -> usize {
    todo!()
}

/// Writes the final return data. If not called before the program exists, the return data will
/// be 0 bytes long. Note that this hostio does not cause the program to exit, which happens
/// naturally when `user_entrypoint` returns.
#[named]
#[no_mangle]
pub unsafe extern "C" fn write_result(data: *const u8, len: usize) {
    frame!(WriteResult { result });
}

/// Returns the length of the last EVM call or deployment return result, or `0` if neither have
/// happened during the program's execution. The semantics are equivalent to that of the EVM's
/// [`RETURN_DATA_SIZE`] opcode.
///
/// [`RETURN_DATA_SIZE`]: https://www.evm.codes/#3d
#[named]
#[no_mangle]
pub unsafe extern "C" fn return_data_size() -> usize {
    todo!()
}

/// Static calls the contract at the given address, with the option to limit the amount of gas
/// supplied. The return status indicates whether the call succeeded, and is nonzero on
/// failure.
///
/// In both cases `return_data_len` will store the length of the result, the bytes of which can
/// be read via the `read_return_data` hostio. The bytes are not returned directly so that the
/// programmer can potentially save gas by choosing which subset of the return result they'd
/// like to copy.
///
/// The semantics are equivalent to that of the EVM's [`STATIC_CALL`] opcode, including the
/// 63/64 gas rule. This means that supplying `u64::MAX` gas can be used to send as much as
/// possible.
///
/// [`STATIC_CALL`]: https://www.evm.codes/#FA
#[named]
#[no_mangle]
pub unsafe extern "C" fn static_call_contract(
    contract: *const u8,
    calldata: *const u8,
    calldata_len: usize,
    gas: u64,
    return_data_len: *mut usize,
) -> u8 {
    todo!()
}

/// Gets the gas price in wei per gas, which on Arbitrum chains equals the basefee. The
/// semantics are equivalent to that of the EVM's [`GAS_PRICE`] opcode.
///
/// [`GAS_PRICE`]: https://www.evm.codes/#3A
#[named]
#[no_mangle]
pub unsafe extern "C" fn tx_gas_price(gas_price: *mut u8) {
    todo!()
}

/// Gets the price of ink in evm gas basis points. See [`Ink and Gas`] for more information on
/// Stylus's compute-pricing model.
///
/// [`Ink and Gas`]: https://developer.arbitrum.io/TODO
#[named]
#[no_mangle]
pub unsafe extern "C" fn tx_ink_price() -> u32 {
    todo!()
}

/// Gets the top-level sender of the transaction. The semantics are equivalent to that of the
/// EVM's [`ORIGIN`] opcode.
///
/// [`ORIGIN`]: https://www.evm.codes/#32
#[named]
#[no_mangle]
pub unsafe extern "C" fn tx_origin(origin: *mut u8) {
    todo!()
}

/// Prints a 32-bit floating point number to the console. Only available in debug mode with
/// floating point enabled.
#[named]
#[no_mangle]
pub unsafe extern "C" fn log_f32(value: f32) {
    todo!()
}

/// Prints a 64-bit floating point number to the console. Only available in debug mode with
/// floating point enabled.
#[named]
#[no_mangle]
pub unsafe extern "C" fn log_f64(value: f64) {
    todo!()
}

/// Prints a 32-bit integer to the console, which can be either signed or unsigned.
/// Only available in debug mode.
#[named]
#[no_mangle]
pub unsafe extern "C" fn log_i32(value: i32) {
    todo!()
}

/// Prints a 64-bit integer to the console, which can be either signed or unsigned.
/// Only available in debug mode.
#[named]
#[no_mangle]
pub unsafe extern "C" fn log_i64(value: i64) {
    todo!()
}

/// Prints a UTF-8 encoded string to the console. Only available in debug mode.
#[named]
#[no_mangle]
pub unsafe extern "C" fn log_txt(text: *const u8, len: usize) {
    todo!()
}