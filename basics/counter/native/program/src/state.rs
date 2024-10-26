use borsh::{BorshDeserialize, BorshSerialize};
use nondet::Nondet;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
#[cfg_attr(feature = "certora", derive(Nondet))]
pub struct Counter {
    pub count: u64,
}
