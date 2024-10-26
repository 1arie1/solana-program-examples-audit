use bytemuck::{Pod, Zeroable};
use nondet::{nondet, Nondet};
use spl_pod::primitives::PodU64;
use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(Debug, Clone, Pod, Copy, Zeroable)]
pub struct Counter {
    pub owner: Pubkey,
    pub count: PodU64,
}

impl Nondet for Counter {
    fn nondet() -> Self {
        Self {
            owner : nondet(),
            count: nondet::<u64>().into(),
        }
    }
}
