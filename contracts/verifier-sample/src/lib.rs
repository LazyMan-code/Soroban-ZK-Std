#![no_std]
use soroban_sdk::{contract, contractimpl, Env, U256};
use zk_soroban::ZkEnv; // This line is crucial!

#[contract]
pub struct Verifier;

#[contractimpl]
impl Verifier {
    pub fn check(env: Env, input: U256) -> bool {
        // Now this will work because ZkEnv is in scope
        env.is_bn254_scalar(input)
    }
}
