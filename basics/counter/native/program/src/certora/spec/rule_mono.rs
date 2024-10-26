#![allow(unused_imports)]

use cvt_macros::rule;
use cvt::{cvt_assert, cvt_assume, cvt_vacuity_check};
use nondet::{nondet, acc_infos_with_mem_layout};
use solana_program::account_info::AccountInfo;
use borsh::{BorshSerialize, BorshDeserialize};

#[rule]
pub fn rule_count_mono() {
    let acc_infos: [AccountInfo; 16] = acc_infos_with_mem_layout!();

    let counter_info = &acc_infos[0];

    // -- crate new account state and write it to data
    let counter : crate::state::Counter = nondet();
    counter.serialize(&mut *counter_info.data.borrow_mut()).unwrap();

    // -- store old count value
    let count_old = counter.count;

    // -- run contract function
    let arg = [nondet::<u8>();8];
    crate::process_increment_counter(&acc_infos, &arg).unwrap();

    // -- extract new value of the state
    let count_new = crate::state::Counter::try_from_slice(*counter_info.data.borrow()).unwrap().count;

    // -- assert that counter increase monotonically
    // -- this assumes that contract call did not revewrt
    // -- and that arithmetic is checked and, thereofre, overflow causes revert
    cvt_assert!(count_new >= count_old);


}