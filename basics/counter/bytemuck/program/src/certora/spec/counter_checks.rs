#![allow(unused_imports)]

use cvt::{cvt_assert, cvt_assume, cvt_vacuity_check};
use cvt_macros::rule;
use nondet::{acc_infos_with_mem_layout, nondet};
use solana_program::account_info::AccountInfo;

#[rule]
pub fn rule_count_mono() {
    let acc_infos: [AccountInfo; 16] = acc_infos_with_mem_layout!();

    let counter_info = &acc_infos[0];
    let authority_info = &acc_infos[1];

    let count_old: u64 = {
        // -- access Counter, it is nondet at this point
        let data = counter_info.data.borrow();
        let counter: &crate::state::Counter = bytemuck::from_bytes(&data[..]);
        // -- store old count value
        counter.count.into()
    };

    // -- run contract function
    let arg = [nondet::<u8>(); 8];
    crate::process_increment_counter(&acc_infos, &arg).unwrap();

    // -- extract new value of the state
    let counter_new = {
        let data = counter_info.data.borrow();
        let counter: &crate::state::Counter = bytemuck::from_bytes(&data[..]);
        *counter
    };
    let count_new : u64 = counter_new.count.into();

    // -- assert that counter increase monotonically
    // -- this assumes that contract call did not revewrt
    // -- and that arithmetic is checked and, thereofre, overflow causes revert
    cvt_assert!(count_new > count_old);

    cvt_assert!(counter_info.owner == &crate::id());
    cvt_assert!(authority_info.key == &counter_new.owner);
    cvt_assert!(authority_info.is_signer);

    cvt_vacuity_check!();
}
