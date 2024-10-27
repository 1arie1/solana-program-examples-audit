#![allow(unused_imports)]

// Here Anchor stuff
use {
    anchor_lang::{context::Context, Key},
    anchor_lang::prelude::{Account, Signer},
};

// Here Certora stuff
use {
    cvt_macros::rule,
    nondet::{*},
    calltrace::{*},
    cvt::{*},
};

use crate::{Counter, Increment, IncrementBumps};
use crate::counter_anchor::increment;


macro_rules! nondet_anchor_account {
    ($type: ident) => {{
        let acc: Account<$type> = Account { account: nondet(), info: nondet_ref()};
        acc
    }};
}

macro_rules! nondet_anchor_signer {
    () => {{
        Signer::try_from(nondet_ref()).unwrap()
    }};
}


#[rule]
pub fn rule_count_mono() {
    // -- create a nondeterministic Increment object
    let mut account = Increment {
        counter: nondet_anchor_account!(Counter),
        owner: nondet_anchor_signer!(),
    };

    // -- remember old value of the counter
    let count_old:u64 = account.counter.count;

    // -- create an Anchor context
    let program_id = &crate::id();
    let remaining_accounts = [];
    let ctx = Context::new(&program_id, &mut account, &remaining_accounts, IncrementBumps::default());

    // -- run contract function
    increment(ctx).unwrap();

    // -- extract new value for the counter
    let count_new:u64 = account.counter.count;

    // -- assert that counter increase monotonically
    // -- this assumes that contract call did not revert
    // -- and that arithmetic is checked and, therefore, overflow causes revert
    cvt_assert!(count_new > count_old);


    cvt_vacuity_check!();
}
