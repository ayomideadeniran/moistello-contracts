#![cfg_attr(not(test), no_std)]
// Deny unused imports at the crate level so stale imports (e.g. a previously
// re-exported `load_round_details` from payout.rs) are caught at compile time
// rather than silently emitting warnings that can be overlooked.
#![deny(unused_imports)]
#[cfg(test)]
#[path = "../../../circle.rs"]
pub mod circle_rs;
mod contract;
mod oracle;
mod payout;
#[cfg(test)]
mod test;
#[cfg(test)]
mod tests;
mod types;
use soroban_sdk::{contract, contractimpl, Address, BytesN, Env};

pub use types::CircleError;

#[contract]
pub struct Circle;
#[contractimpl]
impl Circle {
    pub fn __constructor(
        env: Env,
        admin: Address,
        factory: Address,
        config: types::CircleConfig,
    ) -> Result<(), types::CircleError> {
        contract::init(&env, &admin, &factory, &config)
    }
    pub fn join(env: Env, member: Address) -> Result<(), types::CircleError> {
        contract::join(&env, &member)
    }
    pub fn contribute(
        env: Env,
        member: Address,
        amount: i128,
        round: u32,
    ) -> Result<(), types::CircleError> {
        contract::contribute(&env, &member, amount, round)
    }
    pub fn check_contribution_deadline(env: Env) -> Result<(), types::CircleError> {
        contract::check_contribution_deadline(&env)
    }
    pub fn trigger_payout(env: Env, caller: Address, round: u32) -> Result<(), types::CircleError> {
        contract::trigger_payout(&env, &caller, round)
    }
    pub fn auction_bid(
        env: Env,
        bidder: Address,
        discount_bips: u32,
        round: u32,
    ) -> Result<(), types::CircleError> {
        contract::auction_bid(&env, &bidder, discount_bips, round)
    }
    /// Refunds a capped batch of losing auction deposits. Returns how many
    /// losers remain; call again until the result is 0 (#436).
    pub fn refund_losing_bids(
        env: Env,
        caller: Address,
        round: u32,
        limit: u32,
    ) -> Result<u32, types::CircleError> {
        contract::refund_losing_bids(&env, &caller, round, limit)
    }
    pub fn init_dutch_auction(
        env: Env,
        caller: Address,
        round: u32,
        start_bips: u32,
        floor_bips: u32,
        decay_bips_per_ledger: u32,
        expiry_ledgers: u32,
    ) -> Result<(), types::CircleError> {
        contract::init_dutch_auction(
            &env,
            &caller,
            round,
            start_bips,
            floor_bips,
            decay_bips_per_ledger,
            expiry_ledgers,
        )
    }
    pub fn dutch_auction_price(env: Env, round: u32) -> Result<(u32, bool), types::CircleError> {
        contract::dutch_auction_price(&env, round)
    }
    pub fn dutch_auction_bid(
        env: Env,
        bidder: Address,
        round: u32,
    ) -> Result<u32, types::CircleError> {
        contract::dutch_auction_bid(&env, &bidder, round)
    }
    pub fn vote_payout(
        env: Env,
        voter: Address,
        vote_for: Address,
        round: u32,
    ) -> Result<(), types::CircleError> {
        contract::vote_payout(&env, &voter, &vote_for, round)
    }
    pub fn exit_circle(env: Env, member: Address) -> Result<(), types::CircleError> {
        contract::exit(&env, &member)
    }
    pub fn cancel_circle(env: Env, caller: Address) -> Result<(), types::CircleError> {
        contract::cancel_circle(&env, &caller)
    }
    pub fn cancel(env: Env, caller: Address) -> Result<(), types::CircleError> {
        contract::cancel(&env, &caller)
    }
    pub fn cancel_auction(env: Env, caller: Address) -> Result<(), types::CircleError> {
        contract::cancel_auction(&env, &caller)
    }
    pub fn query_top_contributors(env: Env, n: u32) -> soroban_sdk::Vec<(Address, i128)> {
        contract::query_top_contributors(&env, n)
    }
    pub fn report_late(
        env: Env,
        reporter: Address,
        late_member: Address,
        round: u32,
    ) -> Result<(), types::CircleError> {
        contract::report_late(&env, &reporter, &late_member, round)
    }
    pub fn dispute(
        env: Env,
        member: Address,
        evidence_hash: BytesN<32>,
    ) -> Result<(), types::CircleError> {
        contract::dispute(&env, &member, &evidence_hash)
    }
    pub fn raise_dispute(
        env: Env,
        member: Address,
        evidence_hash: BytesN<32>,
    ) -> Result<(), types::CircleError> {
        contract::raise_dispute(&env, &member, &evidence_hash)
    }
    pub fn resolve_dispute(
        env: Env,
        admin: Address,
        resolution: u32,
    ) -> Result<(), types::CircleError> {
        contract::resolve_dispute(&env, &admin, resolution)
    }
    pub fn get_status(env: Env) -> types::Circle {
        contract::get_status(&env)
    }
    pub fn get_dispute_resolution(env: Env) -> Option<types::DisputeResolutionRecord> {
        contract::get_dispute_resolution(&env)
    }
    pub fn get_members(env: Env) -> soroban_sdk::Vec<types::Member> {
        contract::get_members(&env)
    }
    pub fn get_contributions(
        env: Env,
        member: Address,
        page: u32,
        page_size: u32,
    ) -> soroban_sdk::Vec<types::Contribution> {
        contract::get_contributions(&env, &member, page, page_size)
    }
    pub fn query_round_config(env: Env, round: u32) -> Result<BytesN<32>, types::CircleError> {
        contract::query_round_config(&env, round)
    }
    pub fn get_round_fee_ledger(env: Env, round: u32) -> types::RoundFeeLedger {
        contract::get_round_fee_ledger(&env, round)
    }
    pub fn schedule_payout(
        env: Env,
        caller: Address,
        round: u32,
    ) -> Result<(), types::CircleError> {
        contract::schedule_payout(&env, &caller, round)
    }
    pub fn is_payout_scheduled(env: Env, round: u32) -> bool {
        contract::is_payout_scheduled(&env, round)
    }
